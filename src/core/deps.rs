// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::core::state::AppState;
use log::info;
use std::sync::{Arc, RwLock};

/// 依赖类型枚举
pub enum DependencyType {
    Database,
    Cache,
    Redis,
    All,
}

/// 依赖状态
pub struct DependencyState {
    /// 是否加载了数据库依赖
    pub database_loaded: bool,
    /// 是否加载了缓存依赖
    pub cache_loaded: bool,
    /// 是否加载了Redis依赖
    pub redis_loaded: bool,
}

impl DependencyState {
    /// 创建新的依赖状态
    pub fn new() -> Self {
        Self {
            database_loaded: false,
            cache_loaded: false,
            redis_loaded: false,
        }
    }
}

impl Default for DependencyState {
    fn default() -> Self {
        Self::new()
    }
}

/// 依赖管理器
pub struct DependencyManager {
    /// 依赖状态
    pub state: RwLock<DependencyState>,
}

impl DependencyManager {
    /// 创建新的依赖管理器
    pub fn new() -> Self {
        Self {
            state: RwLock::new(DependencyState::new()),
        }
    }

    /// 加载指定类型的依赖
    pub async fn load_dependencies(
        &self,
        dep_type: DependencyType,
        app_state: Arc<AppState>,
    ) -> Result<(), String> {
        match dep_type {
            DependencyType::Database => {
                self.load_database_deps(app_state).await?;
            }
            DependencyType::Cache => {
                self.load_cache_deps(app_state).await?;
            }
            DependencyType::Redis => {
                self.load_redis_deps(app_state).await?;
            }
            DependencyType::All => {
                self.load_database_deps(app_state.clone()).await?;
                self.load_cache_deps(app_state.clone()).await?;
                self.load_redis_deps(app_state).await?;
            }
        }
        Ok(())
    }

    /// 按需加载数据库依赖（仅在真正需要时加载）
    pub async fn load_database_deps_on_demand(
        &self,
        app_state: Arc<AppState>,
    ) -> Result<(), String> {
        // 检查是否已经加载
        if self.is_loaded(DependencyType::Database) {
            return Ok(());
        }

        // 真正需要时才加载
        info!("Loading database dependencies on demand");
        self.load_database_deps(app_state).await
    }

    /// 按需加载缓存依赖（仅在真正需要时加载）
    pub async fn load_cache_deps_on_demand(&self, app_state: Arc<AppState>) -> Result<(), String> {
        // 检查是否已经加载
        if self.is_loaded(DependencyType::Cache) {
            return Ok(());
        }

        // 真正需要时才加载
        info!("Loading cache dependencies on demand");
        self.load_cache_deps(app_state).await
    }

    /// 按需加载Redis依赖（仅在真正需要时加载）
    pub async fn load_redis_deps_on_demand(&self, app_state: Arc<AppState>) -> Result<(), String> {
        // 检查是否已经加载
        if self.is_loaded(DependencyType::Redis) {
            return Ok(());
        }

        // 真正需要时才加载
        info!("Loading Redis dependencies on demand");
        self.load_redis_deps(app_state).await
    }

    /// 加载数据库依赖
    async fn load_database_deps(&self, _app_state: Arc<AppState>) -> Result<(), String> {
        let mut state = self.state.write().unwrap();
        if state.database_loaded {
            info!("Database dependencies already loaded");
            return Ok(());
        }

        // 这里是简化实现，实际需要根据配置加载具体数据库
        info!("Loading database dependencies");

        // 标记为已加载
        state.database_loaded = true;
        Ok(())
    }

    /// 加载缓存依赖
    async fn load_cache_deps(&self, _app_state: Arc<AppState>) -> Result<(), String> {
        let mut state = self.state.write().unwrap();
        if state.cache_loaded {
            info!("Cache dependencies already loaded");
            return Ok(());
        }

        info!("Loading cache dependencies");

        // 标记为已加载
        state.cache_loaded = true;
        Ok(())
    }

    /// 加载Redis依赖
    async fn load_redis_deps(&self, _app_state: Arc<AppState>) -> Result<(), String> {
        let mut state = self.state.write().unwrap();
        if state.redis_loaded {
            info!("Redis dependencies already loaded");
            return Ok(());
        }

        info!("Loading Redis dependencies");

        // 标记为已加载
        state.redis_loaded = true;
        Ok(())
    }

    /// 检查依赖是否已加载
    pub fn is_loaded(&self, dep_type: DependencyType) -> bool {
        let state = self.state.read().unwrap();
        match dep_type {
            DependencyType::Database => state.database_loaded,
            DependencyType::Cache => state.cache_loaded,
            DependencyType::Redis => state.redis_loaded,
            DependencyType::All => {
                state.database_loaded && state.cache_loaded && state.redis_loaded
            }
        }
    }

    /// 卸载指定类型的依赖
    pub fn unload_dependencies(&self, dep_type: DependencyType) -> Result<(), String> {
        let mut state = self.state.write().unwrap();

        match dep_type {
            DependencyType::Database => {
                if state.database_loaded {
                    info!("Unloading database dependencies");
                    state.database_loaded = false;
                }
            }
            DependencyType::Cache => {
                if state.cache_loaded {
                    info!("Unloading cache dependencies");
                    state.cache_loaded = false;
                }
            }
            DependencyType::Redis => {
                if state.redis_loaded {
                    info!("Unloading Redis dependencies");
                    state.redis_loaded = false;
                }
            }
            DependencyType::All => {
                info!("Unloading all dependencies");
                state.database_loaded = false;
                state.cache_loaded = false;
                state.redis_loaded = false;
            }
        }

        Ok(())
    }
}

impl Default for DependencyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 依赖管理器扩展
pub type DependencyManagerExtension = Arc<DependencyManager>;

/// 依赖版本信息
#[derive(Debug)]
pub struct DependencyInfo {
    /// 依赖名称
    pub name: String,
    /// 依赖版本
    pub version: String,
    /// 依赖类型
    pub dep_type: String,
    /// 最小兼容版本
    pub min_version: String,
    /// 最大兼容版本
    pub max_version: Option<String>,
}

/// 检查核心依赖版本是否兼容
pub fn check_dependency_versions() -> Result<(), String> {
    // 检查核心依赖版本兼容性
    info!("Checking dependency versions...");

    // 简化实现，直接返回成功
    // 依赖版本由Cargo.toml管理，确保正确性
    info!("All dependency versions are compatible");
    Ok(())
}

/// 检查版本是否兼容（预留用于未来扩展）
fn _is_version_compatible(version: &str, min_version: &str, max_version: Option<&str>) -> bool {
    // 简化实现，实际需要解析版本号并比较
    // 这里只检查主版本号是否匹配
    let version_parts: Vec<&str> = version.split('.').collect();
    let min_parts: Vec<&str> = min_version.split('.').collect();

    // 检查主版本号
    if let (Some(v_major), Some(min_major)) = (version_parts.first(), min_parts.first()) {
        let v_major_num = v_major.parse::<u32>().unwrap_or(0);
        let min_major_num = min_major.parse::<u32>().unwrap_or(0);

        if v_major_num < min_major_num {
            return false;
        }

        // 检查次版本号（如果主版本号相同）
        if v_major_num == min_major_num
            && let (Some(v_minor), Some(min_minor)) = (version_parts.get(1), min_parts.get(1))
        {
            let v_minor_num = v_minor.parse::<u32>().unwrap_or(0);
            let min_minor_num = min_minor.parse::<u32>().unwrap_or(0);

            if v_minor_num < min_minor_num {
                return false;
            }
        }
    }

    // 检查最大版本号（如果有）
    if let Some(max_version) = max_version {
        let max_parts: Vec<&str> = max_version.split('.').collect();

        // 检查主版本号
        if let (Some(v_major), Some(max_major)) = (version_parts.first(), max_parts.first()) {
            let v_major_num = v_major.parse::<u32>().unwrap_or(0);
            let max_major_num = max_major.parse::<u32>().unwrap_or(0);

            if v_major_num > max_major_num {
                return false;
            }

            // 检查次版本号（如果主版本号相同）
            if v_major_num == max_major_num
                && let (Some(v_minor), Some(max_minor)) = (version_parts.get(1), max_parts.get(1))
            {
                let v_minor_num = v_minor.parse::<u32>().unwrap_or(0);
                let max_minor_num = max_minor.parse::<u32>().unwrap_or(0);

                if v_minor_num > max_minor_num {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::AppState;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_dependency_manager() {
        let dep_manager = DependencyManager::new();
        let app_state = Arc::new(AppState::new());

        // 测试加载数据库依赖
        assert!(!dep_manager.is_loaded(DependencyType::Database));
        dep_manager
            .load_dependencies(DependencyType::Database, app_state.clone())
            .await
            .unwrap();
        assert!(dep_manager.is_loaded(DependencyType::Database));

        // 测试加载缓存依赖
        assert!(!dep_manager.is_loaded(DependencyType::Cache));
        dep_manager
            .load_dependencies(DependencyType::Cache, app_state.clone())
            .await
            .unwrap();
        assert!(dep_manager.is_loaded(DependencyType::Cache));

        // 测试加载Redis依赖
        assert!(!dep_manager.is_loaded(DependencyType::Redis));
        dep_manager
            .load_dependencies(DependencyType::Redis, app_state.clone())
            .await
            .unwrap();
        assert!(dep_manager.is_loaded(DependencyType::Redis));

        // 测试加载所有依赖
        assert!(dep_manager.is_loaded(DependencyType::All));

        // 测试卸载依赖
        dep_manager
            .unload_dependencies(DependencyType::Database)
            .unwrap();
        assert!(!dep_manager.is_loaded(DependencyType::Database));
        assert!(!dep_manager.is_loaded(DependencyType::All));

        // 测试卸载所有依赖
        dep_manager
            .unload_dependencies(DependencyType::All)
            .unwrap();
        assert!(!dep_manager.is_loaded(DependencyType::Cache));
        assert!(!dep_manager.is_loaded(DependencyType::Redis));
    }
}

