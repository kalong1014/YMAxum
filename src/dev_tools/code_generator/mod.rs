//! 代码生成器模块
//! 
//! 提供代码模板的生成、管理和定制等功能

use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 代码模板类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    Api,
    Model,
    Controller,
    Service,
    Middleware,
    Config,
    Test,
    Documentation,
    Other,
}

/// 代码模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTemplate {
    pub id: String,
    pub name: String,
    pub template_type: TemplateType,
    pub language: String,
    pub content: String,
    pub description: Option<String>,
    pub variables: Vec<TemplateVariable>,
    pub examples: Vec<String>,
}

/// 模板变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation: Option<String>,
}

/// 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationConfig {
    pub template_id: String,
    pub output_path: String,
    pub variables: HashMap<String, String>,
    pub overwrite: bool,
    pub format: bool,
    pub include_tests: bool,
    pub include_documentation: bool,
}

/// 代码生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationResult {
    pub template_id: String,
    pub output_path: String,
    pub success: bool,
    pub files_generated: u32,
    pub message: Option<String>,
    pub duration_ms: u64,
}

/// 代码生成器
#[derive(Debug, Clone)]
pub struct CodeGenerator {
    templates: HashMap<String, CodeTemplate>,
    template_dir: String,
    cache: HashMap<String, String>, // 缓存渲染结果
    cache_enabled: bool,
}

impl CodeGenerator {
    /// 创建新的代码生成器
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            template_dir: "./templates".to_string(),
            cache: HashMap::new(),
            cache_enabled: true,
        }
    }

    /// 初始化代码生成器
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化代码生成器
        self.load_templates().await?;
        Ok(())
    }

    /// 加载模板
    async fn load_templates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 加载模板
        let template_dir = Path::new(&self.template_dir);
        if template_dir.exists() {
            // 加载目录中的模板
            info!("Loading templates from {}", self.template_dir);
        } else {
            // 创建模板目录
            fs::create_dir_all(template_dir)?;
            info!("Created template directory: {}", self.template_dir);
        }

        // 注册默认模板
        self.register_default_templates().await?;
        // 注册其他语言的模板
        self.register_additional_language_templates().await?;

        Ok(())
    }

    /// 注册默认模板
    async fn register_default_templates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 注册默认模板
        let templates = vec![
            CodeTemplate {
                id: "rust-api".to_string(),
                name: "Rust API".to_string(),
                template_type: TemplateType::Api,
                language: "rust".to_string(),
                content: r#"use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/api/{{endpoint}}", get(handler))
}

async fn handler() -> &'static str {
    "Hello, World!"
}
"#.to_string(),
                description: Some("Rust API template".to_string()),
                variables: vec![TemplateVariable {
                    name: "endpoint".to_string(),
                    description: "API endpoint".to_string(),
                    required: true,
                    default_value: Some("hello".to_string()),
                    validation: None,
                }],
                examples: vec!["rust-api".to_string()],
            },
            CodeTemplate {
                id: "rust-model".to_string(),
                name: "Rust Model".to_string(),
                template_type: TemplateType::Model,
                language: "rust".to_string(),
                content: r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{Model}} {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
"#.to_string(),
                description: Some("Rust model template".to_string()),
                variables: vec![TemplateVariable {
                    name: "Model".to_string(),
                    description: "Model name".to_string(),
                    required: true,
                    default_value: Some("User".to_string()),
                    validation: None,
                }],
                examples: vec!["rust-model".to_string()],
            },
            CodeTemplate {
                id: "rust-service".to_string(),
                name: "Rust Service".to_string(),
                template_type: TemplateType::Service,
                language: "rust".to_string(),
                content: r#"pub struct {{Service}}Service {
    // Service dependencies
}

impl {{Service}}Service {
    pub fn new() -> Self {
        Self {
            // Initialize dependencies
        }
    }

    pub async fn get_all(&self) -> Result<Vec<{{Service}}>, Box<dyn std::error::Error>> {
        // Implementation
        Ok(Vec::new())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<{{Service}}, Box<dyn std::error::Error>> {
        // Implementation
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Not found")))
    }

    pub async fn create(&self, {{service}}: {{Service}}) -> Result<{{Service}}, Box<dyn std::error::Error>> {
        // Implementation
        Ok({{service}})
    }

    pub async fn update(&self, id: &str, {{service}}: {{Service}}) -> Result<{{Service}}, Box<dyn std::error::Error>> {
        // Implementation
        Ok({{service}})
    }

    pub async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation
        Ok(())
    }
}
"#.to_string(),
                description: Some("Rust service template".to_string()),
                variables: vec![
                    TemplateVariable {
                        name: "Service".to_string(),
                        description: "Service name".to_string(),
                        required: true,
                        default_value: Some("User".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "service".to_string(),
                        description: "Service variable name".to_string(),
                        required: true,
                        default_value: Some("user".to_string()),
                        validation: None,
                    },
                ],
                examples: vec!["rust-service".to_string()],
            },
            CodeTemplate {
                id: "rust-middleware".to_string(),
                name: "Rust Middleware".to_string(),
                template_type: TemplateType::Middleware,
                language: "rust".to_string(),
                content: r#"use axum::middleware::Next;
use axum::response::Response;
use axum::Request;

pub async fn {{middleware}}_middleware(request: Request, next: Next) -> Response {
    // Pre-processing logic
    println!("Executing {{middleware}} middleware");
    
    let response = next.run(request).await;
    
    // Post-processing logic
    println!("{{middleware}} middleware completed");
    
    response
}
"#.to_string(),
                description: Some("Rust middleware template".to_string()),
                variables: vec![TemplateVariable {
                    name: "middleware".to_string(),
                    description: "Middleware name".to_string(),
                    required: true,
                    default_value: Some("logger".to_string()),
                    validation: None,
                }],
                examples: vec!["rust-middleware".to_string()],
            },
            CodeTemplate {
                id: "rust-config".to_string(),
                name: "Rust Config".to_string(),
                template_type: TemplateType::Config,
                language: "rust".to_string(),
                content: r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{Config}}Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub secret_key: String,
    pub debug: bool,
}

impl Default for {{Config}}Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3000,
            database_url: "sqlite://./db.sqlite".to_string(),
            secret_key: "your-secret-key-here".to_string(),
            debug: true,
        }
    }
}

impl {{Config}}Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Implementation for loading config from file or environment
        Ok(Self::default())
    }
}
"#.to_string(),
                description: Some("Rust config template".to_string()),
                variables: vec![TemplateVariable {
                    name: "Config".to_string(),
                    description: "Config name".to_string(),
                    required: true,
                    default_value: Some("App".to_string()),
                    validation: None,
                }],
                examples: vec!["rust-config".to_string()],
            },
            CodeTemplate {
                id: "rust-test".to_string(),
                name: "Rust Test".to_string(),
                template_type: TemplateType::Test,
                language: "rust".to_string(),
                content: r#"use super::*;

#[tokio::test]
async fn test_{{test_name}}() {
    // Test setup
    let result = 2 + 2;
    
    // Test assertion
    assert_eq!(result, 4);
}

#[tokio::test]
async fn test_{{test_name}}_error() {
    // Test error case
    let result = std::fs::read_to_string("non_existent_file.txt");
    
    // Test assertion
    assert!(result.is_err());
}
"#.to_string(),
                description: Some("Rust test template".to_string()),
                variables: vec![TemplateVariable {
                    name: "test_name".to_string(),
                    description: "Test name".to_string(),
                    required: true,
                    default_value: Some("basic".to_string()),
                    validation: None,
                }],
                examples: vec!["rust-test".to_string()],
            },
            CodeTemplate {
                id: "rust-controller".to_string(),
                name: "Rust Controller".to_string(),
                template_type: TemplateType::Controller,
                language: "rust".to_string(),
                content: r#"use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{Controller}}Request {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{Controller}}Response {
    pub id: String,
    pub name: String,
    pub value: String,
    pub status: String,
}

pub async fn create_{{controller}}(Json(request): Json<{{Controller}}Request>) -> Json<{{Controller}}Response> {
    // Implementation
    Json({{Controller}}Response {
        id: "1".to_string(),
        name: request.name,
        value: request.value,
        status: "created".to_string(),
    })
}

pub async fn get_{{controller}}(Path(id): Path<String>) -> Json<{{Controller}}Response> {
    // Implementation
    Json({{Controller}}Response {
        id,
        name: "Test".to_string(),
        value: "Test value".to_string(),
        status: "ok".to_string(),
    })
}

pub async fn update_{{controller}}(Path(id): Path<String>, Json(request): Json<{{Controller}}Request>) -> Json<{{Controller}}Response> {
    // Implementation
    Json({{Controller}}Response {
        id,
        name: request.name,
        value: request.value,
        status: "updated".to_string(),
    })
}

pub async fn delete_{{controller}}(Path(id): Path<String>) -> Json<{{Controller}}Response> {
    // Implementation
    Json({{Controller}}Response {
        id,
        name: "".to_string(),
        value: "".to_string(),
        status: "deleted".to_string(),
    })
}
"#.to_string(),
                description: Some("Rust controller template".to_string()),
                variables: vec![
                    TemplateVariable {
                        name: "Controller".to_string(),
                        description: "Controller name".to_string(),
                        required: true,
                        default_value: Some("Item".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "controller".to_string(),
                        description: "Controller variable name".to_string(),
                        required: true,
                        default_value: Some("item".to_string()),
                        validation: None,
                    },
                ],
                examples: vec!["rust-controller".to_string()],
            },
        ];

        for template in templates {
            self.register_template(template).await?;
        }

        Ok(())
    }

    /// 注册模板
    pub async fn register_template(&mut self, template: CodeTemplate) -> Result<(), Box<dyn std::error::Error>> {
        self.templates.insert(template.id.clone(), template);
        Ok(())
    }

    /// 生成代码
    pub async fn generate_code(
        &mut self,
        config: CodeGenerationConfig,
    ) -> Result<CodeGenerationResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();

        // 查找模板
        if let Some(template) = self.templates.get(&config.template_id) {
            // 渲染模板
            let rendered_content = self.render_template(template, &config.variables).await?;

            // 确保输出目录存在
            let output_path = Path::new(&config.output_path);
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // 检查文件是否存在
            if output_path.exists() && !config.overwrite {
                return Err(format!("File already exists: {}", config.output_path).into());
            }

            // 写入文件
            fs::write(output_path, rendered_content)?;

            let duration = start_time.elapsed().as_millis() as u64;

            Ok(CodeGenerationResult {
                template_id: config.template_id,
                output_path: config.output_path,
                success: true,
                files_generated: 1,
                message: Some("Code generated successfully".to_string()),
                duration_ms: duration,
            })
        } else {
            Err(format!("Template not found: {}", config.template_id).into())
        }
    }

    /// 渲染模板
    async fn render_template(
        &mut self,
        template: &CodeTemplate,
        variables: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 生成缓存键
        let mut cache_key = template.id.clone();
        for (name, value) in variables {
            cache_key.push_str(&format!("_{}={}", name, value));
        }

        // 检查缓存
        if self.cache_enabled && self.cache.contains_key(&cache_key) {
            return Ok(self.cache.get(&cache_key).unwrap().clone());
        }

        // 渲染模板
        let mut rendered = template.content.clone();

        for (name, value) in variables {
            let placeholder = format!("{{{{{}}}}}", name);
            rendered = rendered.replace(&placeholder, value);
        }

        // 缓存结果
        if self.cache_enabled {
            self.cache.insert(cache_key, rendered.clone());
        }

        Ok(rendered)
    }

    /// 获取模板
    pub async fn get_template(&self, template_id: &str) -> Option<CodeTemplate> {
        self.templates.get(template_id).cloned()
    }

    /// 获取所有模板
    pub async fn get_all_templates(&self) -> Vec<CodeTemplate> {
        self.templates.values().cloned().collect()
    }

    /// 更新模板
    pub async fn update_template(&mut self, template: CodeTemplate) -> Result<(), Box<dyn std::error::Error>> {
        if self.templates.contains_key(&template.id) {
            self.templates.insert(template.id.clone(), template);
            Ok(())
        } else {
            Err(format!("Template not found: {}", template.id).into())
        }
    }

    /// 删除模板
    pub async fn delete_template(&mut self, template_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.templates.remove(template_id).is_some() {
            Ok(())
        } else {
            Err(format!("Template not found: {}", template_id).into())
        }
    }

    /// 导出模板
    pub async fn export_template(
        &self,
        template_id: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(template) = self.templates.get(template_id) {
            let content = serde_json::to_string_pretty(template)?;
            fs::write(output_path, content)?;
            Ok(())
        } else {
            Err(format!("Template not found: {}", template_id).into())
        }
    }

    /// 导入模板
    pub async fn import_template(
        &mut self,
        input_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(input_path)?;
        let template: CodeTemplate = serde_json::from_str(&content)?;
        self.register_template(template).await
    }

    /// 清除缓存
    pub async fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 启用缓存
    pub async fn enable_cache(&mut self) {
        self.cache_enabled = true;
    }

    /// 禁用缓存
    pub async fn disable_cache(&mut self) {
        self.cache_enabled = false;
        self.cache.clear();
    }

    /// 获取缓存大小
    pub async fn get_cache_size(&self) -> usize {
        self.cache.len()
    }

    /// 注册其他语言的模板
    async fn register_additional_language_templates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 注册JavaScript模板
        let js_templates = vec![
            CodeTemplate {
                id: "js-api".to_string(),
                name: "JavaScript API".to_string(),
                template_type: TemplateType::Api,
                language: "javascript".to_string(),
                content: r#"const express = require('express');
const router = express.Router();

router.get('/api/{{endpoint}}', (req, res) => {
    res.json({ message: 'Hello, World!' });
});

module.exports = router;
"#.to_string(),
                description: Some("JavaScript API template".to_string()),
                variables: vec![TemplateVariable {
                    name: "endpoint".to_string(),
                    description: "API endpoint".to_string(),
                    required: true,
                    default_value: Some("hello".to_string()),
                    validation: None,
                }],
                examples: vec!["js-api".to_string()],
            },
            CodeTemplate {
                id: "js-model".to_string(),
                name: "JavaScript Model".to_string(),
                template_type: TemplateType::Model,
                language: "javascript".to_string(),
                content: r#"class {{Model}} {
    constructor(id, name) {
        this.id = id;
        this.name = name;
        this.createdAt = new Date().toISOString();
        this.updatedAt = new Date().toISOString();
    }

    toJSON() {
        return {
            id: this.id,
            name: this.name,
            createdAt: this.createdAt,
            updatedAt: this.updatedAt,
        };
    }
}

module.exports = {{Model}};
"#.to_string(),
                description: Some("JavaScript model template".to_string()),
                variables: vec![TemplateVariable {
                    name: "Model".to_string(),
                    description: "Model name".to_string(),
                    required: true,
                    default_value: Some("User".to_string()),
                    validation: None,
                }],
                examples: vec!["js-model".to_string()],
            },
        ];

        // 注册Python模板
        let python_templates = vec![
            CodeTemplate {
                id: "python-api".to_string(),
                name: "Python API".to_string(),
                template_type: TemplateType::Api,
                language: "python".to_string(),
                content: r#"from fastapi import FastAPI

app = FastAPI()

@app.get('/api/{{endpoint}}')
async def read_root():
    return {"message": "Hello, World!"}
"#.to_string(),
                description: Some("Python API template".to_string()),
                variables: vec![TemplateVariable {
                    name: "endpoint".to_string(),
                    description: "API endpoint".to_string(),
                    required: true,
                    default_value: Some("hello".to_string()),
                    validation: None,
                }],
                examples: vec!["python-api".to_string()],
            },
            CodeTemplate {
                id: "python-model".to_string(),
                name: "Python Model".to_string(),
                template_type: TemplateType::Model,
                language: "python".to_string(),
                content: r#"from datetime import datetime

class {{Model}}:
    def __init__(self, id, name):
        self.id = id
        self.name = name
        self.created_at = datetime.now().isoformat()
        self.updated_at = datetime.now().isoformat()

    def to_dict(self):
        return {
            'id': self.id,
            'name': self.name,
            'created_at': self.created_at,
            'updated_at': self.updated_at,
        }
"#.to_string(),
                description: Some("Python model template".to_string()),
                variables: vec![TemplateVariable {
                    name: "Model".to_string(),
                    description: "Model name".to_string(),
                    required: true,
                    default_value: Some("User".to_string()),
                    validation: None,
                }],
                examples: vec!["python-model".to_string()],
            },
        ];

        // 注册TypeScript模板
        let ts_templates = vec![
            CodeTemplate {
                id: "ts-api".to_string(),
                name: "TypeScript API".to_string(),
                template_type: TemplateType::Api,
                language: "typescript".to_string(),
                content: r#"import express from 'express';
const router = express.Router();

router.get('/api/{{endpoint}}', (req: express.Request, res: express.Response) => {
    res.json({ message: 'Hello, World!' });
});

export default router;
"#.to_string(),
                description: Some("TypeScript API template".to_string()),
                variables: vec![TemplateVariable {
                    name: "endpoint".to_string(),
                    description: "API endpoint".to_string(),
                    required: true,
                    default_value: Some("hello".to_string()),
                    validation: None,
                }],
                examples: vec!["ts-api".to_string()],
            },
            CodeTemplate {
                id: "ts-model".to_string(),
                name: "TypeScript Model".to_string(),
                template_type: TemplateType::Model,
                language: "typescript".to_string(),
                content: r#"export class {{Model}} {
    constructor(
        public id: string,
        public name: string,
        public createdAt: string = new Date().toISOString(),
        public updatedAt: string = new Date().toISOString()
    ) {}

    toJSON() {
        return {
            id: this.id,
            name: this.name,
            createdAt: this.createdAt,
            updatedAt: this.updatedAt,
        };
    }
}
"#.to_string(),
                description: Some("TypeScript model template".to_string()),
                variables: vec![TemplateVariable {
                    name: "Model".to_string(),
                    description: "Model name".to_string(),
                    required: true,
                    default_value: Some("User".to_string()),
                    validation: None,
                }],
                examples: vec!["ts-model".to_string()],
            },
        ];

        // 注册所有模板
        for template in js_templates.into_iter().chain(python_templates).chain(ts_templates) {
            self.register_template(template).await?;
        }

        Ok(())
    }
}