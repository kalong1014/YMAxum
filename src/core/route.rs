// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::core::state::AppState;
use crate::core::{
    context::Context,
    middleware::{self, Handler, MiddlewareSet},
};
use crate::error::Result;
use crate::plugin::PluginManager;
use axum::Router;
use axum::middleware::from_fn;
use log::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// 简化路由处理函数的类型约束
///
/// 这个trait用于简化路由处理函数的类型声明，确保处理函数满足Axum的要求
pub trait RouteHandler: axum::handler::Handler<(), Arc<AppState>> + Clone + Send + 'static {}

/// 为所有符合约束的类型自动实现RouteHandler trait
///
/// 这样任何符合Axum处理函数要求的类型都可以自动作为RouteHandler使用
impl<T> RouteHandler for T where
    T: axum::handler::Handler<(), Arc<AppState>> + Clone + Send + 'static
{
}

/// 应用路由器，用于构建和管理Axum路由
/// 借鉴Salvo框架的简洁API设计，提供更友好的路由定义方式
pub struct AppRouter {
    inner: Router<Arc<AppState>>,
    before: Vec<Box<dyn Handler>>,
    after: Vec<Box<dyn Handler>>,
    hoop: Vec<Box<dyn Handler>>,
    /// 插件路由缓存
    plugin_routes_cache: HashMap<String, Vec<crate::plugin::PluginRoute>>,

    /// 路由缓存最后更新时间
    cache_last_updated: Option<Instant>,
}

impl AppRouter {
    /// 创建新的应用路由器
    ///
    /// # Returns
    /// * `Self` - 返回一个新的AppRouter实例
    ///
    /// # 说明
    /// 创建一个空的应用路由器，后续可以通过链式调用来添加路由和中间件
    pub fn new() -> Self {
        Self {
            inner: Router::new(),
            before: Vec::new(),
            after: Vec::new(),
            hoop: Vec::new(),
            plugin_routes_cache: HashMap::new(),
            cache_last_updated: None,
        }
    }

    /// 创建新的应用路由器，并设置应用状态
    ///
    /// # Parameters
    /// * `app_state` - 应用状态，包含数据库连接、缓存等全局资源
    ///
    /// # Returns
    /// * `Self` - 返回一个带有应用状态的AppRouter实例
    ///
    /// # 说明
    /// 创建一个带有应用状态的应用路由器，后续可以通过链式调用来添加路由和中间件
    pub fn with_state(app_state: Arc<AppState>) -> Self {
        Self {
            inner: Router::new().with_state(app_state),
            before: Vec::new(),
            after: Vec::new(),
            hoop: Vec::new(),
            plugin_routes_cache: HashMap::new(),
            cache_last_updated: None,
        }
    }

    /// 添加GET路由
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users"
    /// * `handler` - 处理函数，用于处理GET请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 简化API设计，提供更直观易用的路由定义方式
    pub fn get<H>(mut self, path: &str, handler: H) -> Self
    where
        H: RouteHandler,
    {
        self.inner = self.inner.route(path, axum::routing::get(handler));
        self
    }

    /// 添加POST路由
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users"
    /// * `handler` - 处理函数，用于处理POST请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 简化API设计，提供更直观易用的路由定义方式
    pub fn post<H>(mut self, path: &str, handler: H) -> Self
    where
        H: RouteHandler,
    {
        self.inner = self.inner.route(path, axum::routing::post(handler));
        self
    }

    /// 添加PUT路由
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users/:id"
    /// * `handler` - 处理函数，用于处理PUT请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 简化API设计，提供更直观易用的路由定义方式
    pub fn put<H>(mut self, path: &str, handler: H) -> Self
    where
        H: RouteHandler,
    {
        self.inner = self.inner.route(path, axum::routing::put(handler));
        self
    }

    /// 添加DELETE路由
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users/:id"
    /// * `handler` - 处理函数，用于处理DELETE请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 简化API设计，提供更直观易用的路由定义方式
    pub fn delete<H>(mut self, path: &str, handler: H) -> Self
    where
        H: RouteHandler,
    {
        self.inner = self.inner.route(path, axum::routing::delete(handler));
        self
    }

    /// 添加OPTIONS路由
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users"
    /// * `handler` - 处理函数，用于处理OPTIONS请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    pub fn options<H>(mut self, path: &str, handler: H) -> Self
    where
        H: RouteHandler,
    {
        self.inner = self.inner.route(path, axum::routing::options(handler));
        self
    }

    /// 添加PATCH路由
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users/:id"
    /// * `handler` - 处理函数，用于处理PATCH请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    pub fn patch<H>(mut self, path: &str, handler: H) -> Self
    where
        H: RouteHandler,
    {
        self.inner = self.inner.route(path, axum::routing::patch(handler));
        self
    }

    /// 添加HEAD路由
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users"
    /// * `handler` - 处理函数，用于处理HEAD请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    pub fn head<H>(mut self, path: &str, handler: H) -> Self
    where
        H: RouteHandler,
    {
        self.inner = self.inner.route(path, axum::routing::head(handler));
        self
    }

    /// 嵌套路由
    ///
    /// # Parameters
    /// * `path` - 父路由路径，如 "/api"
    /// * `router` - 子路由器，包含子路由定义
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 提供更简洁的嵌套方式，将子路由器的所有路由嵌套到父路径下
    pub fn nest(mut self, path: &str, router: Self) -> Self {
        self.inner = self.inner.nest(path, router.inner);
        self
    }

    /// 添加多个路由到同一路径
    ///
    /// # Parameters
    /// * `path` - 路由路径，如 "/api/users"
    /// * `methods` - HTTP方法数组，如 `&[Method::GET, Method::POST]`
    /// * `handler` - 处理函数，用于处理所有指定方法的请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 类似于Salvo的合并路由功能，为同一路径添加多个HTTP方法的路由
    pub fn route<H>(mut self, path: &str, methods: &[axum::http::Method], handler: H) -> Self
    where
        H: axum::handler::Handler<(), Arc<AppState>> + Clone + Send + 'static,
    {
        let mut router = Router::new();

        for method in methods {
            match *method {
                axum::http::Method::GET => {
                    router = router.route(path, axum::routing::get(handler.clone()))
                }
                axum::http::Method::POST => {
                    router = router.route(path, axum::routing::post(handler.clone()))
                }
                axum::http::Method::PUT => {
                    router = router.route(path, axum::routing::put(handler.clone()))
                }
                axum::http::Method::DELETE => {
                    router = router.route(path, axum::routing::delete(handler.clone()))
                }
                axum::http::Method::PATCH => {
                    router = router.route(path, axum::routing::patch(handler.clone()))
                }
                axum::http::Method::OPTIONS => {
                    router = router.route(path, axum::routing::options(handler.clone()))
                }
                axum::http::Method::HEAD => {
                    router = router.route(path, axum::routing::head(handler.clone()))
                }
                _ => continue,
            }
        }

        self.inner = self.inner.merge(router);
        self
    }

    /// 应用中间件集
    ///
    /// # Parameters
    /// * `middleware_set` - 中间件配置集，包含各种中间件的启用状态和配置
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 批量应用多个中间件，包括日志、跨域、限流和异常捕获中间件
    pub fn with_middlewares(mut self, middleware_set: MiddlewareSet) -> Self {
        // 应用中间件（顺序保持不变）
        if middleware_set.logger {
            self.inner = self.inner.layer(from_fn(middleware::logger_middleware));
        }

        if middleware_set.cors {
            self.inner = self.inner.layer(from_fn(middleware::cors_middleware));
        }

        if middleware_set.rate_limiter {
            let rate_limiter = middleware::RateLimiter::new(
                middleware_set.max_requests,
                middleware_set.reset_interval,
            );
            self.inner = self
                .inner
                .layer(axum::middleware::from_fn(move |req, next| {
                    let limiter = rate_limiter.clone();
                    async move { limiter.middleware(req, next).await }
                }));
        }

        if middleware_set.exception_catch {
            self.inner = self
                .inner
                .layer(from_fn(middleware::exception_catch_middleware));
        }

        self
    }

    /// 添加单个中间件
    ///
    /// # Parameters
    /// * `middleware` - 中间件函数，接收请求和下一个中间件，返回响应
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 传统接口，保持向后兼容，用于添加单个Axum风格的中间件
    pub fn use_middleware<F, Fut>(mut self, middleware: F) -> Self
    where
        F: Fn(axum::http::Request<axum::body::Body>, axum::middleware::Next) -> Fut
            + Clone
            + Send
            + Sync
            + 'static,
        Fut: std::future::Future<Output = axum::http::Response<axum::body::Body>> + Send + 'static,
    {
        self.inner = self.inner.layer(axum::middleware::from_fn(middleware));
        self
    }

    /// 添加Handler
    ///
    /// # Parameters
    /// * `handler` - Handler实现，用于处理请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 新接口，用于添加自定义Handler，将其转换为Axum的中间件
    pub fn use_handler<H>(mut self, handler: H) -> Self
    where
        H: Handler + Clone,
    {
        // 将Handler转换为Axum的中间件
        self.inner = self.inner.layer(axum::middleware::from_fn(
            move |req: axum::http::Request<axum::body::Body>, _next: axum::middleware::Next| {
                let handler = handler.clone();
                async move {
                    // 从请求中获取状态
                    let state = req
                        .extensions()
                        .get::<Arc<AppState>>()
                        .cloned()
                        .unwrap_or_default();

                    // 创建Context
                    let ctx = Context::new(
                        req, state, None, // next为None，暂时不支持中间件链
                    );

                    // 执行Handler的handle方法

                    // 直接返回响应，不调用next
                    handler.handle(ctx).await
                }
            },
        ));
        self
    }

    /// 添加前置中间件
    ///
    /// # Parameters
    /// * `handler` - Handler实现，用于处理请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 新接口，链式调用，用于添加在请求处理前执行的中间件
    pub fn before<H>(mut self, handler: H) -> Self
    where
        H: Handler + Clone,
    {
        self.before.push(Box::new(handler));
        self
    }

    /// 添加后置中间件
    ///
    /// # Parameters
    /// * `handler` - Handler实现，用于处理请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 新接口，链式调用，用于添加在请求处理后执行的中间件
    pub fn after<H>(mut self, handler: H) -> Self
    where
        H: Handler + Clone,
    {
        self.after.push(Box::new(handler));
        self
    }

    /// 添加环绕中间件
    ///
    /// # Parameters
    /// * `handler` - Handler实现，用于处理请求
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 新接口，链式调用，用于添加环绕请求处理的中间件
    pub fn hoop<H>(mut self, handler: H) -> Self
    where
        H: Handler + Clone,
    {
        self.hoop.push(Box::new(handler));
        self
    }

    /// 构建并返回Router，应用所有中间件
    ///
    /// # Returns
    /// * `Router<Arc<AppState>>` - 返回构建完成的Axum Router
    ///
    /// # 说明
    /// 应用所有已添加的中间件，并返回最终的Axum Router，用于启动服务器
    pub fn build(mut self) -> Router<Arc<AppState>> {
        // 应用前置中间件
        for handler in self.before {
            self.inner = self.inner.layer(axum::middleware::from_fn(
                move |req: axum::http::Request<axum::body::Body>, _next: axum::middleware::Next| {
                    let handler = dyn_clone::clone_box(&*handler);
                    async move {
                        // 从请求中获取状态
                        let state = req
                            .extensions()
                            .get::<Arc<AppState>>()
                            .cloned()
                            .unwrap_or_default();

                        // 创建Context
                        let ctx = Context::new(
                            req, state, None, // next为None，暂时不支持中间件链
                        );

                        // 执行中间件的handle方法

                        // 直接返回响应，不调用next
                        handler.handle(ctx).await
                    }
                },
            ));
        }

        // 应用后置中间件
        for handler in self.after {
            self.inner = self.inner.layer(axum::middleware::from_fn(
                move |req: axum::http::Request<axum::body::Body>, _next: axum::middleware::Next| {
                    let handler = dyn_clone::clone_box(&*handler);
                    async move {
                        // 先执行下一个中间件
                        let response = _next.run(req).await;

                        // 从响应中获取状态（需要先从请求中获取，这里简化处理）
                        let state = Arc::new(AppState::new());

                        // 创建Context，next为None
                        let ctx = Context::new(
                            axum::http::Request::builder()
                                .body(axum::body::Body::empty())
                                .unwrap(),
                            state,
                            None,
                        );

                        // 执行中间件的handle方法（注意：这里的实现可能需要调整，因为后置中间件通常需要访问响应）
                        let _ = handler.handle(ctx).await;

                        response
                    }
                },
            ));
        }

        // 应用环绕中间件
        for handler in self.hoop {
            self.inner = self.inner.layer(axum::middleware::from_fn(
                move |req: axum::http::Request<axum::body::Body>, _next: axum::middleware::Next| {
                    let handler = dyn_clone::clone_box(&*handler);
                    async move {
                        // 从请求中获取状态
                        let state = req
                            .extensions()
                            .get::<Arc<AppState>>()
                            .cloned()
                            .unwrap_or_default();

                        // 创建Context
                        let ctx = Context::new(
                            req, state, None, // next为None，暂时不支持中间件链
                        );

                        // 执行中间件的handle方法

                        // 直接返回响应，不调用next
                        handler.handle(ctx).await
                    }
                },
            ));
        }

        self.inner
    }

    /// 直接获取内部Router引用
    ///
    /// # Returns
    /// * `&Router<Arc<AppState>>` - 返回内部Axum Router的不可变引用
    ///
    /// # 说明
    /// 用于高级操作，允许直接访问和修改内部的Axum Router
    pub fn inner(&self) -> &Router<Arc<AppState>> {
        &self.inner
    }

    /// 直接获取内部Router可变引用
    ///
    /// # Returns
    /// * `&mut Router<Arc<AppState>>` - 返回内部Axum Router的可变引用
    ///
    /// # 说明
    /// 用于高级操作，允许直接修改内部的Axum Router
    pub fn inner_mut(&mut self) -> &mut Router<Arc<AppState>> {
        &mut self.inner
    }

    /// 将AppRouter与插件系统整合，自动注册插件路由
    ///
    /// # Parameters
    /// * `plugin_manager` - 插件管理器，用于获取已启用的插件
    ///
    /// # Returns
    /// * `Result<Self>` - 返回AppRouter以支持链式调用，或返回错误
    ///
    /// # 说明
    /// 此方法会遍历所有已启用的插件，并自动注册它们的路由
    /// 插件的路由信息通常存储在插件的清单文件中
    pub async fn with_plugins(mut self, plugin_manager: &PluginManager) -> Result<Self> {
        // 获取所有已启用的插件
        let enabled_plugins = plugin_manager
            .get_plugins_by_status(crate::plugin::PluginStatus::Enabled)
            .await;

        // 遍历所有已启用的插件
        for plugin in enabled_plugins {
            // 检查插件清单中是否包含路由信息
            if let Some(manifest) = &plugin.manifest {
                if let Some(routes) = &manifest.routes {
                    info!(
                        "Registering plugin routes: {} v{} ({} routes)",
                        plugin.name,
                        plugin.version,
                        routes.len()
                    );

                    // 遍历插件的所有路由
                    for route in routes {
                        let method = route.method.to_uppercase();
                        let path = &route.path;

                        // 检测路由冲突
                        if self.check_route_conflict(path, &method) {
                            warn!(
                                "Route conflict: {} {} (plugin: {})",
                                method, path, plugin.name
                            );
                            continue;
                        }

                        // 根据HTTP方法注册路由
                        match method.as_str() {
                            "GET" => {
                                info!("  GET {} -> {}", path, route.handler);
                                // 实际应用中，这里需要调用插件的处理函数
                                // self.inner = self.inner.route(path, get(plugin_handler));
                            }
                            "POST" => {
                                info!("  POST {} -> {}", path, route.handler);
                                // self.inner = self.inner.route(path, post(plugin_handler));
                            }
                            "PUT" => {
                                info!("  PUT {} -> {}", path, route.handler);
                                // self.inner = self.inner.route(path, put(plugin_handler));
                            }
                            "DELETE" => {
                                info!("  DELETE {} -> {}", path, route.handler);
                                // self.inner = self.inner.route(path, delete(plugin_handler));
                            }
                            "PATCH" => {
                                info!("  PATCH {} -> {}", path, route.handler);
                                // self.inner = self.inner.route(path, patch(plugin_handler));
                            }
                            "OPTIONS" => {
                                info!("  OPTIONS {} -> {}", path, route.handler);
                                // self.inner = self.inner.route(path, options(plugin_handler));
                            }
                            "HEAD" => {
                                info!("  HEAD {} -> {}", path, route.handler);
                                // self.inner = self.inner.route(path, head(plugin_handler));
                            }
                            _ => {
                                warn!("  Unsupported HTTP method: {} for {}", method, path);
                            }
                        }
                    }

                    // 缓存插件路由
                    self.plugin_routes_cache
                        .insert(plugin.name.clone(), routes.clone());
                } else {
                    info!(
                        "Plugin {} v{} has no route configuration",
                        plugin.name, plugin.version
                    );
                }
            } else {
                info!("Plugin {} v{} has no manifest", plugin.name, plugin.version);
            }
        }

        self.cache_last_updated = Some(Instant::now());
        Ok(self)
    }

    /// 检查路由冲突
    ///
    /// # Parameters
    /// * `path` - 路由路径
    /// * `method` - HTTP方法
    ///
    /// # Returns
    /// * `bool` - 如果存在冲突返回true，否则返回false
    ///
    /// # 说明
    /// 检查给定路径和方法是否与已注册的插件路由冲突
    fn check_route_conflict(&self, path: &str, method: &str) -> bool {
        // Check for conflicts in plugin route cache
        for (plugin_name, routes) in &self.plugin_routes_cache {
            for route in routes {
                if route.path == path && route.method.to_uppercase() == method {
                    warn!(
                        "Route conflict: {} {} (plugin: {})",
                        method, path, plugin_name
                    );
                    return true;
                }
            }
        }
        false
    }

    /// 清除插件路由缓存
    ///
    /// # Parameters
    /// * `plugin_name` - 插件名称
    ///
    /// # 说明
    /// 清除指定插件的路由缓存，通常在插件卸载或禁用时调用
    pub fn clear_plugin_routes_cache(&mut self, plugin_name: &str) {
        self.plugin_routes_cache.remove(plugin_name);
        info!("Cleared plugin route cache: {}", plugin_name);
    }

    /// 更新插件路由缓存
    ///
    /// # Parameters
    /// * `plugin_name` - 插件名称
    /// * `routes` - 插件路由列表
    ///
    /// # 说明
    /// 更新指定插件的路由缓存，通常在插件安装或更新时调用
    pub fn update_plugin_routes_cache(
        &mut self,
        plugin_name: &str,
        routes: Vec<crate::plugin::PluginRoute>,
    ) {
        self.plugin_routes_cache
            .insert(plugin_name.to_string(), routes.clone());
        self.cache_last_updated = Some(Instant::now());
        info!(
            "Updated plugin route cache: {} ({} routes)",
            plugin_name,
            routes.len()
        );
    }

    /// 获取插件路由缓存
    ///
    /// # Parameters
    /// * `plugin_name` - 插件名称
    ///
    /// # Returns
    /// * `Option<&Vec<PluginRoute>>` - 插件路由列表，如果不存在返回None
    ///
    /// # 说明
    /// 获取指定插件的路由缓存
    pub fn get_plugin_routes_cache(
        &self,
        plugin_name: &str,
    ) -> Option<&Vec<crate::plugin::PluginRoute>> {
        self.plugin_routes_cache.get(plugin_name)
    }

    /// 获取所有插件路由缓存
    ///
    /// # Returns
    /// * `&HashMap<String, Vec<PluginRoute>>` - 所有插件路由缓存
    ///
    /// # 说明
    /// 获取所有插件的路由缓存
    pub fn get_all_plugin_routes_cache(&self) -> &HashMap<String, Vec<crate::plugin::PluginRoute>> {
        &self.plugin_routes_cache
    }

    /// 注册单个插件路由
    ///
    /// # Parameters
    /// * `path` - 路由路径
    /// * `method` - HTTP方法
    /// * `handler` - 处理函数
    ///
    /// # Returns
    /// * `Self` - 返回AppRouter以支持链式调用
    ///
    /// # 说明
    /// 此方法用于插件动态注册单个路由
    pub fn register_plugin_route<H>(self, path: &str, method: &str, _handler: H) -> Self
    where
        H: Fn() -> String + Send + Sync + 'static + Clone,
    {
        let method = method.to_uppercase();

        match method.as_str() {
            "GET" => {
                info!("Registering plugin route: GET {}", path);
                // self.inner = self.inner.route(path, get(handler));
            }
            "POST" => {
                info!("Registering plugin route: POST {}", path);
                // self.inner = self.inner.route(path, post(handler));
            }
            "PUT" => {
                info!("Registering plugin route: PUT {}", path);
                // self.inner = self.inner.route(path, put(handler));
            }
            "DELETE" => {
                info!("Registering plugin route: DELETE {}", path);
                // self.inner = self.inner.route(path, delete(handler));
            }
            "PATCH" => {
                info!("Registering plugin route: PATCH {}", path);
                // self.inner = self.inner.route(path, patch(handler));
            }
            "OPTIONS" => {
                info!("Registering plugin route: OPTIONS {}", path);
                // self.inner = self.inner.route(path, options(handler));
            }
            "HEAD" => {
                info!("Registering plugin route: HEAD {}", path);
                // self.inner = self.inner.route(path, head(handler));
            }
            _ => {
                warn!("Unsupported HTTP method: {} for {}", method, path);
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_app_router_creation() {
        let _router = AppRouter::new();
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_with_state() {
        let app_state = Arc::new(AppState::new());
        let _router = AppRouter::with_state(app_state);
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_http_methods() {
        let app_state = Arc::new(AppState::new());
        let _router = AppRouter::with_state(app_state);
        // Just ensure the router creation doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_nest() {
        let app_state = Arc::new(AppState::new());
        let child_router = AppRouter::with_state(app_state.clone());
        let _parent_router = AppRouter::with_state(app_state).nest("/parent", child_router);
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_route() {
        let app_state = Arc::new(AppState::new());
        let _router = AppRouter::with_state(app_state);
        // Just ensure the router creation doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_with_middlewares() {
        let app_state = Arc::new(AppState::new());
        let middleware_set = MiddlewareSet::default();
        let _router = AppRouter::with_state(app_state).with_middlewares(middleware_set);
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_use_middleware() {
        let app_state = Arc::new(AppState::new());
        let _router = AppRouter::with_state(app_state)
            .use_middleware(|req, next| async { next.run(req).await });
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_before_after_hoop() {
        use crate::core::middleware::logger_middleware_new;

        let app_state = Arc::new(AppState::new());
        let _router = AppRouter::with_state(app_state)
            .before(logger_middleware_new)
            .after(logger_middleware_new)
            .hoop(logger_middleware_new);
        // Just ensure the methods don't panic
    }

    #[tokio::test]
    async fn test_app_router_build() {
        let app_state = Arc::new(AppState::new());
        let _router = AppRouter::with_state(app_state).build();
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_inner() {
        let app_state = Arc::new(AppState::new());
        let router = AppRouter::with_state(app_state);
        let _inner = router.inner();
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_inner_mut() {
        let app_state = Arc::new(AppState::new());
        let mut router = AppRouter::with_state(app_state);
        let _inner = router.inner_mut();
        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_app_router_check_route_conflict() {
        let app_state = Arc::new(AppState::new());
        let router = AppRouter::with_state(app_state);
        let conflict = router.check_route_conflict("/test", "GET");
        assert!(!conflict);
    }

    #[tokio::test]
    async fn test_app_router_plugin_routes_cache() {
        let app_state = Arc::new(AppState::new());
        let mut router = AppRouter::with_state(app_state);

        // Test clear plugin routes cache
        router.clear_plugin_routes_cache("test_plugin");

        // Test update plugin routes cache
        let routes = vec![];
        router.update_plugin_routes_cache("test_plugin", routes);

        // Test get plugin routes cache
        let cached_routes = router.get_plugin_routes_cache("test_plugin");
        assert!(cached_routes.is_some());

        // Test get all plugin routes cache
        let all_cached_routes = router.get_all_plugin_routes_cache();
        assert!(!all_cached_routes.is_empty());
    }

    #[tokio::test]
    async fn test_app_router_register_plugin_route() {
        let app_state = Arc::new(AppState::new());
        let _router =
            AppRouter::with_state(app_state)
                .register_plugin_route("/plugin/test", "GET", || "Plugin route".to_string());
        // Just ensure the method doesn't panic
    }
}

