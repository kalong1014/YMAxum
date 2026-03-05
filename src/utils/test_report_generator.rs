//! 测试报告生成器
//! 用于生成测试报告

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

/// 测试报告生成器
pub struct TestReportGenerator;

impl TestReportGenerator {
    /// 生成测试报告
    pub fn generate_test_report(
        output_dir: &Path,
        test_results: &TestResults,
    ) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_dir)?;

        // 生成测试报告
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

## 失败的测试
{}

## 测试详情
{}

## 建议
1. 修复失败的测试
2. 增加测试覆盖率
3. 优化测试性能
4. 定期运行测试

## 技术信息
- 测试时间：{}
- 测试环境：{}
- 测试工具：{}
            "#,
            test_results.timestamp,
            if test_results.failed == 0 { "全部通过" } else { "存在失败测试" },
            test_results.total,
            test_results.passed,
            test_results.failed,
            (test_results.passed as f64 / test_results.total as f64) * 100.0,
            if test_results.failed_tests.is_empty() { "无".to_string() } else { test_results.failed_tests.join("\n- ") },
            test_results.test_details.join("\n"),
            test_results.timestamp,
            test_results.environment,
            test_results.test_tool
        };

        let output_path = output_dir.join("test_report.md");
        let mut file = File::create(output_path)?;
        file.write_all(report.as_bytes())?;

        // 生成HTML格式的测试报告
        Self::generate_html_report(output_dir, test_results)?;

        Ok(())
    }

    /// 生成HTML格式的测试报告
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
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #333; }}
        h2 {{ color: #666; }}
        .summary {{ background-color: #f5f5f5; padding: 15px; border-radius: 5px; margin-bottom: 20px; }}
        .stats {{ display: flex; gap: 20px; margin-bottom: 20px; }}
        .stat {{ background-color: #e3f2fd; padding: 10px; border-radius: 5px; }}
        .pass {{ color: #4caf50; font-weight: bold; }}
        .fail {{ color: #f44336; font-weight: bold; }}
        .test-detail {{ margin: 10px 0; padding: 10px; border-left: 4px solid #2196f3; }}
        .test-fail {{ border-left-color: #f44336; background-color: #ffebee; }}
        .test-pass {{ border-left-color: #4caf50; background-color: #e8f5e8; }}
    </style>
</head>
<body>
    <h1>YMAxum 测试报告</h1>
    
    <div class="summary">
        <h2>测试结果</h2>
        <p>{}</p>
        <p>测试时间：{}</p>
    </div>
    
    <div class="stats">
        <div class="stat">
            <strong>总测试数：</strong>{}
        </div>
        <div class="stat">
            <strong>通过测试数：</strong><span class="pass">{}</span>
        </div>
        <div class="stat">
            <strong>失败测试数：</strong><span class="fail">{}</span>
        </div>
        <div class="stat">
            <strong>测试通过率：</strong>{:.2}%
        </div>
    </div>
    
    <h2>测试详情</h2>
    {}
    
    <h2>建议</h2>
    <ul>
        <li>修复失败的测试</li>
        <li>增加测试覆盖率</li>
        <li>优化测试性能</li>
        <li>定期运行测试</li>
    </ul>
    
    <h2>技术信息</h2>
    <p>测试环境：{}</p>
    <p>测试工具：{}</p>
</body>
</html>
            "#,
            if test_results.failed == 0 { "全部通过" } else { "存在失败测试" },
            test_results.timestamp,
            test_results.total,
            test_results.passed,
            test_results.failed,
            (test_results.passed as f64 / test_results.total as f64) * 100.0,
            Self::generate_html_test_details(test_results),
            test_results.environment,
            test_results.test_tool
        };

        let output_path = output_dir.join("test_report.html");
        let mut file = File::create(output_path)?;
        file.write_all(html_report.as_bytes())?;

        Ok(())
    }

    /// 生成HTML格式的测试详情
    fn generate_html_test_details(test_results: &TestResults) -> String {
        let mut details = String::new();

        for detail in &test_results.test_details {
            if detail.contains("FAILED") {
                details.push_str(&format!(
                    "<div class=\"test-detail test-fail\">{}</div>\n",
                    detail
                ));
            } else {
                details.push_str(&format!(
                    "<div class=\"test-detail test-pass\">{}</div>\n",
                    detail
                ));
            }
        }

        details
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
        }
    }
}
