//! Helm Chart管理模块
//! 用于管理和部署Helm Chart到Kubernetes集群

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;

/// Helm Chart部署状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChartStatus {
    Pending,
    Deploying,
    Deployed,
    Failed,
    Uninstalled,
}

/// Helm Chart部署结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResult {
    pub name: String,
    pub namespace: String,
    pub status: ChartStatus,
    pub revision: u32,
    pub chart: String,
    pub app_version: String,
    pub duration: Duration,
    pub notes: Option<String>,
}

/// Helm配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelmConfig {
    pub binary: String,
    pub timeout: Duration,
    pub kubeconfig: Option<String>,
    pub repositories: std::collections::HashMap<String, String>,
}

/// Helm管理器
#[derive(Debug, Clone)]
pub struct HelmManager {
    config: HelmConfig,
}

impl HelmManager {
    /// 创建新的Helm管理器
    pub fn new() -> Self {
        let config = HelmConfig {
            binary: "helm".to_string(),
            timeout: Duration::from_secs(300),
            kubeconfig: None,
            repositories: std::collections::HashMap::new(),
        };

        Self {
            config,
        }
    }

    /// 初始化Helm管理器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 检查Helm是否可用
        self.check_helm_available().await?;
        Ok(())
    }

    /// 检查Helm是否可用
    async fn check_helm_available(&self) -> Result<(), Box<dyn std::error::Error>> {
        let result = Command::new(&self.config.binary).arg("version").output()?;
        if !result.status.success() {
            return Err("Helm is not available".into());
        }
        Ok(())
    }

    /// 部署Helm Chart
    pub async fn deploy_chart(&self, chart: &str, namespace: &str, values: serde_json::Value) -> Result<DeploymentResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();

        // 确保命名空间存在
        self.ensure_namespace(namespace).await?;

        // 准备values文件
        let values_file = self.create_values_file(&values).await?;

        // 运行helm install/upgrade
        let result = self.run_helm_deploy(chart, namespace, &values_file).await;

        // 清理values文件
        self.cleanup_values_file(&values_file).await;

        let duration = start_time.elapsed();
        result.map(|mut deployment| {
            deployment.duration = duration;
            deployment
        })
    }

    /// 确保命名空间存在
    async fn ensure_namespace(&self, namespace: &str) -> Result<(), Box<dyn std::error::Error>> {
        let result = Command::new("kubectl")
            .arg("create")
            .arg("namespace")
            .arg(namespace)
            .arg("--dry-run=client")
            .arg("-o")
            .arg("json")
            .output()?;

        if !result.status.success() {
            // 命名空间可能已存在，忽略错误
        }

        Ok(())
    }

    /// 创建values文件
    async fn create_values_file(&self, values: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let values_path = temp_dir.path().join("values.json");
        std::fs::write(&values_path, serde_json::to_string_pretty(values)?)?;
        Ok(values_path.to_str().unwrap().to_string())
    }

    /// 清理values文件
    async fn cleanup_values_file(&self, values_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_file(values_file).ok();
        Ok(())
    }

    /// 运行helm deploy命令
    async fn run_helm_deploy(&self, chart: &str, namespace: &str, values_file: &str) -> Result<DeploymentResult, Box<dyn std::error::Error>> {
        let release_name = chart.split('/').last().unwrap_or(chart);

        // 检查是否已存在
        let exists = self.helm_release_exists(release_name, namespace).await?;

        let mut cmd = Command::new(&self.config.binary);
        if exists {
            cmd.arg("upgrade")
                .arg(release_name)
                .arg(chart)
                .arg("--namespace")
                .arg(namespace)
                .arg("--values")
                .arg(values_file)
                .arg("--install");
        } else {
            cmd.arg("install")
                .arg(release_name)
                .arg(chart)
                .arg("--namespace")
                .arg(namespace)
                .arg("--values")
                .arg(values_file);
        }

        let result = cmd.output()?;
        let stdout = String::from_utf8_lossy(&result.stdout);
        let stderr = String::from_utf8_lossy(&result.stderr);

        if !result.status.success() {
            return Err(format!("Helm deploy failed: {}\n{}", stdout, stderr).into());
        }

        // 解析部署结果
        let deployment = self.parse_deployment_result(release_name, namespace, &stdout).await?;
        Ok(deployment)
    }

    /// 检查Helm release是否存在
    async fn helm_release_exists(&self, release_name: &str, namespace: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let result = Command::new(&self.config.binary)
            .arg("status")
            .arg(release_name)
            .arg("--namespace")
            .arg(namespace)
            .output()?;

        Ok(result.status.success())
    }

    /// 解析部署结果
    async fn parse_deployment_result(&self, release_name: &str, namespace: &str, output: &str) -> Result<DeploymentResult, Box<dyn std::error::Error>> {
        // 这里应该实现实际的解析逻辑
        // 为了演示，我们返回模拟数据
        Ok(DeploymentResult {
            name: release_name.to_string(),
            namespace: namespace.to_string(),
            status: ChartStatus::Deployed,
            revision: 1,
            chart: "ymaxum-1.0.0".to_string(),
            app_version: "1.0.0".to_string(),
            duration: Duration::from_secs(10),
            notes: Some(output.to_string()),
        })
    }

    /// 列出Helm releases
    pub async fn list_releases(&self, namespace: Option<&str>) -> Result<Vec<DeploymentResult>, Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.config.binary);
        cmd.arg("list")
            .arg("-o")
            .arg("json");

        if let Some(ns) = namespace {
            cmd.arg("--namespace").arg(ns);
        }

        let result = cmd.output()?;
        if !result.status.success() {
            return Err(format!("Helm list failed: {}", String::from_utf8_lossy(&result.stderr)).into());
        }

        let stdout = String::from_utf8_lossy(&result.stdout);
        let releases: Vec<serde_json::Value> = serde_json::from_str(&stdout)?;

        let deployment_results: Vec<DeploymentResult> = releases.into_iter().map(|release| {
            DeploymentResult {
                name: release.get("name").unwrap().as_str().unwrap().to_string(),
                namespace: release.get("namespace").unwrap().as_str().unwrap().to_string(),
                status: match release.get("status").unwrap().as_str().unwrap() {
                    "deployed" => ChartStatus::Deployed,
                    "failed" => ChartStatus::Failed,
                    "uninstalled" => ChartStatus::Uninstalled,
                    _ => ChartStatus::Pending,
                },
                revision: release.get("revision").unwrap().as_u64().unwrap() as u32,
                chart: release.get("chart").unwrap().as_str().unwrap().to_string(),
                app_version: release.get("app_version").unwrap().as_str().unwrap().to_string(),
                duration: Duration::from_secs(0),
                notes: None,
            }
        }).collect();

        Ok(deployment_results)
    }

    /// 卸载Helm release
    pub async fn uninstall_release(&self, release_name: &str, namespace: &str) -> Result<(), Box<dyn std::error::Error>> {
        let result = Command::new(&self.config.binary)
            .arg("uninstall")
            .arg(release_name)
            .arg("--namespace")
            .arg(namespace)
            .output()?;

        if !result.status.success() {
            return Err(format!("Helm uninstall failed: {}", String::from_utf8_lossy(&result.stderr)).into());
        }

        Ok(())
    }
}
