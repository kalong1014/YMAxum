// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 文档生成工具模块
//! 用于基于代码注释自动生成API文档

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read, Write};

/// 文档生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationGeneratorConfig {
    /// 是否启用代码注释解析
    pub enable_comment_parsing: bool,
    /// 是否生成Markdown格式文档
    pub generate_markdown: bool,
    /// 是否生成HTML格式文档
    pub generate_html: bool,
    /// 是否生成JSON格式文档
    pub generate_json: bool,
    /// 文档输出路径
    pub output_path: String,
    /// 支持的编程语言
    pub supported_languages: Vec<String>,
    /// 忽略的文件和目录
    pub ignored_paths: Vec<String>,
    /// 文档模板配置
    pub template_config: TemplateConfig,
    /// 文档版本控制配置
    pub version_control: VersionControlConfig,
}

/// 文档模板配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Markdown模板路径
    pub markdown_template: Option<String>,
    /// HTML模板路径
    pub html_template: Option<String>,
    /// 是否使用默认模板
    pub use_default_templates: bool,
    /// 文档主题
    pub theme: String,
}

/// 版本控制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfig {
    /// 是否启用版本控制
    pub enabled: bool,
    /// 当前文档版本
    pub current_version: String,
    /// 版本历史路径
    pub history_path: String,
    /// 是否自动检测版本变更
    pub auto_detect_changes: bool,
}

/// 文档元素类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocElementType {
    /// 模块
    Module,
    /// 函数
    Function,
    /// 结构体
    Struct,
    /// 枚举
    Enum,
    /// 特质
    Trait,
    /// 常量
    Constant,
    /// 类型别名
    TypeAlias,
    /// 其他
    Other,
}

/// 文档元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocElement {
    /// 元素ID
    pub id: String,
    /// 元素名称
    pub name: String,
    /// 元素类型
    pub element_type: DocElementType,
    /// 文档注释
    pub documentation: String,
    /// 文件路径
    pub file_path: String,
    /// 行号
    pub line: u32,
    /// 列号
    pub column: u32,
    /// 函数签名（如果是函数）
    pub signature: Option<String>,
    /// 参数信息（如果是函数）
    pub parameters: Option<Vec<ParameterInfo>>,
    /// 返回类型（如果是函数）
    pub return_type: Option<String>,
    /// 结构体字段（如果是结构体）
    pub fields: Option<Vec<FieldInfo>>,
    /// 枚举变体（如果是枚举）
    pub variants: Option<Vec<VariantInfo>>,
    /// 特质方法（如果是特质）
    pub methods: Option<Vec<MethodInfo>>,
    /// 依赖关系
    pub dependencies: Vec<String>,
    /// 示例代码
    pub examples: Vec<String>,
    /// 版本信息
    pub version_info: VersionInfo,
}

/// 参数信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    /// 参数名称
    pub name: String,
    /// 参数类型
    pub param_type: String,
    /// 参数文档
    pub documentation: String,
    /// 是否可选
    pub optional: bool,
    /// 默认值
    pub default_value: Option<String>,
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub field_type: String,
    /// 字段文档
    pub documentation: String,
    /// 是否可选
    pub optional: bool,
    /// 默认值
    pub default_value: Option<String>,
}

/// 枚举变体信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantInfo {
    /// 变体名称
    pub name: String,
    /// 变体文档
    pub documentation: String,
    /// 变体类型
    pub variant_type: Option<String>,
}

/// 方法信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodInfo {
    /// 方法名称
    pub name: String,
    /// 方法签名
    pub signature: String,
    /// 方法文档
    pub documentation: String,
    /// 参数信息
    pub parameters: Vec<ParameterInfo>,
    /// 返回类型
    pub return_type: Option<String>,
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// 创建版本
    pub created_version: String,
    /// 最后修改版本
    pub last_modified_version: String,
    /// 创建时间
    pub created_time: String,
    /// 最后修改时间
    pub last_modified_time: String,
    /// 变更说明
    pub change_description: String,
}

/// 文档模块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocModule {
    /// 模块名称
    pub name: String,
    /// 模块路径
    pub path: String,
    /// 模块文档
    pub documentation: String,
    /// 子模块
    pub submodules: Vec<DocModule>,
    /// 文档元素
    pub elements: Vec<DocElement>,
    /// 版本信息
    pub version_info: VersionInfo,
}

/// 文档生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationResult {
    /// 生成ID
    pub generation_id: String,
    /// 生成时间
    pub generation_time: String,
    /// 项目路径
    pub project_path: String,
    /// 分析的文件数量
    pub files_analyzed: usize,
    /// 生成的文档数量
    pub documents_generated: usize,
    /// 文档模块
    pub modules: Vec<DocModule>,
    /// 生成的文档路径
    pub generated_paths: Vec<String>,
    /// 文档统计
    pub statistics: DocumentationStatistics,
    /// 生成状态
    pub status: String,
    /// 生成持续时间(秒)
    pub duration: u64,
}

/// 文档统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationStatistics {
    /// 按类型统计的文档元素
    pub elements_by_type: std::collections::HashMap<String, usize>,
    /// 按语言统计的文件数
    pub files_by_language: std::collections::HashMap<String, usize>,
    /// 有文档注释的元素比例
    pub documented_elements_ratio: f64,
    /// 示例代码数量
    pub example_count: usize,
    /// 平均文档长度
    pub average_documentation_length: f64,
}

/// 文档生成工具
#[derive(Debug, Clone)]
pub struct DocumentationGenerator {
    /// 配置
    config: DocumentationGeneratorConfig,
    /// 生成历史
    generation_history: std::sync::Arc<tokio::sync::RwLock<Vec<DocumentationResult>>>,
}

impl DocumentationGenerator {
    /// 创建新的文档生成工具
    pub fn new(config: DocumentationGeneratorConfig) -> Self {
        Self {
            config,
            generation_history: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// 初始化文档生成工具
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 确保输出路径存在
        std::fs::create_dir_all(&self.config.output_path).map_err(|e| format!("创建输出路径失败: {}", e))?;
        
        // 如果启用版本控制，确保历史路径存在
        if self.config.version_control.enabled {
            std::fs::create_dir_all(&self.config.version_control.history_path).map_err(|e| format!("创建历史路径失败: {}", e))?;
        }
        
        Ok(())
    }

    /// 生成项目文档
    pub async fn generate_documentation(&mut self, project_path: &str) -> Result<DocumentationResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let generation_id = format!("gen-{}-{}", chrono::Utc::now().timestamp(), rand::random::<u32>());
        
        let project_path = Path::new(project_path).canonicalize()?;
        
        // 收集项目文件
        let files = self.collect_files(&project_path).await?;
        
        // 解析文件并提取文档元素
        let modules = self.parse_files(&files).await?;
        
        // 生成文档
        let generated_paths = self.generate_documents(&modules).await?;
        
        // 生成统计信息
        let statistics = self.generate_statistics(&modules, files.len()).await;
        
        // 生成结果
        let result = DocumentationResult {
            generation_id: generation_id.clone(),
            generation_time: chrono::Utc::now().to_string(),
            project_path: project_path.to_string_lossy().to_string(),
            files_analyzed: files.len(),
            documents_generated: generated_paths.len(),
            modules,
            generated_paths,
            statistics,
            status: "completed".to_string(),
            duration: start_time.elapsed().as_secs(),
        };
        
        // 保存生成结果
        self.save_generation_result(&result).await?;
        
        Ok(result)
    }

    /// 收集项目文件
    async fn collect_files(&self, project_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        self.walk_directory(project_path, &mut files).await?;
        Ok(files)
    }

    /// 遍历目录收集文件
    async fn walk_directory(&self, path: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        if path.is_dir() {
            // 检查是否需要忽略该目录
            let path_str = path.to_string_lossy().to_string();
            if self.config.ignored_paths.iter().any(|ignored| path_str.contains(ignored)) {
                return Ok(());
            }
            
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                
                if entry_path.is_dir() {
                    self.walk_directory(&entry_path, files).await?;
                } else if entry_path.is_file() {
                    // 检查文件是否支持的语言
                    if self.is_supported_file(&entry_path) {
                        files.push(entry_path);
                    }
                }
            }
        }
        
        Ok(())
    }

    /// 检查文件是否为支持的语言
    fn is_supported_file(&self, file_path: &Path) -> bool {
        // 根据文件扩展名判断语言
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            
            // 映射文件扩展名到语言
            let language_map = std::collections::HashMap::from([
                ("rs", "rust"),
                ("py", "python"),
                ("js", "javascript"),
                ("ts", "typescript"),
                ("jsx", "javascript"),
                ("tsx", "typescript"),
                ("c", "c"),
                ("cpp", "cpp"),
                ("h", "c"),
                ("hpp", "cpp"),
                ("go", "go"),
                ("java", "java"),
                ("cs", "csharp"),
            ]);
            
            if let Some(language) = language_map.get(ext_str.as_str()) {
                return self.config.supported_languages.contains(language);
            }
        }
        
        false
    }

    /// 解析文件并提取文档元素
    async fn parse_files(&self, files: &Vec<PathBuf>) -> Result<Vec<DocModule>, Box<dyn std::error::Error>> {
        let mut modules = Vec::new();
        
        for file_path in files {
            let file_modules = self.parse_file(file_path).await?;
            modules.extend(file_modules);
        }
        
        Ok(modules)
    }

    /// 解析单个文件
    async fn parse_file(&self, file_path: &Path) -> Result<Vec<DocModule>, Box<dyn std::error::Error>> {
        let mut modules = Vec::new();
        
        // 读取文件内容
        let content = self.read_file(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        // 根据文件扩展名选择解析器
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            
            match ext_str.as_str() {
                "rs" => {
                    // 解析Rust文件
                    let rust_modules = self.parse_rust_file(file_path, &lines).await?;
                    modules.extend(rust_modules);
                }
                "py" => {
                    // 解析Python文件
                    let python_modules = self.parse_python_file(file_path, &lines).await?;
                    modules.extend(python_modules);
                }
                _ => {
                    // 其他语言的解析（简化实现）
                }
            }
        }
        
        Ok(modules)
    }

    /// 解析Rust文件
    async fn parse_rust_file(&self, file_path: &Path, lines: &[&str]) -> Result<Vec<DocModule>, Box<dyn std::error::Error>> {
        let mut modules = Vec::new();
        let mut current_module = None;
        let mut current_elements = Vec::new();
        let mut current_documentation = String::new();
        let mut in_doc_comment = false;
        
        for (i, line) in lines.iter().enumerate() {
            let line_num = (i + 1) as u32;
            
            // 检测文档注释
            if line.starts_with("///") {
                in_doc_comment = true;
                // 提取文档注释内容
                let doc_content = line.trim_start_matches("///").trim();
                if !doc_content.is_empty() {
                    current_documentation.push_str(doc_content);
                    current_documentation.push_str("\n");
                }
            } else if line.starts_with("//! ") {
                // 模块级文档注释
                in_doc_comment = true;
                let doc_content = line.trim_start_matches("//!").trim();
                if !doc_content.is_empty() {
                    current_documentation.push_str(doc_content);
                    current_documentation.push_str("\n");
                }
            } else {
                if in_doc_comment {
                    // 文档注释结束，检查下一行是否是模块、函数、结构体等定义
                    if line.starts_with("mod ") {
                        // 解析模块定义
                        let module_name = self.extract_rust_identifier(line, "mod");
                        if !module_name.is_empty() {
                            let module = DocModule {
                                name: module_name,
                                path: file_path.to_string_lossy().to_string(),
                                documentation: current_documentation.clone(),
                                submodules: Vec::new(),
                                elements: Vec::new(),
                                version_info: self.create_version_info(),
                            };
                            modules.push(module);
                        }
                    } else if line.starts_with("fn ") {
                        // 解析函数定义
                        let function_name = self.extract_rust_identifier(line, "fn");
                        if !function_name.is_empty() {
                            let element = DocElement {
                                id: format!("{}-{}", function_name, rand::random::<u32>()),
                                name: function_name,
                                element_type: DocElementType::Function,
                                documentation: current_documentation.clone(),
                                file_path: file_path.to_string_lossy().to_string(),
                                line: line_num,
                                column: 0,
                                signature: Some(line.to_string()),
                                parameters: None, // 简化实现，实际需要解析参数
                                return_type: None, // 简化实现，实际需要解析返回类型
                                fields: None,
                                variants: None,
                                methods: None,
                                dependencies: Vec::new(),
                                examples: Vec::new(), // 简化实现，实际需要提取示例
                                version_info: self.create_version_info(),
                            };
                            current_elements.push(element);
                        }
                    } else if line.starts_with("struct ") {
                        // 解析结构体定义
                        let struct_name = self.extract_rust_identifier(line, "struct");
                        if !struct_name.is_empty() {
                            let element = DocElement {
                                id: format!("{}-{}", struct_name, rand::random::<u32>()),
                                name: struct_name,
                                element_type: DocElementType::Struct,
                                documentation: current_documentation.clone(),
                                file_path: file_path.to_string_lossy().to_string(),
                                line: line_num,
                                column: 0,
                                signature: Some(line.to_string()),
                                parameters: None,
                                return_type: None,
                                fields: None, // 简化实现，实际需要解析字段
                                variants: None,
                                methods: None,
                                dependencies: Vec::new(),
                                examples: Vec::new(),
                                version_info: self.create_version_info(),
                            };
                            current_elements.push(element);
                        }
                    } else if line.starts_with("enum ") {
                        // 解析枚举定义
                        let enum_name = self.extract_rust_identifier(line, "enum");
                        if !enum_name.is_empty() {
                            let element = DocElement {
                                id: format!("{}-{}", enum_name, rand::random::<u32>()),
                                name: enum_name,
                                element_type: DocElementType::Enum,
                                documentation: current_documentation.clone(),
                                file_path: file_path.to_string_lossy().to_string(),
                                line: line_num,
                                column: 0,
                                signature: Some(line.to_string()),
                                parameters: None,
                                return_type: None,
                                fields: None,
                                variants: None, // 简化实现，实际需要解析变体
                                methods: None,
                                dependencies: Vec::new(),
                                examples: Vec::new(),
                                version_info: self.create_version_info(),
                            };
                            current_elements.push(element);
                        }
                    } else if line.starts_with("trait ") {
                        // 解析特质定义
                        let trait_name = self.extract_rust_identifier(line, "trait");
                        if !trait_name.is_empty() {
                            let element = DocElement {
                                id: format!("{}-{}", trait_name, rand::random::<u32>()),
                                name: trait_name,
                                element_type: DocElementType::Trait,
                                documentation: current_documentation.clone(),
                                file_path: file_path.to_string_lossy().to_string(),
                                line: line_num,
                                column: 0,
                                signature: Some(line.to_string()),
                                parameters: None,
                                return_type: None,
                                fields: None,
                                variants: None,
                                methods: None, // 简化实现，实际需要解析方法
                                dependencies: Vec::new(),
                                examples: Vec::new(),
                                version_info: self.create_version_info(),
                            };
                            current_elements.push(element);
                        }
                    }
                    
                    // 重置文档注释
                    current_documentation.clear();
                    in_doc_comment = false;
                }
            }
        }
        
        // 处理文件级模块
        if current_elements.len() > 0 {
            let file_module = DocModule {
                name: file_path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                path: file_path.to_string_lossy().to_string(),
                documentation: current_documentation.clone(),
                submodules: Vec::new(),
                elements: current_elements,
                version_info: self.create_version_info(),
            };
            modules.push(file_module);
        }
        
        Ok(modules)
    }

    /// 解析Python文件
    async fn parse_python_file(&self, file_path: &Path, lines: &[&str]) -> Result<Vec<DocModule>, Box<dyn std::error::Error>> {
        let mut modules = Vec::new();
        let mut current_elements = Vec::new();
        let mut current_documentation = String::new();
        let mut in_doc_string = false;
        
        for (i, line) in lines.iter().enumerate() {
            let line_num = (i + 1) as u32;
            
            // 检测文档字符串
            if line.starts_with('"""') || line.starts_with("'''") {
                in_doc_string = !in_doc_string;
                // 提取文档字符串内容（简化实现）
                let doc_content = line.trim_start_matches('"""').trim_start_matches("'''");
                if !doc_content.is_empty() {
                    current_documentation.push_str(doc_content);
                    current_documentation.push_str("\n");
                }
            } else if in_doc_string {
                // 文档字符串内容
                current_documentation.push_str(line);
                current_documentation.push_str("\n");
            } else {
                // 检查函数、类定义
                if line.starts_with("def ") {
                    // 解析函数定义
                    let function_name = self.extract_python_identifier(line, "def");
                    if !function_name.is_empty() {
                        let element = DocElement {
                            id: format!("{}-{}", function_name, rand::random::<u32>()),
                            name: function_name,
                            element_type: DocElementType::Function,
                            documentation: current_documentation.clone(),
                            file_path: file_path.to_string_lossy().to_string(),
                            line: line_num,
                            column: 0,
                            signature: Some(line.to_string()),
                            parameters: None,
                            return_type: None,
                            fields: None,
                            variants: None,
                            methods: None,
                            dependencies: Vec::new(),
                            examples: Vec::new(),
                            version_info: self.create_version_info(),
                        };
                        current_elements.push(element);
                        // 重置文档注释
                        current_documentation.clear();
                    }
                } else if line.starts_with("class ") {
                    // 解析类定义
                    let class_name = self.extract_python_identifier(line, "class");
                    if !class_name.is_empty() {
                        let element = DocElement {
                            id: format!("{}-{}", class_name, rand::random::<u32>()),
                            name: class_name,
                            element_type: DocElementType::Struct,
                            documentation: current_documentation.clone(),
                            file_path: file_path.to_string_lossy().to_string(),
                            line: line_num,
                            column: 0,
                            signature: Some(line.to_string()),
                            parameters: None,
                            return_type: None,
                            fields: None,
                            variants: None,
                            methods: None,
                            dependencies: Vec::new(),
                            examples: Vec::new(),
                            version_info: self.create_version_info(),
                        };
                        current_elements.push(element);
                        // 重置文档注释
                        current_documentation.clear();
                    }
                }
            }
        }
        
        // 处理文件级模块
        if current_elements.len() > 0 {
            let file_module = DocModule {
                name: file_path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                path: file_path.to_string_lossy().to_string(),
                documentation: current_documentation.clone(),
                submodules: Vec::new(),
                elements: current_elements,
                version_info: self.create_version_info(),
            };
            modules.push(file_module);
        }
        
        Ok(modules)
    }

    /// 提取Rust标识符
    fn extract_rust_identifier(&self, line: &str, keyword: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == keyword {
            let identifier_part = parts[1];
            // 移除可能的括号
            if let Some(paren_idx) = identifier_part.find('(') {
                return identifier_part[..paren_idx].to_string();
            }
            // 移除可能的分号
            if let Some(semicolon_idx) = identifier_part.find(';') {
                return identifier_part[..semicolon_idx].to_string();
            }
            return identifier_part.to_string();
        }
        String::new()
    }

    /// 提取Python标识符
    fn extract_python_identifier(&self, line: &str, keyword: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == keyword {
            let identifier_part = parts[1];
            // 移除可能的括号
            if let Some(paren_idx) = identifier_part.find('(') {
                return identifier_part[..paren_idx].to_string();
            }
            return identifier_part.to_string();
        }
        String::new()
    }

    /// 创建版本信息
    fn create_version_info(&self) -> VersionInfo {
        VersionInfo {
            created_version: self.config.version_control.current_version.clone(),
            last_modified_version: self.config.version_control.current_version.clone(),
            created_time: chrono::Utc::now().to_string(),
            last_modified_time: chrono::Utc::now().to_string(),
            change_description: "Initial documentation".to_string(),
        }
    }

    /// 生成文档
    async fn generate_documents(&self, modules: &Vec<DocModule>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut generated_paths = Vec::new();
        
        // 生成Markdown文档
        if self.config.generate_markdown {
            let md_paths = self.generate_markdown_documents(modules).await?;
            generated_paths.extend(md_paths);
        }
        
        // 生成HTML文档
        if self.config.generate_html {
            let html_paths = self.generate_html_documents(modules).await?;
            generated_paths.extend(html_paths);
        }
        
        // 生成JSON文档
        if self.config.generate_json {
            let json_paths = self.generate_json_documents(modules).await?;
            generated_paths.extend(json_paths);
        }
        
        Ok(generated_paths)
    }

    /// 生成Markdown文档
    async fn generate_markdown_documents(&self, modules: &Vec<DocModule>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut generated_paths = Vec::new();
        
        for module in modules {
            let md_content = self.generate_markdown_content(module).await;
            let file_name = format!("{}.md", module.name.replace(':', '_'));
            let file_path = format!("{}/markdown/{}", self.config.output_path, file_name);
            
            // 确保目录存在
            std::fs::create_dir_all(format!("{}/markdown", self.config.output_path)).unwrap();
            
            // 写入文件
            let mut file = File::create(&file_path)?;
            file.write_all(md_content.as_bytes())?;
            
            generated_paths.push(file_path);
        }
        
        Ok(generated_paths)
    }

    /// 生成Markdown内容
    async fn generate_markdown_content(&self, module: &DocModule) -> String {
        let mut content = String::new();
        
        // 模块标题
        content.push_str(&format!("# {}\n\n", module.name));
        
        // 模块文档
        if !module.documentation.is_empty() {
            content.push_str(&module.documentation);
            content.push_str("\n\n");
        }
        
        // 模块路径
        content.push_str(&format!("## 模块路径\n{}\n\n", module.path));
        
        // 文档元素
        if !module.elements.is_empty() {
            content.push_str("## 文档元素\n\n");
            
            for element in &module.elements {
                // 元素标题
                let element_type_str = self.element_type_to_string(&element.element_type);
                content.push_str(&format!("### {}: {}\n\n", element_type_str, element.name));
                
                // 元素文档
                if !element.documentation.is_empty() {
                    content.push_str(&element.documentation);
                    content.push_str("\n\n");
                }
                
                // 元素签名
                if let Some(signature) = &element.signature {
                    content.push_str("#### 签名\n");
                    content.push_str(&format!("```rust\n{}\n```\n\n", signature));
                }
                
                // 元素位置
                content.push_str(&format!("#### 位置\n文件: {} (第{}行)\n\n", element.file_path, element.line));
            }
        }
        
        // 版本信息
        content.push_str("## 版本信息\n");
        content.push_str(&format!("- 创建版本: {}\n", module.version_info.created_version));
        content.push_str(&format!("- 最后修改版本: {}\n", module.version_info.last_modified_version));
        content.push_str(&format!("- 创建时间: {}\n", module.version_info.created_time));
        content.push_str(&format!("- 最后修改时间: {}\n", module.version_info.last_modified_time));
        
        content
    }

    /// 生成HTML文档
    async fn generate_html_documents(&self, modules: &Vec<DocModule>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut generated_paths = Vec::new();
        
        for module in modules {
            let html_content = self.generate_html_content(module).await;
            let file_name = format!("{}.html", module.name.replace(':', '_'));
            let file_path = format!("{}/html/{}", self.config.output_path, file_name);
            
            // 确保目录存在
            std::fs::create_dir_all(format!("{}/html", self.config.output_path)).unwrap();
            
            // 写入文件
            let mut file = File::create(&file_path)?;
            file.write_all(html_content.as_bytes())?;
            
            generated_paths.push(file_path);
        }
        
        Ok(generated_paths)
    }

    /// 生成HTML内容
    async fn generate_html_content(&self, module: &DocModule) -> String {
        let mut content = String::new();
        
        // HTML头部
        content.push_str("<!DOCTYPE html>\n");
        content.push_str("<html lang=\"zh-CN\">\n");
        content.push_str("<head>\n");
        content.push_str(&format!("<title>{}</title>\n", module.name));
        content.push_str("<style>\n");
        content.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        content.push_str("h1 { color: #333; }\n");
        content.push_str("h2 { color: #555; border-bottom: 1px solid #ddd; padding-bottom: 5px; }\n");
        content.push_str("h3 { color: #777; }\n");
        content.push_str("pre { background: #f4f4f4; padding: 10px; border-radius: 5px; }\n");
        content.push_str("code { background: #f4f4f4; padding: 2px 4px; border-radius: 3px; }\n");
        content.push_str("table { border-collapse: collapse; width: 100%; margin: 20px 0; }\n");
        content.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        content.push_str("th { background-color: #f2f2f2; }\n");
        content.push_str("</style>\n");
        content.push_str("</head>\n");
        content.push_str("<body>\n");
        
        // 模块标题
        content.push_str(&format!("<h1>{}</h1>\n", module.name));
        
        // 模块文档
        if !module.documentation.is_empty() {
            content.push_str(&format!("<div>{}</div>\n\n", module.documentation.replace('\n', '<br>')));
        }
        
        // 模块路径
        content.push_str("<h2>模块路径</h2>\n");
        content.push_str(&format!("<p>{}</p>\n\n", module.path));
        
        // 文档元素
        if !module.elements.is_empty() {
            content.push_str("<h2>文档元素</h2>\n\n");
            
            for element in &module.elements {
                // 元素标题
                let element_type_str = self.element_type_to_string(&element.element_type);
                content.push_str(&format!("<h3>{}: {}</h3>\n\n", element_type_str, element.name));
                
                // 元素文档
                if !element.documentation.is_empty() {
                    content.push_str(&format!("<div>{}</div>\n\n", element.documentation.replace('\n', '<br>')));
                }
                
                // 元素签名
                if let Some(signature) = &element.signature {
                    content.push_str("<h4>签名</h4>\n");
                    content.push_str(&format!("<pre><code>{}</code></pre>\n\n", signature));
                }
                
                // 元素位置
                content.push_str("<h4>位置</h4>\n");
                content.push_str(&format!("<p>文件: {} (第{}行)</p>\n\n", element.file_path, element.line));
            }
        }
        
        // 版本信息
        content.push_str("<h2>版本信息</h2>\n");
        content.push_str("<ul>\n");
        content.push_str(&format!("<li>创建版本: {}</li>\n", module.version_info.created_version));
        content.push_str(&format!("<li>最后修改版本: {}</li>\n", module.version_info.last_modified_version));
        content.push_str(&format!("<li>创建时间: {}</li>\n", module.version_info.created_time));
        content.push_str(&format!("<li>最后修改时间: {}</li>\n", module.version_info.last_modified_time));
        content.push_str("</ul>\n");
        
        // HTML尾部
        content.push_str("</body>\n");
        content.push_str("</html>\n");
        
        content
    }

    /// 生成JSON文档
    async fn generate_json_documents(&self, modules: &Vec<DocModule>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut generated_paths = Vec::new();
        
        let json_content = serde_json::to_string_pretty(modules)?;
        let file_path = format!("{}/documentation.json", self.config.output_path);
        
        // 写入文件
        let mut file = File::create(&file_path)?;
        file.write_all(json_content.as_bytes())?;
        
        generated_paths.push(file_path);
        
        Ok(generated_paths)
    }

    /// 元素类型转字符串
    fn element_type_to_string(&self, element_type: &DocElementType) -> String {
        match element_type {
            DocElementType::Module => "Module",
            DocElementType::Function => "Function",
            DocElementType::Struct => "Struct",
            DocElementType::Enum => "Enum",
            DocElementType::Trait => "Trait",
            DocElementType::Constant => "Constant",
            DocElementType::TypeAlias => "Type Alias",
            DocElementType::Other => "Other",
        }.to_string()
    }

    /// 生成统计信息
    async fn generate_statistics(&self, modules: &Vec<DocModule>, files_analyzed: usize) -> DocumentationStatistics {
        let mut elements_by_type = std::collections::HashMap::new();
        let mut files_by_language = std::collections::HashMap::new();
        let mut total_elements = 0;
        let mut documented_elements = 0;
        let mut total_documentation_length = 0;
        let mut example_count = 0;
        
        for module in modules {
            // 统计模块文件的语言
            if let Some(ext) = Path::new(&module.path).extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                let language = self.get_language_from_extension(&ext_str);
                *files_by_language.entry(language).or_insert(0) += 1;
            }
            
            // 统计模块元素
            for element in &module.elements {
                total_elements += 1;
                
                // 统计元素类型
                let element_type_str = self.element_type_to_string(&element.element_type);
                *elements_by_type.entry(element_type_str).or_insert(0) += 1;
                
                // 统计有文档的元素
                if !element.documentation.is_empty() {
                    documented_elements += 1;
                    total_documentation_length += element.documentation.len();
                }
                
                // 统计示例代码
                example_count += element.examples.len();
            }
        }
        
        // 计算有文档注释的元素比例
        let documented_elements_ratio = if total_elements > 0 {
            documented_elements as f64 / total_elements as f64
        } else {
            0.0
        };
        
        // 计算平均文档长度
        let average_documentation_length = if documented_elements > 0 {
            total_documentation_length as f64 / documented_elements as f64
        } else {
            0.0
        };
        
        DocumentationStatistics {
            elements_by_type,
            files_by_language,
            documented_elements_ratio,
            example_count,
            average_documentation_length,
        }
    }

    /// 从文件扩展名获取语言
    fn get_language_from_extension(&self, extension: &str) -> String {
        let language_map = std::collections::HashMap::from([
            ("rs", "rust"),
            ("py", "python"),
            ("js", "javascript"),
            ("ts", "typescript"),
            ("jsx", "javascript"),
            ("tsx", "typescript"),
            ("c", "c"),
            ("cpp", "cpp"),
            ("h", "c"),
            ("hpp", "cpp"),
            ("go", "go"),
            ("java", "java"),
            ("cs", "csharp"),
        ]);
        
        language_map.get(extension).unwrap_or(&"unknown").to_string()
    }

    /// 读取文件内容
    fn read_file(&self, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 保存生成结果
    async fn save_generation_result(&self, result: &DocumentationResult) -> Result<(), Box<dyn std::error::Error>> {
        let mut generation_history = self.generation_history.write().await;
        generation_history.push(result.clone());
        
        // 限制历史记录数量
        if generation_history.len() > 50 { // 保留最近50条记录
            generation_history.drain(0..generation_history.len() - 50);
        }
        
        Ok(())
    }

    /// 获取生成历史
    pub async fn get_generation_history(&self) -> Result<Vec<DocumentationResult>, Box<dyn std::error::Error>> {
        let generation_history = self.generation_history.read().await;
        Ok(generation_history.clone())
    }

    /// 获取最新的生成结果
    pub async fn get_latest_generation(&self) -> Result<Option<DocumentationResult>, Box<dyn std::error::Error>> {
        let generation_history = self.generation_history.read().await;
        Ok(generation_history.last().cloned())
    }
}

