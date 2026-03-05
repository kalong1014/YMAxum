// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::scene::SceneAdapter;
use crate::scene::game::{
    connection::ConnectionManager, inventory::InventoryManager, player::PlayerManager,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct GameScene {
    player_manager: Arc<RwLock<PlayerManager>>,
    connection_manager: Arc<RwLock<ConnectionManager>>,
    _inventory_manager: Arc<RwLock<InventoryManager>>,
    tcp_port: u16,
    udp_port: u16,
    _max_players: u32,
}

impl GameScene {
    pub async fn new(
        max_players: u32,
        tcp_port: u16,
        udp_port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let player_manager = Arc::new(RwLock::new(PlayerManager::new(max_players)));
        let connection_manager = Arc::new(RwLock::new(ConnectionManager::new(max_players).await?));
        let inventory_manager = Arc::new(RwLock::new(InventoryManager::new(max_players)));

        Ok(Self {
            player_manager,
            connection_manager,
            _inventory_manager: inventory_manager,
            tcp_port,
            udp_port,
            _max_players: max_players,
        })
    }

    pub async fn start_servers(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn_manager = self.connection_manager.write().await;

        // 设置消息处理器
        conn_manager.set_message_handler(Box::new(|conn_id: String, data: Vec<u8>| {
            println!("Received message from {}: {:?}", conn_id, data);
            // 这里可以添加游戏逻辑处理
        }));

        // 启动TCP服务器
        let tcp_addr = format!("0.0.0.0:{}", self.tcp_port);
        conn_manager.start_tcp_server(&tcp_addr).await?;
        println!("TCP game server started on {}", tcp_addr);

        // 启动UDP服务器
        let udp_addr = format!("0.0.0.0:{}", self.udp_port);
        conn_manager.start_udp_server(&udp_addr).await?;
        println!("UDP game server started on {}", udp_addr);

        Ok(())
    }

    pub async fn get_player_count(&self) -> usize {
        let player_manager = self.player_manager.read().await;
        player_manager.get_player_count().await
    }

    pub async fn broadcast_message(&self, message: Vec<u8>) {
        let conn_manager = self.connection_manager.read().await;
        conn_manager.broadcast_message(message).await;
    }
}

impl SceneAdapter for GameScene {
    fn name(&self) -> &'static str {
        "GameScene"
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // This is a synchronous wrapper, actual implementation is async
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Game scene stopped");
        Ok(())
    }
}

