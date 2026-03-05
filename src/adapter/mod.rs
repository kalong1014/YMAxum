// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

pub mod axum;
pub mod godot;
pub mod other;

/// 适配器层核心特质
pub trait Adapter {
    /// 初始化适配器
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 处理请求
    fn handle_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>>;
    
    /// 关闭适配器
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
