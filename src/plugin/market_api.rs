// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Plugin marketplace API routes
//! Provides HTTP API for plugin marketplace functionality

use crate::plugin::market::{
    FeedbackStatus, FeedbackType, MarketplacePlugin, PluginCategory, PluginFeedback,
    PluginMarketplace, PluginRating,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Plugin list query parameters
#[derive(Debug, Deserialize)]
pub struct PluginListQuery {
    /// Search query
    pub q: Option<String>,
    /// Plugin category
    pub category: Option<String>,
    /// Page number (default: 1)
    pub page: Option<u32>,
    /// Page size (default: 20)
    pub page_size: Option<u32>,
}

/// GUF plugin search parameters
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    /// Search query
    pub q: Option<String>,
    /// Plugin category
    pub category: Option<String>,
    /// Page number (default: 1)
    pub page: Option<u32>,
    /// Page size (default: 10)
    pub limit: Option<u32>,
}

/// Plugin rating request
#[derive(Debug, Deserialize, Serialize)]
pub struct PluginRatingRequest {
    /// Rating (0-5)
    pub rating: f32,
    /// User comment (optional)
    pub comment: Option<String>,
    /// User name
    pub user_name: String,
}

/// Plugin feedback request
#[derive(Debug, Deserialize, Serialize)]
pub struct PluginFeedbackRequest {
    /// Feedback type
    pub feedback_type: String,
    /// Feedback content
    pub content: String,
    /// User name
    pub user_name: String,
}

/// API response
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    /// Response message
    pub message: String,
    /// Response data
    pub data: Option<T>,
}

/// List all plugins
pub async fn list_plugins(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Query(params): Query<PluginListQuery>,
) -> impl IntoResponse {
    let query = params.q.unwrap_or_default();
    let category = params.category.and_then(|c| match c.as_str() {
        "basic" => Some(PluginCategory::Basic),
        "customer_service" => Some(PluginCategory::CustomerService),
        "im" => Some(PluginCategory::IM),
        "scene" => Some(PluginCategory::Scene),
        "security" => Some(PluginCategory::Security),
        "performance" => Some(PluginCategory::Performance),
        "monitoring" => Some(PluginCategory::Monitoring),
        "other" => Some(PluginCategory::Other),
        _ => None,
    });
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    let result = marketplace
        .search_plugins(&query, category, page, page_size)
        .await;

    let response = ApiResponse {
        success: true,
        message: format!("查询成功，共 {} 个插件", result.total),
        data: Some(result),
    };

    (StatusCode::OK, Json(response))
}

/// List GUF compatible plugins
async fn list_guf_plugins(State(marketplace): State<Arc<PluginMarketplace>>) -> impl IntoResponse {
    let plugins = marketplace.get_guf_compatible_plugins().await;
    let response = serde_json::json! {
        {
            "success": true,
            "plugins": plugins,
            "total": plugins.len()
        }
    };
    (StatusCode::OK, Json(response))
}

/// Search GUF compatible plugins
async fn search_guf_plugins(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let query = params.q.unwrap_or_default();
    let category = params.category.and_then(|c| match c.as_str() {
        "basic" => Some(PluginCategory::Basic),
        "customer_service" => Some(PluginCategory::CustomerService),
        "im" => Some(PluginCategory::IM),
        "scene" => Some(PluginCategory::Scene),
        "security" => Some(PluginCategory::Security),
        "performance" => Some(PluginCategory::Performance),
        "monitoring" => Some(PluginCategory::Monitoring),
        "other" => Some(PluginCategory::Other),
        _ => None,
    });
    let page = params.page.unwrap_or(1);
    let page_size = params.limit.unwrap_or(10);

    let result = marketplace
        .search_guf_plugins(&query, category, page, page_size)
        .await;
    let response = serde_json::json! {
        {
            "success": true,
            "result": result
        }
    };
    (StatusCode::OK, Json(response))
}

/// Check if plugin is GUF compatible
async fn check_guf_compatibility(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let compatible = marketplace.is_plugin_guf_compatible(&name).await;
    let guf_version = marketplace.get_plugin_guf_version(&name).await;
    let response = serde_json::json! {
        {
            "success": true,
            "compatible": compatible,
            "guf_version": guf_version
        }
    };
    (StatusCode::OK, Json(response))
}

/// Get plugin by name
pub async fn get_plugin(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Path(plugin_name): Path<String>,
) -> impl IntoResponse {
    match marketplace.get_plugin(&plugin_name).await {
        Some(plugin) => {
            let response = ApiResponse {
                success: true,
                message: "查询成功".to_string(),
                data: Some(plugin),
            };
            (StatusCode::OK, Json(response))
        }
        None => {
            let response = ApiResponse::<MarketplacePlugin> {
                success: false,
                message: format!("插件不存在: {}", plugin_name),
                data: None,
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

/// Get trending plugins
pub async fn get_trending_plugins(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Query(params): Query<PluginListQuery>,
) -> impl IntoResponse {
    let limit = params.page_size.unwrap_or(10);
    let plugins = marketplace.get_trending_plugins(limit).await;

    let response = ApiResponse {
        success: true,
        message: format!("查询成功，共 {} 个热门插件", plugins.len()),
        data: Some(plugins),
    };

    (StatusCode::OK, Json(response))
}

/// Get top rated plugins
pub async fn get_top_rated_plugins(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Query(params): Query<PluginListQuery>,
) -> impl IntoResponse {
    let limit = params.page_size.unwrap_or(10);
    let plugins = marketplace.get_top_rated_plugins(limit).await;

    let response = ApiResponse {
        success: true,
        message: format!("查询成功，共 {} 个高评分插件", plugins.len()),
        data: Some(plugins),
    };

    (StatusCode::OK, Json(response))
}

/// Get recently updated plugins
pub async fn get_recently_updated_plugins(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Query(params): Query<PluginListQuery>,
) -> impl IntoResponse {
    let limit = params.page_size.unwrap_or(10);
    let plugins = marketplace.get_recently_updated_plugins(limit).await;

    let response = ApiResponse {
        success: true,
        message: format!("查询成功，共 {} 个最近更新插件", plugins.len()),
        data: Some(plugins),
    };

    (StatusCode::OK, Json(response))
}

/// Get plugin ratings
pub async fn get_plugin_ratings(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Path(plugin_name): Path<String>,
) -> impl IntoResponse {
    let ratings = marketplace.get_plugin_ratings(&plugin_name).await;

    let response = ApiResponse {
        success: true,
        message: format!("查询成功，共 {} 条评分", ratings.len()),
        data: Some(ratings),
    };

    (StatusCode::OK, Json(response))
}

/// Rate plugin
pub async fn rate_plugin(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Path(plugin_name): Path<String>,
    Json(request): Json<PluginRatingRequest>,
) -> impl IntoResponse {
    if request.rating < 0.0 || request.rating > 5.0 {
        let response = ApiResponse::<()> {
            success: false,
            message: "评分必须在0-5之间".to_string(),
            data: None,
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let rating = PluginRating {
        plugin_name: plugin_name.clone(),
        rating: request.rating,
        comment: request.comment,
        user_name: request.user_name,
        timestamp: chrono::Utc::now().timestamp(),
        upvotes: 0,
        downvotes: 0,
        replies: Vec::new(),
    };

    match marketplace.rate_plugin(rating).await {
        Ok(_) => {
            let response = ApiResponse::<()> {
                success: true,
                message: "评分成功".to_string(),
                data: None,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()> {
                success: false,
                message: format!("评分失败: {}", e),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

/// Get plugin feedback
pub async fn get_plugin_feedback(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Path(plugin_name): Path<String>,
) -> impl IntoResponse {
    let feedback = marketplace.get_plugin_feedback(&plugin_name).await;

    let response = ApiResponse {
        success: true,
        message: format!("查询成功，共 {} 条反馈", feedback.len()),
        data: Some(feedback),
    };

    (StatusCode::OK, Json(response))
}

/// Submit plugin feedback
pub async fn submit_plugin_feedback(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Path(plugin_name): Path<String>,
    Json(request): Json<PluginFeedbackRequest>,
) -> impl IntoResponse {
    let feedback_type = match request.feedback_type.as_str() {
        "bug" => FeedbackType::Bug,
        "feature" => FeedbackType::FeatureRequest,
        "question" => FeedbackType::Question,
        _ => FeedbackType::Other,
    };

    let feedback = PluginFeedback {
        id: uuid::Uuid::new_v4().to_string(),
        plugin_name: plugin_name.clone(),
        feedback_type,
        content: request.content,
        user_name: request.user_name,
        timestamp: chrono::Utc::now().timestamp(),
        status: FeedbackStatus::Pending,
    };

    match marketplace.submit_feedback(feedback).await {
        Ok(_) => {
            let response = ApiResponse::<()> {
                success: true,
                message: "反馈提交成功".to_string(),
                data: None,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()> {
                success: false,
                message: format!("反馈提交失败: {}", e),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

/// Download plugin
pub async fn download_plugin(
    State(marketplace): State<Arc<PluginMarketplace>>,
    Path(plugin_name): Path<String>,
) -> impl IntoResponse {
    match marketplace.download_plugin(&plugin_name).await {
        Ok(_) => {
            let response = ApiResponse::<()> {
                success: true,
                message: "下载成功".to_string(),
                data: None,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()> {
                success: false,
                message: format!("下载失败: {}", e),
                data: None,
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

/// Get marketplace statistics
pub async fn get_marketplace_stats(
    State(marketplace): State<Arc<PluginMarketplace>>,
) -> impl IntoResponse {
    let stats = marketplace.get_stats().await;

    let response = ApiResponse {
        success: true,
        message: "查询成功".to_string(),
        data: Some(stats),
    };

    (StatusCode::OK, Json(response))
}

/// Register plugin marketplace routes
pub fn register_marketplace_routes(marketplace: Arc<PluginMarketplace>) -> axum::Router {
    axum::Router::new()
        .route("/api/marketplace/plugins", axum::routing::get(list_plugins))
        .route(
            "/api/marketplace/plugins/:name",
            axum::routing::get(get_plugin),
        )
        .route(
            "/api/marketplace/trending",
            axum::routing::get(get_trending_plugins),
        )
        .route(
            "/api/marketplace/top-rated",
            axum::routing::get(get_top_rated_plugins),
        )
        .route(
            "/api/marketplace/recently-updated",
            axum::routing::get(get_recently_updated_plugins),
        )
        .route(
            "/api/marketplace/guf/plugins",
            axum::routing::get(list_guf_plugins),
        )
        .route(
            "/api/marketplace/guf/search",
            axum::routing::get(search_guf_plugins),
        )
        .route(
            "/api/marketplace/guf/compatible/:name",
            axum::routing::get(check_guf_compatibility),
        )
        .route(
            "/api/marketplace/plugins/:name/ratings",
            axum::routing::get(get_plugin_ratings),
        )
        .route(
            "/api/marketplace/plugins/:name/rate",
            axum::routing::post(rate_plugin),
        )
        .route(
            "/api/marketplace/plugins/:name/feedback",
            axum::routing::get(get_plugin_feedback),
        )
        .route(
            "/api/marketplace/plugins/:name/feedback",
            axum::routing::post(submit_plugin_feedback),
        )
        .route(
            "/api/marketplace/plugins/:name/download",
            axum::routing::get(download_plugin),
        )
        .route(
            "/api/marketplace/stats",
            axum::routing::get(get_marketplace_stats),
        )
        .with_state(marketplace)
}

