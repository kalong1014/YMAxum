//! 认证和授权插件
//! 提供用户验证、API密钥验证和JWT令牌生成功能

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono;

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
}

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// 注册请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// 刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// 认证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

/// 认证服务
pub struct AuthService {
    users: Arc<RwLock<Vec<User>>>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            users: Arc::new(RwLock::new(Vec::new())),
            jwt_secret,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<User, String> {
        let mut users = self.users.write().await;
        
        // 检查邮箱是否已存在
        if users.iter().any(|u| u.email == request.email) {
            return Err("邮箱已被注册".to_string());
        }
        
        // 生成密码哈希
        let password_hash = self.hash_password(&request.password);
        
        // 创建新用户
        let user = User {
            id: format!("user_{}", chrono::Utc::now().timestamp()),
            username: request.username,
            email: request.email,
            password_hash,
            role: "user".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        
        users.push(user.clone());
        Ok(user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<(User, String, String), String> {
        let users = self.users.read().await;
        
        // 查找用户
        let user = users.iter().find(|u| u.email == request.email).ok_or("用户不存在".to_string())?;
        
        // 验证密码
        if !self.verify_password(&request.password, &user.password_hash) {
            return Err("密码错误".to_string());
        }
        
        // 生成访问令牌和刷新令牌
        let access_token = self.generate_token(&user, "access", 3600);
        let refresh_token = self.generate_token(&user, "refresh", 86400 * 7);
        
        Ok((user.clone(), access_token, refresh_token))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), String> {
        // 验证刷新令牌
        let claims = self.validate_token(refresh_token)?;
        
        if claims.token_type != "refresh" {
            return Err("无效的刷新令牌".to_string());
        }
        
        // 查找用户
        let users = self.users.read().await;
        let user = users.iter().find(|u| u.id == claims.sub).ok_or("用户不存在".to_string())?;
        
        // 生成新的访问令牌和刷新令牌
        let access_token = self.generate_token(&user, "access", 3600);
        let new_refresh_token = self.generate_token(&user, "refresh", 86400 * 7);
        
        Ok((access_token, new_refresh_token))
    }

    pub async fn get_user(&self, user_id: &str) -> Option<User> {
        let users = self.users.read().await;
        users.iter().find(|u| u.id == user_id).cloned()
    }

    /// 生成密码哈希
    fn hash_password(&self, password: &str) -> String {
        // 实际应用中应该使用bcrypt等安全的哈希算法
        // 这里使用简化的实现
        format!("hash_{}", password)
    }

    /// 验证密码
    fn verify_password(&self, password: &str, hash: &str) -> bool {
        // 实际应用中应该使用bcrypt等安全的哈希算法
        // 这里使用简化的实现
        let expected_hash = format!("hash_{}", password);
        expected_hash == hash
    }

    /// 生成JWT令牌
    fn generate_token(&self, user: &User, token_type: &str, expires_in: i64) -> String {
        // 实际应用中应该使用真实的JWT库
        // 这里使用简化的实现
        format!("{}_{}_{}_{}", token_type, user.id, expires_in, self.jwt_secret)
    }

    /// 验证JWT令牌
    fn validate_token(&self, token: &str) -> Result<TokenClaims, String> {
        // 实际应用中应该使用真实的JWT库
        // 这里使用简化的实现
        let parts: Vec<&str> = token.split('_').collect();
        if parts.len() != 4 {
            return Err("无效的令牌".to_string());
        }
        
        Ok(TokenClaims {
            sub: parts[1].to_string(),
            token_type: parts[0].to_string(),
            exp: chrono::Utc::now().timestamp() + parts[2].parse().unwrap_or(0),
        })
    }
}

/// 令牌声明
#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub token_type: String,
    pub exp: i64,
}

/// 插件生命周期实现
pub struct AuthPlugin {
    auth_service: Option<AuthService>,
    initialized: bool,
    started: bool,
}

impl AuthPlugin {
    pub fn new() -> Self {
        Self {
            auth_service: None,
            initialized: false,
            started: false,
        }
    }
}

/// 插件生命周期接口实现
impl crate::plugin::runtime::PluginLifecycle for AuthPlugin {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            return Ok(());
        }
        
        // 初始化认证服务
        self.auth_service = Some(AuthService::new("your-secret-key".to_string()));
        self.initialized = true;
        
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("插件未初始化".into());
        }
        
        if self.started {
            return Ok(());
        }
        
        self.started = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.started {
            return Ok(());
        }
        
        self.started = false;
        Ok(())
    }
}

/// 登录处理器
pub async fn login_handler(req: axum::extract::Json<LoginRequest>) -> axum::response::Json<AuthResponse> {
    // 实际应用中应该从应用状态中获取AuthService
    let auth_service = AuthService::new("your-secret-key".to_string());
    
    match auth_service.login(req.0).await {
        Ok((user, access_token, refresh_token)) => {
            axum::response::Json(AuthResponse {
                access_token,
                refresh_token,
                user,
            })
        }
        Err(err) => {
            // 实际应用中应该返回错误响应
            panic!("登录失败: {}", err);
        }
    }
}

/// 注册处理器
pub async fn register_handler(req: axum::extract::Json<RegisterRequest>) -> axum::response::Json<AuthResponse> {
    // 实际应用中应该从应用状态中获取AuthService
    let auth_service = AuthService::new("your-secret-key".to_string());
    
    match auth_service.register(req.0).await {
        Ok(user) => {
            // 自动登录
            let login_req = LoginRequest {
                email: user.email.clone(),
                password: req.0.password.clone(),
            };
            
            match auth_service.login(login_req).await {
                Ok((_, access_token, refresh_token)) => {
                    axum::response::Json(AuthResponse {
                        access_token,
                        refresh_token,
                        user,
                    })
                }
                Err(err) => {
                    panic!("注册后登录失败: {}", err);
                }
            }
        }
        Err(err) => {
            panic!("注册失败: {}", err);
        }
    }
}

/// 刷新令牌处理器
pub async fn refresh_handler(req: axum::extract::Json<RefreshRequest>) -> axum::response::Json<serde_json::Value> {
    // 实际应用中应该从应用状态中获取AuthService
    let auth_service = AuthService::new("your-secret-key".to_string());
    
    match auth_service.refresh_token(&req.0.refresh_token).await {
        Ok((access_token, refresh_token)) => {
            axum::response::Json(serde_json::json!({
                "access_token": access_token,
                "refresh_token": refresh_token
            }))
        }
        Err(err) => {
            panic!("刷新令牌失败: {}", err);
        }
    }
}

/// 登出处理器
pub async fn logout_handler() -> axum::response::Json<serde_json::Value> {
    // 实际应用中应该处理令牌撤销
    axum::response::Json(serde_json::json!({
        "message": "登出成功"
    }))
}
