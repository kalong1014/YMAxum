/// GUF v4.4版本适配器
///
/// 针对Godot UI Framework v4.4版本的特定适配
use super::{GufAdapter, GufVersion};
use async_trait::async_trait;
use uuid;

/// GUF v4.4适配器实现
pub struct GufAdapterV4_4 {
    min_version: GufVersion,
    max_version: GufVersion,
}

impl GufAdapterV4_4 {
    /// 创建新的v4.4适配器
    pub fn new() -> Self {
        Self {
            min_version: GufVersion {
                major: 4,
                minor: 4,
                patch: 0,
            },
            max_version: GufVersion {
                major: 4,
                minor: 4,
                patch: 9,
            },
        }
    }
}

#[async_trait]
impl GufAdapter for GufAdapterV4_4 {
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
        // 初始化v4.4适配器
        log::info!("GUF v4.4 adapter initialized");
        Ok(())
    }

    async fn create_component(
        &self,
        component_type: &str,
        props: serde_json::Value,
    ) -> Result<serde_json::Value, crate::error::Error> {
        // v4.4版本特定的组件创建逻辑
        let component_id = format!("v4_4_{}_{}", component_type, uuid::Uuid::new_v4());

        // 添加v4.4特定的属性处理
        let mut processed_props = props;
        if let Some(props_obj) = processed_props.as_object_mut() {
            // 添加v4.4特定的属性
            props_obj.insert(
                "guf_version".to_string(),
                serde_json::Value::String("4.4".to_string()),
            );
        }

        Ok(serde_json::json!({
            "id": component_id,
            "type": component_type,
            "props": processed_props,
            "status": "created",
            "guf_version": "4.4"
        }))
    }

    async fn update_component(
        &self,
        component_id: &str,
        props: serde_json::Value,
    ) -> Result<(), crate::error::Error> {
        // v4.4版本特定的组件更新逻辑
        log::debug!(
            "Updating v4.4 component {} with props: {:?}",
            component_id,
            props
        );
        Ok(())
    }

    async fn destroy_component(&self, component_id: &str) -> Result<(), crate::error::Error> {
        // v4.4版本特定的组件销毁逻辑
        log::debug!("Destroying v4.4 component {}", component_id);
        Ok(())
    }

    async fn trigger_event(
        &self,
        component_id: &str,
        event_name: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, crate::error::Error> {
        // v4.4版本特定的事件触发逻辑
        log::debug!(
            "Triggering v4.4 event {} on component {} with data: {:?}",
            event_name,
            component_id,
            data
        );

        Ok(serde_json::json!({
            "component_id": component_id,
            "event_name": event_name,
            "data": data,
            "status": "triggered",
            "guf_version": "4.4"
        }))
    }
}
