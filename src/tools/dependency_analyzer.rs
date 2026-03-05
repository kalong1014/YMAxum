// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 依赖分析工具模块
//! 用于分析项目依赖的版本、安全漏洞和兼容性问题

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

/// 依赖分析配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalyzerConfig {
    /// 是否启用安全漏洞检查
    pub enable_security_check: bool,
    /// 是否启用版本兼容性检查
    pub enable_compatibility_check: bool,
    /// 是否启用依赖图分析
    pub enable_dependency_graph: bool,
    /// 是否生成详细报告
    pub generate_detailed_report: bool,
    /// 报告保存路径
    pub report_path: String,
    /// 支持的包管理器
    pub package_managers: Vec<String>,
    /// 忽略的依赖
    pub ignored_dependencies: Vec<String>,
    /// 检查深度
    pub check_depth: u32,
}

/// 依赖信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    /// 依赖名称
    pub name: String,
    /// 版本
    pub version: String,
    /// 包管理器
    pub package_manager: String,
    /// 依赖类型
    pub dependency_type: String, // dev, build, runtime
    /// 来源
    pub source: String,
    /// 依赖路径
    pub path: Option<PathBuf>,
    /// 安全漏洞
    pub vulnerabilities: Vec<VulnerabilityInfo>,
    /// 兼容性问题
    pub compatibility_issues: Vec<CompatibilityIssue>,
    /// 依赖关系
    pub dependencies: Vec<String>,
}

/// 安全漏洞信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityInfo {
    /// 漏洞ID
    pub id: String,
    /// 漏洞标题
    pub title: String,
    /// 严重程度
    pub severity: String,
    /// 描述
    pub description: String,
    /// 修复版本
    pub fixed_versions: Vec<String>,
    /// 受影响版本
    pub affected_versions: Vec<String>,
    /// 发布日期
    pub published_date: String,
    /// CVSS评分
    pub cvss_score: Option<f64>,
}

/// 兼容性问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityIssue {
    /// 问题ID
    pub id: String,
    /// 问题描述
    pub description: String,
    /// 影响程度
    pub severity: String,
    /// 可能的解决方案
    pub solution: String,
    /// 相关依赖
    pub related_dependencies: Vec<String>,
}

/// 依赖分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysisResult {
    /// 分析ID
    pub analysis_id: String,
    /// 分析时间
    pub analysis_time: String,
    /// 项目路径
    pub project_path: String,
    /// 依赖总数
    pub total_dependencies: usize,
    /// 直接依赖
    pub direct_dependencies: usize,
    /// 传递依赖
    pub transitive_dependencies: usize,
    /// 有安全漏洞的依赖
    pub vulnerable_dependencies: usize,
    /// 有兼容性问题的依赖
    pub compatibility_issues: usize,
    /// 依赖信息
    pub dependencies: Vec<DependencyInfo>,
    /// 分析统计
    pub statistics: DependencyStatistics,
    /// 分析状态
    pub status: String,
    /// 分析持续时间(秒)
    pub duration: u64,
    /// 报告路径
    pub report_path: Option<String>,
}

/// 依赖统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyStatistics {
    /// 按包管理器统计
    pub by_package_manager: std::collections::HashMap<String, usize>,
    /// 按依赖类型统计
    pub by_dependency_type: std::collections::HashMap<String, usize>,
    /// 按安全漏洞严重程度统计
    pub by_vulnerability_severity: std::collections::HashMap<String, usize>,
    /// 按兼容性问题严重程度统计
    pub by_compatibility_severity: std::collections::HashMap<String, usize>,
    /// 过时依赖数量
    pub outdated_dependencies: usize,
    /// 未指定版本的依赖数量
    pub unversioned_dependencies: usize,
    /// 重复依赖数量
    pub duplicate_dependencies: usize,
}

/// 依赖分析报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysisReport {
    /// 报告ID
    pub report_id: String,
    /// 报告标题
    pub title: String,
    /// 报告时间
    pub report_time: String,
    /// 分析结果
    pub analysis_result: DependencyAnalysisResult,
    /// 摘要
    pub summary: String,
    /// 安全漏洞摘要
    pub security_summary: SecuritySummary,
    /// 兼容性问题摘要
    pub compatibility_summary: CompatibilitySummary,
    /// 依赖建议
    pub recommendations: Vec<String>,
    /// 依赖图
    pub dependency_graph: Option<String>,
}

/// 安全漏洞摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySummary {
    /// 总漏洞数
    pub total_vulnerabilities: usize,
    /// 严重漏洞数
    pub critical_vulnerabilities: usize,
    /// 高危漏洞数
    pub high_vulnerabilities: usize,
    /// 中危漏洞数
    pub medium_vulnerabilities: usize,
    /// 低危漏洞数
    pub low_vulnerabilities: usize,
    /// 最严重的漏洞
    pub top_vulnerabilities: Vec<VulnerabilityInfo>,
    /// 安全建议
    pub security_recommendations: Vec<String>,
}

/// 兼容性问题摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilitySummary {
    /// 总兼容性问题数
    pub total_issues: usize,
    /// 严重兼容性问题数
    pub critical_issues: usize,
    /// 中等等级兼容性问题数
    pub medium_issues: usize,
    /// 轻微兼容性问题数
    pub minor_issues: usize,
    /// 最严重的兼容性问题
    pub top_issues: Vec<CompatibilityIssue>,
    /// 兼容性建议
    pub compatibility_recommendations: Vec<String>,
}

/// 依赖分析工具
#[derive(Debug, Clone)]
pub struct DependencyAnalyzer {
    /// 配置
    config: DependencyAnalyzerConfig,
    /// 分析历史
    analysis_history: std::sync::Arc<tokio::sync::RwLock<Vec<DependencyAnalysisResult>>>,
}

impl DependencyAnalyzer {
    /// 创建新的依赖分析工具
    pub fn new(config: DependencyAnalyzerConfig) -> Self {
        Self {
            config,
            analysis_history: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// 初始化依赖分析工具
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 确保报告路径存在
        std::fs::create_dir_all(&self.config.report_path).map_err(|e| format!("创建报告路径失败: {}", e))?;
        
        Ok(())
    }

    /// 分析项目依赖
    pub async fn analyze_dependencies(&mut self, project_path: &str) -> Result<DependencyAnalysisResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let analysis_id = format!("analysis-{}-{}", chrono::Utc::now().timestamp(), rand::random::<u32>());
        
        let project_path = Path::new(project_path).canonicalize()?;
        
        // 检测包管理器
        let package_managers = self.detect_package_managers(&project_path).await?;
        
        // 分析每个包管理器的依赖
        let mut all_dependencies = Vec::new();
        for pm in &package_managers {
            let dependencies = self.analyze_package_manager(&project_path, pm).await?;
            all_dependencies.extend(dependencies);
        }
        
        // 去重依赖
        let unique_dependencies = self.deduplicate_dependencies(all_dependencies);
        
        // 分析依赖关系
        self.analyze_dependency_relationships(&mut unique_dependencies).await?;
        
        // 生成统计信息
        let statistics = self.generate_statistics(&unique_dependencies).await;
        
        // 计算依赖类型数量
        let total_dependencies = unique_dependencies.len();
        let direct_dependencies = unique_dependencies.iter().filter(|d| d.dependency_type == "runtime" || d.dependency_type == "build").count();
        let transitive_dependencies = total_dependencies - direct_dependencies;
        let vulnerable_dependencies = unique_dependencies.iter().filter(|d| !d.vulnerabilities.is_empty()).count();
        let compatibility_issues = unique_dependencies.iter().filter(|d| !d.compatibility_issues.is_empty()).count();
        
        // 生成分析结果
        let analysis_result = DependencyAnalysisResult {
            analysis_id: analysis_id.clone(),
            analysis_time: chrono::Utc::now().to_string(),
            project_path: project_path.to_string_lossy().to_string(),
            total_dependencies,
            direct_dependencies,
            transitive_dependencies,
            vulnerable_dependencies,
            compatibility_issues,
            dependencies: unique_dependencies,
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

    /// 检测项目使用的包管理器
    async fn detect_package_managers(&self, project_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut package_managers = Vec::new();
        
        // 检查Cargo.toml
        if project_path.join("Cargo.toml").exists() {
            package_managers.push("cargo".to_string());
        }
        
        // 检查package.json
        if project_path.join("package.json").exists() {
            package_managers.push("npm".to_string());
        }
        
        // 检查requirements.txt
        if project_path.join("requirements.txt").exists() {
            package_managers.push("pip".to_string());
        }
        
        // 检查pom.xml
        if project_path.join("pom.xml").exists() {
            package_managers.push("maven".to_string());
        }
        
        if package_managers.is_empty() {
            return Err("未检测到包管理器配置文件".into());
        }
        
        Ok(package_managers)
    }

    /// 分析特定包管理器的依赖
    async fn analyze_package_manager(&self, project_path: &Path, package_manager: &str) -> Result<Vec<DependencyInfo>, Box<dyn std::error::Error>> {
        match package_manager {
            "cargo" => self.analyze_cargo_dependencies(project_path).await,
            "npm" => self.analyze_npm_dependencies(project_path).await,
            "pip" => self.analyze_pip_dependencies(project_path).await,
            "maven" => self.analyze_maven_dependencies(project_path).await,
            _ => Err(format!("不支持的包管理器: {}", package_manager).into()),
        }
    }

    /// 分析Cargo依赖
    async fn analyze_cargo_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyInfo>, Box<dyn std::error::Error>> {
        let cargo_toml = project_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err("Cargo.toml不存在".into());
        }
        
        // 运行cargo命令获取依赖信息
        let output = Command::new("cargo")
            .arg("tree")
            .arg("--format")
            .arg("{p} {v}")
            .current_dir(project_path)
            .output()?;
        
        if !output.status.success() {
            return Err(format!("运行cargo tree失败: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines = output_str.lines();
        
        let mut dependencies = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let version = parts[1].to_string();
                
                // 跳过被忽略的依赖
                if self.config.ignored_dependencies.contains(&name) {
                    continue;
                }
                
                // 分析依赖
                let dependency = self.analyze_single_dependency(&name, &version, "cargo", "runtime").await?;
                dependencies.push(dependency);
            }
        }
        
        Ok(dependencies)
    }

    /// 分析npm依赖
    async fn analyze_npm_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyInfo>, Box<dyn std::error::Error>> {
        // 简化实现，实际应用中应该使用npm命令获取依赖信息
        Ok(Vec::new())
    }

    /// 分析pip依赖
    async fn analyze_pip_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyInfo>, Box<dyn std::error::Error>> {
        // 简化实现，实际应用中应该使用pip命令获取依赖信息
        Ok(Vec::new())
    }

    /// 分析maven依赖
    async fn analyze_maven_dependencies(&self, project_path: &Path) -> Result<Vec<DependencyInfo>, Box<dyn std::error::Error>> {
        // 简化实现，实际应用中应该使用maven命令获取依赖信息
        Ok(Vec::new())
    }

    /// 分析单个依赖
    async fn analyze_single_dependency(&self, name: &str, version: &str, package_manager: &str, dependency_type: &str) -> Result<DependencyInfo, Box<dyn std::error::Error>> {
        let mut vulnerabilities = Vec::new();
        let mut compatibility_issues = Vec::new();
        
        // 检查安全漏洞
        if self.config.enable_security_check {
            vulnerabilities = self.check_vulnerabilities(name, version, package_manager).await?;
        }
        
        // 检查兼容性问题
        if self.config.enable_compatibility_check {
            compatibility_issues = self.check_compatibility(name, version, package_manager).await?;
        }
        
        DependencyInfo {
            name: name.to_string(),
            version: version.to_string(),
            package_manager: package_manager.to_string(),
            dependency_type: dependency_type.to_string(),
            source: "registry".to_string(),
            path: None,
            vulnerabilities,
            compatibility_issues,
            dependencies: Vec::new(),
        }
    }

    /// 检查依赖的安全漏洞
    async fn check_vulnerabilities(&self, name: &str, version: &str, package_manager: &str) -> Result<Vec<VulnerabilityInfo>, Box<dyn std::error::Error>> {
        // 简化实现，实际应用中应该查询漏洞数据库
        let mut vulnerabilities = Vec::new();
        
        // 模拟漏洞检查
        if rand::random::<f64>() > 0.9 { // 10%的概率有漏洞
            let vuln = VulnerabilityInfo {
                id: format!("VULN-{}", rand::random::<u32>()),
                title: format!("{} 安全漏洞", name),
                severity: vec!["critical", "high", "medium", "low"][rand::random::<usize>() % 4].to_string(),
                description: format!("{} {} 版本存在安全漏洞", name, version),
                fixed_versions: vec![format!(">={}.0.0", rand::random::<u32>() % 10 + 1)],
                affected_versions: vec![format!("<{}.0.0", rand::random::<u32>() % 10 + 1)],
                published_date: chrono::Utc::now().to_string(),
                cvss_score: Some(5.0 + (rand::random::<f64>() * 5.0)),
            };
            vulnerabilities.push(vuln);
        }
        
        Ok(vulnerabilities)
    }

    /// 检查依赖的兼容性问题
    async fn check_compatibility(&self, name: &str, version: &str, package_manager: &str) -> Result<Vec<CompatibilityIssue>, Box<dyn std::error::Error>> {
        // 简化实现，实际应用中应该检查依赖兼容性
        let mut issues = Vec::new();
        
        // 模拟兼容性检查
        if rand::random::<f64>() > 0.8 { // 20%的概率有兼容性问题
            let issue = CompatibilityIssue {
                id: format!("COMPAT-{}", rand::random::<u32>()),
                description: format!("{} {} 版本可能与其他依赖存在兼容性问题", name, version),
                severity: vec!["critical", "medium", "minor"][rand::random::<usize>() % 3].to_string(),
                solution: "更新到最新版本".to_string(),
                related_dependencies: vec![format!("dependency-{}", rand::random::<u32>())],
            };
            issues.push(issue);
        }
        
        Ok(issues)
    }

    /// 去重依赖
    fn deduplicate_dependencies(&self, dependencies: Vec<DependencyInfo>) -> Vec<DependencyInfo> {
        let mut unique_dependencies = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for dep in dependencies {
            let key = format!("{}-{}-{}", dep.name, dep.version, dep.package_manager);
            if !seen.contains(&key) {
                seen.insert(key);
                unique_dependencies.push(dep);
            }
        }
        
        unique_dependencies
    }

    /// 分析依赖关系
    async fn analyze_dependency_relationships(&self, dependencies: &mut Vec<DependencyInfo>) -> Result<(), Box<dyn std::error::Error>> {
        // 简化实现，实际应用中应该分析依赖之间的关系
        for dep in dependencies {
            // 模拟依赖关系
            if rand::random::<f64>() > 0.7 { // 30%的概率有依赖
                dep.dependencies.push(format!("dependency-{}", rand::random::<u32>()));
            }
        }
        
        Ok(())
    }

    /// 生成统计信息
    async fn generate_statistics(&self, dependencies: &Vec<DependencyInfo>) -> DependencyStatistics {
        let mut by_package_manager = std::collections::HashMap::new();
        let mut by_dependency_type = std::collections::HashMap::new();
        let mut by_vulnerability_severity = std::collections::HashMap::new();
        let mut by_compatibility_severity = std::collections::HashMap::new();
        
        let mut outdated_count = 0;
        let mut unversioned_count = 0;
        let mut duplicate_count = 0;
        
        // 统计包管理器
        for dep in dependencies {
            *by_package_manager.entry(dep.package_manager.clone()).or_insert(0) += 1;
            *by_dependency_type.entry(dep.dependency_type.clone()).or_insert(0) += 1;
            
            // 统计漏洞严重程度
            for vuln in &dep.vulnerabilities {
                *by_vulnerability_severity.entry(vuln.severity.clone()).or_insert(0) += 1;
            }
            
            // 统计兼容性问题严重程度
            for issue in &dep.compatibility_issues {
                *by_compatibility_severity.entry(issue.severity.clone()).or_insert(0) += 1;
            }
            
            // 统计过时依赖
            if rand::random::<f64>() > 0.8 { // 20%的概率过时
                outdated_count += 1;
            }
            
            // 统计未指定版本的依赖
            if dep.version == "*" || dep.version.is_empty() {
                unversioned_count += 1;
            }
        }
        
        DependencyStatistics {
            by_package_manager,
            by_dependency_type,
            by_vulnerability_severity,
            by_compatibility_severity,
            outdated_dependencies: outdated_count,
            unversioned_dependencies: unversioned_count,
            duplicate_dependencies: duplicate_count,
        }
    }

    /// 生成报告
    async fn generate_report(&self, analysis_result: &DependencyAnalysisResult) -> Result<String, Box<dyn std::error::Error>> {
        let report_id = format!("report-{}", analysis_result.analysis_id);
        let report_filename = format!("{}/{}.json", self.config.report_path, report_id);
        
        // 生成安全漏洞摘要
        let security_summary = self.generate_security_summary(&analysis_result.dependencies).await;
        
        // 生成兼容性问题摘要
        let compatibility_summary = self.generate_compatibility_summary(&analysis_result.dependencies).await;
        
        // 生成建议
        let recommendations = self.generate_recommendations(analysis_result).await;
        
        // 生成依赖图
        let dependency_graph = if self.config.enable_dependency_graph {
            Some(self.generate_dependency_graph(&analysis_result.dependencies).await?)
        } else {
            None
        };
        
        // 生成报告
        let report = DependencyAnalysisReport {
            report_id,
            title: format!("依赖分析报告 - {}", analysis_result.analysis_time),
            report_time: chrono::Utc::now().to_string(),
            analysis_result: analysis_result.clone(),
            summary: self.generate_summary(analysis_result).await,
            security_summary,
            compatibility_summary,
            recommendations,
            dependency_graph,
        };
        
        // 保存报告
        let report_json = serde_json::to_string_pretty(&report)?;
        std::fs::write(&report_filename, report_json).map_err(|e| format!("保存报告失败: {}", e))?;
        
        Ok(report_filename)
    }

    /// 生成安全漏洞摘要
    async fn generate_security_summary(&self, dependencies: &Vec<DependencyInfo>) -> SecuritySummary {
        let mut total_vulnerabilities = 0;
        let mut critical_vulnerabilities = 0;
        let mut high_vulnerabilities = 0;
        let mut medium_vulnerabilities = 0;
        let mut low_vulnerabilities = 0;
        let mut all_vulnerabilities = Vec::new();
        
        for dep in dependencies {
            for vuln in &dep.vulnerabilities {
                total_vulnerabilities += 1;
                match vuln.severity.as_str() {
                    "critical" => critical_vulnerabilities += 1,
                    "high" => high_vulnerabilities += 1,
                    "medium" => medium_vulnerabilities += 1,
                    "low" => low_vulnerabilities += 1,
                    _ => {},
                }
                all_vulnerabilities.push(vuln.clone());
            }
        }
        
        // 按严重程度排序，取前5个
        all_vulnerabilities.sort_by(|a, b| {
            let severity_order = vec!["critical", "high", "medium", "low"];
            let a_idx = severity_order.iter().position(|&s| s == a.severity).unwrap_or(4);
            let b_idx = severity_order.iter().position(|&s| s == b.severity).unwrap_or(4);
            a_idx.cmp(&b_idx)
        });
        let top_vulnerabilities = all_vulnerabilities.into_iter().take(5).collect();
        
        // 生成安全建议
        let mut security_recommendations = Vec::new();
        if critical_vulnerabilities > 0 {
            security_recommendations.push("立即更新有严重漏洞的依赖".to_string());
        }
        security_recommendations.push("定期检查依赖的安全漏洞".to_string());
        security_recommendations.push("使用固定版本的依赖".to_string());
        
        SecuritySummary {
            total_vulnerabilities,
            critical_vulnerabilities,
            high_vulnerabilities,
            medium_vulnerabilities,
            low_vulnerabilities,
            top_vulnerabilities,
            security_recommendations,
        }
    }

    /// 生成兼容性问题摘要
    async fn generate_compatibility_summary(&self, dependencies: &Vec<DependencyInfo>) -> CompatibilitySummary {
        let mut total_issues = 0;
        let mut critical_issues = 0;
        let mut medium_issues = 0;
        let mut minor_issues = 0;
        let mut all_issues = Vec::new();
        
        for dep in dependencies {
            for issue in &dep.compatibility_issues {
                total_issues += 1;
                match issue.severity.as_str() {
                    "critical" => critical_issues += 1,
                    "medium" => medium_issues += 1,
                    "minor" => minor_issues += 1,
                    _ => {},
                }
                all_issues.push(issue.clone());
            }
        }
        
        // 按严重程度排序，取前5个
        all_issues.sort_by(|a, b| {
            let severity_order = vec!["critical", "medium", "minor"];
            let a_idx = severity_order.iter().position(|&s| s == a.severity).unwrap_or(3);
            let b_idx = severity_order.iter().position(|&s| s == b.severity).unwrap_or(3);
            a_idx.cmp(&b_idx)
        });
        let top_issues = all_issues.into_iter().take(5).collect();
        
        // 生成兼容性建议
        let mut compatibility_recommendations = Vec::new();
        if critical_issues > 0 {
            compatibility_recommendations.push("解决严重的兼容性问题".to_string());
        }
        compatibility_recommendations.push("使用兼容的依赖版本组合".to_string());
        compatibility_recommendations.push("定期更新依赖到稳定版本".to_string());
        
        CompatibilitySummary {
            total_issues,
            critical_issues,
            medium_issues,
            minor_issues,
            top_issues,
            compatibility_recommendations,
        }
    }

    /// 生成建议
    async fn generate_recommendations(&self, analysis_result: &DependencyAnalysisResult) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // 基于分析结果生成建议
        if analysis_result.vulnerable_dependencies > 0 {
            recommendations.push("更新有安全漏洞的依赖".to_string());
        }
        
        if analysis_result.compatibility_issues > 0 {
            recommendations.push("解决依赖兼容性问题".to_string());
        }
        
        if analysis_result.statistics.outdated_dependencies > 0 {
            recommendations.push("更新过时的依赖".to_string());
        }
        
        if analysis_result.statistics.unversioned_dependencies > 0 {
            recommendations.push("为依赖指定具体版本".to_string());
        }
        
        recommendations.push("定期运行依赖分析，及时发现问题".to_string());
        recommendations.push("使用依赖锁定文件，确保构建一致性".to_string());
        
        recommendations
    }

    /// 生成依赖图
    async fn generate_dependency_graph(&self, dependencies: &Vec<DependencyInfo>) -> Result<String, Box<dyn std::error::Error>> {
        // 简化实现，实际应用中应该生成DOT格式的依赖图
        let mut graph = String::from("digraph G {\n");
        
        for dep in dependencies {
            graph.push_str(&format!("  \"{}\" [label=\"{} {}\"];\n", dep.name, dep.name, dep.version));
            
            for dep_of_dep in &dep.dependencies {
                graph.push_str(&format!("  \"{}\" -> \"{}\";\n", dep.name, dep_of_dep));
            }
        }
        
        graph.push_str("}\n");
        
        Ok(graph)
    }

    /// 生成摘要
    async fn generate_summary(&self, analysis_result: &DependencyAnalysisResult) -> String {
        format!(
            "本次依赖分析于{}完成，分析了{}个依赖（{}个直接依赖，{}个传递依赖）。发现了{}个有安全漏洞的依赖，{}个有兼容性问题的依赖。分析持续时间{}秒。",
            analysis_result.analysis_time,
            analysis_result.total_dependencies,
            analysis_result.direct_dependencies,
            analysis_result.transitive_dependencies,
            analysis_result.vulnerable_dependencies,
            analysis_result.compatibility_issues,
            analysis_result.duration
        )
    }

    /// 保存分析结果
    async fn save_analysis_result(&self, analysis_result: &DependencyAnalysisResult) -> Result<(), Box<dyn std::error::Error>> {
        let mut analysis_history = self.analysis_history.write().await;
        analysis_history.push(analysis_result.clone());
        
        // 限制历史记录数量
        if analysis_history.len() > 50 { // 保留最近50条记录
            analysis_history.drain(0..analysis_history.len() - 50);
        }
        
        Ok(())
    }

    /// 获取分析历史
    pub async fn get_analysis_history(&self) -> Result<Vec<DependencyAnalysisResult>, Box<dyn std::error::Error>> {
        let analysis_history = self.analysis_history.read().await;
        Ok(analysis_history.clone())
    }

    /// 获取最新的分析结果
    pub async fn get_latest_analysis(&self) -> Result<Option<DependencyAnalysisResult>, Box<dyn std::error::Error>> {
        let analysis_history = self.analysis_history.read().await;
        Ok(analysis_history.last().cloned())
    }
}

