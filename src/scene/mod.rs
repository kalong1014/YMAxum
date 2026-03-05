// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use libloading::{Library, Symbol};
use log::{info, error};
use std::collections::HashMap;
use std::path::Path;

pub mod education;
pub mod energy;
pub mod finance;
pub mod game;
pub mod government;
pub mod logistics;
pub mod manufacturing;
pub mod mall;
pub mod medical;
pub mod newbie;
pub mod retail;
pub mod saas;
pub mod social;
pub mod transportation;

/// 场景适配器接口
/// 
/// 所有场景适配器都必须实现此接口，以确保标准化的生命周期管理
/// 
/// # 方法说明
/// - `name`: 返回适配器的名称，用于标识和管理
/// - `init`: 初始化适配器，设置必要的资源和配置
/// - `start`: 启动适配器，开始处理业务逻辑
/// - `stop`: 停止适配器，释放资源
pub trait SceneAdapter {
    /// 返回适配器的名称
    fn name(&self) -> &'static str;
    
    /// 初始化适配器
    /// 
    /// 在适配器被注册后调用，用于设置必要的资源和配置
    /// 
    /// # 返回值
    /// - `Ok(())`: 初始化成功
    /// - `Err(Box<dyn std::error::Error>)`: 初始化失败
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 启动适配器
    /// 
    /// 在初始化成功后调用，开始处理业务逻辑
    /// 
    /// # 返回值
    /// - `Ok(())`: 启动成功
    /// - `Err(Box<dyn std::error::Error>)`: 启动失败
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 停止适配器
    /// 
    /// 在适配器需要被卸载或系统关闭时调用，释放资源
    /// 
    /// # 返回值
    /// - `Ok(())`: 停止成功
    /// - `Err(Box<dyn std::error::Error>)`: 停止失败
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct SceneManager {
    pub adapters: Vec<Box<dyn SceneAdapter>>,
    dynamic_libraries: HashMap<String, Library>,
    resource_usage: HashMap<String, usize>,
}

// 导出所有场景类型
pub use education::adapter::EducationSceneAdapter;
pub use energy::adapter::EnergySceneAdapter;
pub use finance::adapter::FinanceSceneAdapter;
pub use game::adapter::GameScene;
pub use government::adapter::GovernmentSceneAdapter;
pub use logistics::adapter::LogisticsSceneAdapter;
pub use mall::adapter::MallSceneAdapter;
pub use manufacturing::adapter::ManufacturingSceneAdapter;
pub use medical::adapter::MedicalSceneAdapter;
pub use newbie::NewbieScene;
pub use retail::adapter::RetailSceneAdapter;
pub use saas::adapter::SaasScene;
pub use social::adapter::SocialScene;
pub use transportation::adapter::TransportationSceneAdapter;

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
            dynamic_libraries: HashMap::new(),
            resource_usage: HashMap::new(),
        }
    }

    pub fn register(&mut self, adapter: Box<dyn SceneAdapter>) {
        let adapter_name = adapter.name().to_string();
        let adapter_name_clone = adapter_name.clone();
        self.adapters.push(adapter);
        // 初始化资源使用跟踪
        self.resource_usage.insert(adapter_name, 0);
        info!("Registered scene adapter: {}", adapter_name_clone);
    }

    pub fn init_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing all scene adapters");
        for adapter in &mut self.adapters {
            let adapter_name = adapter.name();
            info!("Initializing adapter: {}", adapter_name);
            match adapter.init() {
                Ok(_) => {
                    info!("Successfully initialized adapter: {}", adapter_name);
                    // 更新资源使用情况
                    if let Some(usage) = self.resource_usage.get_mut(adapter_name) {
                        *usage += 1;
                    }
                }
                Err(e) => {
                    error!("Failed to initialize adapter {}: {}", adapter_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn start_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting all scene adapters");
        for adapter in &mut self.adapters {
            let adapter_name = adapter.name();
            info!("Starting adapter: {}", adapter_name);
            match adapter.start() {
                Ok(_) => {
                    info!("Successfully started adapter: {}", adapter_name);
                }
                Err(e) => {
                    error!("Failed to start adapter {}: {}", adapter_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn stop_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Stopping all scene adapters");
        for adapter in &mut self.adapters {
            let adapter_name = adapter.name();
            info!("Stopping adapter: {}", adapter_name);
            match adapter.stop() {
                Ok(_) => {
                    info!("Successfully stopped adapter: {}", adapter_name);
                }
                Err(e) => {
                    error!("Failed to stop adapter {}: {}", adapter_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn get_adapter(&self, name: &str) -> Option<&dyn SceneAdapter> {
        self.adapters
            .iter()
            .find(|a| a.name() == name)
            .map(|a| a.as_ref())
    }

    /// Load scene adapter dynamically from a shared library
    pub fn load_adapter<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        info!("Loading scene adapter from: {:?}", path);

        let library = match unsafe {
            Library::new(path)
        } {
            Ok(library) => library,
            Err(e) => {
                error!("Failed to load library from {:?}: {}", path, e);
                return Err(e.into());
            }
        };

        // Get the adapter creation function
        let create_adapter: Symbol<unsafe fn() -> *mut dyn SceneAdapter> = match unsafe {
            library.get(b"create_adapter")
        } {
            Ok(symbol) => symbol,
            Err(e) => {
                error!("Failed to get create_adapter symbol: {}", e);
                return Err(e.into());
            }
        };

        // Create the adapter
        let adapter_ptr = unsafe {
            create_adapter()
        };
        let adapter = unsafe {
            Box::from_raw(adapter_ptr)
        };

        // Register the adapter
        let adapter_name = adapter.name().to_string();
        let adapter_name_clone = adapter_name.clone();
        self.adapters.push(adapter);
        self.dynamic_libraries.insert(adapter_name, library);
        self.resource_usage.insert(adapter_name_clone.clone(), 1);

        info!("Successfully loaded scene adapter: {}", adapter_name_clone);
        Ok(())
    }

    /// Unload scene adapter by name
    pub fn unload_adapter(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Unloading scene adapter: {}", name);

        // Find and remove the adapter
        if let Some(index) = self.adapters.iter().position(|a| a.name() == name) {
            // Stop the adapter before removing it
            match self.adapters[index].stop() {
                Ok(_) => {
                    info!("Successfully stopped adapter: {}", name);
                }
                Err(e) => {
                    error!("Failed to stop adapter {}: {}", name, e);
                    // Continue with removal even if stop fails
                }
            }
            // Remove the adapter
            self.adapters.remove(index);
            // Remove the library
            if self.dynamic_libraries.remove(name).is_some() {
                // Remove resource usage tracking
                self.resource_usage.remove(name);
                info!("Successfully unloaded scene adapter: {}", name);
                Ok(())
            } else {
                let error_msg = format!("Library for adapter '{}' not found", name);
                error!("{}", error_msg);
                Err(error_msg.into())
            }
        } else {
            let error_msg = format!("Adapter '{}' not found", name);
            error!("{}", error_msg);
            Err(error_msg.into())
        }
    }

    /// Get all loaded adapters
    pub fn get_all_adapters(&self) -> Vec<&dyn SceneAdapter> {
        self.adapters.iter().map(|a| a.as_ref()).collect()
    }

    /// Get resource usage of adapters
    pub fn get_resource_usage(&self) -> &HashMap<String, usize> {
        &self.resource_usage
    }

    /// Reload scene adapter
    pub fn reload_adapter<P: AsRef<Path>>(&mut self, name: &str, path: P) -> Result<(), Box<dyn std::error::Error>> {
        info!("Reloading scene adapter: {}", name);
        // Unload the existing adapter
        if self.adapters.iter().any(|a| a.name() == name) {
            self.unload_adapter(name)?;
        }
        // Load the new adapter
        self.load_adapter(path)
    }
}

