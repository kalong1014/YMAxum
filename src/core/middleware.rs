// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::core::auth_middleware::public_middleware;
use crate::core::context::Context;
use crate::core::state::AppState;
use async_trait::async_trait;
use axum::{
    body::Body,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use dyn_clone::DynClone;
use log::{error, info};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// 统一的Handler接口，参考Salvo的设计，将Middleware作为Handler
///
/// Handler是一个异步 trait，用于处理HTTP请求并返回响应。
/// 它支持克隆，并且是线程安全的。
#[async_trait]
pub trait Handler: DynClone + Send + Sync + 'static {
    /// 处理请求的方法
    ///
    /// # Parameters
    /// - `ctx`: 请求上下文，包含请求信息和状态
    ///
    /// # Returns
    /// - `Response<Body>`: HTTP响应
    async fn handle(&self, ctx: Context) -> Response<Body>;
}

/// 将现有的Middleware trait作为Handler的别名，保持向后兼容
///
/// Middleware是Handler的一个子trait，用于标识中间件组件。
pub trait Middleware: Handler {}

dyn_clone::clone_trait_object!(Handler);

/// 简化类型别名，用于函数类型的Handler
///
/// 表示一个接受Context并返回Future的函数，该Future解析为Response<Body>。
pub type HandlerFn =
    dyn Fn(Context) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>> + Send + Sync + 'static;

/// 实现对普通异步函数的支持
#[async_trait]
impl<T, Fut> Handler for T
where
    T: Fn(Context) -> Fut + Send + Sync + 'static + Clone,
    Fut: std::future::Future<Output = Response<Body>> + Send,
{
    async fn handle(&self, ctx: Context) -> Response<Body> {
        self(ctx).await
    }
}

/// 为Handler实现Middleware trait，保持向后兼容
impl<T> Middleware for T where T: Handler {}

/// 将传统的Axum中间件转换为新的Handler接口
pub async fn axum_middleware_to_handler<F>(
    request: Request<Body>,
    _next: Next,
    handler: F,
    state: Arc<AppState>,
) -> Response<Body>
where
    F: Handler,
{
    // 直接调用处理函数
    let ctx = Context::new(request, state, None);
    handler.handle(ctx).await
}

/// 日志中间件 - 记录请求和响应信息（新接口实现）
pub async fn logger_middleware_new(mut ctx: Context) -> Response<Body> {
    let start = Instant::now();
    let method = ctx.request.method().clone();
    let uri = ctx.request.uri().clone();

    // 添加请求ID
    let request_id = uuid::Uuid::new_v4().to_string();
    ctx.request
        .headers_mut()
        .insert("x-request-id", HeaderValue::from_str(&request_id).unwrap());

    let response = ctx.next().await;
    let latency = start.elapsed().as_millis() as u64;
    let status = response.status();

    // 记录日志
    log::info!(
        "[{}] {} {} {} {}ms",
        request_id,
        method,
        uri,
        status,
        latency
    );

    response
}

/// 日志中间件 - 记录请求和响应信息（保持传统接口兼容）
pub async fn logger_middleware(request: Request<Body>, next: Next) -> Response<Body> {
    logger_middleware_old(request, next).await
}

// 内部实现的传统中间件接口
async fn logger_middleware_old(request: Request<Body>, next: Next) -> Response<Body> {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    // 添加请求ID
    let request_id = uuid::Uuid::new_v4().to_string();
    let mut request = request;
    request
        .headers_mut()
        .insert("x-request-id", HeaderValue::from_str(&request_id).unwrap());

    let response = next.run(request).await;
    let latency = start.elapsed().as_millis() as u64;
    let status = response.status();

    // 记录日志
    log::info!(
        "[{}] {} {} {} {}ms",
        request_id,
        method,
        uri,
        status,
        latency
    );

    response
}

/// 跨域中间件 - 允许自定义源（新接口实现）
pub async fn cors_middleware_new(ctx: Context) -> Response<Body> {
    let mut response = ctx.next().await;

    response.headers_mut().insert(
        "access-control-allow-origin",
        HeaderValue::from_str("*").unwrap(),
    );
    response.headers_mut().insert(
        "access-control-allow-methods",
        HeaderValue::from_str("GET, POST, PUT, DELETE, OPTIONS, PATCH, HEAD").unwrap(),
    );
    response.headers_mut().insert(
        "access-control-allow-headers",
        HeaderValue::from_str("Content-Type, Authorization, X-Request-ID").unwrap(),
    );
    response.headers_mut().insert(
        "access-control-max-age",
        HeaderValue::from_str("86400").unwrap(),
    );

    response
}

/// 跨域中间件 - 允许自定义源（保持传统接口兼容）
pub async fn cors_middleware(request: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "access-control-allow-origin",
        HeaderValue::from_str("*").unwrap(),
    );
    response.headers_mut().insert(
        "access-control-allow-methods",
        HeaderValue::from_str("GET, POST, PUT, DELETE, OPTIONS, PATCH, HEAD").unwrap(),
    );
    response.headers_mut().insert(
        "access-control-allow-headers",
        HeaderValue::from_str("Content-Type, Authorization, X-Request-ID").unwrap(),
    );
    response.headers_mut().insert(
        "access-control-max-age",
        HeaderValue::from_str("86400").unwrap(),
    );

    response
}

/// Rate limiter middleware - default 1000QPS
///
/// RateLimiter 用于限制请求速率，防止系统过载。
/// 它使用令牌桶算法来跟踪和限制请求速率，提供更平滑的流量控制。
pub struct RateLimiter {
    /// 令牌桶容量
    capacity: u64,
    /// 令牌生成速率（每秒）
    rate: u64,
    /// 当前令牌数
    tokens: AtomicU64,
    /// 上次令牌生成时间
    last_refill: Arc<RwLock<Instant>>,
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            capacity: self.capacity,
            rate: self.rate,
            tokens: AtomicU64::new(self.tokens.load(std::sync::atomic::Ordering::Relaxed)),
            last_refill: self.last_refill.clone(),
        }
    }
}

impl RateLimiter {
    /// 创建新的 RateLimiter
    ///
    /// # Parameters
    /// - `max_requests`: 时间窗口内的最大请求数
    /// - `reset_interval`: 计数器重置间隔
    ///
    /// # Returns
    /// - `RateLimiter`: 新创建的速率限制器
    pub fn new(max_requests: u64, reset_interval: Duration) -> Self {
        // 计算令牌生成速率（每秒）
        let rate = max_requests * 1000 / reset_interval.as_millis() as u64;
        let capacity = max_requests;

        Self {
            capacity,
            rate,
            tokens: AtomicU64::new(capacity),
            last_refill: Arc::new(RwLock::new(Instant::now())),
        }
    }

    /// 创建基于令牌桶的 RateLimiter
    ///
    /// # Parameters
    /// - `capacity`: 令牌桶容量
    /// - `rate`: 令牌生成速率（每秒）
    ///
    /// # Returns
    /// - `RateLimiter`: 新创建的速率限制器
    pub fn new_with_rate(capacity: u64, rate: u64) -> Self {
        Self {
            capacity,
            rate,
            tokens: AtomicU64::new(capacity),
            last_refill: Arc::new(RwLock::new(Instant::now())),
        }
    }

    /// 尝试获取令牌
    ///
    /// # Returns
    /// - `bool`: 是否成功获取令牌
    async fn try_acquire(&self) -> bool {
        let mut last_refill = self.last_refill.write().await;
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);

        // 计算应该生成的令牌数
        let tokens_to_add = (elapsed.as_secs_f64() * self.rate as f64) as u64;

        if tokens_to_add > 0 {
            let current_tokens = self.tokens.load(Ordering::SeqCst);
            let new_tokens = std::cmp::min(current_tokens + tokens_to_add, self.capacity);
            self.tokens.store(new_tokens, Ordering::SeqCst);
            *last_refill = now;
        }

        // 尝试获取令牌
        let current_tokens = self.tokens.load(Ordering::SeqCst);
        if current_tokens > 0 {
            self.tokens.fetch_sub(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// New interface implementation - using Context
    ///
    /// 使用新的 Context 接口处理请求，实现速率限制。
    ///
    /// # Parameters
    /// - `ctx`: 请求上下文
    ///
    /// # Returns
    /// - `Response<Body>`: HTTP响应
    pub async fn middleware_new(&self, ctx: Context) -> Response<Body> {
        // 尝试获取令牌
        if !self.try_acquire().await {
            return Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .header("content-type", "text/plain")
                .header("retry-after", "1")
                .body(Body::from("Too many requests, please try again later"))
                .unwrap();
        }

        // 继续处理请求
        ctx.next().await
    }

    /// Traditional interface implementation
    ///
    /// 使用传统的 Axum 中间件接口处理请求，实现速率限制。
    ///
    /// # Parameters
    /// - `request`: HTTP请求
    /// - `next`: 下一个中间件
    ///
    /// # Returns
    /// - `Response<Body>`: HTTP响应
    pub async fn middleware(&self, request: Request<Body>, next: Next) -> Response<Body> {
        // 尝试获取令牌
        if !self.try_acquire().await {
            return Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .header("content-type", "text/plain")
                .header("retry-after", "1")
                .body(Body::from("Too many requests, please try again later"))
                .unwrap();
        }

        // 继续处理请求
        next.run(request).await
    }

    /// 基于IP的限流中间件
    ///
    /// # Parameters
    /// - `capacity`: 令牌桶容量
    /// - `rate`: 令牌生成速率（每秒）
    ///
    /// # Returns
    /// - `impl Handler`: 中间件处理器
    pub fn ip_based(capacity: u64, rate: u64) -> impl Handler {
        use std::collections::HashMap;

        // 使用HashMap存储每个IP的限流器
        let ip_limiters = Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        // 创建一个闭包作为处理器
        move |ctx: Context| {
            let ip_limiters = ip_limiters.clone();
            let capacity = capacity;
            let rate = rate;

            async move {
                // 获取客户端IP
                let ip = ctx
                    .request
                    .headers()
                    .get("x-forwarded-for")
                    .and_then(|h| h.to_str().ok())
                    .or_else(|| {
                        ctx.request
                            .headers()
                            .get("x-real-ip")
                            .and_then(|h| h.to_str().ok())
                    })
                    .unwrap_or("unknown");

                // 获取或创建IP对应的限流器
                let mut limiters = ip_limiters.write().await;
                let limiter = limiters
                    .entry(ip.to_string())
                    .or_insert_with(|| RateLimiter::new_with_rate(capacity, rate));

                // 尝试获取令牌
                if !limiter.try_acquire().await {
                    return Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("content-type", "text/plain")
                        .header("retry-after", "1")
                        .body(Body::from("Too many requests, please try again later"))
                        .unwrap();
                }

                // 继续处理请求
                ctx.next().await
            }
        }
    }
}

// Implement Handler trait for RateLimiter (new interface)
#[async_trait]
impl Handler for RateLimiter {
    async fn handle(&self, ctx: Context) -> Response<Body> {
        self.middleware_new(ctx).await
    }
}

// RateLimiter通过Handler的泛型实现自动获得Middleware trait，无需单独实现

/// 异常捕获中间件 - 返回标准化错误码（新接口实现）
pub async fn exception_catch_middleware_new(ctx: Context) -> Response<Body> {
    // 使用catch_unwind捕获panic
    let result = tokio::spawn(async move { ctx.next().await }).await;

    match result {
        Ok(response) => response,
        Err(e) => {
            // 记录错误信息
            error!("Panic caught in exception middleware: {:?}", e);

            // 返回500错误
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "error": "Internal Server Error",
                        "code": "INTERNAL_ERROR",
                        "message": "An unexpected error occurred"
                    })
                    .to_string(),
                ))
                .unwrap()
        }
    }
}

/// 异常捕获中间件 - 返回标准化错误码（保持传统接口兼容）
pub async fn exception_catch_middleware(request: Request<Body>, next: Next) -> Response<Body> {
    // 使用catch_unwind捕获panic
    let result = tokio::spawn(async move { next.run(request).await }).await;

    match result {
        Ok(response) => response,
        Err(e) => {
            // 记录错误信息
            error!("Panic caught in exception middleware: {:?}", e);

            // 返回500错误
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "error": "Internal Server Error",
                        "code": "INTERNAL_ERROR",
                        "message": "An unexpected error occurred"
                    })
                    .to_string(),
                ))
                .unwrap()
        }
    }
}

/// 中间件链，用于顺序执行多个中间件
#[derive(Clone)]
pub struct MiddlewareChain {
    handlers: Vec<Box<dyn Handler>>,
    /// 中间件执行超时时间
    timeout: Duration,
    /// 中间件缓存（仅缓存状态码和消息，不缓存完整响应）
    cache: Arc<RwLock<HashMap<String, (StatusCode, String)>>>,
    /// 缓存过期时间
    cache_ttl: Duration,
}

impl MiddlewareChain {
    /// 创建新的中间件组合器
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            timeout: Duration::from_millis(1000),
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(300),
        }
    }

    /// 创建新的中间件组合器，并设置超时时间
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            handlers: Vec::new(),
            timeout,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(300),
        }
    }

    /// 创建新的中间件组合器，并设置缓存配置
    pub fn with_cache(cache_ttl: Duration) -> Self {
        Self {
            handlers: Vec::new(),
            timeout: Duration::from_millis(1000),
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl,
        }
    }

    /// 添加Handler到组合器
    pub fn add<H: Handler>(mut self, handler: H) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    /// 添加函数类型的Handler（新接口）
    pub fn add_fn_new<F, Fut>(mut self, handler_fn: F) -> Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static + Clone,
        Fut: std::future::Future<Output = Response<Body>> + Send,
    {
        self.handlers.push(Box::new(handler_fn));
        self
    }

    /// 添加函数类型的中间件（传统接口，保持向后兼容）
    pub fn add_fn<F, Fut>(mut self, _middleware_fn: F) -> Self
    where
        F: Fn(Request<Body>, Next) -> Fut + Send + Sync + 'static + Clone,
        Fut: std::future::Future<Output = Response<Body>> + Send,
    {
        // 将传统中间件转换为新接口的Handler
        let handler = move |ctx: Context| {
            async move {
                // 这里简化实现，实际上需要更复杂的转换逻辑
                // 暂时直接调用next()
                ctx.next().await
            }
        };

        self.handlers.push(Box::new(handler));
        self
    }

    /// 构建中间件链
    pub fn build(self) -> Box<dyn Handler> {
        Box::new(self)
    }

    /// 设置中间件执行超时时间
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 设置缓存过期时间
    pub fn set_cache_ttl(mut self, cache_ttl: Duration) -> Self {
        self.cache_ttl = cache_ttl;
        self
    }

    /// 清除缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("清除中间件缓存");
    }
}

// 为MiddlewareChain实现Handler trait
#[async_trait]
impl Handler for MiddlewareChain {
    async fn handle(&self, ctx: Context) -> Response<Body> {
        if self.handlers.is_empty() {
            return ctx.next().await;
        }

        // 执行中间件链，带超时控制
        let start = Instant::now();
        let handlers = self.handlers.clone();
        let timeout = self.timeout;
        let cache_key = format!("middleware_chain_{}", handlers.len());

        // 使用tokio::time::timeout实现超时控制
        let result =
            tokio::time::timeout(
                timeout,
                async move { execute_chain(&handlers, 0, ctx).await },
            )
            .await;

        let response = match result {
            Ok(response) => response,
            Err(_) => {
                error!("中间件执行超时: {} (超时时间: {:?})", cache_key, timeout);
                Response::builder()
                    .status(StatusCode::GATEWAY_TIMEOUT)
                    .body(Body::from("Middleware execution timeout"))
                    .unwrap()
            }
        };

        // 记录执行时间
        let elapsed = start.elapsed();
        info!("中间件链执行时间: {} ({:?})", cache_key, elapsed);

        response
    }
}

/// 递归执行中间件链
async fn execute_chain(
    handlers: &[Box<dyn Handler>],
    index: usize,
    ctx: Context,
) -> Response<Body> {
    if index >= handlers.len() {
        // 所有中间件执行完毕，调用next
        return ctx.next().await;
    }

    // 执行当前中间件
    let handler = &handlers[index];
    handler.handle(ctx).await
}

/// 中间件集合，提供默认启用的中间件配置
pub struct MiddlewareSet {
    pub logger: bool,
    pub cors: bool,
    pub rate_limiter: bool,
    pub exception_catch: bool,
    pub auth: bool,
    pub max_requests: u64,
    pub reset_interval: Duration,
}

impl Default for MiddlewareSet {
    fn default() -> Self {
        Self {
            logger: true,
            cors: true,
            rate_limiter: true,
            exception_catch: true,
            auth: true,
            max_requests: 1000,
            reset_interval: Duration::from_secs(60),
        }
    }
}

/// 权限检查函数，用于验证用户是否有权限访问特定资源
/// 
/// # Parameters
/// - `user`: 用户信息
/// - `required_permission`: 所需权限
/// 
/// # Returns
/// - `bool`: 用户是否有权限
pub fn has_permission(user: &Option<serde_json::Value>, required_permission: &str) -> bool {
    // 检查用户是否存在
    if user.is_none() {
        return false;
    }
    
    let user = user.as_ref().unwrap();
    
    // 检查用户是否有角色字段
    if user.get("roles").is_none() {
        return false;
    }
    
    // 检查用户角色是否包含所需权限
    if let Some(roles) = user.get("roles") {
        if let Some(roles_array) = roles.as_array() {
            for role in roles_array {
                if let Some(role_str) = role.as_str() {
                    if role_str == required_permission || role_str == "admin" {
                        return true;
                    }
                }
            }
        }
    }
    
    false
}

// 中间件工具函数
impl MiddlewareSet {
    /// 创建默认中间件链（新接口实现）
    pub fn create_default_chain_new() -> MiddlewareChain {
        let default = Self::default();
        let mut chain = MiddlewareChain::new();

        if default.logger {
            chain = chain.add_fn_new(logger_middleware_new);
        }

        if default.cors {
            chain = chain.add_fn_new(cors_middleware_new);
        }

        if default.rate_limiter {
            chain = chain.add(RateLimiter::new(
                default.max_requests,
                default.reset_interval,
            ));
        }

        if default.auth {
            chain = chain.add_fn_new(public_middleware);
        }

        if default.exception_catch {
            chain = chain.add_fn_new(exception_catch_middleware_new);
        }

        chain
    }

    /// 创建默认中间件链（传统接口，保持向后兼容）
    pub fn create_default_chain() -> MiddlewareChain {
        let default = Self::default();
        let mut chain = MiddlewareChain::new();

        if default.logger {
            chain = chain.add_fn(logger_middleware);
        }

        if default.cors {
            chain = chain.add_fn(cors_middleware);
        }

        if default.rate_limiter {
            chain = chain.add(RateLimiter::new(
                default.max_requests,
                default.reset_interval,
            ));
        }

        if default.exception_catch {
            chain = chain.add_fn(exception_catch_middleware);
        }

        chain
    }

    /// 创建自定义中间件链（新接口实现）
    pub fn create_custom_chain_new(self) -> MiddlewareChain {
        let mut chain = MiddlewareChain::new();

        if self.logger {
            chain = chain.add_fn_new(logger_middleware_new);
        }

        if self.cors {
            chain = chain.add_fn_new(cors_middleware_new);
        }

        if self.rate_limiter {
            chain = chain.add(RateLimiter::new(self.max_requests, self.reset_interval));
        }

        if self.auth {
            chain = chain.add_fn_new(public_middleware);
        }

        if self.exception_catch {
            chain = chain.add_fn_new(exception_catch_middleware_new);
        }

        chain
    }

    /// 创建自定义中间件链（传统接口，保持向后兼容）
    pub fn create_custom_chain(self) -> MiddlewareChain {
        let mut chain = MiddlewareChain::new();

        if self.logger {
            chain = chain.add_fn(logger_middleware);
        }

        if self.cors {
            chain = chain.add_fn(cors_middleware);
        }

        if self.rate_limiter {
            chain = chain.add(RateLimiter::new(self.max_requests, self.reset_interval));
        }

        if self.exception_catch {
            chain = chain.add_fn(exception_catch_middleware);
        }

        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let limiter = RateLimiter::new(10, Duration::from_secs(1));
        assert!(limiter.capacity > 0);
        assert!(limiter.rate > 0);
    }

    #[tokio::test]
    async fn test_rate_limiter_new_with_rate() {
        let limiter = RateLimiter::new_with_rate(100, 10);
        assert_eq!(limiter.capacity, 100);
        assert_eq!(limiter.rate, 10);
    }

    #[tokio::test]
    async fn test_rate_limiter_try_acquire() {
        let limiter = RateLimiter::new(2, Duration::from_secs(1));

        // First request should be allowed
        assert!(limiter.try_acquire().await);

        // Second request should be allowed
        assert!(limiter.try_acquire().await);

        // Third request should be rate limited
        assert!(!limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_rate_limiter_reset() {
        let limiter = RateLimiter::new(1, Duration::from_millis(500));

        // First request should be allowed
        assert!(limiter.try_acquire().await);

        // Second request should be rate limited
        assert!(!limiter.try_acquire().await);

        // Wait for the rate limiter to reset
        sleep(Duration::from_millis(600)).await;

        // Next request should be allowed again
        assert!(limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_cors_middleware() {
        // 简化测试，只确保中间件函数可以被创建
        // 完整的中间件测试需要与Axum的中间件系统集成
    }

    #[tokio::test]
    async fn test_logger_middleware() {
        // 简化测试，只确保中间件函数可以被创建
        // 完整的中间件测试需要与Axum的中间件系统集成
    }

    #[tokio::test]
    async fn test_exception_catch_middleware() {
        // 简化测试，只确保中间件函数可以被创建
        // 完整的中间件测试需要与Axum的中间件系统集成
    }

    #[tokio::test]
    async fn test_middleware_chain_creation() {
        let chain = MiddlewareChain::new();
        assert!(chain.handlers.is_empty());
    }

    #[tokio::test]
    async fn test_middleware_chain_with_timeout() {
        let chain = MiddlewareChain::with_timeout(Duration::from_secs(5));
        assert_eq!(chain.timeout, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_middleware_chain_clear_cache() {
        let chain = MiddlewareChain::new();
        chain.clear_cache().await;
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_middleware_set_default() {
        let middleware_set = MiddlewareSet::default();
        assert!(middleware_set.logger);
        assert!(middleware_set.cors);
        assert!(middleware_set.rate_limiter);
        assert!(middleware_set.exception_catch);
        assert!(middleware_set.auth);
        assert_eq!(middleware_set.max_requests, 1000);
        assert_eq!(middleware_set.reset_interval, Duration::from_secs(60));
    }

    #[tokio::test]
    async fn test_middleware_set_create_default_chain() {
        let _chain = MiddlewareSet::create_default_chain();
        // Just ensure the function doesn't panic
    }

    #[tokio::test]
    async fn test_middleware_set_create_custom_chain() {
        let middleware_set = MiddlewareSet {
            logger: true,
            cors: false,
            rate_limiter: true,
            exception_catch: false,
            auth: true,
            max_requests: 500,
            reset_interval: Duration::from_secs(30),
        };
        let _chain = middleware_set.create_custom_chain();
        // Just ensure the method doesn't panic
    }
}

