// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Profit type
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ProfitType {
    /// Percentage profit
    Percentage,
    /// Tiered profit
    Tiered,
    /// Ladder profit
    Ladder,
}

/// Profit rule
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfitRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Profit type
    pub profit_type: ProfitType,
    /// Base profit rate (percentage)
    pub base_rate: f64,
    /// Tiered profit configuration
    pub tier_config: Option<Vec<TierConfig>>,
    /// Ladder profit configuration
    pub ladder_config: Option<Vec<LadderConfig>>,
    /// Applicable product categories
    pub applicable_categories: Vec<String>,
    /// Applicable merchants
    pub applicable_merchants: Vec<String>,
    /// Is enabled
    pub enabled: bool,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
}

/// Tiered profit configuration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    /// Tier
    pub tier: u32,
    /// Profit rate (percentage)
    pub rate: f64,
    /// Maximum profit amount
    pub max_amount: Option<f64>,
}

/// Ladder profit configuration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LadderConfig {
    /// Start amount
    pub start_amount: f64,
    /// End amount
    pub end_amount: Option<f64>,
    /// Profit rate (percentage)
    pub rate: f64,
}

/// Profit record
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfitRecord {
    /// Record ID
    pub id: String,
    /// Order ID
    pub order_id: String,
    /// Merchant ID
    pub merchant_id: String,
    /// Profit amount
    pub profit_amount: f64,
    /// Order total amount
    pub order_amount: f64,
    /// Profit rate
    pub profit_rate: f64,
    /// Profit type
    pub profit_type: ProfitType,
    /// Profit rule ID
    pub rule_id: String,
    /// Profit status
    pub status: ProfitStatus,
    /// Profit time
    pub profit_at: u64,
    /// Settled time
    pub settled_at: Option<u64>,
    /// Remark
    pub remark: Option<String>,
}

/// Profit status
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ProfitStatus {
    /// Pending
    Pending,
    /// Calculated
    Calculated,
    /// Settled
    Settled,
    /// Refunded
    Refunded,
    /// Exception
    Exception,
}

/// Profit manager
#[derive(Clone)]
pub struct ProfitManager {
    /// Profit rule map
    rules: Arc<RwLock<HashMap<String, ProfitRule>>>,
    /// Profit record map
    records: Arc<RwLock<HashMap<String, ProfitRecord>>>,
    /// Merchant profit summary
    merchant_profit_summary: Arc<RwLock<HashMap<String, f64>>>,
}

impl ProfitManager {
    /// Create new profit manager
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
            records: Arc::new(RwLock::new(HashMap::new())),
            merchant_profit_summary: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create profit rule
    pub async fn create_rule(&self, rule: ProfitRule) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        if rules.contains_key(&rule.id) {
            return Err("Rule already exists".to_string());
        }

        rules.insert(rule.id.clone(), rule.clone());
        info!("Profit rule created: {}", rule.id);
        Ok(())
    }

    /// Update profit rule
    pub async fn update_rule(&self, rule: ProfitRule) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        if !rules.contains_key(&rule.id) {
            return Err("Rule does not exist".to_string());
        }

        rules.insert(rule.id.clone(), rule.clone());
        info!("Profit rule updated: {}", rule.id);
        Ok(())
    }

    /// Get profit rule
    pub async fn get_rule(&self, rule_id: &str) -> Option<ProfitRule> {
        let rules = self.rules.read().await;
        rules.get(rule_id).cloned()
    }

    /// Enable/disable profit rule
    pub async fn toggle_rule(&self, rule_id: &str, enabled: bool) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        let rule = rules
            .get_mut(rule_id)
            .ok_or("Rule does not exist".to_string())?;

        rule.enabled = enabled;
        rule.updated_at = chrono::Utc::now().timestamp() as u64;
        info!("Profit rule status updated: {} -> {}", rule_id, enabled);
        Ok(())
    }

    /// Calculate profit
    pub async fn calculate_profit(
        &self,
        order_id: &str,
        merchant_id: &str,
        order_amount: f64,
        rule_id: &str,
        category: &str,
    ) -> Result<ProfitRecord, String> {
        let rules = self.rules.read().await;
        let rule = rules
            .get(rule_id)
            .ok_or("Rule does not exist".to_string())?;

        if !rule.enabled {
            return Err("Rule is disabled".to_string());
        }

        // Check if product category is applicable
        if !rule.applicable_categories.contains(&category.to_string()) {
            return Err("Product category not applicable to this profit rule".to_string());
        }

        // Check if merchant is applicable
        if !rule.applicable_merchants.contains(&merchant_id.to_string()) {
            return Err("Merchant not applicable to this profit rule".to_string());
        }

        // Calculate profit amount based on profit type
        let (profit_amount, profit_rate) = match rule.profit_type {
            ProfitType::Percentage => {
                let amount = order_amount * rule.base_rate / 100.0;
                (amount, rule.base_rate)
            }
            ProfitType::Tiered => self.calculate_tiered_profit(order_amount, rule),
            ProfitType::Ladder => self.calculate_ladder_profit(order_amount, rule),
        };

        // Create profit record
        let record = ProfitRecord {
            id: format!("profit_{}", uuid::Uuid::new_v4()),
            order_id: order_id.to_string(),
            merchant_id: merchant_id.to_string(),
            profit_amount,
            order_amount,
            profit_rate,
            profit_type: rule.profit_type.clone(),
            rule_id: rule.id.clone(),
            status: ProfitStatus::Calculated,
            profit_at: chrono::Utc::now().timestamp() as u64,
            settled_at: None,
            remark: None,
        };

        // Save profit record
        let mut records = self.records.write().await;
        records.insert(record.id.clone(), record.clone());

        // Update merchant profit summary
        let mut summary = self.merchant_profit_summary.write().await;
        let current_profit = *summary.get(merchant_id).unwrap_or(&0.0);
        summary.insert(merchant_id.to_string(), current_profit + profit_amount);

        info!(
            "Profit calculation completed: OrderID={}, MerchantID={}, ProfitAmount={:.2}",
            order_id, merchant_id, profit_amount
        );

        Ok(record)
    }

    /// Calculate tiered profit
    fn calculate_tiered_profit(&self, order_amount: f64, rule: &ProfitRule) -> (f64, f64) {
        if let Some(tier_config) = &rule.tier_config {
            // Simple implementation: determine tier based on order amount
            // Actual project may need more complex logic
            let _tier = tier_config.len() as u32;
            if let Some(tier_cfg) = tier_config.last() {
                let amount = order_amount * tier_cfg.rate / 100.0;
                (amount, tier_cfg.rate)
            } else {
                (order_amount * rule.base_rate / 100.0, rule.base_rate)
            }
        } else {
            (order_amount * rule.base_rate / 100.0, rule.base_rate)
        }
    }

    /// Calculate ladder profit
    fn calculate_ladder_profit(&self, order_amount: f64, rule: &ProfitRule) -> (f64, f64) {
        if let Some(ladder_config) = &rule.ladder_config {
            // Find matching ladder
            for config in ladder_config {
                if let Some(end_amount) = config.end_amount {
                    if order_amount >= config.start_amount && order_amount < end_amount {
                        let amount = order_amount * config.rate / 100.0;
                        return (amount, config.rate);
                    }
                } else {
                    // Last ladder, no upper limit
                    if order_amount >= config.start_amount {
                        let amount = order_amount * config.rate / 100.0;
                        return (amount, config.rate);
                    }
                }
            }
            // Default use base rate
            (order_amount * rule.base_rate / 100.0, rule.base_rate)
        } else {
            (order_amount * rule.base_rate / 100.0, rule.base_rate)
        }
    }

    /// Batch settle profit
    pub async fn batch_settle_profit(
        &self,
        merchant_id: &str,
    ) -> Result<Vec<ProfitRecord>, String> {
        let mut records = self.records.write().await;
        let mut settled_records = Vec::new();
        let now = chrono::Utc::now().timestamp() as u64;

        // Find pending settlement profit records for this merchant
        for record in records.values_mut() {
            if record.merchant_id == merchant_id && record.status == ProfitStatus::Calculated {
                record.status = ProfitStatus::Settled;
                record.settled_at = Some(now);
                settled_records.push(record.clone());
            }
        }

        if settled_records.is_empty() {
            return Ok(settled_records);
        }

        // Reset merchant profit summary
        let mut summary = self.merchant_profit_summary.write().await;
        summary.remove(merchant_id);

        info!(
            "Merchant profit settled: {}, total {} records",
            merchant_id,
            settled_records.len()
        );
        Ok(settled_records)
    }

    /// Get merchant profit summary
    pub async fn get_merchant_profit_summary(&self, merchant_id: &str) -> f64 {
        let summary = self.merchant_profit_summary.read().await;
        *summary.get(merchant_id).unwrap_or(&0.0)
    }

    /// Get profit record
    pub async fn get_profit_record(&self, record_id: &str) -> Option<ProfitRecord> {
        let records = self.records.read().await;
        records.get(record_id).cloned()
    }

    /// Get merchant profit record list
    pub async fn get_merchant_profit_records(
        &self,
        merchant_id: &str,
        status: Option<ProfitStatus>,
    ) -> Vec<ProfitRecord> {
        let records = self.records.read().await;
        let mut result = Vec::new();

        for record in records.values() {
            if record.merchant_id == merchant_id {
                if let Some(ref record_status) = status {
                    if record.status == *record_status {
                        result.push(record.clone());
                    }
                } else {
                    result.push(record.clone());
                }
            }
        }

        result
    }

    /// Handle refund profit
    pub async fn handle_refund(&self, order_id: &str) -> Result<(), String> {
        let mut records = self.records.write().await;

        // Find profit record for this order
        for record in records.values_mut() {
            if record.order_id == order_id && record.status != ProfitStatus::Refunded {
                record.status = ProfitStatus::Refunded;

                // Deduct from merchant profit summary
                let mut summary = self.merchant_profit_summary.write().await;
                if let Some(current_profit) = summary.get_mut(&record.merchant_id) {
                    *current_profit -= record.profit_amount;
                }
            }
        }

        info!("Order profit refunded: {}", order_id);
        Ok(())
    }
}

impl Default for ProfitManager {
    fn default() -> Self {
        Self::new()
    }
}

