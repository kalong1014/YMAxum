// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub level: u32,
    pub experience: u64,
    pub coins: u64,
    pub status: PlayerStatus,
    pub last_login: u64,
    pub data: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PlayerStatus {
    Online,
    Offline,
    AFK,
    Playing,
}

impl Player {
    pub fn new(id: &str, username: &str) -> Self {
        Self {
            id: id.to_string(),
            username: username.to_string(),
            level: 1,
            experience: 0,
            coins: 1000,
            status: PlayerStatus::Online,
            last_login: chrono::Utc::now().timestamp() as u64,
            data: HashMap::new(),
        }
    }

    pub fn update_status(&mut self, status: PlayerStatus) {
        self.status = status;
    }

    pub fn add_experience(&mut self, exp: u64) -> bool {
        self.experience += exp;
        self.check_level_up()
    }

    pub fn add_coins(&mut self, coins: u64) {
        self.coins += coins;
    }

    pub fn remove_coins(&mut self, coins: u64) -> bool {
        if self.coins >= coins {
            self.coins -= coins;
            true
        } else {
            false
        }
    }

    fn check_level_up(&mut self) -> bool {
        let required_exp = (self.level as u64) * 1000;
        if self.experience >= required_exp {
            self.level += 1;
            self.experience -= required_exp;
            true
        } else {
            false
        }
    }
}

pub struct PlayerManager {
    players: Arc<RwLock<HashMap<String, Player>>>,
    max_players: u32,
}

impl PlayerManager {
    pub fn new(max_players: u32) -> Self {
        Self {
            players: Arc::new(RwLock::new(HashMap::new())),
            max_players,
        }
    }

    pub async fn add_player(&self, player: Player) -> Result<(), Box<dyn std::error::Error>> {
        let mut players = self.players.write().await;

        if players.len() >= self.max_players as usize {
            return Err("Player limit reached".into());
        }

        if players.contains_key(&player.id) {
            return Err("Player already exists".into());
        }

        players.insert(player.id.clone(), player);
        Ok(())
    }

    pub async fn remove_player(&self, player_id: &str) -> bool {
        let mut players = self.players.write().await;
        players.remove(player_id).is_some()
    }

    pub async fn get_player(&self, player_id: &str) -> Option<Player> {
        let players = self.players.read().await;
        players.get(player_id).cloned()
    }

    pub async fn update_player(&self, player: Player) -> bool {
        let mut players = self.players.write().await;
        if players.contains_key(&player.id) {
            players.insert(player.id.clone(), player);
            true
        } else {
            false
        }
    }

    pub async fn get_online_players(&self) -> Vec<Player> {
        let players = self.players.read().await;
        players
            .values()
            .filter(|p| p.status == PlayerStatus::Online || p.status == PlayerStatus::Playing)
            .cloned()
            .collect()
    }

    pub async fn get_player_count(&self) -> usize {
        let players = self.players.read().await;
        players.len()
    }

    pub async fn save_player_data(
        &self,
        _player_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn load_player_data(&self, _player_id: &str) -> Option<Player> {
        None
    }
}

