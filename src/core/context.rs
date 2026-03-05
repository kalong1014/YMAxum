// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::core::state::AppState;
use crate::error::YMAxumError;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Simplified type definition for next handler function
/// Uses type alias to simplify complex function types
pub type NextFn =
    Box<dyn Fn(Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>> + Send>;

/// Context struct that encapsulates state and next function for unified Handler/Middleware interface
/// Inspired by Salvo framework's design, providing a cleaner API
pub struct Context {
    /// Request object
    pub request: Request<Body>,
    /// Application state
    state: Arc<AppState>,
    /// Next handler function
    next: Option<NextFn>,
}

impl Context {
    /// Create new context
    pub fn new(request: Request<Body>, state: Arc<AppState>, next: Option<NextFn>) -> Self {
        Self {
            request,
            state,
            next,
        }
    }

    /// Create context without next function (for final handlers)
    pub fn without_next(request: Request<Body>, state: Arc<AppState>) -> Self {
        Self::new(request, state, None)
    }

    /// Get application state
    pub fn state(&self) -> &Arc<AppState> {
        &self.state
    }

    /// Get request reference
    pub fn request(&self) -> &Request<Body> {
        &self.request
    }

    /// Get mutable request reference
    pub fn request_mut(&mut self) -> &mut Request<Body> {
        &mut self.request
    }

    /// Call next handler function
    pub async fn next(mut self) -> Response<Body> {
        if let Some(next) = self.next.take() {
            next(self.request).await
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
        }
    }

    /// Get reference to next handler function
    pub fn next_ref(&self) -> &Option<NextFn> {
        &self.next
    }

    /// Replace next handler function, returning old one
    pub fn replace_next(&mut self, next: Option<NextFn>) -> Option<NextFn> {
        std::mem::replace(&mut self.next, next)
    }

    /// Get clone of application state
    pub fn state_clone(&self) -> Arc<AppState> {
        self.state.clone()
    }

    /// Create response
    pub fn into_response(self, response: Response<Body>) -> Response<Body> {
        response
    }

    /// Check if there's a next handler function
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Create JSON response
    pub fn json<T: serde::Serialize>(self, data: T) -> Response<Body> {
        match serde_json::to_string(&data) {
            Ok(json) => Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to serialize JSON"))
                .unwrap(),
        }
    }

    /// Create text response
    pub fn text(self, text: impl Into<String>) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Body::from(text.into()))
            .unwrap()
    }

    /// Create HTML response
    pub fn html(self, html: impl Into<String>) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/html; charset=utf-8")
            .body(Body::from(html.into()))
            .unwrap()
    }

    /// Create error response
    pub fn error(self, status: StatusCode, message: impl Into<String>) -> Response<Body> {
        let error_json = serde_json::json!({
            "error": {
                "status": status.as_u16(),
                "message": message.into()
            }
        });

        match serde_json::to_string(&error_json) {
            Ok(json) => Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to serialize error"))
                .unwrap(),
        }
    }

    /// Create error response from YMAxumError
    pub fn yma_error(self, error: YMAxumError) -> Response<Body> {
        let (status, body) = error.to_http_response();
        Response::builder()
            .status(StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_context_creation() {
        let app_state = Arc::new(AppState::new());

        // Test context creation without next function (simpler test)
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let _context = Context::without_next(request, app_state);

        // Just ensure the method doesn't panic
    }

    #[tokio::test]
    async fn test_context_state() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context = Context::without_next(request, app_state.clone());

        // Test state() method
        let state_ref = context.state();
        assert_eq!(state_ref.version, app_state.version);

        // Test state_clone() method
        let state_clone = context.state_clone();
        assert_eq!(state_clone.version, app_state.version);
    }

    #[tokio::test]
    async fn test_context_request() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let mut context = Context::without_next(request, app_state);

        // Test request() method
        let request_ref = context.request();
        assert_eq!(request_ref.uri().to_string(), "/test");

        // Test request_mut() method
        let request_mut = context.request_mut();
        assert_eq!(request_mut.uri().to_string(), "/test");
    }

    #[tokio::test]
    async fn test_context_has_next() {
        let app_state = Arc::new(AppState::new());

        // Test has_next() without next function (simpler test)
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context_without_next = Context::without_next(request, app_state);
        assert!(!context_without_next.has_next());
    }

    #[tokio::test]
    async fn test_context_next_ref() {
        let app_state = Arc::new(AppState::new());

        // Test next_ref() without next function (simpler test)
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context_without_next = Context::without_next(request, app_state);
        assert!(context_without_next.next_ref().is_none());
    }

    #[tokio::test]
    async fn test_context_replace_next() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();

        // Test replace_next() with a simple context
        let mut context = Context::without_next(request, app_state);

        // Just ensure the method doesn't panic
        let old_next = context.replace_next(None);
        assert!(old_next.is_none());
    }

    #[tokio::test]
    async fn test_context_into_response() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context = Context::without_next(request, app_state);

        // Test into_response()
        let response = Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap();
        let result = context.into_response(response);
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_context_json() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context = Context::without_next(request, app_state);

        // Test json() method
        let test_data = serde_json::json!({
            "message": "Hello, World!",
            "status": "ok"
        });
        let response = context.json(test_data);
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_context_text() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context = Context::without_next(request, app_state);

        // Test text() method
        let response = context.text("Hello, World!");
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_context_html() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context = Context::without_next(request, app_state);

        // Test html() method
        let response = context.html("<html><body>Hello, World!</body></html>");
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_context_error() {
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context = Context::without_next(request, app_state);

        // Test error() method
        let response = context.error(StatusCode::NOT_FOUND, "Resource not found");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_context_next() {
        // Test next() method without next function
        let app_state = Arc::new(AppState::new());
        let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let context = Context::without_next(request, app_state);
        let response = context.next().await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

