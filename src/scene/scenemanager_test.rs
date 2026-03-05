// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_scene_manager_creation() {
    let scene_manager = SceneManager::new();
    assert!(scene_manager.adapters.is_empty());
}

#[tokio::test]
async fn test_scene_manager_register() {
    let mut scene_manager = SceneManager::new();
    let newbie_scene = NewbieScene::new();
    scene_manager.register(Box::new(newbie_scene));
    assert_eq!(scene_manager.adapters.len(), 1);
}

#[tokio::test]
async fn test_scene_manager_init_all() {
    let mut scene_manager = SceneManager::new();
    let newbie_scene = NewbieScene::new();
    scene_manager.register(Box::new(newbie_scene));
    let result = scene_manager.init_all();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scene_manager_start_all() {
    let mut scene_manager = SceneManager::new();
    let newbie_scene = NewbieScene::new();
    scene_manager.register(Box::new(newbie_scene));
    let result = scene_manager.start_all();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scene_manager_stop_all() {
    let mut scene_manager = SceneManager::new();
    let newbie_scene = NewbieScene::new();
    scene_manager.register(Box::new(newbie_scene));
    let result = scene_manager.stop_all();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scene_manager_get_adapter() {
    let mut scene_manager = SceneManager::new();
    let newbie_scene = NewbieScene::new();
    scene_manager.register(Box::new(newbie_scene));
    let adapter = scene_manager.get_adapter("NewbieScene");
    assert!(adapter.is_some());
}

#[tokio::test]
async fn test_game_scene_creation() {
    let game_scene = GameScene::new(1000, 8081, 8082).await;
    assert!(game_scene.is_ok());
}

#[tokio::test]
async fn test_game_scene_start_servers() {
    let game_scene = GameScene::new(1000, 8081, 8082).await.unwrap();
    let result = game_scene.start_servers().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_mall_scene_adapter() {
    let mall_scene = MallSceneAdapter::new();
    assert!(mall_scene.name() == "MallScene");
    let init_result = mall_scene.init();
    assert!(init_result.is_ok());
    let start_result = mall_scene.start();
    assert!(start_result.is_ok());
    let stop_result = mall_scene.stop();
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_saas_scene() {
    let saas_scene = SaasScene::new(10);
    assert!(saas_scene.name() == "SaasScene");
    let init_result = saas_scene.init();
    assert!(init_result.is_ok());
    let start_result = saas_scene.start();
    assert!(start_result.is_ok());
    let stop_result = saas_scene.stop();
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_social_scene() {
    let social_scene = SocialScene::new();
    assert!(social_scene.name() == "SocialScene");
    let init_result = social_scene.init();
    assert!(init_result.is_ok());
    let start_result = social_scene.start();
    assert!(start_result.is_ok());
    let stop_result = social_scene.stop();
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_newbie_scene() {
    let mut newbie_scene = NewbieScene::new();
    assert!(newbie_scene.name() == "NewbieScene");
    let init_result = newbie_scene.init();
    assert!(init_result.is_ok());
    let start_result = newbie_scene.start();
    assert!(start_result.is_ok());
    let stop_result = newbie_scene.stop();
    assert!(stop_result.is_ok());
}
