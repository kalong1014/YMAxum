// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::core::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

/// 路由生成器，用于自动生成标准CRUD接口
pub struct RouteGenerator {
    app_state: Arc<AppState>,
}

impl RouteGenerator {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    /// 生成标准CRUD路由
    pub fn generate_crud_routes(&self, table_name: &str) -> Router<Arc<AppState>> {
        let base_path = format!("/api/{}", table_name);

        Router::new()
            .route(&base_path, axum::routing::get(Self::list_items))
            .route(&base_path, axum::routing::post(Self::create_item))
            .route(
                &format!("{}/{{id}}", base_path),
                axum::routing::get(Self::get_item),
            )
            .route(
                &format!("{}/{{id}}", base_path),
                axum::routing::put(Self::update_item),
            )
            .route(
                &format!("{}/{{id}}", base_path),
                axum::routing::delete(Self::delete_item),
            )
            .with_state(self.app_state.clone())
    }

    /// 列出所有项目
    async fn list_items(
        State(_state): State<Arc<AppState>>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        Ok(Json(serde_json::json!({
            "status": "success",
            "message": "List items",
            "data": [],
        })))
    }

    /// 创建新项目
    async fn create_item(
        State(_state): State<Arc<AppState>>,
        Json(item): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Item created",
            "data": item,
        })))
    }

    /// 获取单个项目
    async fn get_item(
        State(_state): State<Arc<AppState>>,
        Path(id): Path<String>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Get item",
            "data": { "id": id },
        })))
    }

    /// 更新项目
    async fn update_item(
        State(_state): State<Arc<AppState>>,
        Path(id): Path<String>,
        Json(item): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Item updated",
            "data": { "id": id, "item": item },
        })))
    }

    /// 删除项目
    async fn delete_item(
        State(_state): State<Arc<AppState>>,
        Path(id): Path<String>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Item deleted",
            "data": { "id": id },
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::AppState;

    #[tokio::test]
    async fn test_route_generator() {
        let app_state = Arc::new(AppState::new());
        let generator = RouteGenerator::new(app_state);
        let _routes = generator.generate_crud_routes("users");
    }
}

