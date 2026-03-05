// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;
use axum::{http::Request, routing::get, Router};  
use tower::ServiceExt;
use hyper::Body;
use std::sync::Arc;
use tokio::time::Duration;

#[tokio::test]
async fn test_create_router() {
    let router = create_router();
    assert!(!router.is_empty());
}

#[tokio::test]
async fn test_register_route() {
    let mut router = create_router();
    let path = "/test_route";
    let handler = get(|| async { "Test route" });
    
    let updated_router = register_route(router, path, handler);
    assert!(!updated_router.is_empty());
}

#[tokio::test]
async fn test_register_routes() {
    let router = create_router();
    let routes = vec![
        ("/test1", get(|| async { "Test 1" })),
        ("/test2", get(|| async { "Test 2" })),
    ];
    
    let updated_router = register_routes(router, routes);
    assert!(!updated_router.is_empty());
}

#[tokio::test]
async fn test_health_check_route() {
    use crate::core::state::AppState;
    
    let app_state = Arc::new(AppState::new());
    let router = create_router_with_state(app_state.clone());
    
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    
    let res = router.oneshot(req).await;
    assert!(res.is_ok());
    let status = res.unwrap().status();
    assert_eq!(status.as_u16(), 200);
}

#[tokio::test]
async fn test_root_route() {
    use crate::core::state::AppState;
    
    let app_state = Arc::new(AppState::new());
    let router = create_router_with_state(app_state.clone());
    
    let req = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();
    
    let res = router.oneshot(req).await;
    assert!(res.is_ok());
    let status = res.unwrap().status();
    assert_eq!(status.as_u16(), 200);
}

#[tokio::test]
async fn test_api_routes() {
    use crate::core::state::AppState;
    
    let app_state = Arc::new(AppState::new());
    let router = create_router_with_state(app_state.clone());
    
    // Test /api/v1/test route
    let req = Request::builder()
        .uri("/api/v1/test")
        .body(Body::empty())
        .unwrap();
    
    let res = router.oneshot(req).await;
    assert!(res.is_ok());
    let status = res.unwrap().status();
    assert_eq!(status.as_u16(), 200);
}

#[tokio::test]
async fn test_metrics_route() {
    use crate::core::state::AppState;
    use crate::performance::monitor::PerformanceMonitor;
    
    let app_state = Arc::new(AppState::new());
    let monitor = PerformanceMonitor::new(app_state.clone()).unwrap();
    app_state.set_performance_monitor(monitor).await;
    
    let router = create_router_with_state(app_state.clone());
    
    let req = Request::builder()
        .uri("/metrics")
        .body(Body::empty())
        .unwrap();
    
    let res = router.oneshot(req).await;
    assert!(res.is_ok());
    let status = res.unwrap().status();
    assert_eq!(status.as_u16(), 200);
}

#[tokio::test]
async fn test_plugins_route() {
    use crate::core::state::AppState;
    use crate::plugin::PluginManager;
    
    let app_state = Arc::new(AppState::new());
    let plugin_manager = PluginManager::new().unwrap();
    app_state.set_plugin_manager(plugin_manager).await;
    
    let router = create_router_with_state(app_state.clone());
    
    let req = Request::builder()
        .uri("/api/v1/plugins")
        .body(Body::empty())
        .unwrap();
    
    let res = router.oneshot(req).await;
    assert!(res.is_ok());
    let status = res.unwrap().status();
    assert_eq!(status.as_u16(), 200);
}

#[tokio::test]
async fn test_marketplace_routes() {
    use crate::core::state::AppState;
    use crate::plugin::market::PluginMarketplace;
    
    let app_state = Arc::new(AppState::new());
    let marketplace = PluginMarketplace::new();
    app_state.set_plugin_marketplace(marketplace).await;
    
    let router = create_router_with_state(app_state.clone());
    
    // Test plugins list route
    let req1 = Request::builder()
        .uri("/api/v1/marketplace/plugins")
        .body(Body::empty())
        .unwrap();
    
    let res1 = router.oneshot(req1).await;
    assert!(res1.is_ok());
    let status1 = res1.unwrap().status();
    assert_eq!(status1.as_u16(), 200);
    
    // Test stats route
    let req2 = Request::builder()
        .uri("/api/v1/marketplace/stats")
        .body(Body::empty())
        .unwrap();
    
    let res2 = router.oneshot(req2).await;
    assert!(res2.is_ok());
    let status2 = res2.unwrap().status();
    assert_eq!(status2.as_u16(), 200);
}

#[tokio::test]
async fn test_routing_with_middleware() {
    use crate::core::state::AppState;
    
    let app_state = Arc::new(AppState::new());
    let router = create_router_with_state(app_state.clone());
    
    // Test that routes work with middleware
    let req = Request::builder()
        .uri("/test")
        .body(Body::empty())
        .unwrap();
    
    let res = router.oneshot(req).await;
    assert!(res.is_ok());
}
