// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_dependency_manager_creation() {
    let dep_manager = DependencyManager::new();
    assert!(!dep_manager.is_loaded(DependencyType::All));
}

#[tokio::test]
async fn test_dependency_manager_load_database() {
    let dep_manager = DependencyManager::new();
    let app_state = Arc::new(AppState::new());
    let result = dep_manager.load_dependencies(DependencyType::Database, app_state).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::Database));
}

#[tokio::test]
async fn test_dependency_manager_load_cache() {
    let dep_manager = DependencyManager::new();
    let app_state = Arc::new(AppState::new());
    let result = dep_manager.load_dependencies(DependencyType::Cache, app_state).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::Cache));
}

#[tokio::test]
async fn test_dependency_manager_load_redis() {
    let dep_manager = DependencyManager::new();
    let app_state = Arc::new(AppState::new());
    let result = dep_manager.load_dependencies(DependencyType::Redis, app_state).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::Redis));
}

#[tokio::test]
async fn test_dependency_manager_load_all() {
    let dep_manager = DependencyManager::new();
    let app_state = Arc::new(AppState::new());
    let result = dep_manager.load_dependencies(DependencyType::All, app_state).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::All));
}

#[tokio::test]
async fn test_dependency_manager_unload() {
    let dep_manager = DependencyManager::new();
    let app_state = Arc::new(AppState::new());
    let result = dep_manager.load_dependencies(DependencyType::All, app_state).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::All));
    let unload_result = dep_manager.unload_dependencies(DependencyType::All);
    assert!(unload_result.is_ok());
    assert!(!dep_manager.is_loaded(DependencyType::All));
}

#[tokio::test]
async fn test_dependency_manager_load_on_demand() {
    let dep_manager = DependencyManager::new();
    let app_state = Arc::new(AppState::new());
    let result = dep_manager.load_database_deps_on_demand(app_state.clone()).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::Database));
    let result = dep_manager.load_cache_deps_on_demand(app_state.clone()).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::Cache));
    let result = dep_manager.load_redis_deps_on_demand(app_state).await;
    assert!(result.is_ok());
    assert!(dep_manager.is_loaded(DependencyType::Redis));
}

#[tokio::test]
async fn test_check_dependency_versions() {
    let result = check_dependency_versions();
    assert!(result.is_ok());
}
