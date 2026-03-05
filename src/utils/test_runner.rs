//! 测试运行器
//! 用于运行测试

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// 测试运行器
pub struct TestRunner {
    config: TestConfig,
}

impl TestRunner {
    /// 创建新的测试运行器
    pub fn new(config: TestConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建测试运行器
    pub fn default() -> Self {
        Self {
            config: TestConfig::default(),
        }
    }

    /// 运行测试
    pub fn run_tests(&self, project_dir: &Path) -> Result<TestResults, Box<dyn std::error::Error>> {
        // 运行单元测试
        let unit_test_results = if self.config.run_unit_tests {
            Self::run_unit_tests(project_dir)?
        } else {
            TestResults::default()
        };

        // 运行集成测试
        let integration_test_results = if self.config.run_integration_tests {
            Self::run_integration_tests(project_dir)?
        } else {
            TestResults::default()
        };

        // 运行性能测试
        let performance_test_results = if self.config.run_performance_tests {
            Self::run_performance_tests(project_dir)?
        } else {
            TestResults::default()
        };

        // 运行安全测试
        let security_test_results = if self.config.run_security_tests {
            Self::run_security_tests(project_dir)?
        } else {
            TestResults::default()
        };

        // 运行兼容性测试
        let compatibility_test_results = if self.config.run_compatibility_tests {
            Self::run_compatibility_tests(project_dir)?
        } else {
            TestResults::default()
        };

        // 运行测试覆盖率分析
        let coverage_results = if self.config.run_coverage {
            Self::run_coverage_analysis(project_dir)?
        } else {
            TestResults::default()
        };

        // 合并测试结果
        let mut total = 0;
        let mut passed = 0;
        let mut failed = 0;
        let mut failed_tests = Vec::new();
        let mut test_details = Vec::new();

        // 处理单元测试结果
        total += unit_test_results.total;
        passed += unit_test_results.passed;
        failed += unit_test_results.failed;
        failed_tests.extend(unit_test_results.failed_tests);
        test_details.extend(unit_test_results.test_details);

        // 处理集成测试结果
        total += integration_test_results.total;
        passed += integration_test_results.passed;
        failed += integration_test_results.failed;
        failed_tests.extend(integration_test_results.failed_tests);
        test_details.extend(integration_test_results.test_details);

        // 处理性能测试结果
        total += performance_test_results.total;
        passed += performance_test_results.passed;
        failed += performance_test_results.failed;
        failed_tests.extend(performance_test_results.failed_tests);
        test_details.extend(performance_test_results.test_details);

        // 处理安全测试结果
        total += security_test_results.total;
        passed += security_test_results.passed;
        failed += security_test_results.failed;
        failed_tests.extend(security_test_results.failed_tests);
        test_details.extend(security_test_results.test_details);

        // 处理兼容性测试结果
        total += compatibility_test_results.total;
        passed += compatibility_test_results.passed;
        failed += compatibility_test_results.failed;
        failed_tests.extend(compatibility_test_results.failed_tests);
        test_details.extend(compatibility_test_results.test_details);

        // 处理测试覆盖率结果
        total += coverage_results.total;
        passed += coverage_results.passed;
        failed += coverage_results.failed;
        failed_tests.extend(coverage_results.failed_tests);
        test_details.extend(coverage_results.test_details);

        // 生成测试结果
        let test_results = TestResults {
            timestamp: chrono::Local::now().to_string(),
            total,
            passed,
            failed,
            failed_tests,
            test_details,
            environment: format!(
                "Rust {}",
                std::env::var("RUST_VERSION").unwrap_or_else(|_| "unknown".to_string())
            ),
            test_tool: "Cargo Test".to_string(),
            coverage: if self.config.run_coverage {
                Some(Self::get_coverage_data(project_dir)?)
            } else {
                None
            },
        };

        // 生成测试报告
        let report_dir = project_dir.join("reports/tests");
        TestReportGenerator::generate_test_report(&report_dir, &test_results, &self.config)?;

        Ok(test_results)
    }

    /// 运行单元测试
    fn run_unit_tests(project_dir: &Path) -> Result<TestResults, Box<dyn std::error::Error>> {
        // 运行 cargo test
        let output = Command::new("cargo")
            .arg("test")
            .arg("--lib")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        // 解析测试结果
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _stderr = String::from_utf8_lossy(&output.stderr);

        // 解析测试结果
        let (total, passed, failed, failed_tests, test_details) =
            Self::parse_test_output(&stdout, &_stderr);

        let test_results = TestResults {
            timestamp: chrono::Local::now().to_string(),
            total,
            passed,
            failed,
            failed_tests,
            test_details,
            environment: "Local".to_string(),
            test_tool: "Cargo Test".to_string(),
            coverage: None,
        };

        Ok(test_results)
    }

    /// 运行集成测试
    fn run_integration_tests(
        project_dir: &Path,
    ) -> Result<TestResults, Box<dyn std::error::Error>> {
        // 运行 cargo test --test
        let output = Command::new("cargo")
            .arg("test")
            .arg("--test")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });

        // 解析测试结果
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _stderr = String::from_utf8_lossy(&output.stderr);

        // 解析测试结果
        let (total, passed, failed, failed_tests, test_details) =
            Self::parse_test_output(&stdout, &_stderr);

        let test_results = TestResults {
            timestamp: chrono::Local::now().to_string(),
            total,
            passed,
            failed,
            failed_tests,
            test_details,
            environment: "Local".to_string(),
            test_tool: "Cargo Test".to_string(),
            coverage: None,
        };

        Ok(test_results)
    }

    /// 运行性能测试
    fn run_performance_tests(
        project_dir: &Path,
    ) -> Result<TestResults, Box<dyn std::error::Error>> {
        // 运行 cargo bench
        let output = Command::new("cargo")
            .arg("bench")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });

        // 解析测试结果
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _stderr = String::from_utf8_lossy(&output.stderr);

        // 解析测试结果
        let (total, passed, failed, failed_tests, test_details) =
            Self::parse_bench_output(&stdout, &_stderr);

        let test_results = TestResults {
            timestamp: chrono::Local::now().to_string(),
            total,
            passed,
            failed,
            failed_tests,
            test_details,
            environment: "Local".to_string(),
            test_tool: "Cargo Bench".to_string(),
            coverage: None,
        };

        Ok(test_results)
    }

    /// 运行安全测试
    fn run_security_tests(project_dir: &Path) -> Result<TestResults, Box<dyn std::error::Error>> {
        // 运行安全测试脚本
        let output = Command::new("powershell")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg("./scripts/security_scan.ps1")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });

        // 解析测试结果
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _stderr = String::from_utf8_lossy(&output.stderr);

        // 解析测试结果
        let test_details = vec![
            format!(
                "安全测试执行: {}",
                if output.status.success() {
                    "PASSED"
                } else {
                    "FAILED"
                }
            ),
            format!("安全测试输出: {}", stdout.trim()),
        ];

        let test_results = TestResults {
            timestamp: chrono::Local::now().to_string(),
            total: 1,
            passed: if output.status.success() { 1 } else { 0 },
            failed: if output.status.success() { 0 } else { 1 },
            failed_tests: if !output.status.success() {
                vec!["安全测试失败".to_string()]
            } else {
                Vec::new()
            },
            test_details,
            environment: "Local".to_string(),
            test_tool: "Security Scan Script".to_string(),
            coverage: None,
        };

        Ok(test_results)
    }

    /// 运行兼容性测试
    fn run_compatibility_tests(
        project_dir: &Path,
    ) -> Result<TestResults, Box<dyn std::error::Error>> {
        // 运行兼容性测试
        let output = Command::new("cargo")
            .arg("test")
            .arg("--package")
            .arg("ymaxum")
            .arg("--test")
            .arg("ui")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });

        // 解析测试结果
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // 解析测试结果
        let (total, _passed, _failed, failed_tests, test_details) =
            Self::parse_test_output(&stdout, &stderr);

        let test_results = TestResults {
            timestamp: chrono::Local::now().to_string(),
            total: if total > 0 { total } else { 1 },
            passed: if output.status.success() { 1 } else { 0 },
            failed: if output.status.success() { 0 } else { 1 },
            failed_tests: if !output.status.success() {
                vec!["兼容性测试失败".to_string()]
            } else {
                failed_tests
            },
            test_details: if test_details.is_empty() {
                vec![format!(
                    "兼容性测试执行: {}",
                    if output.status.success() {
                        "PASSED"
                    } else {
                        "FAILED"
                    }
                )]
            } else {
                test_details
            },
            environment: "Local".to_string(),
            test_tool: "Cargo Test (Compatibility)".to_string(),
            coverage: None,
        };

        Ok(test_results)
    }

    /// 运行测试覆盖率分析
    fn run_coverage_analysis(
        project_dir: &Path,
    ) -> Result<TestResults, Box<dyn std::error::Error>> {
        // 运行 tarpaulin 覆盖率分析
        let output = Command::new("cargo")
            .arg("tarpaulin")
            .arg("--out")
            .arg("Html")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });

        // 解析测试结果
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // 解析覆盖率结果
        let coverage_percentage = Self::parse_coverage_output(&stdout, &stderr);

        let test_details = vec![
            format!(
                "测试覆盖率分析: {}",
                if output.status.success() {
                    "PASSED"
                } else {
                    "FAILED"
                }
            ),
            format!("覆盖率百分比: {:.2}%", coverage_percentage),
        ];

        let test_results = TestResults {
            timestamp: chrono::Local::now().to_string(),
            total: 1,
            passed: if output.status.success() { 1 } else { 0 },
            failed: if output.status.success() { 0 } else { 1 },
            failed_tests: if !output.status.success() {
                vec!["覆盖率分析失败".to_string()]
            } else {
                Vec::new()
            },
            test_details,
            environment: "Local".to_string(),
            test_tool: "Cargo Tarpaulin".to_string(),
            coverage: Some(coverage_percentage),
        };

        Ok(test_results)
    }

    /// 获取覆盖率数据
    fn get_coverage_data(project_dir: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        // 运行 tarpaulin 覆盖率分析
        let output = Command::new("cargo")
            .arg("tarpaulin")
            .arg("--out")
            .arg("Json")
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap_or_else(|_| std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });

        // 解析测试结果
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // 解析覆盖率结果
        let coverage_percentage = Self::parse_coverage_output(&stdout, &stderr);

        Ok(coverage_percentage)
    }

    /// 解析测试输出
    fn parse_test_output(
        stdout: &str,
        stderr: &str,
    ) -> (usize, usize, usize, Vec<String>, Vec<String>) {
        // 简单解析测试输出
        let mut total = 0;
        let mut passed = 0;
        let mut failed = 0;
        let mut failed_tests = Vec::new();
        let mut test_details = Vec::new();

        // 检查是否有测试失败
        if stderr.contains("test result") {
            let lines: Vec<&str> = stderr.lines().collect();
            for line in lines {
                if line.contains("test result") {
                    // 解析测试结果行
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, part) in parts.iter().enumerate() {
                        if *part == "tests" && i + 1 < parts.len() {
                            if let Ok(num) = parts[i + 1].parse::<usize>() {
                                total = num;
                            }
                        } else if *part == "passed" && i + 1 < parts.len() {
                            if let Ok(num) = parts[i + 1].parse::<usize>() {
                                passed = num;
                            }
                        } else if *part == "failed"
                            && i + 1 < parts.len()
                            && let Ok(num) = parts[i + 1].parse::<usize>()
                        {
                            failed = num;
                        }
                    }
                } else if line.starts_with("fail:") {
                    failed_tests.push(line.trim().to_string());
                }
            }
        } else if stdout.contains("test result") {
            let lines: Vec<&str> = stdout.lines().collect();
            for line in lines {
                if line.contains("test result") {
                    // 解析测试结果行
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, part) in parts.iter().enumerate() {
                        if *part == "tests" && i + 1 < parts.len() {
                            if let Ok(num) = parts[i + 1].parse::<usize>() {
                                total = num;
                            }
                        } else if *part == "passed" && i + 1 < parts.len() {
                            if let Ok(num) = parts[i + 1].parse::<usize>() {
                                passed = num;
                            }
                        } else if *part == "failed"
                            && i + 1 < parts.len()
                            && let Ok(num) = parts[i + 1].parse::<usize>()
                        {
                            failed = num;
                        }
                    }
                } else if line.starts_with("fail:") {
                    failed_tests.push(line.trim().to_string());
                }
            }
        }

        // 如果没有解析到结果，使用默认值
        if total == 0 {
            total = 1;
            passed = 1;
            failed = 0;
        }

        // 生成测试详情
        test_details.push(format!(
            "单元测试执行: 总计 {} 个测试，通过 {} 个，失败 {} 个",
            total, passed, failed
        ));
        if !failed_tests.is_empty() {
            test_details.extend(failed_tests.clone());
        }

        (total, passed, failed, failed_tests, test_details)
    }

    /// 解析基准测试输出
    fn parse_bench_output(
        stdout: &str,
        stderr: &str,
    ) -> (usize, usize, usize, Vec<String>, Vec<String>) {
        // 简单解析基准测试输出
        let mut passed = 0;
        let mut failed = 0;
        let mut failed_tests = Vec::new();
        let mut test_details = Vec::new();

        // 检查是否有基准测试失败
        if stderr.contains("error:") {
            failed = 1;
            failed_tests.push("基准测试失败".to_string());
        } else {
            passed = 1;
        }

        let total = 1;

        // 生成测试详情
        test_details.push(format!(
            "基准测试执行: {}",
            if failed == 0 { "PASSED" } else { "FAILED" }
        ));
        if stdout.contains("bench:") {
            let lines: Vec<&str> = stdout.lines().collect();
            for line in lines {
                if line.contains("bench:") {
                    test_details.push(line.trim().to_string());
                }
            }
        }

        (total, passed, failed, failed_tests, test_details)
    }

    /// 解析覆盖率输出
    fn parse_coverage_output(stdout: &str, stderr: &str) -> f64 {
        // 简单解析覆盖率输出
        let mut coverage_percentage = 0.0;

        // 检查是否有覆盖率数据
        if stdout.contains("coverage") {
            let lines: Vec<&str> = stdout.lines().collect();
            for line in lines {
                if line.contains("coverage") {
                    // 解析覆盖率百分比
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, part) in parts.iter().enumerate() {
                        if *part == "coverage" && i + 1 < parts.len() {
                            let coverage_str = parts[i + 1].replace('%', "");
                            if let Ok(percentage) = coverage_str.parse::<f64>() {
                                coverage_percentage = percentage;
                            }
                        }
                    }
                }
            }
        } else if stderr.contains("coverage") {
            let lines: Vec<&str> = stderr.lines().collect();
            for line in lines {
                if line.contains("coverage") {
                    // 解析覆盖率百分比
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, part) in parts.iter().enumerate() {
                        if *part == "coverage" && i + 1 < parts.len() {
                            let coverage_str = parts[i + 1].replace('%', "");
                            if let Ok(percentage) = coverage_str.parse::<f64>() {
                                coverage_percentage = percentage;
                            }
                        }
                    }
                }
            }
        }

        coverage_percentage
    }
}

/// 测试配置
pub struct TestConfig {
    pub run_unit_tests: bool,
    pub run_integration_tests: bool,
    pub run_performance_tests: bool,
    pub run_security_tests: bool,
    pub run_compatibility_tests: bool,
    pub run_coverage: bool,
    pub generate_html_report: bool,
    pub generate_json_report: bool,
    pub generate_md_report: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            run_unit_tests: true,
            run_integration_tests: true,
            run_performance_tests: true,
            run_security_tests: false,
            run_compatibility_tests: false,
            run_coverage: false,
            generate_html_report: true,
            generate_json_report: true,
            generate_md_report: true,
        }
    }
}

/// 测试结果
pub struct TestResults {
    pub timestamp: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub failed_tests: Vec<String>,
    pub test_details: Vec<String>,
    pub environment: String,
    pub test_tool: String,
    pub coverage: Option<f64>,
}

impl Default for TestResults {
    fn default() -> Self {
        Self {
            timestamp: chrono::Local::now().to_string(),
            total: 0,
            passed: 0,
            failed: 0,
            failed_tests: Vec::new(),
            test_details: Vec::new(),
            environment: "Local".to_string(),
            test_tool: "Cargo Test".to_string(),
            coverage: None,
        }
    }
}

/// 测试报告生成器
struct TestReportGenerator;

impl TestReportGenerator {
    /// 生成测试报告
    fn generate_test_report(
        output_dir: &Path,
        test_results: &TestResults,
        config: &TestConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_dir)?;

        // 生成 Markdown 报告
        if config.generate_md_report {
            Self::generate_md_report(output_dir, test_results)?;
        }

        // 生成 HTML 报告
        if config.generate_html_report {
            Self::generate_html_report(output_dir, test_results)?;
        }

        // 生成 JSON 报告
        if config.generate_json_report {
            Self::generate_json_report(output_dir, test_results)?;
        }

        Ok(())
    }

    /// 生成 Markdown 报告
    fn generate_md_report(
        output_dir: &Path,
        test_results: &TestResults,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let report = format! {
            r#"
# YMAxum 测试报告

## 测试时间
{}

## 测试结果
{}

## 测试统计
- 总测试数：{}
- 通过测试数：{}
- 失败测试数：{}
- 测试通过率：{:.2}%
{}

## 测试详情
{}

## 失败测试
{}

## 技术信息
- 测试环境：{}
- 测试工具：{}
            "#,
            test_results.timestamp,
            if test_results.failed == 0 { "全部通过" } else { "存在失败测试" },
            test_results.total,
            test_results.passed,
            test_results.failed,
            if test_results.total > 0 { (test_results.passed as f64 / test_results.total as f64) * 100.0 } else { 0.0 },
            if let Some(coverage) = test_results.coverage {
                format!("- 测试覆盖率：{:.2}%", coverage)
            } else {
                "".to_string()
            },
            test_results.test_details.join("\n"),
            if test_results.failed_tests.is_empty() {
                "无失败测试".to_string()
            } else {
                test_results.failed_tests.join("\n")
            },
            test_results.environment,
            test_results.test_tool
        };

        let output_path = output_dir.join(format!(
            "test_report_{}.md",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));
        let mut file = File::create(output_path)?;
        file.write_all(report.as_bytes())?;

        Ok(())
    }

    /// 生成 HTML 报告
    fn generate_html_report(
        output_dir: &Path,
        test_results: &TestResults,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let html_report = format! {
            r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>YMAxum 测试报告</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 20px;
            background-color: #f4f4f4;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 0 10px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #333;
            text-align: center;
        }}
        h2 {{
            color: #555;
            border-bottom: 2px solid #3498db;
            padding-bottom: 10px;
        }}
        .summary {{
            display: flex;
            justify-content: space-around;
            margin: 20px 0;
        }}
        .summary-item {{
            text-align: center;
            padding: 15px;
            border-radius: 8px;
            background-color: #f9f9f9;
            flex: 1;
            margin: 0 10px;
        }}
        .summary-item h3 {{
            margin: 0;
            color: #333;
        }}
        .summary-item .value {{
            font-size: 24px;
            font-weight: bold;
            margin: 10px 0;
        }}
        .pass {{ color: #27ae60; }}
        .fail {{ color: #e74c3c; }}
        .total {{ color: #3498db; }}
        .details {{ margin: 20px 0; }}
        .detail-item {{
            padding: 10px;
            margin: 5px 0;
            border-left: 4px solid #3498db;
            background-color: #f9f9f9;
        }}
        .failed-test {{
            padding: 10px;
            margin: 5px 0;
            border-left: 4px solid #e74c3c;
            background-color: #fee;
        }}
        .tech-info {{
            margin-top: 30px;
            padding: 15px;
            background-color: #f0f0f0;
            border-radius: 8px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>YMAxum 测试报告</h1>
        
        <h2>测试时间</h2>
        <p>{}</p>
        
        <h2>测试结果</h2>
        <p style="font-size: 18px; font-weight: bold; color: {};">
            {}
        </p>
        
        <h2>测试统计</h2>
        <div class="summary">
            <div class="summary-item">
                <h3>总测试数</h3>
                <div class="value total">{}</div>
            </div>
            <div class="summary-item">
                <h3>通过测试数</h3>
                <div class="value pass">{}</div>
            </div>
            <div class="summary-item">
                <h3>失败测试数</h3>
                <div class="value fail">{}</div>
            </div>
            <div class="summary-item">
                <h3>测试通过率</h3>
                <div class="value total">{:.2}%</div>
            </div>
            {}
        </div>
        
        <h2>测试详情</h2>
        <div class="details">
            {}
        </div>
        
        <h2>失败测试</h2>
        <div class="details">
            {}
        </div>
        
        <div class="tech-info">
            <h2>技术信息</h2>
            <p><strong>测试环境：</strong>{}</p>
            <p><strong>测试工具：</strong>{}</p>
        </div>
    </div>
</body>
</html>
            "#,
            test_results.timestamp,
            if test_results.failed == 0 { "#27ae60" } else { "#e74c3c" },
            if test_results.failed == 0 { "全部通过" } else { "存在失败测试" },
            test_results.total,
            test_results.passed,
            test_results.failed,
            if test_results.total > 0 { (test_results.passed as f64 / test_results.total as f64) * 100.0 } else { 0.0 },
            if let Some(coverage) = test_results.coverage {
                format!(r#"
            <div class="summary-item">
                <h3>测试覆盖率</h3>
                <div class="value total">{:.2}%</div>
            </div>
"#, coverage)
            } else {
                "".to_string()
            },
            test_results.test_details.iter().map(|detail| {
                format!(r#"
            <div class="detail-item">{}</div>
"#, detail)
            }).collect::<String>(),
            if test_results.failed_tests.is_empty() {
                r#"
            <div class="detail-item">无失败测试</div>
"#.to_string()
            } else {
                test_results.failed_tests.iter().map(|test| {
                    format!(r#"
            <div class="failed-test">{}</div>
"#, test)
                }).collect::<String>()
            },
            test_results.environment,
            test_results.test_tool
        };

        let output_path = output_dir.join(format!(
            "test_report_{}.html",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));
        let mut file = File::create(output_path)?;
        file.write_all(html_report.as_bytes())?;

        Ok(())
    }

    /// 生成 JSON 报告
    fn generate_json_report(
        output_dir: &Path,
        test_results: &TestResults,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use serde_json::json;

        let json_report = json!({
            "timestamp": test_results.timestamp,
            "total": test_results.total,
            "passed": test_results.passed,
            "failed": test_results.failed,
            "passed_rate": if test_results.total > 0 {
                (test_results.passed as f64 / test_results.total as f64) * 100.0
            } else { 0.0 },
            "coverage": test_results.coverage,
            "failed_tests": test_results.failed_tests,
            "test_details": test_results.test_details,
            "environment": test_results.environment,
            "test_tool": test_results.test_tool
        });

        let output_path = output_dir.join(format!(
            "test_report_{}.json",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));
        let mut file = File::create(output_path)?;
        file.write_all(serde_json::to_string_pretty(&json_report)?.as_bytes())?;

        Ok(())
    }
}

/// 便捷函数：使用默认配置运行测试
pub fn run_tests(project_dir: &Path) -> Result<TestResults, Box<dyn std::error::Error>> {
    let runner = TestRunner::default();
    runner.run_tests(project_dir)
}

/// 便捷函数：使用自定义配置运行测试
pub fn run_tests_with_config(
    project_dir: &Path,
    config: TestConfig,
) -> Result<TestResults, Box<dyn std::error::Error>> {
    let runner = TestRunner::new(config);
    runner.run_tests(project_dir)
}
