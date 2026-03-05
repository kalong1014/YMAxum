//! 云原生支持模块
//! 用于Helm Chart部署、Kubernetes集群支持、服务网格集成、自动扩缩容、多区域部署

pub mod helm;
pub mod kubernetes;
pub mod service_mesh;
pub mod auto_scaling;
pub mod multi_region;

/// 云原生支持管理器
#[derive(Debug, Clone)]
pub struct CloudNativeManager {
    helm: helm::HelmManager,
    kubernetes: kubernetes::KubernetesClient,
    service_mesh: service_mesh::ServiceMeshManager,
    auto_scaling: auto_scaling::AutoScaler,
    multi_region: multi_region::MultiRegionManager,
    global_load_balancer: multi_region::GlobalLoadBalancer,
}

impl CloudNativeManager {
    /// 创建新的云原生支持管理器
    pub fn new() -> Self {
        let auto_scaling_config = auto_scaling::AutoScalingConfig {
            min_replicas: 1,
            max_replicas: 10,
            target_cpu_utilization: 70,
            target_memory_utilization: 70,
            cooldown_period: 300,
            polling_interval: 30,
            strategy: auto_scaling::ScalingStrategy::Combined,
        };

        let multi_region_config = multi_region::MultiRegionConfig {
            load_balancing_strategy: multi_region::LoadBalancingStrategy::WeightedRoundRobin,
            health_check_interval: 10,
            health_check_timeout: 3,
            health_check_retries: 3,
            failover_threshold: 3,
            recovery_threshold: 3,
            regions: vec![],
            auto_failover: true,
            auto_recovery: true,
        };

        let multi_region_manager = multi_region::MultiRegionManager::new(multi_region_config);
        let global_load_balancer = multi_region::GlobalLoadBalancer::new(std::sync::Arc::new(multi_region_manager.clone()));

        Self {
            helm: helm::HelmManager::new(),
            kubernetes: kubernetes::KubernetesClient::new(),
            service_mesh: service_mesh::ServiceMeshManager::new(),
            auto_scaling: auto_scaling::AutoScaler::new(auto_scaling_config),
            multi_region: multi_region_manager,
            global_load_balancer,
        }
    }

    /// 初始化云原生支持
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.helm.initialize().await?;
        self.kubernetes.initialize().await?;
        self.service_mesh.initialize().await?;
        self.multi_region.initialize().await?;
        self.global_load_balancer.initialize().await?;
        Ok(())
    }

    /// 部署应用到Kubernetes
    pub async fn deploy_to_kubernetes(&self, chart: &str, namespace: &str, values: serde_json::Value) -> Result<kubernetes::DeploymentResult, Box<dyn std::error::Error>> {
        self.helm.deploy_chart(chart, namespace, values).await
    }

    /// 管理Kubernetes资源
    pub async fn manage_kubernetes_resource(&self, resource: &str, action: &str) -> Result<kubernetes::ResourceResult, Box<dyn std::error::Error>> {
        self.kubernetes.manage_resource(resource, action).await
    }

    /// 配置服务网格
    pub async fn configure_service_mesh(&self, config: serde_json::Value) -> Result<service_mesh::ServiceMeshResult, Box<dyn std::error::Error>> {
        self.service_mesh.configure(config).await
    }

    /// 启动自动扩缩容
    pub async fn start_auto_scaling(&mut self, resource: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.auto_scaling.start(resource).await
    }

    /// 添加区域
    pub async fn add_region(&self, region: multi_region::RegionConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.add_region(region).await
    }

    /// 移除区域
    pub async fn remove_region(&self, region_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.remove_region(region_id).await
    }

    /// 更新区域配置
    pub async fn update_region(&self, region: multi_region::RegionConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.update_region(region).await
    }

    /// 获取所有区域
    pub async fn get_regions(&self) -> Vec<multi_region::RegionConfig> {
        self.multi_region.get_regions().await
    }

    /// 获取活跃区域
    pub async fn get_active_regions(&self) -> Vec<String> {
        self.multi_region.get_active_regions().await
    }

    /// 执行故障转移
    pub async fn perform_failover(&self, failed_region_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.perform_failover(failed_region_id).await
    }

    /// 执行区域恢复
    pub async fn perform_recovery(&self, recovered_region_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.perform_recovery(recovered_region_id).await
    }

    /// 路由请求到最佳区域
    pub async fn route_request(&self, client_ip: Option<&str>) -> Option<String> {
        self.global_load_balancer.route_request(client_ip).await
    }

    /// 获取负载均衡统计信息
    pub async fn get_load_balancing_stats(&self) -> serde_json::Value {
        self.global_load_balancer.get_load_balancing_stats().await
    }

    /// 更新负载均衡策略
    pub async fn update_load_balancing_strategy(&self, strategy: multi_region::LoadBalancingStrategy) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.update_load_balancing_strategy(strategy).await
    }

    /// 启用/禁用自动故障转移
    pub async fn set_auto_failover(&self, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.set_auto_failover(enabled).await
    }

    /// 启用/禁用自动恢复
    pub async fn set_auto_recovery(&self, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.multi_region.set_auto_recovery(enabled).await
    }
}
