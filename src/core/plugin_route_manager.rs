// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件路由管理器
//! 用于管理插件路由的动态注册和注销

use crate::core::state::AppState;
use crate::plugin::format::PluginRoute;
use axum::{
    body::Body,
    http::{Method, Request, Response, StatusCode},
};
use log::{error, info, warn};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// 插件路由处理器类型
pub type PluginHandler = Arc<
    dyn Fn(Request<Body>, Arc<AppState>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>>
        + Send
        + Sync,
>;

/// 插件路由管理器
#[derive(Clone)]
pub struct PluginRouteManager {
    /// 插件路由映射
    routes: Arc<tokio::sync::RwLock<HashMap<String, HashMap<Method, PluginHandler>>>>,
    /// 应用状态
    state: Arc<AppState>,
}

impl PluginRouteManager {
    /// 创建新的插件路由管理器
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            routes: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            state,
        }
    }

    /// 注册插件路由
    ///
    /// # 参数
    /// * `plugin_name` - 插件名称
    /// * `routes` - 路由列表
    ///
    /// # 返回
    /// * `Result<(), String>` - 注册结果
    pub async fn register_routes(
        &self,
        plugin_name: &str,
        routes: Vec<PluginRoute>,
    ) -> Result<(), String> {
        let mut routes_map = self.routes.write().await;

        // 检查路由冲突
        for route in &routes {
            let path = &route.path;
            let method = route.method.to_uppercase();

            if let Ok(method) = Method::from_bytes(method.as_bytes()) {
                if let Some(path_routes) = routes_map.get(path)
                    && path_routes.contains_key(&method)
                {
                    return Err(format!("路由冲突: {} {} 已被其他插件注册", method, path));
                }
            } else {
                return Err(format!("不支持的HTTP方法: {}", method));
            }
        }

        // 注册路由
        let routes_len = routes.len();
        for route in &routes {
            let path = route.path.clone();
            let method_str = route.method.to_uppercase();

            if let Ok(method) = Method::from_bytes(method_str.as_bytes()) {
                let handler = self.create_plugin_handler(route);
                let method_clone = method.clone();

                routes_map
                    .entry(path.clone())
                    .or_insert_with(HashMap::new)
                    .insert(method, handler);

                info!(
                    "注册插件路由: {} {} -> {}",
                    method_clone, path, route.handler
                );
            } else {
                warn!("不支持的HTTP方法: {} for {}", method_str, route.path);
            }
        }

        info!("插件路由注册成功: {} ({} 个路由)", plugin_name, routes_len);
        Ok(())
    }

    /// 注销插件路由
    ///
    /// # 参数
    /// * `plugin_name` - 插件名称
    /// * `routes` - 路由列表
    pub async fn unregister_routes(&self, plugin_name: &str, routes: Vec<PluginRoute>) {
        let mut routes_map = self.routes.write().await;
        let routes_count = routes.len();

        for route in routes {
            let path = route.path;
            let method_str = route.method.to_uppercase();

            if let Ok(method) = Method::from_bytes(method_str.as_bytes())
                && let Some(path_routes) = routes_map.get_mut(&path)
            {
                path_routes.remove(&method);

                // 如果路径下没有路由了，删除路径
                if path_routes.is_empty() {
                    routes_map.remove(&path);
                }

                info!("注销插件路由: {} {} (插件: {})", method, path, plugin_name);
            }
        }

        info!(
            "插件路由注销成功: {} ({} 个路由)",
            plugin_name, routes_count
        );
    }

    /// 处理插件路由请求
    ///
    /// # 参数
    /// * `path` - 路由路径
    /// * `method` - HTTP方法
    /// * `request` - HTTP请求
    ///
    /// # 返回
    /// * `Option<Response<Body>>` - 响应或None
    pub async fn handle_request(
        &self,
        path: &str,
        method: Method,
        request: Request<Body>,
    ) -> Option<Response<Body>> {
        let routes_map = self.routes.read().await;

        if let Some(path_routes) = routes_map.get(path)
            && let Some(handler) = path_routes.get(&method)
        {
            let state = self.state.clone();
            return Some(handler(request, state).await);
        }

        None
    }

    /// 创建插件路由处理器
    ///
    /// # 参数
    /// * `route` - 路由配置
    ///
    /// # 返回
    /// * `PluginHandler` - 路由处理器
    fn create_plugin_handler(&self, route: &PluginRoute) -> PluginHandler {
        let handler_name = route.handler.clone();
        let _state = self.state.clone();

        Arc::new(move |request: Request<Body>, state: Arc<AppState>| {
            let _state = state.clone();
            let handler_name = handler_name.clone();

            Box::pin(async move {
                // TODO: 调用插件的处理函数
                // 这里需要实现插件函数的动态调用
                // 可以使用dlopen或wasmi等库来实现动态加载

                error!(
                    "插件处理器未实现: {} (路径: {})",
                    handler_name,
                    request.uri().path()
                );

                Response::builder()
                    .status(StatusCode::NOT_IMPLEMENTED)
                    .body(Body::from(format!("插件处理器未实现: {}", handler_name)))
                    .unwrap()
            })
        })
    }

    /// 获取所有路由
    ///
    /// # 返回
    /// * `HashMap<String, Vec<Method>>` - 所有路由映射
    pub async fn get_all_routes(&self) -> HashMap<String, Vec<Method>> {
        let routes_map = self.routes.read().await;
        let mut all_routes = HashMap::new();

        for (path, path_routes) in routes_map.iter() {
            let methods: Vec<Method> = path_routes.keys().cloned().collect();
            all_routes.insert(path.clone(), methods);
        }

        all_routes
    }

    /// 清空所有路由
    pub async fn clear_all(&self) {
        let mut routes_map = self.routes.write().await;
        routes_map.clear();
        info!("清空所有插件路由");
    }
}

