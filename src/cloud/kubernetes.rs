//! Kubernetes客户端模块
//! 用于管理Kubernetes集群资源

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;

/// Kubernetes资源状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Unknown,
}

/// Kubernetes资源结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceResult {
    pub kind: String,
    pub name: String,
    pub namespace: Option<String>,
    pub status: ResourceStatus,
    pub action: String,
    pub duration: Duration,
    pub message: Option<String>,
}

/// Kubernetes配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesConfig {
    pub binary: String,
    pub timeout: Duration,
    pub kubeconfig: Option<String>,
    pub context: Option<String>,
}

/// Kubernetes客户端
#[derive(Debug, Clone)]
pub struct KubernetesClient {
    config: KubernetesConfig,
}

impl KubernetesClient {
    /// 创建新的Kubernetes客户端
    pub fn new() -> Self {
        let config = KubernetesConfig {
            binary: "kubectl".to_string(),
            timeout: Duration::from_secs(300),
            kubeconfig: None,
            context: None,
        };

        Self {
            config,
        }
    }

    /// 初始化Kubernetes客户端
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 检查kubectl是否可用
        self.check_kubectl_available().await?;
        // 检查集群连接
        self.check_cluster_connection().await?;
        Ok(())
    }

    /// 检查kubectl是否可用
    async fn check_kubectl_available(&self) -> Result<(), Box<dyn std::error::Error>> {
        let result = Command::new(&self.config.binary).arg("version").arg("--client").output()?;
        if !result.status.success() {
            return Err("kubectl is not available".into());
        }
        Ok(())
    }

    /// 检查集群连接
    async fn check_cluster_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.config.binary);
        cmd.arg("cluster-info");
        
        if let Some(kubeconfig) = &self.config.kubeconfig {
            cmd.arg("--kubeconfig").arg(kubeconfig);
        }
        
        if let Some(context) = &self.config.context {
            cmd.arg("--context").arg(context);
        }
        
        let result = cmd.output()?;
        if !result.status.success() {
            return Err("Cannot connect to Kubernetes cluster".into());
        }
        Ok(())
    }

    /// 管理Kubernetes资源
    pub async fn manage_resource(&self, resource: &str, action: &str) -> Result<ResourceResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();

        // 解析资源参数
        let (kind, name, namespace) = self.parse_resource(resource)?;

        // 执行资源操作
        let (status, message) = self.execute_resource_action(&kind, name, namespace, action).await?;

        let duration = start_time.elapsed();

        Ok(ResourceResult {
            kind: kind.to_string(),
            name: name.to_string(),
            namespace: namespace.map(|s| s.to_string()),
            status,
            action: action.to_string(),
            duration,
            message,
        })
    }

    /// 解析资源参数
    fn parse_resource(&self, resource: &str) -> Result<(&str, &str, Option<&str>), Box<dyn std::error::Error>> {
        // 支持格式: kind/name 或 kind/namespace/name
        let parts: Vec<&str> = resource.split('/').collect();
        
        if parts.len() < 2 {
            return Err("Invalid resource format. Expected: kind/name or kind/namespace/name".into());
        }
        
        let kind = parts[0];
        let (name, namespace) = if parts.len() == 3 {
            (parts[2], Some(parts[1]))
        } else {
            (parts[1], None)
        };
        
        Ok((kind, name, namespace))
    }

    /// 执行资源操作
    async fn execute_resource_action(&self, kind: &str, name: &str, namespace: Option<&str>, action: &str) -> Result<(ResourceStatus, Option<String>), Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.config.binary);
        
        match action {
            "create" => {
                cmd.arg(action).arg(kind).arg(name);
            }
            "delete" => {
                cmd.arg(action).arg(kind).arg(name);
            }
            "get" => {
                cmd.arg(action).arg(kind).arg(name);
            }
            "apply" => {
                cmd.arg(action).arg("-f").arg(name);
            }
            "edit" => {
                cmd.arg(action).arg(kind).arg(name);
            }
            "scale" => {
                cmd.arg(action).arg(kind).arg(name);
            }
            "rollout" => {
                cmd.arg(action).arg("status").arg(kind).arg(name);
            }
            _ => {
                return Err(format!("Unsupported action: {}", action).into());
            }
        }
        
        if let Some(namespace) = namespace {
            cmd.arg("--namespace").arg(namespace);
        }
        
        if let Some(kubeconfig) = &self.config.kubeconfig {
            cmd.arg("--kubeconfig").arg(kubeconfig);
        }
        
        if let Some(context) = &self.config.context {
            cmd.arg("--context").arg(context);
        }
        
        let result = cmd.output()?;
        let stdout = String::from_utf8_lossy(&result.stdout);
        let stderr = String::from_utf8_lossy(&result.stderr);
        
        if !result.status.success() {
            return Err(format!("Kubernetes action failed: {}\n{}", stdout, stderr).into());
        }
        
        // 解析状态
        let status = self.parse_resource_status(&stdout, kind).await;
        
        Ok((status, Some(stdout.to_string())))
    }

    /// 解析资源状态
    async fn parse_resource_status(&self, output: &str, kind: &str) -> ResourceStatus {
        // 这里应该实现实际的状态解析逻辑
        // 为了演示，我们返回模拟状态
        match kind {
            "pod" => {
                if output.contains("Running") {
                    ResourceStatus::Running
                } else if output.contains("Pending") {
                    ResourceStatus::Pending
                } else if output.contains("Failed") {
                    ResourceStatus::Failed
                } else if output.contains("Succeeded") {
                    ResourceStatus::Succeeded
                } else {
                    ResourceStatus::Unknown
                }
            }
            "deployment" => {
                if output.contains("available") {
                    ResourceStatus::Running
                } else {
                    ResourceStatus::Pending
                }
            }
            _ => ResourceStatus::Unknown,
        }
    }

    /// 列出Kubernetes资源
    pub async fn list_resources(&self, kind: &str, namespace: Option<&str>) -> Result<Vec<ResourceResult>, Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.config.binary);
        cmd.arg("get").arg(kind).arg("-o").arg("json");
        
        if let Some(namespace) = namespace {
            cmd.arg("--namespace").arg(namespace);
        }
        
        if let Some(kubeconfig) = &self.config.kubeconfig {
            cmd.arg("--kubeconfig").arg(kubeconfig);
        }
        
        if let Some(context) = &self.config.context {
            cmd.arg("--context").arg(context);
        }
        
        let result = cmd.output()?;
        if !result.status.success() {
            return Err(format!("Kubernetes list failed: {}", String::from_utf8_lossy(&result.stderr)).into());
        }
        
        let stdout = String::from_utf8_lossy(&result.stdout);
        let resources: serde_json::Value = serde_json::from_str(&stdout)?;
        
        let items = resources.get("items").unwrap().as_array().unwrap();
        let resource_results: Vec<ResourceResult> = items.into_iter().map(|item| {
            let metadata = item.get("metadata").unwrap();
            let name = metadata.get("name").unwrap().as_str().unwrap();
            let namespace = metadata.get("namespace").map(|ns| ns.as_str().unwrap());
            
            let status = item.get("status").unwrap();
            let resource_status = self.parse_status_from_json(status, kind);
            
            ResourceResult {
                kind: kind.to_string(),
                name: name.to_string(),
                namespace: namespace.map(|s| s.to_string()),
                status: resource_status,
                action: "list".to_string(),
                duration: Duration::from_secs(0),
                message: None,
            }
        }).collect();
        
        Ok(resource_results)
    }

    /// 从JSON解析状态
    fn parse_status_from_json(&self, status: &serde_json::Value, kind: &str) -> ResourceStatus {
        match kind {
            "pod" => {
                if let Some(phase) = status.get("phase") {
                    match phase.as_str().unwrap() {
                        "Running" => ResourceStatus::Running,
                        "Pending" => ResourceStatus::Pending,
                        "Succeeded" => ResourceStatus::Succeeded,
                        "Failed" => ResourceStatus::Failed,
                        _ => ResourceStatus::Unknown,
                    }
                } else {
                    ResourceStatus::Unknown
                }
            }
            "deployment" => {
                if let Some(conditions) = status.get("conditions").and_then(|c| c.as_array()) {
                    for condition in conditions {
                        if condition.get("type").unwrap().as_str().unwrap() == "Available" && 
                           condition.get("status").unwrap().as_str().unwrap() == "True" {
                            return ResourceStatus::Running;
                        }
                    }
                }
                ResourceStatus::Pending
            }
            _ => ResourceStatus::Unknown,
        }
    }

    /// 获取集群信息
    pub async fn get_cluster_info(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.config.binary);
        cmd.arg("cluster-info").arg("-o").arg("json");
        
        if let Some(kubeconfig) = &self.config.kubeconfig {
            cmd.arg("--kubeconfig").arg(kubeconfig);
        }
        
        if let Some(context) = &self.config.context {
            cmd.arg("--context").arg(context);
        }
        
        let result = cmd.output()?;
        if !result.status.success() {
            return Err(format!("Failed to get cluster info: {}", String::from_utf8_lossy(&result.stderr)).into());
        }
        
        let stdout = String::from_utf8_lossy(&result.stdout);
        let info: serde_json::Value = serde_json::from_str(&stdout)?;
        
        Ok(info)
    }

    /// 获取节点信息
    pub async fn get_nodes(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.config.binary);
        cmd.arg("get").arg("nodes").arg("-o").arg("json");
        
        if let Some(kubeconfig) = &self.config.kubeconfig {
            cmd.arg("--kubeconfig").arg(kubeconfig);
        }
        
        if let Some(context) = &self.config.context {
            cmd.arg("--context").arg(context);
        }
        
        let result = cmd.output()?;
        if !result.status.success() {
            return Err(format!("Failed to get nodes: {}", String::from_utf8_lossy(&result.stderr)).into());
        }
        
        let stdout = String::from_utf8_lossy(&result.stdout);
        let nodes: serde_json::Value = serde_json::from_str(&stdout)?;
        
        let items = nodes.get("items").unwrap().as_array().unwrap();
        Ok(items.to_vec())
    }
}
