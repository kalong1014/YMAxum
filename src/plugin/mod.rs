// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件生态系统模块
//! 支持插件的安装、启用、停用、卸载、更新等全生命周期管理

pub mod bus;
pub mod cross_language;
pub mod dependency;
pub mod error;
pub mod format;
pub mod hot_reload;
pub mod manager;
pub mod market;
pub mod market_api;
pub mod permission;
pub mod runtime;
pub mod sandbox;
pub mod sign;
pub mod version_manager;

// 重新导出常用组件
pub use cross_language::{CrossLanguageRuntime, LanguageRuntime, PluginLanguage, RuntimeConfig};
pub use dependency::{DependencyManager, DependencyResolution, DependencyType, PluginDependency};
pub use format::{PluginFormat, PluginManifest, PluginRoute};
pub use hot_reload::PluginHotReloader;
pub use manager::{PluginInfo, PluginManager, PluginStatus};
pub use market::{
    FeedbackStatus, FeedbackType, MarketplacePlugin, MarketplaceSearchResult, MarketplaceStats,
    PluginCategory, PluginFeedback, PluginMarketplace, PluginRating,
};
pub use permission::{
    PluginPermission, PluginPermissionChecker, PluginPermissionConfig, PluginPermissionManager,
};
pub use runtime::{PluginRuntime, PluginRuntimeInfo, PluginRuntimeStatus};
pub use sandbox::PluginSandbox;
pub use sign::PluginSigner;
pub use version_manager::{PluginVersionInfo, PluginVersionManager};

