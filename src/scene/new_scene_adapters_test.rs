// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::scene::*;

#[test]
fn test_government_scene_adapter() {
    // 创建政府场景适配器
    let mut adapter = GovernmentSceneAdapter::new();
    
    // 验证适配器名称
    assert_eq!(adapter.name(), "government");
    
    // 初始化适配器
    assert!(adapter.init().is_ok());
    
    // 启动适配器
    assert!(adapter.start().is_ok());
    
    // 停止适配器
    assert!(adapter.stop().is_ok());
}

#[test]
fn test_retail_scene_adapter() {
    // 创建零售场景适配器
    let mut adapter = RetailSceneAdapter::new();
    
    // 验证适配器名称
    assert_eq!(adapter.name(), "retail");
    
    // 初始化适配器
    assert!(adapter.init().is_ok());
    
    // 启动适配器
    assert!(adapter.start().is_ok());
    
    // 停止适配器
    assert!(adapter.stop().is_ok());
}

#[test]
fn test_transportation_scene_adapter() {
    // 创建交通场景适配器
    let mut adapter = TransportationSceneAdapter::new();
    
    // 验证适配器名称
    assert_eq!(adapter.name(), "transportation");
    
    // 初始化适配器
    assert!(adapter.init().is_ok());
    
    // 启动适配器
    assert!(adapter.start().is_ok());
    
    // 停止适配器
    assert!(adapter.stop().is_ok());
}

#[test]
fn test_scene_manager_with_new_adapters() {
    // 创建场景管理器
    let mut manager = SceneManager::new();
    
    // 注册政府场景适配器
    manager.register(Box::new(GovernmentSceneAdapter::new()));
    
    // 注册零售场景适配器
    manager.register(Box::new(RetailSceneAdapter::new()));
    
    // 注册交通场景适配器
    manager.register(Box::new(TransportationSceneAdapter::new()));
    
    // 初始化所有适配器
    assert!(manager.init_all().is_ok());
    
    // 启动所有适配器
    assert!(manager.start_all().is_ok());
    
    // 停止所有适配器
    assert!(manager.stop_all().is_ok());
    
    // 验证适配器数量
    assert_eq!(manager.get_all_adapters().len(), 3);
    
    // 验证资源使用情况
    let resource_usage = manager.get_resource_usage();
    assert!(resource_usage.contains_key("government"));
    assert!(resource_usage.contains_key("retail"));
    assert!(resource_usage.contains_key("transportation"));
}
