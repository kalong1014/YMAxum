//! 函数管理模块
//! 负责函数的部署、执行和管理

use super::ServerlessConfig;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 函数状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionStatus {
    /// 初始化中
    Initializing,
    /// 就绪
    Ready,
    /// 执行中
    Running,
    /// 失败
    Failed,
    /// 已停止
    Stopped,
}

/// 函数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// 函数名称
    pub name: String,
    /// 函数处理程序
    pub handler: String,
    /// 运行时
    pub runtime: String,
    /// 内存大小（MB）
    pub memory: usize,
    /// 超时时间（秒）
    pub timeout: usize,
    /// 环境变量
    pub environment: Option<serde_json::Value>,
    /// 函数状态
    pub status: FunctionStatus,
    /// 部署时间
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}

/// 函数执行请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInvokeRequest {
    /// 函数名称
    pub function_name: String,
    /// 输入数据
    pub input: serde_json::Value,
    /// 执行上下文
    pub context: Option<serde_json::Value>,
}

/// 函数执行响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInvokeResponse {
    /// 执行结果
    pub result: serde_json::Value,
    /// 执行时间（毫秒）
    pub duration: u64,
    /// 内存使用（MB）
    pub memory_used: usize,
}

/// 函数实例
#[derive(Debug, Clone)]
pub struct FunctionInstance {
    /// 实例ID
    instance_id: String,
    /// 函数名称
    _function_name: String,
    /// 实例状态
    status: FunctionStatus,
    /// 创建时间
    _created_at: chrono::DateTime<chrono::Utc>,
    /// 上次使用时间
    last_used_at: chrono::DateTime<chrono::Utc>,
}

/// 函数管理器
#[derive(Debug, Clone)]
pub struct FunctionManager {
    /// 配置
    config: Arc<ServerlessConfig>,
    /// 函数存储
    functions: Arc<RwLock<HashMap<String, Function>>>,
    /// 函数实例池
    instance_pool: Arc<RwLock<HashMap<String, Vec<FunctionInstance>>>>,
    /// 预热配置
    warmup_config: WarmupConfig,
}

/// 预热配置
#[derive(Debug, Clone)]
pub struct WarmupConfig {
    /// 是否启用预热
    enabled: bool,
    /// 每个函数的预热实例数
    instances_per_function: usize,
    /// 实例超时时间（秒）
    _instance_timeout: usize,
}

impl Default for WarmupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            instances_per_function: 2,
            _instance_timeout: 300,
        }
    }
}

impl FunctionManager {
    /// 创建新的函数管理器
    pub fn new(config: Arc<ServerlessConfig>) -> Self {
        Self {
            config,
            functions: Arc::new(RwLock::new(HashMap::new())),
            instance_pool: Arc::new(RwLock::new(HashMap::new())),
            warmup_config: WarmupConfig::default(),
        }
    }

    /// 初始化函数管理器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("初始化函数管理器");

        // 部署配置中的函数
        for function_config in &self.config.functions {
            self.deploy_function(function_config.clone()).await?;
        }

        Ok(())
    }

    /// 部署函数
    pub async fn deploy_function(
        &self,
        function_config: super::FunctionConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!("部署函数: {}", function_config.name);

        let function = Function {
            name: function_config.name.clone(),
            handler: function_config.handler,
            runtime: function_config.runtime,
            memory: function_config.memory,
            timeout: function_config.timeout,
            environment: function_config.environment,
            status: FunctionStatus::Initializing,
            deployed_at: chrono::Utc::now(),
        };

        // 存储函数
        let mut functions = self.functions.write().await;
        functions.insert(function_config.name.clone(), function);

        // 模拟部署过程
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // 更新函数状态
        let mut functions = self.functions.write().await;
        if let Some(function) = functions.get_mut(&function_config.name) {
            function.status = FunctionStatus::Ready;
        }

        // 预热函数实例
        if self.warmup_config.enabled {
            self.warmup_function(&function_config.name).await?;
        }

        info!("函数部署完成: {}", function_config.name);
        Ok(function_config.name)
    }

    /// 预热函数实例
    async fn warmup_function(&self, function_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("预热函数实例: {}", function_name);

        let mut instance_pool = self.instance_pool.write().await;
        let instances = instance_pool
            .entry(function_name.to_string())
            .or_insert_with(Vec::new);

        // 创建预热实例
        for i in 0..self.warmup_config.instances_per_function {
            let instance_id = format!("{}-instance-{}", function_name, i);
            let instance = FunctionInstance {
                instance_id: instance_id.clone(),
                _function_name: function_name.to_string(),
                status: FunctionStatus::Ready,
                _created_at: chrono::Utc::now(),
                last_used_at: chrono::Utc::now(),
            };
            instances.push(instance);
            info!("创建预热实例: {}", instance_id);
        }

        info!(
            "函数 {} 预热完成，创建了 {} 个实例",
            function_name, self.warmup_config.instances_per_function
        );
        Ok(())
    }

    /// 执行函数
    pub async fn invoke_function(
        &self,
        function_name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("执行函数: {}", function_name);

        // 检查函数是否存在
        let functions = self.functions.read().await;
        let function = functions
            .get(function_name)
            .ok_or_else(|| format!("函数不存在: {}", function_name))?;

        // 检查函数状态
        if function.status != FunctionStatus::Ready {
            return Err(format!("函数未就绪: {}", function_name).into());
        }
        drop(functions);

        // 从实例池获取或创建实例
        let instance_id = self.get_or_create_instance(function_name).await?;
        info!(
            "使用实例执行函数: {} on instance {}",
            function_name, instance_id
        );

        // 更新函数状态为执行中
        let mut functions = self.functions.write().await;
        if let Some(function) = functions.get_mut(function_name) {
            function.status = FunctionStatus::Running;
        }
        drop(functions);

        // 更新实例状态
        self.update_instance_status(function_name, &instance_id, FunctionStatus::Running)
            .await;

        // 模拟函数执行（使用预热实例，减少执行时间）
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 生成执行结果
        let result = serde_json::json!({
            "message": format!("函数 {} 执行成功", function_name),
            "input": input,
            "timestamp": chrono::Utc::now().to_string(),
            "instance_id": instance_id
        });

        // 更新函数状态为就绪
        let mut functions = self.functions.write().await;
        if let Some(function) = functions.get_mut(function_name) {
            function.status = FunctionStatus::Ready;
        }
        drop(functions);

        // 更新实例状态和上次使用时间
        self.update_instance_status(function_name, &instance_id, FunctionStatus::Ready)
            .await;
        self.update_instance_last_used(function_name, &instance_id)
            .await;

        info!("函数执行完成: {}", function_name);
        Ok(result)
    }

    /// 获取或创建函数实例
    async fn get_or_create_instance(
        &self,
        function_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut instance_pool = self.instance_pool.write().await;
        let instances = instance_pool
            .entry(function_name.to_string())
            .or_insert_with(Vec::new);

        // 查找就绪的实例
        if let Some(instance) = instances.iter().find(|i| i.status == FunctionStatus::Ready) {
            let instance_id = instance.instance_id.clone();
            drop(instance_pool);
            Ok(instance_id)
        } else {
            // 如果没有就绪实例，创建新实例
            let instance_id = format!(
                "{}-instance-{}",
                function_name,
                chrono::Utc::now().timestamp_millis()
            );
            let instance = FunctionInstance {
                instance_id: instance_id.clone(),
                _function_name: function_name.to_string(),
                status: FunctionStatus::Ready,
                _created_at: chrono::Utc::now(),
                last_used_at: chrono::Utc::now(),
            };
            instances.push(instance);
            drop(instance_pool);

            // 模拟实例初始化
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

            Ok(instance_id)
        }
    }

    /// 更新实例状态
    async fn update_instance_status(
        &self,
        function_name: &str,
        instance_id: &str,
        status: FunctionStatus,
    ) {
        let mut instance_pool = self.instance_pool.write().await;
        if let Some(instances) = instance_pool.get_mut(function_name) {
            for instance in instances {
                if instance.instance_id == instance_id {
                    instance.status = status;
                    break;
                }
            }
        }
    }

    /// 更新实例上次使用时间
    async fn update_instance_last_used(&self, function_name: &str, instance_id: &str) {
        let mut instance_pool = self.instance_pool.write().await;
        if let Some(instances) = instance_pool.get_mut(function_name) {
            for instance in instances {
                if instance.instance_id == instance_id {
                    instance.last_used_at = chrono::Utc::now();
                    break;
                }
            }
        }
    }

    /// 获取函数状态
    pub async fn get_function_status(
        &self,
        function_name: &str,
    ) -> Result<FunctionStatus, Box<dyn std::error::Error>> {
        let functions = self.functions.read().await;
        let function = functions
            .get(function_name)
            .ok_or_else(|| format!("函数不存在: {}", function_name))?;
        Ok(function.status.clone())
    }

    /// 列出所有函数
    pub async fn list_functions(&self) -> Result<Vec<Function>, Box<dyn std::error::Error>> {
        let functions = self.functions.read().await;
        Ok(functions.values().cloned().collect())
    }

    /// 删除函数
    pub async fn delete_function(
        &self,
        function_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("删除函数: {}", function_name);

        let mut functions = self.functions.write().await;
        if functions.remove(function_name).is_none() {
            return Err(format!("函数不存在: {}", function_name).into());
        }

        info!("函数删除完成: {}", function_name);
        Ok(())
    }
}
