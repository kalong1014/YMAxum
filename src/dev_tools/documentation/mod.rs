//! 文档生成器模块
//! 
//! 提供API文档、用户指南等文档的生成和管理功能

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 文档类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentationType {
    Api,
    UserGuide,
    DeveloperGuide,
    Architecture,
    InstallationGuide,
    ConfigurationGuide,
    Troubleshooting,
    ReleaseNotes,
    Other,
}

/// 文档格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentationFormat {
    Markdown,
    Html,
    Pdf,
    Json,
    Yaml,
    Xml,
    Other,
}

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationInfo {
    pub id: String,
    pub title: String,
    pub documentation_type: DocumentationType,
    pub format: DocumentationFormat,
    pub version: String,
    pub author: String,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub dependencies: Vec<String>,
    pub status: String,
}

/// 文档生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationGenerationConfig {
    pub documentation_id: String,
    pub output_path: String,
    pub format: DocumentationFormat,
    pub template: Option<String>,
    pub variables: HashMap<String, String>,
    pub overwrite: bool,
    pub include_toc: bool,
    pub include_examples: bool,
    pub include_version: bool,
}

/// 文档生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationGenerationResult {
    pub documentation_id: String,
    pub output_path: String,
    pub format: DocumentationFormat,
    pub success: bool,
    pub files_generated: u32,
    pub message: Option<String>,
    pub duration_ms: u64,
}

/// 文档生成器
#[derive(Debug, Clone)]
pub struct DocumentationGenerator {
    documentations: HashMap<String, DocumentationInfo>,
    doc_dir: String,
}

impl DocumentationGenerator {
    /// 创建新的文档生成器
    pub fn new() -> Self {
        Self {
            documentations: HashMap::new(),
            doc_dir: "./docs".to_string(),
        }
    }

    /// 初始化文档生成器
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化文档生成器
        self.load_documentations().await?;
        Ok(())
    }

    /// 加载文档
    async fn load_documentations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 加载文档
        let doc_dir = Path::new(&self.doc_dir);
        if doc_dir.exists() {
            // 加载目录中的文档
            info!("Loading documentations from {}", self.doc_dir);
        } else {
            // 创建文档目录
            fs::create_dir_all(doc_dir)?;
            info!("Created documentation directory: {}", self.doc_dir);
        }

        // 注册默认文档
        self.register_default_documentations().await?;

        Ok(())
    }

    /// 注册默认文档
    async fn register_default_documentations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 注册默认文档
        let documentations = vec![
            DocumentationInfo {
                id: "api-docs".to_string(),
                title: "API Documentation".to_string(),
                documentation_type: DocumentationType::Api,
                format: DocumentationFormat::Markdown,
                version: "1.0.0".to_string(),
                author: "YMAxum Team".to_string(),
                description: Some("API documentation for YMAxum".to_string()),
                keywords: vec!["api", "documentation", "ymaxum"].into_iter().map(|s| s.to_string()).collect(),
                dependencies: vec![],
                status: "active".to_string(),
            },
            DocumentationInfo {
                id: "user-guide".to_string(),
                title: "User Guide".to_string(),
                documentation_type: DocumentationType::UserGuide,
                format: DocumentationFormat::Markdown,
                version: "1.0.0".to_string(),
                author: "YMAxum Team".to_string(),
                description: Some("User guide for YMAxum".to_string()),
                keywords: vec!["user", "guide", "ymaxum"].into_iter().map(|s| s.to_string()).collect(),
                dependencies: vec![],
                status: "active".to_string(),
            },
            DocumentationInfo {
                id: "developer-guide".to_string(),
                title: "Developer Guide".to_string(),
                documentation_type: DocumentationType::DeveloperGuide,
                format: DocumentationFormat::Markdown,
                version: "1.0.0".to_string(),
                author: "YMAxum Team".to_string(),
                description: Some("Developer guide for YMAxum".to_string()),
                keywords: vec!["developer", "guide", "ymaxum"].into_iter().map(|s| s.to_string()).collect(),
                dependencies: vec![],
                status: "active".to_string(),
            },
        ];

        for doc in documentations {
            self.register_documentation(doc).await?;
        }

        Ok(())
    }

    /// 注册文档
    pub async fn register_documentation(&mut self, documentation: DocumentationInfo) -> Result<(), Box<dyn std::error::Error>> {
        self.documentations.insert(documentation.id.clone(), documentation);
        Ok(())
    }

    /// 生成文档
    pub async fn generate_documentation(
        &self,
        config: DocumentationGenerationConfig,
    ) -> Result<DocumentationGenerationResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();

        // 查找文档
        if let Some(doc) = self.documentations.get(&config.documentation_id) {
            // 生成文档内容
            let content = self.generate_documentation_content(doc, &config.variables, config.format).await?;

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
            fs::write(output_path, content)?;

            let duration = start_time.elapsed().as_millis() as u64;

            Ok(DocumentationGenerationResult {
                documentation_id: config.documentation_id,
                output_path: config.output_path,
                format: config.format,
                success: true,
                files_generated: 1,
                message: Some("Documentation generated successfully".to_string()),
                duration_ms: duration,
            })
        } else {
            Err(format!("Documentation not found: {}", config.documentation_id).into())
        }
    }

    /// 生成文档内容
    async fn generate_documentation_content(
        &self,
        doc: &DocumentationInfo,
        variables: &HashMap<String, String>,
        format: DocumentationFormat,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 生成基础内容
        let mut content = String::new();

        // 根据格式生成不同的内容
        match format {
            DocumentationFormat::Markdown => {
                // 添加标题
                content.push_str(&format!("# {}\n\n", doc.title));

                // 添加描述
                if let Some(description) = &doc.description {
                    content.push_str(&format!("{}\n\n", description));
                }

                // 添加版本
                content.push_str(&format!("## Version\n{}\n\n", doc.version));

                // 添加作者
                content.push_str(&format!("## Author\n{}\n\n", doc.author));

                // 添加关键词
                if !doc.keywords.is_empty() {
                    content.push_str("## Keywords\n");
                    for keyword in &doc.keywords {
                        content.push_str(&format!("- {}\n", keyword));
                    }
                    content.push_str("\n");
                }

                // 添加内容
                content.push_str("## Content\n");
                content.push_str("This is a placeholder for documentation content.\n");
                content.push_str("You can generate more detailed documentation using the documentation generator.\n");
            },
            DocumentationFormat::Html => {
                content.push_str(&format!(r#"<!DOCTYPE html>\n<html>\n<head>\n    <title>{}</title>\n    <style>\n        body {{ font-family: Arial, sans-serif; margin: 20px; }}\n        h1 {{ color: #333; }}\n        h2 {{ color: #666; }}\n        p {{ line-height: 1.6; }}\n        ul {{ margin-left: 20px; }}\n    </style>\n</head>\n<body>\n    <h1>{}</h1>\n"#, doc.title, doc.title));

                if let Some(description) = &doc.description {
                    content.push_str(&format!("    <p>{}</p>\n", description));
                }

                content.push_str(&format!(r#"    <h2>Version</h2>\n    <p>{}</p>\n    <h2>Author</h2>\n    <p>{}</p>\n"#, doc.version, doc.author));

                if !doc.keywords.is_empty() {
                    content.push_str("    <h2>Keywords</h2>\n    <ul>\n");
                    for keyword in &doc.keywords {
                        content.push_str(&format!("        <li>{}</li>\n", keyword));
                    }
                    content.push_str("    </ul>\n");
                }

                content.push_str(r#"    <h2>Content</h2>\n    <p>This is a placeholder for documentation content.</p>\n    <p>You can generate more detailed documentation using the documentation generator.</p>\n</body>\n</html>\n"#);
            },
            DocumentationFormat::Json => {
                use serde_json::json;
                let doc_json = json!({{
                    "title": doc.title,
                    "description": doc.description,
                    "version": doc.version,
                    "author": doc.author,
                    "keywords": doc.keywords,
                    "content": "This is a placeholder for documentation content. You can generate more detailed documentation using the documentation generator."
                }});
                content = serde_json::to_string_pretty(&doc_json)?;
            },
            DocumentationFormat::Yaml => {
                let mut yaml_content = String::new();
                yaml_content.push_str(&format!("title: {}\n", doc.title));
                if let Some(description) = &doc.description {
                    yaml_content.push_str(&format!("description: {}\n", description));
                }
                yaml_content.push_str(&format!("version: {}\n", doc.version));
                yaml_content.push_str(&format!("author: {}\n", doc.author));
                if !doc.keywords.is_empty() {
                    yaml_content.push_str("keywords:\n");
                    for keyword in &doc.keywords {
                        yaml_content.push_str(&format!("  - {}\n", keyword));
                    }
                }
                yaml_content.push_str("content: |\n  This is a placeholder for documentation content.\n  You can generate more detailed documentation using the documentation generator.\n");
                content = yaml_content;
            },
            _ => {
                // 默认格式
                content.push_str(&format!("# {}\n\n", doc.title));
                if let Some(description) = &doc.description {
                    content.push_str(&format!("{}\n\n", description));
                }
                content.push_str(&format!("Version: {}\nAuthor: {}\n", doc.version, doc.author));
                if !doc.keywords.is_empty() {
                    content.push_str("Keywords: ");
                    content.push_str(&doc.keywords.join(", "));
                    content.push_str("\n");
                }
                content.push_str("\nContent:\nThis is a placeholder for documentation content.\n");
            }
        }

        // 渲染变量
        for (name, value) in variables {
            let placeholder = format!("{{{{{}}}}}", name);
            content = content.replace(&placeholder, value);
        }

        Ok(content)
    }

    /// 获取文档
    pub async fn get_documentation(&self, documentation_id: &str) -> Option<DocumentationInfo> {
        self.documentations.get(documentation_id).cloned()
    }

    /// 获取所有文档
    pub async fn get_all_documentations(&self) -> Vec<DocumentationInfo> {
        self.documentations.values().cloned().collect()
    }

    /// 更新文档
    pub async fn update_documentation(&mut self, documentation: DocumentationInfo) -> Result<(), Box<dyn std::error::Error>> {
        if self.documentations.contains_key(&documentation.id) {
            self.documentations.insert(documentation.id.clone(), documentation);
            Ok(())
        } else {
            Err(format!("Documentation not found: {}", documentation.id).into())
        }
    }

    /// 删除文档
    pub async fn delete_documentation(&mut self, documentation_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.documentations.remove(documentation_id).is_some() {
            Ok(())
        } else {
            Err(format!("Documentation not found: {}", documentation_id).into())
        }
    }

    /// 导出文档
    pub async fn export_documentation(
        &self,
        documentation_id: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(doc) = self.documentations.get(documentation_id) {
            let content = serde_json::to_string_pretty(doc)?;
            fs::write(output_path, content)?;
            Ok(())
        } else {
            Err(format!("Documentation not found: {}", documentation_id).into())
        }
    }

    /// 导入文档
    pub async fn import_documentation(
        &mut self,
        input_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(input_path)?;
        let documentation: DocumentationInfo = serde_json::from_str(&content)?;
        self.register_documentation(documentation).await
    }
}