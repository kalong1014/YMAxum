// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub quantity: u32,
    pub max_quantity: u32,
    pub value: u64,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Material,
    Quest,
}

#[derive(Clone, Debug)]
pub struct Inventory {
    pub player_id: String,
    pub items: HashMap<String, Item>,
    pub max_slots: u32,
    pub used_slots: u32,
}

impl Inventory {
    pub fn new(player_id: &str, max_slots: u32) -> Self {
        Self {
            player_id: player_id.to_string(),
            items: HashMap::new(),
            max_slots,
            used_slots: 0,
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        if self.used_slots >= self.max_slots {
            return Err("Inventory is full".into());
        }

        if let Some(existing_item) = self.items.get_mut(&item.id) {
            if existing_item.quantity + item.quantity > existing_item.max_quantity {
                return Err("Item quantity exceeds maximum".into());
            }
            existing_item.quantity += item.quantity;
        } else {
            self.used_slots += 1;
            self.items.insert(item.id.clone(), item);
        }

        Ok(())
    }

    pub fn remove_item(
        &mut self,
        item_id: &str,
        quantity: u32,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        if let Some(item) = self.items.get_mut(item_id) {
            if item.quantity < quantity {
                return Err("Insufficient item quantity".into());
            }

            item.quantity -= quantity;

            if item.quantity == 0 {
                self.used_slots -= 1;
                let removed_item = self.items.remove(item_id).unwrap();
                Ok(removed_item)
            } else {
                Ok(item.clone())
            }
        } else {
            Err("Item not found".into())
        }
    }

    pub fn get_item(&self, item_id: &str) -> Option<&Item> {
        self.items.get(item_id)
    }

    pub fn get_all_items(&self) -> Vec<Item> {
        self.items.values().cloned().collect()
    }

    pub fn get_item_count(&self) -> usize {
        self.items.len()
    }

    pub fn get_available_slots(&self) -> u32 {
        self.max_slots - self.used_slots
    }

    pub fn is_full(&self) -> bool {
        self.used_slots >= self.max_slots
    }
}

pub struct InventoryManager {
    inventories: Arc<RwLock<HashMap<String, Inventory>>>,
    max_slots_per_player: u32,
}

impl InventoryManager {
    pub fn new(max_slots_per_player: u32) -> Self {
        Self {
            inventories: Arc::new(RwLock::new(HashMap::new())),
            max_slots_per_player,
        }
    }

    pub async fn create_inventory(
        &self,
        player_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut inventories = self.inventories.write().await;

        if inventories.contains_key(player_id) {
            return Err("Inventory already exists for player".into());
        }

        let inventory = Inventory::new(player_id, self.max_slots_per_player);
        inventories.insert(player_id.to_string(), inventory);

        Ok(())
    }

    pub async fn remove_inventory(&self, player_id: &str) -> bool {
        let mut inventories = self.inventories.write().await;
        inventories.remove(player_id).is_some()
    }

    pub async fn get_inventory(&self, player_id: &str) -> Option<Inventory> {
        let inventories = self.inventories.read().await;
        inventories.get(player_id).cloned()
    }

    pub async fn add_item_to_player(
        &self,
        player_id: &str,
        item: Item,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut inventories = self.inventories.write().await;
        if let Some(inventory) = inventories.get_mut(player_id) {
            inventory.add_item(item)
        } else {
            Err("Inventory not found for player".into())
        }
    }

    pub async fn remove_item_from_player(
        &self,
        player_id: &str,
        item_id: &str,
        quantity: u32,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        let mut inventories = self.inventories.write().await;
        if let Some(inventory) = inventories.get_mut(player_id) {
            inventory.remove_item(item_id, quantity)
        } else {
            Err("Inventory not found for player".into())
        }
    }

    pub async fn get_player_items(&self, player_id: &str) -> Vec<Item> {
        let inventories = self.inventories.read().await;
        if let Some(inventory) = inventories.get(player_id) {
            inventory.get_all_items()
        } else {
            Vec::new()
        }
    }

    pub async fn get_player_item_count(&self, player_id: &str) -> usize {
        let inventories = self.inventories.read().await;
        if let Some(inventory) = inventories.get(player_id) {
            inventory.get_item_count()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_creation() {
        let inventory = Inventory::new("player1", 20);
        assert_eq!(inventory.player_id, "player1");
        assert_eq!(inventory.max_slots, 20);
        assert_eq!(inventory.used_slots, 0);
        assert!(!inventory.is_full());
    }

    #[test]
    fn test_add_item() {
        let mut inventory = Inventory::new("player1", 20);
        let item = Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        };

        let result = inventory.add_item(item);
        assert!(result.is_ok());
        assert_eq!(inventory.used_slots, 1);
    }

    #[test]
    fn test_remove_item() {
        let mut inventory = Inventory::new("player1", 20);
        let item = Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 10,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        };

        inventory.add_item(item.clone()).unwrap();
        let result = inventory.remove_item("item1", 5);
        assert!(result.is_ok());
        assert_eq!(inventory.get_item("item1").unwrap().quantity, 5);
    }

    #[test]
    fn test_inventory_full() {
        let mut inventory = Inventory::new("player1", 1);
        let item = Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        };

        inventory.add_item(item.clone()).unwrap();
        let result = inventory.add_item(item);
        assert!(result.is_err());
        assert!(inventory.is_full());
    }

    #[tokio::test]
    async fn test_inventory_manager() {
        let manager = InventoryManager::new(20);
        let item = Item {
            id: "item1".to_string(),
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            quantity: 1,
            max_quantity: 99,
            value: 100,
            description: "A sharp sword".to_string(),
        };

        let result = manager.create_inventory("player1").await;
        assert!(result.is_ok());

        let result = manager.add_item_to_player("player1", item).await;
        assert!(result.is_ok());

        let items = manager.get_player_items("player1").await;
        assert_eq!(items.len(), 1);
    }
}

