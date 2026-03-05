//! GUF 集成测试
//! 测试 GUF 集成的各个方面，包括组件管理、事件处理、配置同步等

use ymaxum::guf::adapter::GufAdapter;
use ymaxum::guf::component_manager::{ComponentPoolConfig, GufComponentManager};
use ymaxum::guf::event_bus::{EventSubscriber, GufEvent, GufEventBus};

#[tokio::test]
async fn test_guf_adapter_initialization() {
    // 启动模拟 GUF 服务器
    let (port, _server_state) = ymaxum::guf::mock_guf_server::start_mock_guf_server().await;
    let server_url = ymaxum::guf::mock_guf_server::get_mock_guf_server_url(port);

    // 测试 GUF 适配器初始化
    let mut adapter = GufAdapter::new();

    // 使用模拟服务器地址创建配置
    let config = ymaxum::guf::adapter::GufRuntimeConfig {
        server_address: server_url,
        auth_info: ymaxum::guf::adapter::GufAuthInfo {
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            auth_token: None,
        },
        connection_timeout: 5,
        heartbeat_interval: 30,
        pool_size: 2,
    };

    let init_result = adapter.initialize(config).await;
    assert!(init_result.is_ok());

    assert!(adapter.is_initialized());
    assert!(adapter.is_connected());
}

#[tokio::test]
async fn test_guf_component_manager() {
    // 测试 GUF 组件管理器
    let component_manager = GufComponentManager::new();

    // 测试组件池创建
    let pool_config = ComponentPoolConfig {
        max_size: 10,
        min_size: 2,
        component_type: "test".to_string(),
        idle_timeout: 300,
        warmup_size: 2,
        cleanup_threshold: 5,
        health_check_interval: 60,
    };

    // 创建简单的组件工厂
    struct TestComponentFactory;
    impl ymaxum::guf::component_manager::GufComponentFactory for TestComponentFactory {
        fn create_component(
            &self,
            component_id: &str,
            _config: serde_json::Value,
        ) -> Result<std::sync::Arc<dyn ymaxum::guf::component_manager::GufComponent>, String>
        {
            Ok(std::sync::Arc::new(
                ymaxum::guf::component_manager::ExampleComponent::new(
                    component_id.to_string(),
                    "Test Component".to_string(),
                    "1.0.0".to_string(),
                    vec![],
                ),
            ))
        }
    }

    let factory = std::sync::Arc::new(TestComponentFactory);
    let create_pool_result = component_manager
        .create_component_pool(
            "test_pool",
            pool_config,
            factory,
            serde_json::json!({ "key": "value" }),
        )
        .await;
    assert!(create_pool_result.is_ok());

    // 测试组件池状态
    let pool_status = component_manager
        .get_component_pool_status("test_pool")
        .await;
    assert!(pool_status.is_ok());
}

#[tokio::test]
async fn test_guf_event_bus() {
    // 测试 GUF 事件总线
    let mut event_bus = GufEventBus::new();

    // 启动事件总线
    event_bus.start().await;

    // 创建事件处理器
    struct TestEventHandler {
        id: String,
    }
    #[async_trait::async_trait]
    impl ymaxum::guf::event_bus::EventHandler for TestEventHandler {
        async fn handle(&self, event: &GufEvent) -> Result<(), String> {
            println!("Received event: {:?}", event);
            Ok(())
        }
        fn id(&self) -> &str {
            &self.id
        }
    }

    let handler = std::sync::Arc::new(TestEventHandler {
        id: "test_handler".to_string(),
    });
    let subscriber = EventSubscriber::new("test_subscriber".to_string(), handler, 10);

    // 订阅事件
    let subscribe_result = event_bus
        .subscribe("test_event".to_string(), subscriber)
        .await;
    assert!(subscribe_result.is_ok());

    // 发布事件
    let event = GufEvent::new(
        "test_event".to_string(),
        serde_json::json!({ "key": "value" }),
        "test_source".to_string(),
    );

    let publish_result = event_bus.publish(event).await;
    assert!(publish_result.is_ok());

    // 等待事件处理
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // 停止事件总线
    event_bus.stop().await;
}

#[tokio::test]
async fn test_guf_performance_basic() {
    // 测试 GUF 基本性能
    let mut event_bus = GufEventBus::new();
    event_bus.start().await;

    // 测试事件发布性能
    let start_time = std::time::Instant::now();

    for i in 0..100 {
        let event = GufEvent::new(
            "perf_test".to_string(),
            serde_json::json!({ "index": i }),
            "test".to_string(),
        );

        let result = event_bus.publish(event).await;
        assert!(result.is_ok());
    }

    let elapsed = start_time.elapsed();
    println!("Published 100 events in {:?}", elapsed);
    // 确保性能合理（100个事件在1秒内完成）
    assert!(elapsed < std::time::Duration::from_secs(1));

    event_bus.stop().await;
}
