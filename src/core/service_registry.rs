// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use std::sync::{Arc, RwLock};
use log::info;

/// 服务类型枚举
#[derive(Debug, PartialEq)]
pub enum ServiceType {
    ConfigHotUpdate,
    LogManager,
    MonitorService,
    FaultHandling,
    IterateService,
    PluginManager,
    PluginMarketplace,
    I18nManager,
    PerformanceMonitor,
    PrometheusMetrics,
    TrafficShaping,
    CircuitBreaker,
    RateLimiter,
    LoadBalancer,
    TrafficMonitor,
    AiManager,
    RightsModule,
    PointsModule,
    UserModule,
    FraudModule,
    ReferralModule,
    // 可以根据需要添加更多服务类型
}

/// 服务注册表
pub struct ServiceRegistry {
    /// 服务存储
    services: RwLock<Vec<Box<dyn Service>>>,
}

/// 服务特质
pub trait Service {
    /// 获取服务类型
    fn service_type(&self) -> ServiceType;
    
    /// 初始化服务
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 关闭服务
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl ServiceRegistry {
    /// 创建新的服务注册表
    pub fn new() -> Self {
        Self {
            services: RwLock::new(Vec::new()),
        }
    }
    
    /// 注册服务
    pub fn register_service(&self, mut service: Box<dyn Service>) -> Result<(), String> {
        let mut services = self.services.write().unwrap();
        
        // 检查服务是否已存在
        let service_type = service.service_type();
        if services.iter().any(|s| s.service_type() == service_type) {
            return Err(format!("Service of type {:?} already registered", service_type));
        }
        
        // 初始化服务
        if let Err(e) = service.initialize() {
            return Err(format!("Failed to initialize service {:?}: {:?}", service_type, e));
        }
        
        // 注册服务
        services.push(service);
        info!("Service {:?} registered successfully", service_type);
        Ok(())
    }
    
    /// 获取服务
    pub fn get_service<T: 'static>(&self, _service_type: ServiceType) -> Option<Arc<T>> {
        // 简化实现，直接返回 None
        // 实际实现中需要根据具体情况处理
        None
    }
    
    /// 卸载服务
    pub fn unregister_service(&self, service_type: ServiceType) -> Result<(), String> {
        let mut services = self.services.write().unwrap();
        
        let index = services.iter().position(|s| s.service_type() == service_type);
        if let Some(index) = index {
            let mut service = services.remove(index);
            if let Err(e) = service.shutdown() {
                return Err(format!("Failed to shutdown service {:?}: {:?}", service_type, e));
            }
            info!("Service {:?} unregistered successfully", service_type);
            Ok(())
        } else {
            Err(format!("Service of type {:?} not found", service_type))
        }
    }
    
    /// 初始化所有服务
    pub fn initialize_all(&self) -> Result<(), String> {
        let mut services = self.services.write().unwrap();
        
        for service in services.iter_mut() {
            if let Err(e) = service.initialize() {
                return Err(format!("Failed to initialize service {:?}: {:?}", service.service_type(), e));
            }
        }
        
        info!("All services initialized successfully");
        Ok(())
    }
    
    /// 关闭所有服务
    pub fn shutdown_all(&self) -> Result<(), String> {
        let mut services = self.services.write().unwrap();
        
        for service in services.iter_mut() {
            if let Err(e) = service.shutdown() {
                return Err(format!("Failed to shutdown service {:?}: {:?}", service.service_type(), e));
            }
        }
        
        info!("All services shutdown successfully");
        Ok(())
    }
}

/// 为 Service 特质添加 as_any 方法
pub trait ServiceExt: Service {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// 为所有实现了 Service 的类型自动实现 ServiceExt
impl<T: Service + std::any::Any> ServiceExt for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// 为 Box<dyn Service> 实现 Service trait
impl Service for Box<dyn Service> {
    fn service_type(&self) -> ServiceType {
        self.as_ref().service_type()
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.as_mut().initialize()
    }
    
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.as_mut().shutdown()
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 服务注册表扩展
pub type ServiceRegistryExtension = Arc<ServiceRegistry>;
