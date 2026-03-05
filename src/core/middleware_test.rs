// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;
use axum::{http::Request, routing::get, Router};  
use tower::ServiceExt;
use hyper::Body;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_rate_limiter_creation() {
    let limiter = RateLimiter::new(10, Duration::from_secs(1));
    assert_eq!(limiter.max_requests, 10);
}

#[tokio::test]
async fn test_rate_limiter_allow() {
    let limiter = RateLimiter::new(2, Duration::from_secs(1));
    
    // First request should be allowed
    let req1 = Request::builder().uri("/test").body(Body::empty()).unwrap();
    let app1 = Router::new().route("/test", get(|| async { "Ok" }));
    let res1 = limiter.middleware(req1, app1).await;
    assert!(res1.is_ok());
    
    // Second request should be allowed
    let req2 = Request::builder().uri("/test").body(Body::empty()).unwrap();
    let app2 = Router::new().route("/test", get(|| async { "Ok" }));
    let res2 = limiter.middleware(req2, app2).await;
    assert!(res2.is_ok());
}

#[tokio::test]
async fn test_rate_limiter_limit() {
    let limiter = RateLimiter::new(1, Duration::from_secs(1));
    
    // First request should be allowed
    let req1 = Request::builder().uri("/test").body(Body::empty()).unwrap();
    let app1 = Router::new().route("/test", get(|| async { "Ok" }));
    let res1 = limiter.middleware(req1, app1).await;
    assert!(res1.is_ok());
    
    // Second request should be rate limited
    let req2 = Request::builder().uri("/test").body(Body::empty()).unwrap();
    let app2 = Router::new().route("/test", get(|| async { "Ok" }));
    let res2 = limiter.middleware(req2, app2).await;
    assert!(res2.is_ok());
    let status = res2.unwrap().status();
    assert_eq!(status.as_u16(), 429); // Too Many Requests
}

#[tokio::test]
async fn test_cors_middleware() {
    let req = Request::builder()
        .uri("/test")
        .header("Origin", "https://localhost:3000")
        .body(Body::empty())
        .unwrap();
    
    let app = Router::new().route("/test", get(|| async { "Ok" }));
    let res = cors_middleware(req, app).await;
    
    assert!(res.is_ok());
    let response = res.unwrap();
    
    // Check CORS headers
    assert!(response.headers().contains_key("access-control-allow-origin"));
    assert!(response.headers().contains_key("access-control-allow-methods"));
    assert!(response.headers().contains_key("access-control-allow-headers"));
}

#[tokio::test]
async fn test_logger_middleware() {
    let req = Request::builder()
        .uri("/test")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    
    let app = Router::new().route("/test", get(|| async { "Ok" }));
    let res = logger_middleware(req, app).await;
    
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_exception_catch_middleware() {
    // Test with a normal request
    let req = Request::builder()
        .uri("/test")
        .body(Body::empty())
        .unwrap();
    
    let app = Router::new().route("/test", get(|| async { "Ok" }));
    let res = exception_catch_middleware(req, app).await;
    
    assert!(res.is_ok());
    let status = res.unwrap().status();
    assert_eq!(status.as_u16(), 200);
}

#[tokio::test]
async fn test_rate_limiter_reset() {
    let limiter = RateLimiter::new(1, Duration::from_millis(500));
    
    // First request should be allowed
    let req1 = Request::builder().uri("/test").body(Body::empty()).unwrap();
    let app1 = Router::new().route("/test", get(|| async { "Ok" }));
    let res1 = limiter.middleware(req1, app1).await;
    assert!(res1.is_ok());
    
    // Wait for the rate limiter to reset
    sleep(Duration::from_millis(600)).await;
    
    // Next request should be allowed again
    let req2 = Request::builder().uri("/test").body(Body::empty()).unwrap();
    let app2 = Router::new().route("/test", get(|| async { "Ok" }));
    let res2 = limiter.middleware(req2, app2).await;
    assert!(res2.is_ok());
}
