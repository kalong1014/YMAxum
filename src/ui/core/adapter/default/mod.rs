/// 默认GUF适配器
///
/// 提供基本的GUF适配功能，作为其他版本适配器的基础
use super::{GufAdapter, GufVersion};
use async_trait::async_trait;
use uuid;

/// 默认GUF适配器实现
pub struct DefaultGufAdapter {
    min_version: GufVersion,
    max_version: GufVersion,
}

impl DefaultGufAdapter {
    /// 创建新的默认适配器
    pub fn new() -> Self {
        Self {
            min_version: GufVersion {
                major: 4,
                minor: 0,
                patch: 0,
            },
            max_version: GufVersion {
                major: 4,
                minor: 9,
                patch: 9,
            },
        }
    }
}

#[async_trait]
impl GufAdapter for DefaultGufAdapter {
    fn min_version(&self) -> GufVersion {
        self.min_version.clone()
    }

    fn max_version(&self) -> GufVersion {
        self.max_version.clone()
    }

    fn supports_version(&self, version: &GufVersion) -> bool {
        let min_cmp = version.compare(&self.min_version);
        let max_cmp = version.compare(&self.max_version);
        min_cmp != std::cmp::Ordering::Less && max_cmp != std::cmp::Ordering::Greater
    }

    async fn initialize(&self) -> Result<(), crate::error::Error> {
        // 初始化默认适配器
        log::info!("Default GUF adapter initialized");
        Ok(())
    }

    async fn create_component(
        &self,
        component_type: &str,
        props: serde_json::Value,
    ) -> Result<serde_json::Value, crate::error::Error> {
        // 创建默认组件
        let component_id = format!("{}_{}", component_type, uuid::Uuid::new_v4());

        Ok(serde_json::json!({
            "id": component_id,
            "type": component_type,
            "props": props,
            "status": "created"
        }))
    }

    async fn update_component(
        &self,
        component_id: &str,
        props: serde_json::Value,
    ) -> Result<(), crate::error::Error> {
        // 更新默认组件
        log::debug!(
            "Updating component {} with props: {:?}",
            component_id,
            props
        );
        Ok(())
    }

    async fn destroy_component(&self, component_id: &str) -> Result<(), crate::error::Error> {
        // 销毁默认组件
        log::debug!("Destroying component {}", component_id);
        Ok(())
    }

    async fn trigger_event(
        &self,
        component_id: &str,
        event_name: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, crate::error::Error> {
        // 触发默认事件
        log::debug!(
            "Triggering event {} on component {} with data: {:?}",
            event_name,
            component_id,
            data
        );

        Ok(serde_json::json!({
            "component_id": component_id,
            "event_name": event_name,
            "data": data,
            "status": "triggered"
        }))
    }
}
