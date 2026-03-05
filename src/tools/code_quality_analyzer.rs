// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 代码质量分析工具模块
//! 用于检查代码风格和潜在问题

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};

/// 代码质量分析配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityAnalyzerConfig {
    /// 是否启用代码风格检查
    pub enable_style_check: bool,
    /// 是否启用潜在问题检测
    pub enable_issue_detection: bool,
    /// 是否启用复杂度分析
    pub enable_complexity_analysis: bool,
    /// 是否生成详细报告
    pub generate_detailed_report: bool,
    /// 报告保存路径
    pub report_path: String,
    /// 支持的编程语言
    pub supported_languages: Vec<String>,
    /// 忽略的文件和目录
    pub ignored_paths: Vec<String>,
    /// 代码风格规则配置
    pub style_rules: StyleRules,
    /// 复杂度阈值配置
    pub complexity_thresholds: ComplexityThresholds,
}

/// 代码风格规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleRules {
    /// 缩进空格数
    pub indent_size: u32,
    /// 是否使用制表符缩进
    pub use_tabs: bool,
    /// 最大行长度
    pub max_line_length: u32,
    /// 是否要求分号
    pub require_semicolons: bool,
    /// 命名规范配置
    pub naming_conventions: NamingConventions,
    /// 空白字符规则
    pub whitespace_rules: WhitespaceRules,
}

/// 命名规范
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingConventions {
    /// 变量命名规范（snake_case, camelCase, PascalCase）
    pub variable_case: String,
    /// 函数命名规范
    pub function_case: String,
    /// 类命名规范
    pub class_case: String,
    /// 常量命名规范
    pub constant_case: String,
}

/// 空白字符规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitespaceRules {
    /// 是否要求操作符周围有空格
    pub spaces_around_operators: bool,
    /// 是否要求括号内有空格
    pub spaces_in_parentheses: bool,
    /// 是否要求大括号放在新行
    pub braces_on_new_line: bool,
    /// 是否要求逗号后有空格
    pub spaces_after_comma: bool,
}

/// 复杂度阈值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityThresholds {
    /// 函数圈复杂度阈值
    pub cyclomatic_complexity: u32,
    /// 函数长度阈值
    pub function_length: u32,
    /// 嵌套深度阈值
    pub nesting_depth: u32,
    /// 参数数量阈值
    pub parameter_count: u32,
}

/// 代码问题类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    /// 代码风格问题
    Style,
    /// 潜在错误
    PotentialError,
    /// 性能问题
    Performance,
    /// 安全问题
    Security,
    /// 复杂度问题
    Complexity,
    /// 其他问题
    Other,
}

/// 问题严重程度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    /// 错误
    Error,
    /// 警告
    Warning,
    /// 提示
    Info,
}

/// 代码问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    /// 问题ID
    pub id: String,
    /// 问题类型
    pub issue_type: IssueType,
    /// 严重程度
    pub severity: Severity,
    /// 问题描述
    pub description: String,
    /// 文件路径
    pub file_path: String,
    /// 行号
    pub line: u32,
    /// 列号
    pub column: u32,
    /// 代码片段
    pub code_snippet: String,
    /// 修复建议
    pub fix_suggestion: String,
    /// 规则ID
    pub rule_id: String,
}

/// 代码复杂度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityInfo {
    /// 函数名称
    pub function_name: String,
    /// 圈复杂度
    pub cyclomatic_complexity: u32,
    /// 函数长度
    pub function_length: u32,
    /// 嵌套深度
    pub nesting_depth: u32,
    /// 参数数量
    pub parameter_count: u32,
    /// 文件路径
    pub file_path: String,
    /// 起始行号
    pub start_line: u32,
    /// 结束行号
    pub end_line: u32,
}

/// 代码质量分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityAnalysisResult {
    /// 分析ID
    pub analysis_id: String,
    /// 分析时间
    pub analysis_time: String,
    /// 项目路径
    pub project_path: String,
    /// 分析的文件数量
    pub files_analyzed: usize,
    /// 发现的问题数量
    pub issues_found: usize,
    /// 按类型统计的问题
    pub issues_by_type: std::collections::HashMap<String, usize>,
    /// 按严重程度统计的问题
    pub issues_by_severity: std::collections::HashMap<String, usize>,
    /// 代码问题列表
    pub issues: Vec<CodeIssue>,
    /// 复杂度分析结果
    pub complexity_info: Vec<ComplexityInfo>,
    /// 代码质量统计
    pub statistics: CodeQualityStatistics,
    /// 分析状态
    pub status: String,
    /// 分析持续时间(秒)
    pub duration: u64,
    /// 报告路径
    pub report_path: Option<String>,
}

/// 代码质量统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityStatistics {
    /// 按语言统计的文件数
    pub files_by_language: std::collections::HashMap<String, usize>,
    /// 平均代码行数 per file
    pub average_lines_per_file: f64,
    /// 平均函数复杂度
    pub average_function_complexity: f64,
    /// 平均函数长度
    pub average_function_length: f64,
    /// 超过复杂度阈值的函数数量
    pub complex_functions: usize,
    /// 代码质量评分 (0-100)
    pub quality_score: u32,
}

/// 代码质量分析报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityAnalysisReport {
    /// 报告ID
    pub report_id: String,
    /// 报告标题
    pub title: String,
    /// 报告时间
    pub report_time: String,
    /// 分析结果
    pub analysis_result: CodeQualityAnalysisResult,
    /// 摘要
    pub summary: String,
    /// 问题摘要
    pub issues_summary: IssuesSummary,
    /// 复杂度摘要
    pub complexity_summary: ComplexitySummary,
    /// 改进建议
    pub recommendations: Vec<String>,
    /// 代码质量趋势
    pub quality_trend: Option<Vec<QualityTrendPoint>>,
}

/// 问题摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesSummary {
    /// 错误数量
    pub error_count: usize,
    /// 警告数量
    pub warning_count: usize,
    /// 提示数量
    pub info_count: usize,
    /// 最常见的问题类型
    pub top_issue_types: Vec<(String, usize)>,
    /// 最严重的问题
    pub top_severe_issues: Vec<CodeIssue>,
}

/// 复杂度摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexitySummary {
    /// 最大圈复杂度
    pub max_cyclomatic_complexity: u32,
    /// 最大函数长度
    pub max_function_length: u32,
    /// 最大嵌套深度
    pub max_nesting_depth: u32,
    /// 最复杂的函数
    pub most_complex_functions: Vec<ComplexityInfo>,
}

/// 代码质量趋势点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrendPoint {
    /// 时间点
    pub timestamp: String,
    /// 代码质量评分
    pub quality_score: u32,
    /// 问题数量
    pub issue_count: usize,
    /// 平均复杂度
    pub average_complexity: f64,
}

/// 代码质量分析工具
#[derive(Debug, Clone)]
pub struct CodeQualityAnalyzer {
    /// 配置
    config: CodeQualityAnalyzerConfig,
    /// 分析历史
    analysis_history: std::sync::Arc<tokio::sync::RwLock<Vec<CodeQualityAnalysisResult>>>,
}

impl CodeQualityAnalyzer {
    /// 创建新的代码质量分析工具
    pub fn new(config: CodeQualityAnalyzerConfig) -> Self {
        Self {
            config,
            analysis_history: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// 初始化代码质量分析工具
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 确保报告路径存在
        std::fs::create_dir_all(&self.config.report_path).map_err(|e| format!("创建报告路径失败: {}", e))?;
        
        Ok(())
    }

    /// 分析项目代码质量
    pub async fn analyze_code_quality(&mut self, project_path: &str) -> Result<CodeQualityAnalysisResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let analysis_id = format!("analysis-{}-{}", chrono::Utc::now().timestamp(), rand::random::<u32>());
        
        let project_path = Path::new(project_path).canonicalize()?;
        
        // 收集项目文件
        let files = self.collect_files(&project_path).await?;
        
        // 分析每个文件
        let mut all_issues = Vec::new();
        let mut all_complexity_info = Vec::new();
        
        for file_path in &files {
            let issues = self.analyze_file(file_path).await?;
            all_issues.extend(issues);
            
            if self.config.enable_complexity_analysis {
                let complexity_info = self.analyze_file_complexity(file_path).await?;
                all_complexity_info.extend(complexity_info);
            }
        }
        
        // 生成统计信息
        let statistics = self.generate_statistics(&files, &all_issues, &all_complexity_info).await;
        
        // 按类型和严重程度统计问题
        let mut issues_by_type = std::collections::HashMap::new();
        let mut issues_by_severity = std::collections::HashMap::new();
        
        for issue in &all_issues {
            let type_key = format!("{:?}", issue.issue_type);
            *issues_by_type.entry(type_key).or_insert(0) += 1;
            
            let severity_key = format!("{:?}", issue.severity);
            *issues_by_severity.entry(severity_key).or_insert(0) += 1;
        }
        
        // 生成分析结果
        let analysis_result = CodeQualityAnalysisResult {
            analysis_id: analysis_id.clone(),
            analysis_time: chrono::Utc::now().to_string(),
            project_path: project_path.to_string_lossy().to_string(),
            files_analyzed: files.len(),
            issues_found: all_issues.len(),
            issues_by_type,
            issues_by_severity,
            issues: all_issues,
            complexity_info: all_complexity_info,
            statistics,
            status: "completed".to_string(),
            duration: start_time.elapsed().as_secs(),
            report_path: None,
        };
        
        // 生成报告
        let report_path = if self.config.generate_detailed_report {
            Some(self.generate_report(&analysis_result).await?)
        } else {
            None
        };
        
        // 更新报告路径
        let mut analysis_result_with_report = analysis_result;
        analysis_result_with_report.report_path = report_path;
        
        // 保存分析结果
        self.save_analysis_result(&analysis_result_with_report).await?;
        
        Ok(analysis_result_with_report)
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

    /// 分析单个文件
    async fn analyze_file(&self, file_path: &Path) -> Result<Vec<CodeIssue>, Box<dyn std::error::Error>> {
        let mut issues = Vec::new();
        
        // 读取文件内容
        let content = self.read_file(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        // 检查代码风格
        if self.config.enable_style_check {
            let style_issues = self.check_style(file_path, &lines).await?;
            issues.extend(style_issues);
        }
        
        // 检测潜在问题
        if self.config.enable_issue_detection {
            let issue_issues = self.detect_issues(file_path, &lines).await?;
            issues.extend(issue_issues);
        }
        
        Ok(issues)
    }

    /// 读取文件内容
    fn read_file(&self, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 检查代码风格
    async fn check_style(&self, file_path: &Path, lines: &[&str]) -> Result<Vec<CodeIssue>, Box<dyn std::error::Error>> {
        let mut issues = Vec::new();
        
        // 检查缩进
        for (i, line) in lines.iter().enumerate() {
            let line_num = (i + 1) as u32;
            
            // 检查行长度
            if line.len() > self.config.style_rules.max_line_length as usize {
                let issue = CodeIssue {
                    id: format!("STYLE-{}", rand::random::<u32>()),
                    issue_type: IssueType::Style,
                    severity: Severity::Warning,
                    description: format!("行长度超过限制 ({} > {})", line.len(), self.config.style_rules.max_line_length),
                    file_path: file_path.to_string_lossy().to_string(),
                    line: line_num,
                    column: 0,
                    code_snippet: line.to_string(),
                    fix_suggestion: "缩短行长度或拆分长行".to_string(),
                    rule_id: "max-line-length".to_string(),
                };
                issues.push(issue);
            }
            
            // 检查缩进（简化实现）
            let leading_whitespace = line.len() - line.trim_start().len();
            if leading_whitespace % self.config.style_rules.indent_size as usize != 0 {
                let issue = CodeIssue {
                    id: format!("STYLE-{}", rand::random::<u32>()),
                    issue_type: IssueType::Style,
                    severity: Severity::Warning,
                    description: format!("缩进不符合规范 ({} 空格，应为 {} 的倍数)", leading_whitespace, self.config.style_rules.indent_size),
                    file_path: file_path.to_string_lossy().to_string(),
                    line: line_num,
                    column: 0,
                    code_snippet: line.to_string(),
                    fix_suggestion: format!("使用 {} 个空格的倍数进行缩进", self.config.style_rules.indent_size),
                    rule_id: "indentation".to_string(),
                };
                issues.push(issue);
            }
        }
        
        Ok(issues)
    }

    /// 检测潜在问题
    async fn detect_issues(&self, file_path: &Path, lines: &[&str]) -> Result<Vec<CodeIssue>, Box<dyn std::error::Error>> {
        let mut issues = Vec::new();
        
        // 检测未使用的变量（简化实现）
        let mut declared_variables = std::collections::HashSet::new();
        let mut used_variables = std::collections::HashSet::new();
        
        for (i, line) in lines.iter().enumerate() {
            let line_num = (i + 1) as u32;
            
            // 简单检测未使用的变量（实际应用中需要更复杂的解析）
            if line.contains("let ") && !line.contains("=") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let var_name = parts[1].trim_matches(';');
                    declared_variables.insert(var_name.to_string());
                }
            }
            
            // 检测死代码
            if line.contains("return") && i < lines.len() - 1 {
                let next_line = lines[i + 1].trim();
                if !next_line.is_empty() && !next_line.starts_with('}') {
                    let issue = CodeIssue {
                        id: format!("ISSUE-{}", rand::random::<u32>()),
                        issue_type: IssueType::PotentialError,
                        severity: Severity::Warning,
                        description: "return 语句后存在不可达代码",
                        file_path: file_path.to_string_lossy().to_string(),
                        line: line_num + 1,
                        column: 0,
                        code_snippet: next_line.to_string(),
                        fix_suggestion: "移除不可达代码或调整代码结构".to_string(),
                        rule_id: "unreachable-code".to_string(),
                    };
                    issues.push(issue);
                }
            }
        }
        
        // 检测未使用的变量
        for var in &declared_variables {
            if !used_variables.contains(var) {
                let issue = CodeIssue {
                    id: format!("ISSUE-{}", rand::random::<u32>()),
                    issue_type: IssueType::Style,
                    severity: Severity::Info,
                    description: format!("变量 {} 可能未使用", var),
                    file_path: file_path.to_string_lossy().to_string(),
                    line: 0,
                    column: 0,
                    code_snippet: "".to_string(),
                    fix_suggestion: "移除未使用的变量或使用它".to_string(),
                    rule_id: "unused-variable".to_string(),
                };
                issues.push(issue);
            }
        }
        
        Ok(issues)
    }

    /// 分析文件复杂度
    async fn analyze_file_complexity(&self, file_path: &Path) -> Result<Vec<ComplexityInfo>, Box<dyn std::error::Error>> {
        let mut complexity_info = Vec::new();
        
        // 读取文件内容
        let content = self.read_file(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        // 简单检测函数（实际应用中需要更复杂的解析）
        let mut in_function = false;
        let mut function_name = String::new();
        let mut function_start = 0;
        let mut function_lines = 0;
        let mut cyclomatic_complexity = 1; // 基础复杂度为1
        let mut max_nesting_depth = 0;
        let mut current_nesting = 0;
        let mut parameter_count = 0;
        
        for (i, line) in lines.iter().enumerate() {
            let line_num = (i + 1) as u32;
            
            // 简单检测函数定义（实际应用中需要更复杂的解析）
            if line.contains("fn ") && !in_function {
                in_function = true;
                function_start = line_num;
                function_lines = 1;
                cyclomatic_complexity = 1;
                max_nesting_depth = 0;
                current_nesting = 0;
                parameter_count = 0;
                
                // 提取函数名（简化实现）
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let func_part = parts[1];
                    if let Some(paren_idx) = func_part.find('(') {
                        function_name = func_part[..paren_idx].to_string();
                        
                        // 简单计算参数数量
                        let params_part = &func_part[paren_idx..];
                        if let Some(end_paren_idx) = params_part.find(')') {
                            let params = &params_part[1..end_paren_idx];
                            if !params.is_empty() {
                                parameter_count = params.split(',').count() as u32;
                            }
                        }
                    }
                }
            } else if in_function {
                function_lines += 1;
                
                // 增加圈复杂度的情况
                if line.contains("if ") || line.contains("for ") || line.contains("while ") || 
                   line.contains("match ") || line.contains("&&") || line.contains("||") {
                    cyclomatic_complexity += 1;
                }
                
                // 计算嵌套深度
                let open_braces = line.matches('{').count();
                let close_braces = line.matches('}').count();
                current_nesting += open_braces - close_braces;
                max_nesting_depth = max_nesting_depth.max(current_nesting as u32);
                
                // 检测函数结束
                if close_braces > 0 && current_nesting <= 0 {
                    in_function = false;
                    
                    let info = ComplexityInfo {
                        function_name: function_name.clone(),
                        cyclomatic_complexity,
                        function_length: function_lines,
                        nesting_depth: max_nesting_depth,
                        parameter_count,
                        file_path: file_path.to_string_lossy().to_string(),
                        start_line: function_start,
                        end_line: line_num,
                    };
                    complexity_info.push(info);
                }
            }
        }
        
        Ok(complexity_info)
    }

    /// 生成统计信息
    async fn generate_statistics(&self, files: &Vec<PathBuf>, issues: &Vec<CodeIssue>, complexity_info: &Vec<ComplexityInfo>) -> CodeQualityStatistics {
        let total_files = files.len();
        let total_lines = files.iter().map(|f| {
            if let Ok(content) = self.read_file(f) {
                content.lines().count() as u64
            } else {
                0
            }
        }).sum::<u64>();
        
        let average_lines_per_file = if total_files > 0 {
            total_lines as f64 / total_files as f64
        } else {
            0.0
        };
        
        let total_complexity: u32 = complexity_info.iter().map(|ci| ci.cyclomatic_complexity).sum();
        let average_function_complexity = if complexity_info.len() > 0 {
            total_complexity as f64 / complexity_info.len() as f64
        } else {
            0.0
        };
        
        let total_function_length: u32 = complexity_info.iter().map(|ci| ci.function_length).sum();
        let average_function_length = if complexity_info.len() > 0 {
            total_function_length as f64 / complexity_info.len() as f64
        } else {
            0.0
        };
        
        let complex_functions = complexity_info.iter()
            .filter(|ci| ci.cyclomatic_complexity > self.config.complexity_thresholds.cyclomatic_complexity)
            .count();
        
        // 计算代码质量评分 (0-100)
        let quality_score = self.calculate_quality_score(total_files, issues.len(), complex_functions, total_lines).await;
        
        // 按语言统计文件数
        let mut files_by_language = std::collections::HashMap::new();
        for file in files {
            if let Some(ext) = file.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                let language = self.get_language_from_extension(&ext_str);
                *files_by_language.entry(language).or_insert(0) += 1;
            }
        }
        
        CodeQualityStatistics {
            files_by_language,
            average_lines_per_file,
            average_function_complexity,
            average_function_length,
            complex_functions,
            quality_score,
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

    /// 计算代码质量评分
    async fn calculate_quality_score(&self, total_files: usize, issue_count: usize, complex_functions: usize, total_lines: u64) -> u32 {
        // 简化的评分算法
        let mut score = 100;
        
        // 问题数量影响
        let issues_per_1000_lines = if total_lines > 0 {
            (issue_count as f64 * 1000.0) / total_lines as f64
        } else {
            0.0
        };
        
        if issues_per_1000_lines > 10.0 {
            score -= 30;
        } else if issues_per_1000_lines > 5.0 {
            score -= 15;
        } else if issues_per_1000_lines > 1.0 {
            score -= 5;
        }
        
        // 复杂函数影响
        if complex_functions > total_files {
            score -= 20;
        } else if complex_functions > 0 {
            score -= complex_functions * 2;
        }
        
        // 确保分数在0-100之间
        score = score.max(0).min(100);
        
        score as u32
    }

    /// 生成报告
    async fn generate_report(&self, analysis_result: &CodeQualityAnalysisResult) -> Result<String, Box<dyn std::error::Error>> {
        let report_id = format!("report-{}", analysis_result.analysis_id);
        let report_filename = format!("{}/{}.json", self.config.report_path, report_id);
        
        // 生成问题摘要
        let issues_summary = self.generate_issues_summary(&analysis_result.issues).await;
        
        // 生成复杂度摘要
        let complexity_summary = self.generate_complexity_summary(&analysis_result.complexity_info).await;
        
        // 生成改进建议
        let recommendations = self.generate_recommendations(analysis_result).await;
        
        // 生成报告
        let report = CodeQualityAnalysisReport {
            report_id,
            title: format!("代码质量分析报告 - {}", analysis_result.analysis_time),
            report_time: chrono::Utc::now().to_string(),
            analysis_result: analysis_result.clone(),
            summary: self.generate_summary(analysis_result).await,
            issues_summary,
            complexity_summary,
            recommendations,
            quality_trend: None,
        };
        
        // 保存报告
        let report_json = serde_json::to_string_pretty(&report)?;
        std::fs::write(&report_filename, report_json).map_err(|e| format!("保存报告失败: {}", e))?;
        
        Ok(report_filename)
    }

    /// 生成问题摘要
    async fn generate_issues_summary(&self, issues: &Vec<CodeIssue>) -> IssuesSummary {
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;
        let mut issues_by_type = std::collections::HashMap::new();
        let mut all_issues = Vec::new();
        
        for issue in issues {
            match issue.severity {
                Severity::Error => error_count += 1,
                Severity::Warning => warning_count += 1,
                Severity::Info => info_count += 1,
            }
            
            let type_key = format!("{:?}", issue.issue_type);
            *issues_by_type.entry(type_key).or_insert(0) += 1;
            all_issues.push(issue.clone());
        }
        
        // 按严重程度排序，取前5个
        all_issues.sort_by(|a, b| {
            let severity_order = vec![Severity::Error, Severity::Warning, Severity::Info];
            let a_idx = severity_order.iter().position(|&s| s == a.severity).unwrap_or(3);
            let b_idx = severity_order.iter().position(|&s| s == b.severity).unwrap_or(3);
            a_idx.cmp(&b_idx)
        });
        let top_severe_issues = all_issues.into_iter().take(5).collect();
        
        // 按问题类型排序，取前5个
        let mut sorted_issue_types: Vec<(String, usize)> = issues_by_type.into_iter().collect();
        sorted_issue_types.sort_by(|a, b| b.1.cmp(&a.1));
        let top_issue_types = sorted_issue_types.into_iter().take(5).collect();
        
        IssuesSummary {
            error_count,
            warning_count,
            info_count,
            top_issue_types,
            top_severe_issues,
        }
    }

    /// 生成复杂度摘要
    async fn generate_complexity_summary(&self, complexity_info: &Vec<ComplexityInfo>) -> ComplexitySummary {
        let mut max_cyclomatic_complexity = 0;
        let mut max_function_length = 0;
        let mut max_nesting_depth = 0;
        let mut all_functions = Vec::new();
        
        for ci in complexity_info {
            max_cyclomatic_complexity = max_cyclomatic_complexity.max(ci.cyclomatic_complexity);
            max_function_length = max_function_length.max(ci.function_length);
            max_nesting_depth = max_nesting_depth.max(ci.nesting_depth);
            all_functions.push(ci.clone());
        }
        
        // 按圈复杂度排序，取前5个
        all_functions.sort_by(|a, b| b.cyclomatic_complexity.cmp(&a.cyclomatic_complexity));
        let most_complex_functions = all_functions.into_iter().take(5).collect();
        
        ComplexitySummary {
            max_cyclomatic_complexity,
            max_function_length,
            max_nesting_depth,
            most_complex_functions,
        }
    }

    /// 生成改进建议
    async fn generate_recommendations(&self, analysis_result: &CodeQualityAnalysisResult) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // 基于分析结果生成建议
        if analysis_result.issues_found > 0 {
            recommendations.push("修复发现的代码问题，提高代码质量".to_string());
        }
        
        if analysis_result.statistics.complex_functions > 0 {
            recommendations.push("重构复杂函数，降低圈复杂度和嵌套深度".to_string());
        }
        
        if analysis_result.statistics.quality_score < 80 {
            recommendations.push("整体改进代码质量，关注代码风格和潜在问题".to_string());
        }
        
        recommendations.push("定期运行代码质量分析，持续改进代码质量".to_string());
        recommendations.push("建立代码审查制度，确保代码质量".to_string());
        recommendations.push("使用自动化工具辅助代码质量检查".to_string());
        
        recommendations
    }

    /// 生成摘要
    async fn generate_summary(&self, analysis_result: &CodeQualityAnalysisResult) -> String {
        format!(
            "本次代码质量分析于{}完成，分析了{}个文件。发现了{}个问题，其中错误{}个，警告{}个，提示{}个。代码质量评分为{}分（满分100分）。分析持续时间{}秒。",
            analysis_result.analysis_time,
            analysis_result.files_analyzed,
            analysis_result.issues_found,
            analysis_result.issues_by_severity.get("Error").unwrap_or(&0),
            analysis_result.issues_by_severity.get("Warning").unwrap_or(&0),
            analysis_result.issues_by_severity.get("Info").unwrap_or(&0),
            analysis_result.statistics.quality_score,
            analysis_result.duration
        )
    }

    /// 保存分析结果
    async fn save_analysis_result(&self, analysis_result: &CodeQualityAnalysisResult) -> Result<(), Box<dyn std::error::Error>> {
        let mut analysis_history = self.analysis_history.write().await;
        analysis_history.push(analysis_result.clone());
        
        // 限制历史记录数量
        if analysis_history.len() > 50 { // 保留最近50条记录
            analysis_history.drain(0..analysis_history.len() - 50);
        }
        
        Ok(())
    }

    /// 获取分析历史
    pub async fn get_analysis_history(&self) -> Result<Vec<CodeQualityAnalysisResult>, Box<dyn std::error::Error>> {
        let analysis_history = self.analysis_history.read().await;
        Ok(analysis_history.clone())
    }

    /// 获取最新的分析结果
    pub async fn get_latest_analysis(&self) -> Result<Option<CodeQualityAnalysisResult>, Box<dyn std::error::Error>> {
        let analysis_history = self.analysis_history.read().await;
        Ok(analysis_history.last().cloned())
    }
}

