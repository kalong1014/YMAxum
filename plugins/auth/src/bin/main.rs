//! GUF 身份验证插件示例

use anyhow::Result;
use guf_auth_plugin::{GufAuthPlugin, AuthRequest, RegisterRequest, AuthResponse};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== GUF Auth Plugin Example ===");

    // 创建插件实例
    let mut plugin = GufAuthPlugin::new();
    println!("Created plugin: {}", plugin.manifest.name);
    println!("Version: {}", plugin.manifest.version);
    println!("Description: {}", plugin.manifest.description);
    println!("GUF Compatible: {}", plugin.manifest.guf_compatible);
    println!("GUF Version: {}", plugin.manifest.guf_version);

    // 初始化插件
    println!("\nInitializing plugin...");
    if let Err(e) = plugin.initialize().await {
        eprintln!("Failed to initialize plugin: {}", e);
        return Err(e);
    }
    println!("Plugin initialized successfully!");

    // 测试用户注册
    println!("\nTesting user registration...");
    let register_request = RegisterRequest {
        username: "testuser".to_string(),
        email: "testuser@example.com".to_string(),
        password: "password123".to_string(),
        confirm_password: "password123".to_string(),
    };
    match plugin.register(register_request).await {
        Ok(response) => {
            println!("Registration response: {:?}", response);
            if response.success {
                println!("User registered successfully!");
            } else {
                println!("Registration failed: {}", response.message);
            }
        }
        Err(e) => {
            eprintln!("Failed to register user: {}", e);
        }
    }

    // 测试用户登录
    println!("\nTesting user login...");
    let login_request = AuthRequest {
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };
    match plugin.login(login_request).await {
        Ok(response) => {
            println!("Login response: {:?}", response);
            if response.success {
                println!("User logged in successfully!");
                
                // 测试令牌刷新
                if let Some(token) = response.token {
                    println!("\nTesting token refresh...");
                    match plugin.refresh_token(&token).await {
                        Ok(refresh_response) => {
                            println!("Token refresh response: {:?}", refresh_response);
                            if refresh_response.success {
                                println!("Token refreshed successfully!");
                            } else {
                                println!("Token refresh failed: {}", refresh_response.message);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to refresh token: {}", e);
                        }
                    }
                }
            } else {
                println!("Login failed: {}", response.message);
            }
        }
        Err(e) => {
            eprintln!("Failed to login user: {}", e);
        }
    }

    // 测试管理员登录
    println!("\nTesting admin login...");
    let admin_login_request = AuthRequest {
        username: "admin".to_string(),
        password: "admin123".to_string(),
    };
    match plugin.login(admin_login_request).await {
        Ok(response) => {
            println!("Admin login response: {:?}", response);
            if response.success {
                println!("Admin logged in successfully!");
            } else {
                println!("Admin login failed: {}", response.message);
            }
        }
        Err(e) => {
            eprintln!("Failed to login admin: {}", e);
        }
    }

    // 停止插件
    println!("\nStopping plugin...");
    if let Err(e) = plugin.stop().await {
        eprintln!("Failed to stop plugin: {}", e);
        return Err(e);
    }
    println!("Plugin stopped successfully!");

    println!("\n=== GUF Auth Plugin Example Complete ===");
    Ok(())
}
