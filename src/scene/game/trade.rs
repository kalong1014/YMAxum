// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::scene::game::inventory::{InventoryManager, Item};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Trade {
    pub id: String,
    pub player1_id: String,
    pub player2_id: String,
    pub player1_items: Vec<Item>,
    pub player2_items: Vec<Item>,
    pub status: TradeStatus,
    pub created_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TradeStatus {
    Pending,
    Accepted,
    Rejected,
    Completed,
    Cancelled,
}

impl Trade {
    pub fn new(
        player1_id: &str,
        player2_id: &str,
        player1_items: Vec<Item>,
        player2_items: Vec<Item>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            player1_id: player1_id.to_string(),
            player2_id: player2_id.to_string(),
            player1_items,
            player2_items,
            status: TradeStatus::Pending,
            created_at: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn accept(&mut self) {
        self.status = TradeStatus::Accepted;
    }

    pub fn reject(&mut self) {
        self.status = TradeStatus::Rejected;
    }

    pub fn complete(&mut self) {
        self.status = TradeStatus::Completed;
    }

    pub fn cancel(&mut self) {
        self.status = TradeStatus::Cancelled;
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, TradeStatus::Pending | TradeStatus::Accepted)
    }

    pub fn is_completed(&self) -> bool {
        self.status == TradeStatus::Completed
    }
}

pub struct TradeManager {
    trades: Arc<RwLock<HashMap<String, Trade>>>,
    inventory_manager: Arc<RwLock<InventoryManager>>,
}

impl TradeManager {
    pub fn new(inventory_manager: Arc<RwLock<InventoryManager>>) -> Self {
        Self {
            trades: Arc::new(RwLock::new(HashMap::new())),
            inventory_manager,
        }
    }

    pub async fn create_trade(
        &self,
        player1_id: &str,
        player2_id: &str,
        player1_items: Vec<Item>,
        player2_items: Vec<Item>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let trade = Trade::new(player1_id, player2_id, player1_items, player2_items);
        let trade_id = trade.id.clone();

        let mut trades = self.trades.write().await;
        trades.insert(trade_id.clone(), trade);

        Ok(trade_id)
    }

    pub async fn accept_trade(&self, trade_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut trades = self.trades.write().await;

        if let Some(trade) = trades.get_mut(trade_id) {
            if !trade.is_active() {
                return Err("Trade is not active".into());
            }

            trade.accept();
            Ok(())
        } else {
            Err("Trade not found".into())
        }
    }

    pub async fn reject_trade(&self, trade_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut trades = self.trades.write().await;

        if let Some(trade) = trades.get_mut(trade_id) {
            if !trade.is_active() {
                return Err("Trade is not active".into());
            }

            trade.reject();
            Ok(())
        } else {
            Err("Trade not found".into())
        }
    }

    pub async fn complete_trade(&self, trade_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut trades = self.trades.write().await;

        if let Some(trade) = trades.get_mut(trade_id) {
            if trade.status != TradeStatus::Accepted {
                return Err("Trade must be accepted first".into());
            }

            let inventory_manager = self.inventory_manager.read().await;

            for item in &trade.player1_items {
                let result = inventory_manager
                    .remove_item_from_player(&trade.player1_id, &item.id, item.quantity)
                    .await;
                if result.is_err() {
                    return Err("Failed to remove items from player1".into());
                }
            }

            for item in &trade.player2_items {
                let result = inventory_manager
                    .remove_item_from_player(&trade.player2_id, &item.id, item.quantity)
                    .await;
                if result.is_err() {
                    return Err("Failed to remove items from player2".into());
                }
            }

            for item in &trade.player1_items {
                let result = inventory_manager
                    .add_item_to_player(&trade.player2_id, item.clone())
                    .await;
                if result.is_err() {
                    return Err("Failed to add items to player2".into());
                }
            }

            for item in &trade.player2_items {
                let result = inventory_manager
                    .add_item_to_player(&trade.player1_id, item.clone())
                    .await;
                if result.is_err() {
                    return Err("Failed to add items to player1".into());
                }
            }

            trade.complete();
            Ok(())
        } else {
            Err("Trade not found".into())
        }
    }

    pub async fn cancel_trade(&self, trade_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut trades = self.trades.write().await;

        if let Some(trade) = trades.get_mut(trade_id) {
            if !trade.is_active() {
                return Err("Trade is not active".into());
            }

            trade.cancel();
            Ok(())
        } else {
            Err("Trade not found".into())
        }
    }

    pub async fn get_trade(&self, trade_id: &str) -> Option<Trade> {
        let trades = self.trades.read().await;
        trades.get(trade_id).cloned()
    }

    pub async fn get_player_trades(&self, player_id: &str) -> Vec<Trade> {
        let trades = self.trades.read().await;
        trades
            .values()
            .filter(|trade| trade.player1_id == player_id || trade.player2_id == player_id)
            .cloned()
            .collect()
    }

    pub async fn cleanup_old_trades(&self, timeout: u64) -> Vec<String> {
        let now = chrono::Utc::now().timestamp() as u64;
        let mut trades = self.trades.write().await;

        let old_trade_ids: Vec<String> = trades
            .values()
            .filter(|trade| !trade.is_completed() && (now - trade.created_at) > timeout)
            .map(|trade| trade.id.clone())
            .collect();

        for id in &old_trade_ids {
            trades.remove(id);
        }

        old_trade_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::game::inventory::{InventoryManager, Item, ItemType};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_trade_creation() {
        let inventory_manager = Arc::new(RwLock::new(InventoryManager::new(20)));
        let trade_manager = TradeManager::new(inventory_manager);

        let player1_items = vec![Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        }];

        let player2_items = vec![Item {
            id: "item2".to_string(),
            name: "Shield".to_string(),
            item_type: ItemType::Armor,
            quantity: 1,
            max_quantity: 99,
            value: 80,
            description: "A sturdy shield".to_string(),
        }];

        let result = trade_manager
            .create_trade("player1", "player2", player1_items, player2_items)
            .await;
        assert!(result.is_ok());
        let trade_id = result.unwrap();
        assert!(!trade_id.is_empty());
    }

    #[tokio::test]
    async fn test_trade_accept() {
        let inventory_manager = Arc::new(RwLock::new(InventoryManager::new(20)));
        let trade_manager = TradeManager::new(inventory_manager);

        let player1_items = vec![Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        }];

        let player2_items = vec![Item {
            id: "item2".to_string(),
            name: "Shield".to_string(),
            item_type: ItemType::Armor,
            quantity: 1,
            max_quantity: 99,
            value: 80,
            description: "A sturdy shield".to_string(),
        }];

        let trade_id = trade_manager
            .create_trade("player1", "player2", player1_items, player2_items)
            .await
            .unwrap();
        let result = trade_manager.accept_trade(&trade_id).await;
        assert!(result.is_ok());

        let trade = trade_manager.get_trade(&trade_id).await.unwrap();
        assert_eq!(trade.status, TradeStatus::Accepted);
    }

    #[tokio::test]
    async fn test_trade_reject() {
        let inventory_manager = Arc::new(RwLock::new(InventoryManager::new(20)));
        let trade_manager = TradeManager::new(inventory_manager);

        let player1_items = vec![Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        }];

        let player2_items = vec![Item {
            id: "item2".to_string(),
            name: "Shield".to_string(),
            item_type: ItemType::Armor,
            quantity: 1,
            max_quantity: 99,
            value: 80,
            description: "A sturdy shield".to_string(),
        }];

        let trade_id = trade_manager
            .create_trade("player1", "player2", player1_items, player2_items)
            .await
            .unwrap();
        let result = trade_manager.reject_trade(&trade_id).await;
        assert!(result.is_ok());

        let trade = trade_manager.get_trade(&trade_id).await.unwrap();
        assert_eq!(trade.status, TradeStatus::Rejected);
    }

    #[tokio::test]
    async fn test_trade_complete() {
        let inventory_manager = Arc::new(RwLock::new(InventoryManager::new(20)));
        let trade_manager = TradeManager::new(inventory_manager.clone());

        let player1_items = vec![Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        }];

        let player2_items = vec![Item {
            id: "item2".to_string(),
            name: "Shield".to_string(),
            item_type: ItemType::Armor,
            quantity: 1,
            max_quantity: 99,
            value: 80,
            description: "A sturdy shield".to_string(),
        }];

        {
            let inventory_manager_guard = inventory_manager.write().await;
            let _ = inventory_manager_guard.create_inventory("player1").await;
            let _ = inventory_manager_guard.create_inventory("player2").await;
        }

        for item in &player1_items {
            let inventory_manager_guard = inventory_manager.write().await;
            let _ = inventory_manager_guard
                .add_item_to_player("player1", item.clone())
                .await;
        }

        for item in &player2_items {
            let inventory_manager_guard = inventory_manager.write().await;
            let _ = inventory_manager_guard
                .add_item_to_player("player2", item.clone())
                .await;
        }

        let trade_id = trade_manager
            .create_trade("player1", "player2", player1_items, player2_items)
            .await
            .unwrap();
        trade_manager.accept_trade(&trade_id).await.unwrap();
        let result = trade_manager.complete_trade(&trade_id).await;
        assert!(result.is_ok());

        let trade = trade_manager.get_trade(&trade_id).await.unwrap();
        assert_eq!(trade.status, TradeStatus::Completed);
    }

    #[tokio::test]
    async fn test_trade_cancel() {
        let inventory_manager = Arc::new(RwLock::new(InventoryManager::new(20)));
        let trade_manager = TradeManager::new(inventory_manager);

        let player1_items = vec![Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        }];

        let player2_items = vec![Item {
            id: "item2".to_string(),
            name: "Shield".to_string(),
            item_type: ItemType::Armor,
            quantity: 1,
            max_quantity: 99,
            value: 80,
            description: "A sturdy shield".to_string(),
        }];

        let trade_id = trade_manager
            .create_trade("player1", "player2", player1_items, player2_items)
            .await
            .unwrap();
        let result = trade_manager.cancel_trade(&trade_id).await;
        assert!(result.is_ok());

        let trade = trade_manager.get_trade(&trade_id).await.unwrap();
        assert_eq!(trade.status, TradeStatus::Cancelled);
    }
}

