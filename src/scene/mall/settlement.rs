// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use chrono::Timelike;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Settlement cycle
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SettlementCycle {
    /// T+0 settlement (same day settlement)
    T0,
    /// T+1 settlement (next day settlement)
    T1,
    /// Custom settlement cycle
    Custom(u32),
}

/// Settlement status
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SettlementStatus {
    /// Pending
    Pending,
    /// Settlement processing
    Processing,
    /// Settlement success
    Success,
    /// Settlement failed
    Failed,
    /// Cancelled
    Cancelled,
}

/// Settlement order
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementOrder {
    /// Settlement order ID
    pub id: String,
    /// Merchant ID
    pub merchant_id: String,
    /// Settlement cycle
    pub cycle: SettlementCycle,
    /// Settlement amount
    pub amount: f64,
    /// Settlement status
    pub status: SettlementStatus,
    /// Settlement start date
    pub start_date: u64,
    /// Settlement end date
    pub end_date: u64,
    /// Settlement application time
    pub apply_time: u64,
    /// Settlement completion time
    pub complete_time: Option<u64>,
    /// Settlement failure reason
    pub fail_reason: Option<String>,
    /// Bank name
    pub bank_name: String,
    /// Bank account
    pub bank_account: String,
    /// Account holder name
    pub account_name: String,
    /// Settlement transaction number
    pub settlement_no: Option<String>,
}

/// Settlement record
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementRecord {
    /// Record ID
    pub id: String,
    /// Settlement order ID
    pub settlement_id: String,
    /// Order ID
    pub order_id: String,
    /// Merchant ID
    pub merchant_id: String,
    /// Order amount
    pub order_amount: f64,
    /// Profit amount
    pub profit_amount: f64,
    /// Handling fee amount
    pub fee_amount: f64,
    /// Actual settlement amount
    pub actual_amount: f64,
    /// Transaction time
    pub transaction_time: u64,
    /// Settlement time
    pub settlement_time: Option<u64>,
}

/// Settlement configuration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementConfig {
    /// Merchant ID
    pub merchant_id: String,
    /// Settlement cycle
    pub cycle: SettlementCycle,
    /// Minimum settlement amount
    pub min_settlement_amount: f64,
    /// Settlement fee rate
    pub fee_rate: f64,
    /// Maximum handling fee amount
    pub max_fee_amount: Option<f64>,
    /// Bank name
    pub bank_name: String,
    /// Bank account
    pub bank_account: String,
    /// Account holder name
    pub account_name: String,
    /// Auto settlement
    pub auto_settlement: bool,
    /// Auto settlement time
    pub auto_settlement_time: Option<u32>, // Hour (0-23)
}

/// Settlement manager
pub struct SettlementManager {
    /// Settlement configuration map
    configs: Arc<RwLock<HashMap<String, SettlementConfig>>>,
    /// Settlement order map
    orders: Arc<RwLock<HashMap<String, SettlementOrder>>>,
    /// Settlement record map
    _records: Arc<RwLock<HashMap<String, SettlementRecord>>>,
    /// Pending amount map
    pending_amounts: Arc<RwLock<HashMap<String, f64>>>,
}

impl SettlementManager {
    /// Create new settlement manager
    pub fn new() -> Self {
        Self {
            configs: Arc::new(RwLock::new(HashMap::new())),
            orders: Arc::new(RwLock::new(HashMap::new())),
            _records: Arc::new(RwLock::new(HashMap::new())),
            pending_amounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set settlement configuration
    pub async fn set_config(&self, config: SettlementConfig) -> Result<(), String> {
        let mut configs = self.configs.write().await;
        configs.insert(config.merchant_id.clone(), config.clone());
        info!(
            "Merchant settlement configuration updated: {}",
            config.merchant_id
        );
        Ok(())
    }

    /// Get settlement configuration
    pub async fn get_config(&self, merchant_id: &str) -> Option<SettlementConfig> {
        let configs = self.configs.read().await;
        configs.get(merchant_id).cloned()
    }

    /// Apply for settlement
    pub async fn apply_settlement(
        &self,
        merchant_id: &str,
        amount: f64,
    ) -> Result<SettlementOrder, String> {
        let configs = self.configs.read().await;
        let config = configs
            .get(merchant_id)
            .ok_or("Merchant has not configured settlement information".to_string())?;

        // Check minimum settlement amount
        if amount < config.min_settlement_amount {
            return Err(format!(
                "Settlement amount is less than minimum settlement amount: {:.2}",
                config.min_settlement_amount
            ));
        }

        // Check if pending amount is sufficient
        let pending_amounts = self.pending_amounts.read().await;
        let current_pending = pending_amounts.get(merchant_id).unwrap_or(&0.0);
        if amount > *current_pending {
            return Err("Settlement amount exceeds pending settlement amount".to_string());
        }

        // Create settlement order
        let cycle = config.cycle.clone();
        let settlement_order = SettlementOrder {
            id: format!("settlement_{}", uuid::Uuid::new_v4()),
            merchant_id: merchant_id.to_string(),
            cycle: cycle.clone(),
            amount,
            status: SettlementStatus::Pending,
            start_date: self.get_cycle_start_date(cycle.clone()).await,
            end_date: self.get_cycle_end_date(cycle.clone()).await,
            apply_time: chrono::Utc::now().timestamp() as u64,
            complete_time: None,
            fail_reason: None,
            bank_name: config.bank_name.clone(),
            bank_account: config.bank_account.clone(),
            account_name: config.account_name.clone(),
            settlement_no: None,
        };

        // Save settlement order
        let mut orders = self.orders.write().await;
        orders.insert(settlement_order.id.clone(), settlement_order.clone());

        // Deduct from pending settlement amount
        let mut pending_amounts_write = self.pending_amounts.write().await;
        let new_pending = current_pending - amount;
        pending_amounts_write.insert(merchant_id.to_string(), new_pending);

        info!(
            "Merchant settlement application submitted: {}, amount: {:.2}",
            merchant_id, amount
        );
        Ok(settlement_order)
    }

    /// Process settlement order
    pub async fn process_settlement(&self, settlement_id: &str) -> Result<SettlementOrder, String> {
        let mut orders = self.orders.write().await;
        let order = orders
            .get_mut(settlement_id)
            .ok_or("Settlement order does not exist".to_string())?;

        if order.status != SettlementStatus::Pending {
            return Err("Settlement order status is incorrect".to_string());
        }

        // Update status to processing
        order.status = SettlementStatus::Processing;

        // Simulate settlement processing
        // Actual project will call third-party payment or bank interface here
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Simulate settlement success
        order.status = SettlementStatus::Success;
        order.complete_time = Some(chrono::Utc::now().timestamp() as u64);
        order.settlement_no = Some(format!("settle_{}", uuid::Uuid::new_v4()));

        info!(
            "Settlement order processing completed: {}, status: {:?}",
            settlement_id, order.status
        );
        Ok(order.clone())
    }

    /// Cancel settlement order
    pub async fn cancel_settlement(&self, settlement_id: &str) -> Result<SettlementOrder, String> {
        let mut orders = self.orders.write().await;
        let mut pending_amounts = self.pending_amounts.write().await;

        let order = orders
            .get_mut(settlement_id)
            .ok_or("Settlement order does not exist".to_string())?;

        if order.status != SettlementStatus::Pending && order.status != SettlementStatus::Processing
        {
            return Err("Settlement order status is incorrect, cannot cancel".to_string());
        }

        // Update status to cancelled
        order.status = SettlementStatus::Cancelled;

        // Return amount to pending settlement amount
        let current_pending = *pending_amounts.get(&order.merchant_id).unwrap_or(&0.0);
        pending_amounts.insert(order.merchant_id.clone(), current_pending + order.amount);

        info!("Settlement order cancelled: {}", settlement_id);
        Ok(order.clone())
    }

    /// Get settlement order
    pub async fn get_settlement(&self, settlement_id: &str) -> Option<SettlementOrder> {
        let orders = self.orders.read().await;
        orders.get(settlement_id).cloned()
    }

    /// Get merchant settlement order list
    pub async fn list_merchant_settlements(
        &self,
        merchant_id: &str,
        status: Option<SettlementStatus>,
    ) -> Vec<SettlementOrder> {
        let orders = self.orders.read().await;
        let mut result = Vec::new();

        for order in orders.values() {
            if order.merchant_id == merchant_id {
                if let Some(ref order_status) = status {
                    if order.status == *order_status {
                        result.push(order.clone());
                    }
                } else {
                    result.push(order.clone());
                }
            }
        }

        result
    }

    /// Add pending amount
    pub async fn add_pending_amount(&self, merchant_id: &str, amount: f64) -> Result<(), String> {
        let mut pending_amounts = self.pending_amounts.write().await;
        // Update merchant pending settlement amount
        let current = *pending_amounts.get(merchant_id).unwrap_or(&0.0);
        pending_amounts.insert(merchant_id.to_string(), current + amount);
        info!(
            "Merchant pending settlement amount updated: {}, new addition: {:.2}",
            merchant_id, amount
        );
        Ok(())
    }

    /// Get pending amount
    pub async fn get_pending_amount(&self, merchant_id: &str) -> f64 {
        let pending_amounts = self.pending_amounts.read().await;
        *pending_amounts.get(merchant_id).unwrap_or(&0.0)
    }

    /// Get cycle start date
    async fn get_cycle_start_date(&self, cycle: SettlementCycle) -> u64 {
        let now = chrono::Utc::now();
        match cycle {
            SettlementCycle::T0 => {
                // T+0 starts from 0:00 of the day
                now.date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp() as u64
            }
            SettlementCycle::T1 => {
                // T+1 starts from 0:00 of yesterday
                (now - chrono::Duration::days(1))
                    .date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp() as u64
            }
            SettlementCycle::Custom(days) => {
                // Custom cycle
                (now - chrono::Duration::days(days as i64))
                    .date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp() as u64
            }
        }
    }

    /// Get cycle end date
    async fn get_cycle_end_date(&self, cycle: SettlementCycle) -> u64 {
        let now = chrono::Utc::now();
        match cycle {
            SettlementCycle::T0 => {
                // T+0 ends at current time
                now.timestamp() as u64
            }
            SettlementCycle::T1 => {
                // T+1 ends at 0:00:00 of today
                now.date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp() as u64
                    - 1
            }
            SettlementCycle::Custom(_) => {
                // Custom cycle ends at current time
                now.timestamp() as u64
            }
        }
    }

    /// Auto settlement processing
    pub async fn auto_settlement(&self) -> Result<Vec<SettlementOrder>, String> {
        let configs = self.configs.read().await;
        let mut settled_orders = Vec::new();

        for config in configs.values() {
            if config.auto_settlement {
                // Check if settlement time has arrived
                if self.is_settlement_time(config).await {
                    // Get pending settlement amount
                    let pending_amount = self.get_pending_amount(&config.merchant_id).await;
                    if pending_amount >= config.min_settlement_amount {
                        // Auto apply for settlement
                        let settlement_order = self
                            .apply_settlement(&config.merchant_id, pending_amount)
                            .await;
                        if let Ok(order) = settlement_order {
                            // Process settlement
                            let processed_order = self.process_settlement(&order.id).await;
                            if let Ok(processed) = processed_order {
                                settled_orders.push(processed);
                            }
                        }
                    }
                }
            }
        }

        info!(
            "Auto settlement completed, processed {} settlement orders",
            settled_orders.len()
        );
        Ok(settled_orders)
    }

    /// Check if settlement time has arrived
    async fn is_settlement_time(&self, config: &SettlementConfig) -> bool {
        // Simple implementation: settle at fixed time every day
        // Actual project can adjust according to configured time
        let now = chrono::Utc::now();
        let hour = now.hour();

        // Default settle at 2:00 AM every day
        let settlement_hour = config.auto_settlement_time.unwrap_or(2);
        hour == settlement_hour
    }
}

impl Default for SettlementManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Settlement bank information
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    /// Bank name
    pub name: String,
    /// Bank code
    pub code: String,
    /// Opening bank name
    pub branch_name: String,
    /// Opening bank code
    pub branch_code: String,
}

