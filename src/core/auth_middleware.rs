// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 授权中间件模块
//! 提供基于角色的访问控制、权限验证和最小权限原则实现

use crate::core::context::Context;
use axum::{body::Body, http::StatusCode, response::Response};
use log::{error, info};
use std::collections::HashSet;

/// 用户角色
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserRole {
    /// 管理员 - 拥有所有权限
    Admin,
    /// 开发者 - 可以管理插件和配置
    Developer,
    /// 运营人员 - 可以管理内容和用户
    Operator,
    /// 普通用户 - 只能访问基本功能
    User,
    /// 访客 - 只能访问公开内容
    Guest,
}

/// 权限类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    /// 访问管理面板
    AccessAdminPanel,
    /// 管理用户
    ManageUsers,
    /// 管理插件
    ManagePlugins,
    /// 管理配置
    ManageConfig,
    /// 管理内容
    ManageContent,
    /// 访问用户信息
    AccessUserInfo,
    /// 访问公开内容
    AccessPublicContent,
    /// 执行系统操作
    ExecuteSystemOperations,
    /// 访问性能监控
    AccessPerformanceMonitor,
    /// 访问安全模块
    AccessSecurityModule,
}

/// 用户信息
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub username: String,
    pub role: UserRole,
    pub permissions: HashSet<Permission>,
}

/// 角色权限映射
pub fn get_role_permissions(role: &UserRole) -> HashSet<Permission> {
    let mut permissions = HashSet::new();

    match role {
        UserRole::Admin => {
            // 管理员拥有所有权限
            permissions.insert(Permission::AccessAdminPanel);
            permissions.insert(Permission::ManageUsers);
            permissions.insert(Permission::ManagePlugins);
            permissions.insert(Permission::ManageConfig);
            permissions.insert(Permission::ManageContent);
            permissions.insert(Permission::AccessUserInfo);
            permissions.insert(Permission::AccessPublicContent);
            permissions.insert(Permission::ExecuteSystemOperations);
            permissions.insert(Permission::AccessPerformanceMonitor);
            permissions.insert(Permission::AccessSecurityModule);
        }
        UserRole::Developer => {
            // 开发者权限
            permissions.insert(Permission::AccessAdminPanel);
            permissions.insert(Permission::ManagePlugins);
            permissions.insert(Permission::ManageConfig);
            permissions.insert(Permission::AccessUserInfo);
            permissions.insert(Permission::AccessPublicContent);
            permissions.insert(Permission::AccessPerformanceMonitor);
        }
        UserRole::Operator => {
            // 运营人员权限
            permissions.insert(Permission::AccessAdminPanel);
            permissions.insert(Permission::ManageUsers);
            permissions.insert(Permission::ManageContent);
            permissions.insert(Permission::AccessUserInfo);
            permissions.insert(Permission::AccessPublicContent);
        }
        UserRole::User => {
            // 普通用户权限
            permissions.insert(Permission::AccessUserInfo);
            permissions.insert(Permission::AccessPublicContent);
        }
        UserRole::Guest => {
            // 访客权限
            permissions.insert(Permission::AccessPublicContent);
        }
    }

    permissions
}

/// 授权中间件
pub struct AuthMiddleware {
    /// 需要的权限
    required_permissions: HashSet<Permission>,
    /// 是否允许匿名访问
    allow_anonymous: bool,
}

impl AuthMiddleware {
    /// 创建新的授权中间件
    ///
    /// # 参数
    /// - `required_permissions`: 需要的权限列表
    /// - `allow_anonymous`: 是否允许匿名访问
    ///
    /// # 返回
    /// - `AuthMiddleware`: 新创建的授权中间件
    pub fn new(required_permissions: Vec<Permission>, allow_anonymous: bool) -> Self {
        Self {
            required_permissions: required_permissions.into_iter().collect(),
            allow_anonymous,
        }
    }

    /// 创建需要管理员权限的中间件
    pub fn admin() -> Self {
        Self::new(vec![Permission::AccessAdminPanel], false)
    }

    /// 创建需要开发者权限的中间件
    pub fn developer() -> Self {
        Self::new(vec![Permission::ManagePlugins], false)
    }

    /// 创建需要用户权限的中间件
    pub fn user() -> Self {
        Self::new(vec![Permission::AccessUserInfo], false)
    }

    /// 创建只需要公开访问权限的中间件
    pub fn public() -> Self {
        Self::new(vec![Permission::AccessPublicContent], true)
    }

    /// 处理请求
    ///
    /// # 参数
    /// - `ctx`: 请求上下文
    ///
    /// # 返回
    /// - `Response<Body>`: HTTP响应
    pub async fn handle(&self, ctx: Context) -> Response<Body> {
        // 模拟从请求中获取用户信息
        // 实际实现中，应该从JWT token或session中获取
        let auth_header = ctx.request.headers().get("Authorization");
        let user_info = self.get_user_info(auth_header).await;

        // 检查用户权限
        if !self.check_permissions(&user_info).await {
            return self.create_unauthorized_response();
        }

        // 将用户信息添加到上下文中
        if let Some(user) = &user_info {
            // 实际实现中，应该将用户信息存储在上下文中
            info!(
                "User {} (role: {:?}) accessing protected resource",
                user.username, user.role
            );
        }

        // 继续处理请求
        ctx.next().await
    }

    /// 从请求中获取用户信息
    async fn get_user_info(
        &self,
        auth_header: Option<&axum::http::HeaderValue>,
    ) -> Option<UserInfo> {
        // 从Authorization header获取并验证JWT token
        if let Some(auth_header) =
            auth_header
            && let Ok(auth_str) = auth_header.to_str()
            && auth_str.starts_with("Bearer ")
        {
            let token = auth_str.strip_prefix("Bearer ").unwrap();
            
            // 验证JWT token
            if let Some(user_info) = self.validate_jwt_token(token).await {
                return Some(user_info);
            }
        }

        // 如果允许匿名访问，返回访客角色
        if self.allow_anonymous {
            return Some(UserInfo {
                user_id: "guest".to_string(),
                username: "guest".to_string(),
                role: UserRole::Guest,
                permissions: get_role_permissions(&UserRole::Guest),
            });
        }

        None
    }

    /// 验证JWT token
    async fn validate_jwt_token(&self, token: &str) -> Option<UserInfo> {
        // 实际实现中，应该使用jwt-simple库验证token
        // 这里暂时使用简化实现，后续会替换为完整的JWT验证
        
        // 检查token格式
        if token.len() < 32 {
            error!("Invalid token format: token too short");
            return None;
        }
        
        // 基于token前缀模拟验证（后续会替换为真正的JWT验证）
        if token.starts_with("admin-") {
            return Some(UserInfo {
                user_id: "admin123".to_string(),
                username: "admin".to_string(),
                role: UserRole::Admin,
                permissions: get_role_permissions(&UserRole::Admin),
            });
        } else if token.starts_with("dev-") {
            return Some(UserInfo {
                user_id: "dev123".to_string(),
                username: "developer".to_string(),
                role: UserRole::Developer,
                permissions: get_role_permissions(&UserRole::Developer),
            });
        } else if token.starts_with("user-") {
            return Some(UserInfo {
                user_id: "user123".to_string(),
                username: "user".to_string(),
                role: UserRole::User,
                permissions: get_role_permissions(&UserRole::User),
            });
        }
        
        error!("Invalid token: {}", token);
        None
    }

    /// 检查用户权限
    async fn check_permissions(&self, user_info: &Option<UserInfo>) -> bool {
        match user_info {
            Some(user) => {
                // 检查用户是否拥有所有需要的权限
                for permission in &self.required_permissions {
                    if !user.permissions.contains(permission) {
                        error!(
                            "User {} lacks required permission: {:?}",
                            user.username, permission
                        );
                        return false;
                    }
                }
                true
            }
            None => {
                // 没有用户信息且不允许匿名访问
                error!("No user information provided and anonymous access not allowed");
                false
            }
        }
    }

    /// 创建未授权响应
    fn create_unauthorized_response(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(r#"{"error": "Unauthorized", "message": "You do not have permission to access this resource"}"#))
            .unwrap()
    }
}

/// 授权中间件函数 - 新接口实现
pub async fn auth_middleware_new(ctx: Context) -> Response<Body> {
    // 默认使用需要用户权限的中间件
    let middleware = AuthMiddleware::user();
    middleware.handle(ctx).await
}

/// 管理员权限中间件
pub async fn admin_middleware(ctx: Context) -> Response<Body> {
    let middleware = AuthMiddleware::admin();
    middleware.handle(ctx).await
}

/// 开发者权限中间件
pub async fn developer_middleware(ctx: Context) -> Response<Body> {
    let middleware = AuthMiddleware::developer();
    middleware.handle(ctx).await
}

/// 公开访问中间件
pub async fn public_middleware(ctx: Context) -> Response<Body> {
    let middleware = AuthMiddleware::public();
    middleware.handle(ctx).await
}

/// 检查用户是否有权限
pub fn has_permission(user: &UserInfo, permission: &Permission) -> bool {
    user.permissions.contains(permission)
}

/// 检查用户是否有角色
pub fn has_role(user: &UserInfo, role: &UserRole) -> bool {
    &user.role == role
}

/// 检查用户是否有至少指定角色的权限
pub fn has_at_least_role(user: &UserInfo, required_role: &UserRole) -> bool {
    // 角色优先级：Admin > Developer > Operator > User > Guest
    match required_role {
        UserRole::Admin => user.role == UserRole::Admin,
        UserRole::Developer => matches!(&user.role, UserRole::Admin | UserRole::Developer),
        UserRole::Operator => matches!(
            &user.role,
            UserRole::Admin | UserRole::Developer | UserRole::Operator
        ),
        UserRole::User => matches!(
            &user.role,
            UserRole::Admin | UserRole::Developer | UserRole::Operator | UserRole::User
        ),
        UserRole::Guest => true, // 所有用户都至少是访客
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permissions() {
        // 测试管理员权限
        let admin_permissions = get_role_permissions(&UserRole::Admin);
        assert!(admin_permissions.contains(&Permission::AccessAdminPanel));
        assert!(admin_permissions.contains(&Permission::ManageUsers));

        // 测试普通用户权限
        let user_permissions = get_role_permissions(&UserRole::User);
        assert!(!user_permissions.contains(&Permission::AccessAdminPanel));
        assert!(user_permissions.contains(&Permission::AccessUserInfo));

        // 测试访客权限
        let guest_permissions = get_role_permissions(&UserRole::Guest);
        assert!(!guest_permissions.contains(&Permission::AccessUserInfo));
        assert!(guest_permissions.contains(&Permission::AccessPublicContent));
    }

    #[test]
    fn test_has_permission() {
        let user = UserInfo {
            user_id: "user123".to_string(),
            username: "testuser".to_string(),
            role: UserRole::User,
            permissions: get_role_permissions(&UserRole::User),
        };

        assert!(has_permission(&user, &Permission::AccessUserInfo));
        assert!(!has_permission(&user, &Permission::AccessAdminPanel));
    }

    #[test]
    fn test_has_role() {
        let user = UserInfo {
            user_id: "user123".to_string(),
            username: "testuser".to_string(),
            role: UserRole::User,
            permissions: get_role_permissions(&UserRole::User),
        };

        assert!(has_role(&user, &UserRole::User));
        assert!(!has_role(&user, &UserRole::Admin));
    }

    #[test]
    fn test_has_at_least_role() {
        let admin = UserInfo {
            user_id: "admin123".to_string(),
            username: "admin".to_string(),
            role: UserRole::Admin,
            permissions: get_role_permissions(&UserRole::Admin),
        };

        let user = UserInfo {
            user_id: "user123".to_string(),
            username: "testuser".to_string(),
            role: UserRole::User,
            permissions: get_role_permissions(&UserRole::User),
        };

        assert!(has_at_least_role(&admin, &UserRole::Developer));
        assert!(!has_at_least_role(&user, &UserRole::Developer));
        assert!(has_at_least_role(&user, &UserRole::User));
    }
}

