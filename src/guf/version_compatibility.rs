use crate::ui::core::adapter::GufVersion;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};

/// 版本兼容层
/// 负责处理不同版本 GUF 框架之间的兼容性
pub struct GufVersionCompatibilityLayer {
    /// 版本兼容映射
    compatibility_map: Arc<RwLock<HashMap<GufVersion, Vec<GufVersion>>>>,
    /// 版本特性映射
    feature_map: Arc<RwLock<HashMap<GufVersion, Vec<GufFeature>>>>,
    /// 版本回退策略
    fallback_strategy: Arc<Mutex<FallbackStrategy>>,

}

/// GUF 特性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufFeature {
    /// 特性名称
    pub name: String,
    /// 特性版本
    pub version: String,
    /// 特性描述
    pub description: String,
    /// 是否是核心特性
    pub is_core: bool,
}

/// 回退策略
#[derive(Debug, Clone)]
pub enum FallbackStrategy {
    /// 最近兼容版本
    NearestCompatible,
    /// 最低兼容版本
    LowestCompatible,
    /// 指定版本
    SpecificVersion(GufVersion),
}

/// 版本兼容性报告
#[derive(Debug, Clone)]
pub struct CompatibilityReport {
    /// 目标版本
    pub target_version: GufVersion,
    /// 兼容版本列表
    pub compatible_versions: Vec<GufVersion>,
    /// 不兼容特性
    pub incompatible_features: Vec<String>,
    /// 建议的回退版本
    pub recommended_fallback: Option<GufVersion>,
    /// 兼容性状态
    pub status: CompatibilityStatus,
}

/// 兼容性状态
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityStatus {
    /// 完全兼容
    FullyCompatible,
    /// 部分兼容
    PartiallyCompatible,
    /// 不兼容
    Incompatible,
}

impl GufVersionCompatibilityLayer {
    /// 创建新的版本兼容层
    pub fn new() -> Self {
        let compatibility_map = Arc::new(RwLock::new(HashMap::new()));
        let feature_map = Arc::new(RwLock::new(HashMap::new()));
        let fallback_strategy = Arc::new(Mutex::new(FallbackStrategy::NearestCompatible));

        Self {
            compatibility_map,
            feature_map,
            fallback_strategy,
        }
    }

    /// 初始化版本兼容层
    pub async fn initialize(&self) -> Result<(), String> {
        // 初始化默认的版本兼容性映射
        self.initialize_default_compatibility_map().await;
        // 初始化默认的特性映射
        self.initialize_default_feature_map().await;
        Ok(())
    }

    /// 初始化默认的版本兼容性映射
    async fn initialize_default_compatibility_map(&self) {
        let mut map = self.compatibility_map.write().await;
        
        // 定义版本兼容性映射
        // 4.3.x 兼容 4.3.x 和 4.4.x
        map.insert(
            GufVersion { major: 4, minor: 3, patch: 0 },
            vec![
                GufVersion { major: 4, minor: 3, patch: 0 },
                GufVersion { major: 4, minor: 3, patch: 1 },
                GufVersion { major: 4, minor: 4, patch: 0 },
            ],
        );
        
        // 4.3.1 兼容 4.3.x 和 4.4.x
        map.insert(
            GufVersion { major: 4, minor: 3, patch: 1 },
            vec![
                GufVersion { major: 4, minor: 3, patch: 0 },
                GufVersion { major: 4, minor: 3, patch: 1 },
                GufVersion { major: 4, minor: 4, patch: 0 },
            ],
        );
        
        // 4.4.x 兼容 4.3.x 和 4.4.x
        map.insert(
            GufVersion { major: 4, minor: 4, patch: 0 },
            vec![
                GufVersion { major: 4, minor: 3, patch: 0 },
                GufVersion { major: 4, minor: 3, patch: 1 },
                GufVersion { major: 4, minor: 4, patch: 0 },
            ],
        );
        
        // 4.4.1 兼容 4.3.x 和 4.4.x
        map.insert(
            GufVersion { major: 4, minor: 4, patch: 1 },
            vec![
                GufVersion { major: 4, minor: 3, patch: 0 },
                GufVersion { major: 4, minor: 3, patch: 1 },
                GufVersion { major: 4, minor: 4, patch: 0 },
                GufVersion { major: 4, minor: 4, patch: 1 },
            ],
        );
    }

    /// 初始化默认的特性映射
    async fn initialize_default_feature_map(&self) {
        let mut map = self.feature_map.write().await;
        
        // 4.3.0 特性
        map.insert(
            GufVersion { major: 4, minor: 3, patch: 0 },
            vec![
                GufFeature {
                    name: "core_ui".to_string(),
                    version: "4.3.0".to_string(),
                    description: "Core UI components".to_string(),
                    is_core: true,
                },
                GufFeature {
                    name: "event_system".to_string(),
                    version: "4.3.0".to_string(),
                    description: "Event system".to_string(),
                    is_core: true,
                },
            ],
        );
        
        // 4.3.1 特性
        map.insert(
            GufVersion { major: 4, minor: 3, patch: 1 },
            vec![
                GufFeature {
                    name: "core_ui".to_string(),
                    version: "4.3.1".to_string(),
                    description: "Core UI components".to_string(),
                    is_core: true,
                },
                GufFeature {
                    name: "event_system".to_string(),
                    version: "4.3.1".to_string(),
                    description: "Event system".to_string(),
                    is_core: true,
                },
                GufFeature {
                    name: "performance_optimization".to_string(),
                    version: "4.3.1".to_string(),
                    description: "Performance optimization improvements".to_string(),
                    is_core: false,
                },
            ],
        );
        
        // 4.4.0 特性
        map.insert(
            GufVersion { major: 4, minor: 4, patch: 0 },
            vec![
                GufFeature {
                    name: "core_ui".to_string(),
                    version: "4.4.0".to_string(),
                    description: "Core UI components".to_string(),
                    is_core: true,
                },
                GufFeature {
                    name: "event_system".to_string(),
                    version: "4.4.0".to_string(),
                    description: "Event system".to_string(),
                    is_core: true,
                },
                GufFeature {
                    name: "component_pool".to_string(),
                    version: "4.4.0".to_string(),
                    description: "Component pool system".to_string(),
                    is_core: false,
                },
            ],
        );
        
        // 4.4.1 特性
        map.insert(
            GufVersion { major: 4, minor: 4, patch: 1 },
            vec![
                GufFeature {
                    name: "core_ui".to_string(),
                    version: "4.4.1".to_string(),
                    description: "Core UI components".to_string(),
                    is_core: true,
                },
                GufFeature {
                    name: "event_system".to_string(),
                    version: "4.4.1".to_string(),
                    description: "Event system".to_string(),
                    is_core: true,
                },
                GufFeature {
                    name: "component_pool".to_string(),
                    version: "4.4.1".to_string(),
                    description: "Component pool system".to_string(),
                    is_core: false,
                },
                GufFeature {
                    name: "version_compatibility".to_string(),
                    version: "4.4.1".to_string(),
                    description: "Enhanced version compatibility support".to_string(),
                    is_core: false,
                },
                GufFeature {
                    name: "advanced_health_check".to_string(),
                    version: "4.4.1".to_string(),
                    description: "Advanced component health check system".to_string(),
                    is_core: false,
                },
            ],
        );
    }

    /// 检查版本兼容性
    pub async fn check_compatibility(&self, source_version: &GufVersion, target_version: &GufVersion) -> CompatibilityStatus {
        let map = self.compatibility_map.read().await;
        let feature_map = self.feature_map.read().await;
        
        // 检查源版本是否在兼容性映射中
        if let Some(compatible_versions) = map.get(source_version) {
            // 检查目标版本是否在兼容列表中
            if compatible_versions.contains(&target_version) {
                return CompatibilityStatus::FullyCompatible;
            }
        }
        
        // 检查主要版本是否相同
        if source_version.major == target_version.major {
            // 检查核心特性是否兼容
            if let (Some(source_features), Some(target_features)) = (
                feature_map.get(source_version),
                feature_map.get(target_version)
            ) {
                let source_core_features: std::collections::HashSet<_> = source_features
                    .iter()
                    .filter(|f| f.is_core)
                    .map(|f| f.name.clone())
                    .collect();
                
                let target_core_features: std::collections::HashSet<_> = target_features
                    .iter()
                    .filter(|f| f.is_core)
                    .map(|f| f.name.clone())
                    .collect();
                
                // 如果核心特性完全匹配，视为完全兼容
                if source_core_features == target_core_features {
                    return CompatibilityStatus::FullyCompatible;
                }
            }
            return CompatibilityStatus::PartiallyCompatible;
        }
        
        CompatibilityStatus::Incompatible
    }

    /// 生成兼容性报告
    pub async fn generate_compatibility_report(&self, target_version: &GufVersion) -> CompatibilityReport {
        let compatibility_map = self.compatibility_map.read().await;
        let feature_map = self.feature_map.read().await;
        
        let mut compatible_versions = Vec::new();
        let mut incompatible_features = Vec::new();
        
        // 找出所有兼容的版本
        for (version, compat_versions) in compatibility_map.iter() {
            if compat_versions.contains(&target_version) {
                compatible_versions.push(version.clone());
            }
        }
        
        // 检查特性兼容性
        if let Some(target_features) = feature_map.get(target_version) {
            for feature in target_features {
                if feature.is_core {
                    // 检查核心特性是否在其他版本中存在
                    let mut found = false;
                    for (version, features) in feature_map.iter() {
                        if version != target_version {
                            if features.iter().any(|f| f.name == feature.name) {
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        incompatible_features.push(feature.name.clone());
                    }
                }
            }
        }
        
        // 确定兼容性状态
        let status = if incompatible_features.is_empty() {
            CompatibilityStatus::FullyCompatible
        } else if !compatible_versions.is_empty() {
            CompatibilityStatus::PartiallyCompatible
        } else {
            CompatibilityStatus::Incompatible
        };
        
        // 确定建议的回退版本
        let recommended_fallback = self.determine_fallback_version(&compatible_versions, target_version).await;
        
        CompatibilityReport {
            target_version: target_version.clone(),
            compatible_versions,
            incompatible_features,
            recommended_fallback,
            status,
        }
    }

    /// 确定回退版本
    async fn determine_fallback_version(&self, compatible_versions: &[GufVersion], target_version: &GufVersion) -> Option<GufVersion> {
        if compatible_versions.is_empty() {
            return None;
        }
        
        // 使用 Mutex 安全地获取 fallback_strategy
        let fallback_strategy = self.fallback_strategy.lock().await;
        match &*fallback_strategy {
            FallbackStrategy::NearestCompatible => {
                // 找到最接近目标版本的兼容版本
                let mut nearest = None;
                let mut min_diff = i32::MAX;
                
                for version in compatible_versions {
                    let diff = (version.major as i32 - target_version.major as i32).abs() * 1000 +
                              (version.minor as i32 - target_version.minor as i32).abs() * 100 +
                              (version.patch as i32 - target_version.patch as i32).abs();
                    
                    if diff < min_diff {
                        min_diff = diff;
                        nearest = Some(version.clone());
                    }
                }
                
                nearest
            },
            FallbackStrategy::LowestCompatible => {
                // 找到最低的兼容版本
                compatible_versions.iter()
                    .min_by(|a, b| {
                        if a.major != b.major {
                            a.major.cmp(&b.major)
                        } else if a.minor != b.minor {
                            a.minor.cmp(&b.minor)
                        } else {
                            a.patch.cmp(&b.patch)
                        }
                    })
                    .cloned()
            },
            FallbackStrategy::SpecificVersion(version) => {
                // 检查指定版本是否兼容
                if compatible_versions.contains(version) {
                    Some(version.clone())
                } else {
                    // 如果指定版本不兼容，回退到最接近的版本
                    // 避免递归，直接使用 NearestCompatible 策略
                    let mut nearest = None;
                    let mut min_diff = i32::MAX;
                    
                    for v in compatible_versions {
                        let diff = (v.major as i32 - version.major as i32).abs() * 1000 +
                                  (v.minor as i32 - version.minor as i32).abs() * 100 +
                                  (v.patch as i32 - version.patch as i32).abs();
                        
                        if diff < min_diff {
                            min_diff = diff;
                            nearest = Some(v.clone());
                        }
                    }
                    
                    nearest
                }
            },
        }
    }

    /// 自动选择最佳回退版本
    pub async fn auto_select_fallback_version(&self, target_version: &GufVersion) -> Option<GufVersion> {
        let report = self.generate_compatibility_report(target_version).await;
        self.determine_fallback_version(&report.compatible_versions, target_version).await
    }

    /// 设置回退策略
    pub async fn set_fallback_strategy(&self, strategy: FallbackStrategy) {
        // 使用 Mutex 安全地修改 fallback_strategy
        let mut fallback_strategy = self.fallback_strategy.lock().await;
        *fallback_strategy = strategy;
    }

    /// 添加版本兼容性映射
    pub async fn add_compatibility_mapping(&self, source_version: GufVersion, compatible_versions: Vec<GufVersion>) {
        let mut map = self.compatibility_map.write().await;
        map.insert(source_version, compatible_versions);
    }

    /// 添加特性映射
    pub async fn add_feature_mapping(&self, version: GufVersion, features: Vec<GufFeature>) {
        let mut map = self.feature_map.write().await;
        map.insert(version, features);
    }

    /// 获取版本特性
    pub async fn get_version_features(&self, version: &GufVersion) -> Option<Vec<GufFeature>> {
        let map = self.feature_map.read().await;
        map.get(version).cloned()
    }

    /// 检查特性是否在版本中可用
    pub async fn is_feature_available(&self, version: &GufVersion, feature_name: &str) -> bool {
        let map = self.feature_map.read().await;
        if let Some(features) = map.get(version) {
            features.iter().any(|f| f.name == feature_name)
        } else {
            false
        }
    }
}

/// 示例使用
pub async fn example_usage() {
    // 创建版本兼容层
    let compatibility_layer = GufVersionCompatibilityLayer::new();
    
    // 初始化
    compatibility_layer.initialize().await.unwrap();
    
    // 检查版本兼容性
    let source_version = GufVersion { major: 4, minor: 3, patch: 0 };
    let target_version = GufVersion { major: 4, minor: 4, patch: 0 };
    
    let status = compatibility_layer.check_compatibility(&source_version, &target_version).await;
    debug!("Compatibility status: {:?}", status);
    
    // 生成兼容性报告
    let report = compatibility_layer.generate_compatibility_report(&target_version).await;
    debug!("Compatibility report: {:?}", report);
    
    // 检查特性是否可用
    let is_available = compatibility_layer.is_feature_available(&target_version, "component_pool").await;
    debug!("Component pool feature available: {:?}", is_available);
}