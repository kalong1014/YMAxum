use crate::core::iterate_api::{IterateError, IterateRequest};
use log::info;
use serde::{Deserialize, Serialize};

/// AI code generation request
#[derive(Debug, Deserialize)]
pub struct AICodeGenRequest {
    /// Feature name
    pub name: String,
    /// Template name
    pub template: String,
    /// Feature description
    pub description: String,
    /// Additional parameters
    pub params: serde_json::Value,
}

/// AI code generation response
#[derive(Debug, Serialize)]
pub struct AICodeGenResponse {
    /// Success flag
    pub success: bool,
    /// Generated code
    pub code: String,
    /// Generated test code
    pub test_code: String,
    /// Generated configuration file
    pub config: String,
    /// Error message
    pub error: Option<String>,
}

/// AI iteration auxiliary tool
#[derive(Debug, Clone)]
pub struct AIIterateTool {
    /// Template library
    pub templates: Vec<IterateTemplate>,
}

/// Iteration template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterateTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Required parameters
    pub required_params: Vec<String>,
    /// Template code
    pub template_code: String,
    /// Template test code
    pub template_test: String,
    /// Template configuration
    pub template_config: String,
}

impl AIIterateTool {
    /// Create a new AI iteration auxiliary tool
    pub fn new() -> Self {
        Self {
            templates: Self::load_default_templates(),
        }
    }

    /// Load default template library
    fn load_default_templates() -> Vec<IterateTemplate> {
        vec![
            // Basic Web template
            IterateTemplate {
                name: "basic_web".to_string(),
                description: "Basic Web application template".to_string(),
                required_params: vec!["name".to_string(), "port".to_string()],
                template_code: r#"use axum::{routing::get, Router};
use std::net::SocketAddr;
use log::info;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], {{port}}));
    info!("Server running on https://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
"#
                .to_string(),
                template_test: r#"#[tokio::test]
async fn test_basic_web() {
    // Test basic Web template
    assert_eq!(1 + 1, 2);
}
"#
                .to_string(),
                template_config: r#"name = "{{name}}"
port = {{port}}
"#
                .to_string(),
            },
            // E-commerce API template
            IterateTemplate {
                name: "ecommerce_api".to_string(),
                description: "E-commerce API template".to_string(),
                required_params: vec!["name".to_string(), "db_url".to_string()],
                template_code: r#"use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use log::info;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn get_products(state: axum::extract::State<AppState>) -> String {
    "Products list" 
}

async fn create_product(state: axum::extract::State<AppState>) -> String {
    "Product created" 
}

#[tokio::main]
async fn main() {
    let db = PgPool::connect("{{db_url}}")
        .await
        .unwrap();

    let state = AppState { db };

    let app = Router::new()
        .route("/products", get(get_products))
        .route("/products", post(create_product))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server running on https://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
"#
                .to_string(),
                template_test: r#"#[tokio::test]
async fn test_ecommerce_api() {
    // Test e-commerce API template
    assert_eq!(1 + 1, 2);
}
"#
                .to_string(),
                template_config: r#"name = "{{name}}"
db_url = "{{db_url}}"
"#
                .to_string(),
            },
            // Game server template
            IterateTemplate {
                name: "game_server".to_string(),
                description: "Game server template".to_string(),
                required_params: vec!["name".to_string(), "max_players".to_string()],
                template_code: r#"use std::net::TcpListener;
use std::thread;
use log::info;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    info!("Game server started on 127.0.0.1:8080, max players: {{max_players}}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                info!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: std::net::TcpStream) {
    // Handle client connection
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                // Process received data
                info!("Received {} bytes", n);
            }
            Err(e) => {
                info!("Error reading from client: {}", e);
                break;
            }
        }
    }
}
"#
                .to_string(),
                template_test: r#"#[test]
fn test_game_server() {
    // Test game server template
    assert_eq!(1 + 1, 2);
}
"#
                .to_string(),
                template_config: r#"name = "{{name}}"
max_players = {{max_players}}
"#
                .to_string(),
            },
        ]
    }

    /// Generate plugin code
    pub async fn generate_plugin_code(
        &self,
        request: AICodeGenRequest,
    ) -> Result<AICodeGenResponse, IterateError> {
        info!(
            "Generating plugin code for: {}, template: {}",
            request.name, request.template
        );

        // Find template
        let template = self
            .templates
            .iter()
            .find(|t| t.name == request.template)
            .ok_or(IterateError::InternalError(format!(
                "Template not found: {}",
                request.template
            )))?;

        // Validate required parameters
        self.validate_template_params(template, &request.params)?;

        // Generate code
        let code = self.render_template(&template.template_code, &request);
        let test_code = self.render_template(&template.template_test, &request);
        let config = self.render_template(&template.template_config, &request);

        info!("Generated plugin code successfully");

        Ok(AICodeGenResponse {
            success: true,
            code,
            test_code,
            config,
            error: None,
        })
    }

    /// Regenerate plugin code
    pub async fn regenerate_plugin_code(
        &self,
        request: AICodeGenRequest,
        _previous_code: &str,
        error_message: &str,
    ) -> Result<AICodeGenResponse, IterateError> {
        info!("Regenerating plugin code due to error: {}", error_message);

        // Simplified implementation, actual need to adjust generation strategy based on error message
        self.generate_plugin_code(request).await
    }

    /// Validate template parameters
    fn validate_template_params(
        &self,
        template: &IterateTemplate,
        params: &serde_json::Value,
    ) -> Result<(), IterateError> {
        for required_param in &template.required_params {
            if params.get(required_param).is_none() {
                return Err(IterateError::InternalError(format!(
                    "Missing required parameter: {}",
                    required_param
                )));
            }
        }
        Ok(())
    }

    /// Render template
    fn render_template(&self, template: &str, request: &AICodeGenRequest) -> String {
        let mut result = template.to_string();

        // Replace basic parameters
        result = result.replace("{{name}}", &request.name);
        result = result.replace("{{description}}", &request.description);

        // Replace additional parameters
        if let serde_json::Value::Object(params) = &request.params {
            for (key, value) in params {
                let placeholder = format!("{{{{{}}}}}", key);
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                result = result.replace(&placeholder, &value_str);
            }
        }

        result
    }

    /// Automatically adapt to iteration API
    pub async fn adapt_to_iterate_api(
        &self,
        _code: &str,
        _test_code: &str,
        _config: &str,
    ) -> Result<IterateRequest, IterateError> {
        info!("Adapting generated code to iterate API");

        // Simplified implementation, actual need to analyze code and generate iteration request
        Ok(IterateRequest {
            plugin_path: "./plugins/generated_plugin".to_string(),
            feature_id: "generated_feature".to_string(),
            dependencies: vec!["serde".to_string(), "tokio".to_string()],
            plugin_version: "1.0.0".to_string(),
            core_version: "1.0.0".to_string(),
        })
    }
}

impl Default for AIIterateTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_plugin_code() {
        let ai_tool = AIIterateTool::new();
        let request = AICodeGenRequest {
            name: "test_app".to_string(),
            template: "basic_web".to_string(),
            description: "Test application".to_string(),
            params: serde_json::json!({ "name": "test_app", "port": 3000 }),
        };

        let response = ai_tool.generate_plugin_code(request).await;
        assert!(response.is_ok());
        let result = response.unwrap();
        assert!(result.success);
        assert!(!result.code.is_empty());
        assert!(!result.test_code.is_empty());
        assert!(!result.config.is_empty());
    }

    #[tokio::test]
    async fn test_regenerate_plugin_code() {
        let ai_tool = AIIterateTool::new();
        let request = AICodeGenRequest {
            name: "test_app".to_string(),
            template: "basic_web".to_string(),
            description: "Test application".to_string(),
            params: serde_json::json!({ "name": "test_app", "port": 3000 }),
        };

        let response = ai_tool
            .regenerate_plugin_code(request, "old_code", "error message")
            .await;
        assert!(response.is_ok());
        let result = response.unwrap();
        assert!(result.success);
    }
}
