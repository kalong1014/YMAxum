// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 制造业场景适配器
//! 提供制造业场景的核心业务逻辑实现

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::scene::{SceneAdapter, manufacturing::*};

/// 制造业场景适配器
pub struct ManufacturingSceneAdapter {
    /// 生产计划存储
    production_plans: Arc<RwLock<Vec<ProductionPlan>>>,
    /// 库存存储
    inventory_items: Arc<RwLock<Vec<InventoryItem>>>,
    /// 质量检测记录存储
    quality_test_records: Arc<RwLock<Vec<QualityTestRecord>>>,
    /// 设备信息存储
    equipment_info: Arc<RwLock<Vec<EquipmentInfo>>>,
    /// 供应链订单存储
    supply_chain_orders: Arc<RwLock<Vec<SupplyChainOrder>>>,
}

impl ManufacturingSceneAdapter {
    /// 创建新的制造业场景适配器
    pub fn new() -> Self {
        Self {
            production_plans: Arc::new(RwLock::new(Vec::new())),
            inventory_items: Arc::new(RwLock::new(Vec::new())),
            quality_test_records: Arc::new(RwLock::new(Vec::new())),
            equipment_info: Arc::new(RwLock::new(Vec::new())),
            supply_chain_orders: Arc::new(RwLock::new(Vec::new())),
        }
    }

    // 生产计划管理

    /// 创建生产计划
    pub async fn create_production_plan(
        &self,
        name: String,
        product_id: String,
        product_name: String,
        planned_quantity: u32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<ProductionPlan> {
        let id = format!("plan_{}", Utc::now().timestamp_millis());
        let now = Utc::now();

        let production_plan = ProductionPlan {
            id,
            name,
            product_id,
            product_name,
            planned_quantity,
            start_time,
            end_time,
            status: ProductionPlanStatus::Planning,
            created_at: now,
            updated_at: now,
        };

        let mut production_plans = self.production_plans.write().await;
        production_plans.push(production_plan.clone());

        Ok(production_plan)
    }

    /// 获取生产计划
    pub async fn get_production_plan(&self, id: &str) -> Result<Option<ProductionPlan>> {
        let production_plans = self.production_plans.read().await;
        Ok(production_plans.iter().find(|plan| plan.id == id).cloned())
    }

    /// 获取所有生产计划
    pub async fn get_all_production_plans(&self) -> Result<Vec<ProductionPlan>> {
        let production_plans = self.production_plans.read().await;
        Ok(production_plans.clone())
    }

    /// 更新生产计划状态
    pub async fn update_production_plan_status(
        &self,
        id: &str,
        status: ProductionPlanStatus,
    ) -> Result<Option<ProductionPlan>> {
        let mut production_plans = self.production_plans.write().await;

        if let Some(plan) = production_plans.iter_mut().find(|plan| plan.id == id) {
            plan.status = status;
            plan.updated_at = Utc::now();
            return Ok(Some(plan.clone()));
        }

        Ok(None)
    }

    /// 删除生产计划
    pub async fn delete_production_plan(&self, id: &str) -> Result<bool> {
        let mut production_plans = self.production_plans.write().await;
        let initial_len = production_plans.len();

        production_plans.retain(|plan| plan.id != id);

        Ok(production_plans.len() < initial_len)
    }

    // 库存管理

    /// 创建库存项目
    pub async fn create_inventory_item(
        &self,
        name: String,
        item_type: String,
        current_quantity: u32,
        min_quantity: u32,
        max_quantity: u32,
        unit: String,
    ) -> Result<InventoryItem> {
        let id = format!("inventory_{}", Utc::now().timestamp_millis());
        let now = Utc::now();

        let status = if current_quantity < min_quantity {
            InventoryStatus::Insufficient
        } else if current_quantity > max_quantity {
            InventoryStatus::Excessive
        } else {
            InventoryStatus::Normal
        };

        let inventory_item = InventoryItem {
            id,
            name,
            item_type,
            current_quantity,
            min_quantity,
            max_quantity,
            unit,
            status,
            last_updated: now,
        };

        let mut inventory_items = self.inventory_items.write().await;
        inventory_items.push(inventory_item.clone());

        Ok(inventory_item)
    }

    /// 获取库存项目
    pub async fn get_inventory_item(&self, id: &str) -> Result<Option<InventoryItem>> {
        let inventory_items = self.inventory_items.read().await;
        Ok(inventory_items.iter().find(|item| item.id == id).cloned())
    }

    /// 获取所有库存项目
    pub async fn get_all_inventory_items(&self) -> Result<Vec<InventoryItem>> {
        let inventory_items = self.inventory_items.read().await;
        Ok(inventory_items.clone())
    }

    /// 更新库存数量
    pub async fn update_inventory_quantity(
        &self,
        id: &str,
        quantity_change: i32,
    ) -> Result<Option<InventoryItem>> {
        let mut inventory_items = self.inventory_items.write().await;

        if let Some(item) = inventory_items.iter_mut().find(|item| item.id == id) {
            let new_quantity = if quantity_change >= 0 {
                item.current_quantity + quantity_change as u32
            } else {
                item.current_quantity.saturating_sub(quantity_change.abs() as u32)
            };

            item.current_quantity = new_quantity;
            item.status = if new_quantity < item.min_quantity {
                InventoryStatus::Insufficient
            } else if new_quantity > item.max_quantity {
                InventoryStatus::Excessive
            } else {
                InventoryStatus::Normal
            };
            item.last_updated = Utc::now();

            return Ok(Some(item.clone()));
        }

        Ok(None)
    }

    /// 删除库存项目
    pub async fn delete_inventory_item(&self, id: &str) -> Result<bool> {
        let mut inventory_items = self.inventory_items.write().await;
        let initial_len = inventory_items.len();

        inventory_items.retain(|item| item.id != id);

        Ok(inventory_items.len() < initial_len)
    }

    // 质量控制

    /// 创建质量检测记录
    pub async fn create_quality_test_record(
        &self,
        product_id: String,
        batch_id: String,
        test_item: String,
        test_value: String,
        standard_value: String,
        tester: String,
        remarks: Option<String>,
    ) -> Result<QualityTestRecord> {
        let id = format!("quality_{}", Utc::now().timestamp_millis());
        let now = Utc::now();

        // 简单的质量检测逻辑，实际应用中可能更复杂
        let result = if test_value == standard_value {
            QualityTestResult::Pass
        } else {
            QualityTestResult::Fail
        };

        let quality_test_record = QualityTestRecord {
            id,
            product_id,
            batch_id,
            test_item,
            result,
            test_value,
            standard_value,
            test_time: now,
            tester,
            remarks,
        };

        let mut quality_test_records = self.quality_test_records.write().await;
        quality_test_records.push(quality_test_record.clone());

        Ok(quality_test_record)
    }

    /// 获取质量检测记录
    pub async fn get_quality_test_record(&self, id: &str) -> Result<Option<QualityTestRecord>> {
        let quality_test_records = self.quality_test_records.read().await;
        Ok(quality_test_records.iter().find(|record| record.id == id).cloned())
    }

    /// 获取批次的质量检测记录
    pub async fn get_quality_test_records_by_batch(
        &self,
        batch_id: &str,
    ) -> Result<Vec<QualityTestRecord>> {
        let quality_test_records = self.quality_test_records.read().await;
        Ok(quality_test_records
            .iter()
            .filter(|record| record.batch_id == batch_id)
            .cloned()
            .collect())
    }

    /// 获取所有质量检测记录
    pub async fn get_all_quality_test_records(&self) -> Result<Vec<QualityTestRecord>> {
        let quality_test_records = self.quality_test_records.read().await;
        Ok(quality_test_records.clone())
    }

    // 设备管理

    /// 创建设备信息
    pub async fn create_equipment_info(
        &self,
        name: String,
        equipment_type: String,
        model: String,
        location: String,
        purchase_date: DateTime<Utc>,
        responsible_person: String,
    ) -> Result<EquipmentInfo> {
        let id = format!("equipment_{}", Utc::now().timestamp_millis());

        let equipment_info = EquipmentInfo {
            id,
            name,
            equipment_type,
            model,
            status: EquipmentStatus::Standby,
            location,
            purchase_date,
            last_maintenance: None,
            next_maintenance: Some(purchase_date + chrono::Duration::days(90)), // 默认90天后需要维护
            responsible_person,
        };

        let mut equipment_info_list = self.equipment_info.write().await;
        equipment_info_list.push(equipment_info.clone());

        Ok(equipment_info)
    }

    /// 获取设备信息
    pub async fn get_equipment_info(&self, id: &str) -> Result<Option<EquipmentInfo>> {
        let equipment_info_list = self.equipment_info.read().await;
        Ok(equipment_info_list.iter().find(|info| info.id == id).cloned())
    }

    /// 获取所有设备信息
    pub async fn get_all_equipment_info(&self) -> Result<Vec<EquipmentInfo>> {
        let equipment_info_list = self.equipment_info.read().await;
        Ok(equipment_info_list.clone())
    }

    /// 更新设备状态
    pub async fn update_equipment_status(
        &self,
        id: &str,
        status: EquipmentStatus,
    ) -> Result<Option<EquipmentInfo>> {
        let mut equipment_info_list = self.equipment_info.write().await;

        if let Some(info) = equipment_info_list.iter_mut().find(|info| info.id == id) {
            info.status = status.clone();
            if status == EquipmentStatus::Maintenance {
                info.last_maintenance = Some(Utc::now());
                info.next_maintenance = Some(Utc::now() + chrono::Duration::days(90));
            }
            return Ok(Some(info.clone()));
        }

        Ok(None)
    }

    /// 删除设备信息
    pub async fn delete_equipment_info(&self, id: &str) -> Result<bool> {
        let mut equipment_info_list = self.equipment_info.write().await;
        let initial_len = equipment_info_list.len();

        equipment_info_list.retain(|info| info.id != id);

        Ok(equipment_info_list.len() < initial_len)
    }

    // 供应链管理

    /// 创建供应链订单
    pub async fn create_supply_chain_order(
        &self,
        supplier_id: String,
        supplier_name: String,
        item_id: String,
        item_name: String,
        quantity: u32,
        unit_price: f64,
        expected_delivery_date: DateTime<Utc>,
        remarks: Option<String>,
    ) -> Result<SupplyChainOrder> {
        let id = format!("order_{}", Utc::now().timestamp_millis());
        let now = Utc::now();
        let total_amount = quantity as f64 * unit_price;

        let supply_chain_order = SupplyChainOrder {
            id,
            supplier_id,
            supplier_name,
            item_id,
            item_name,
            quantity,
            unit_price,
            total_amount,
            order_date: now,
            expected_delivery_date,
            actual_delivery_date: None,
            status: SupplyChainOrderStatus::Pending,
            remarks,
        };

        let mut supply_chain_orders = self.supply_chain_orders.write().await;
        supply_chain_orders.push(supply_chain_order.clone());

        Ok(supply_chain_order)
    }

    /// 获取供应链订单
    pub async fn get_supply_chain_order(&self, id: &str) -> Result<Option<SupplyChainOrder>> {
        let supply_chain_orders = self.supply_chain_orders.read().await;
        Ok(supply_chain_orders.iter().find(|order| order.id == id).cloned())
    }

    /// 获取所有供应链订单
    pub async fn get_all_supply_chain_orders(&self) -> Result<Vec<SupplyChainOrder>> {
        let supply_chain_orders = self.supply_chain_orders.read().await;
        Ok(supply_chain_orders.clone())
    }

    /// 更新供应链订单状态
    pub async fn update_supply_chain_order_status(
        &self,
        id: &str,
        status: SupplyChainOrderStatus,
    ) -> Result<Option<SupplyChainOrder>> {
        let mut supply_chain_orders = self.supply_chain_orders.write().await;

        if let Some(order) = supply_chain_orders.iter_mut().find(|order| order.id == id) {
            order.status = status.clone();
            if status == SupplyChainOrderStatus::Completed {
                order.actual_delivery_date = Some(Utc::now());
                // 自动更新库存
                self.update_inventory_quantity(&order.item_id, order.quantity as i32)
                    .await?;
            }
            return Ok(Some(order.clone()));
        }

        Ok(None)
    }

    /// 删除供应链订单
    pub async fn delete_supply_chain_order(&self, id: &str) -> Result<bool> {
        let mut supply_chain_orders = self.supply_chain_orders.write().await;
        let initial_len = supply_chain_orders.len();

        supply_chain_orders.retain(|order| order.id != id);

        Ok(supply_chain_orders.len() < initial_len)
    }

    // 统计和分析

    /// 获取生产计划统计
    pub async fn get_production_plan_stats(&self) -> Result<serde_json::Value> {
        let production_plans = self.production_plans.read().await;

        let mut stats = serde_json::json!({
            "total_plans": production_plans.len(),
            "status_counts": {
                "planning": 0,
                "approved": 0,
                "executing": 0,
                "completed": 0,
                "cancelled": 0
            }
        });

        for plan in production_plans.iter() {
            match plan.status {
                ProductionPlanStatus::Planning => {
                    stats["status_counts"]["planning"] = 
                        (stats["status_counts"]["planning"].as_i64().unwrap() + 1).into();
                }
                ProductionPlanStatus::Approved => {
                    stats["status_counts"]["approved"] = 
                        (stats["status_counts"]["approved"].as_i64().unwrap() + 1).into();
                }
                ProductionPlanStatus::Executing => {
                    stats["status_counts"]["executing"] = 
                        (stats["status_counts"]["executing"].as_i64().unwrap() + 1).into();
                }
                ProductionPlanStatus::Completed => {
                    stats["status_counts"]["completed"] = 
                        (stats["status_counts"]["completed"].as_i64().unwrap() + 1).into();
                }
                ProductionPlanStatus::Cancelled => {
                    stats["status_counts"]["cancelled"] = 
                        (stats["status_counts"]["cancelled"].as_i64().unwrap() + 1).into();
                }
            }
        }

        Ok(stats)
    }

    /// 获取库存统计
    pub async fn get_inventory_stats(&self) -> Result<serde_json::Value> {
        let inventory_items = self.inventory_items.read().await;

        let mut stats = serde_json::json!({
            "total_items": inventory_items.len(),
            "status_counts": {
                "normal": 0,
                "insufficient": 0,
                "excessive": 0
            },
            "total_value": 0
        });

        for item in inventory_items.iter() {
            match item.status {
                InventoryStatus::Normal => {
                    stats["status_counts"]["normal"] = 
                        (stats["status_counts"]["normal"].as_i64().unwrap() + 1).into();
                }
                InventoryStatus::Insufficient => {
                    stats["status_counts"]["insufficient"] = 
                        (stats["status_counts"]["insufficient"].as_i64().unwrap() + 1).into();
                }
                InventoryStatus::Excessive => {
                    stats["status_counts"]["excessive"] = 
                        (stats["status_counts"]["excessive"].as_i64().unwrap() + 1).into();
                }
            }
        }

        Ok(stats)
    }

    /// 获取设备状态统计
    pub async fn get_equipment_stats(&self) -> Result<serde_json::Value> {
        let equipment_info_list = self.equipment_info.read().await;

        let mut stats = serde_json::json!({
            "total_equipment": equipment_info_list.len(),
            "status_counts": {
                "running": 0,
                "standby": 0,
                "maintenance": 0,
                "fault": 0,
                "decommissioned": 0
            }
        });

        for equipment in equipment_info_list.iter() {
            match equipment.status {
                EquipmentStatus::Running => {
                    stats["status_counts"]["running"] = 
                        (stats["status_counts"]["running"].as_i64().unwrap() + 1).into();
                }
                EquipmentStatus::Standby => {
                    stats["status_counts"]["standby"] = 
                        (stats["status_counts"]["standby"].as_i64().unwrap() + 1).into();
                }
                EquipmentStatus::Maintenance => {
                    stats["status_counts"]["maintenance"] = 
                        (stats["status_counts"]["maintenance"].as_i64().unwrap() + 1).into();
                }
                EquipmentStatus::Fault => {
                    stats["status_counts"]["fault"] = 
                        (stats["status_counts"]["fault"].as_i64().unwrap() + 1).into();
                }
                EquipmentStatus::Decommissioned => {
                    stats["status_counts"]["decommissioned"] = 
                        (stats["status_counts"]["decommissioned"].as_i64().unwrap() + 1).into();
                }
            }
        }

        Ok(stats)
    }
}

impl SceneAdapter for ManufacturingSceneAdapter {
    fn name(&self) -> &'static str {
        "manufacturing"
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Initializing manufacturing scene adapter");
        // Initialize manufacturing-specific resources
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting manufacturing scene adapter");
        // Start manufacturing-specific services
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Stopping manufacturing scene adapter");
        // Stop manufacturing-specific services
        Ok(())
    }
}

/// 制造业场景请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManufacturingRequest {
    /// 创建生产计划
    CreateProductionPlan {
        name: String,
        product_id: String,
        product_name: String,
        planned_quantity: u32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    },
    /// 获取生产计划
    GetProductionPlan { id: String },
    /// 获取所有生产计划
    GetAllProductionPlans,
    /// 更新生产计划状态
    UpdateProductionPlanStatus {
        id: String,
        status: ProductionPlanStatus,
    },
    /// 删除生产计划
    DeleteProductionPlan { id: String },
    /// 创建库存项目
    CreateInventoryItem {
        name: String,
        item_type: String,
        current_quantity: u32,
        min_quantity: u32,
        max_quantity: u32,
        unit: String,
    },
    /// 获取库存项目
    GetInventoryItem { id: String },
    /// 获取所有库存项目
    GetAllInventoryItems,
    /// 更新库存数量
    UpdateInventoryQuantity {
        id: String,
        quantity_change: i32,
    },
    /// 删除库存项目
    DeleteInventoryItem { id: String },
    /// 创建质量检测记录
    CreateQualityTestRecord {
        product_id: String,
        batch_id: String,
        test_item: String,
        test_value: String,
        standard_value: String,
        tester: String,
        remarks: Option<String>,
    },
    /// 获取质量检测记录
    GetQualityTestRecord { id: String },
    /// 获取批次的质量检测记录
    GetQualityTestRecordsByBatch { batch_id: String },
    /// 获取所有质量检测记录
    GetAllQualityTestRecords,
    /// 创建设备信息
    CreateEquipmentInfo {
        name: String,
        equipment_type: String,
        model: String,
        location: String,
        purchase_date: DateTime<Utc>,
        responsible_person: String,
    },
    /// 获取设备信息
    GetEquipmentInfo { id: String },
    /// 获取所有设备信息
    GetAllEquipmentInfo,
    /// 更新设备状态
    UpdateEquipmentStatus {
        id: String,
        status: EquipmentStatus,
    },
    /// 删除设备信息
    DeleteEquipmentInfo { id: String },
    /// 创建供应链订单
    CreateSupplyChainOrder {
        supplier_id: String,
        supplier_name: String,
        item_id: String,
        item_name: String,
        quantity: u32,
        unit_price: f64,
        expected_delivery_date: DateTime<Utc>,
        remarks: Option<String>,
    },
    /// 获取供应链订单
    GetSupplyChainOrder { id: String },
    /// 获取所有供应链订单
    GetAllSupplyChainOrders,
    /// 更新供应链订单状态
    UpdateSupplyChainOrderStatus {
        id: String,
        status: SupplyChainOrderStatus,
    },
    /// 删除供应链订单
    DeleteSupplyChainOrder { id: String },
    /// 获取生产计划统计
    GetProductionPlanStats,
    /// 获取库存统计
    GetInventoryStats,
    /// 获取设备状态统计
    GetEquipmentStats,
}

/// 制造业场景响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManufacturingResponse {
    /// 成功
    Success(serde_json::Value),
    /// 失败
    Error(String),
}

/// 处理制造业场景请求
pub async fn handle_manufacturing_request(
    adapter: Arc<ManufacturingSceneAdapter>,
    request: ManufacturingRequest,
) -> Result<ManufacturingResponse> {
    match request {
        ManufacturingRequest::CreateProductionPlan {
            name,
            product_id,
            product_name,
            planned_quantity,
            start_time,
            end_time,
        } => {
            let plan = adapter
                .create_production_plan(
                    name,
                    product_id,
                    product_name,
                    planned_quantity,
                    start_time,
                    end_time,
                )
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(plan)?))
        }
        ManufacturingRequest::GetProductionPlan { id } => {
            let plan = adapter.get_production_plan(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(plan)?))
        }
        ManufacturingRequest::GetAllProductionPlans => {
            let plans = adapter.get_all_production_plans().await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(plans)?))
        }
        ManufacturingRequest::UpdateProductionPlanStatus { id, status } => {
            let plan = adapter.update_production_plan_status(&id, status).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(plan)?))
        }
        ManufacturingRequest::DeleteProductionPlan { id } => {
            let result = adapter.delete_production_plan(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(result)?))
        }
        ManufacturingRequest::CreateInventoryItem {
            name,
            item_type,
            current_quantity,
            min_quantity,
            max_quantity,
            unit,
        } => {
            let item = adapter
                .create_inventory_item(
                    name,
                    item_type,
                    current_quantity,
                    min_quantity,
                    max_quantity,
                    unit,
                )
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(item)?))
        }
        ManufacturingRequest::GetInventoryItem { id } => {
            let item = adapter.get_inventory_item(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(item)?))
        }
        ManufacturingRequest::GetAllInventoryItems => {
            let items = adapter.get_all_inventory_items().await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(items)?))
        }
        ManufacturingRequest::UpdateInventoryQuantity { id, quantity_change } => {
            let item = adapter
                .update_inventory_quantity(&id, quantity_change)
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(item)?))
        }
        ManufacturingRequest::DeleteInventoryItem { id } => {
            let result = adapter.delete_inventory_item(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(result)?))
        }
        ManufacturingRequest::CreateQualityTestRecord {
            product_id,
            batch_id,
            test_item,
            test_value,
            standard_value,
            tester,
            remarks,
        } => {
            let record = adapter
                .create_quality_test_record(
                    product_id,
                    batch_id,
                    test_item,
                    test_value,
                    standard_value,
                    tester,
                    remarks,
                )
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(record)?))
        }
        ManufacturingRequest::GetQualityTestRecord { id } => {
            let record = adapter.get_quality_test_record(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(record)?))
        }
        ManufacturingRequest::GetQualityTestRecordsByBatch { batch_id } => {
            let records = adapter
                .get_quality_test_records_by_batch(&batch_id)
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(records)?))
        }
        ManufacturingRequest::GetAllQualityTestRecords => {
            let records = adapter.get_all_quality_test_records().await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(records)?))
        }
        ManufacturingRequest::CreateEquipmentInfo {
            name,
            equipment_type,
            model,
            location,
            purchase_date,
            responsible_person,
        } => {
            let info = adapter
                .create_equipment_info(
                    name,
                    equipment_type,
                    model,
                    location,
                    purchase_date,
                    responsible_person,
                )
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(info)?))
        }
        ManufacturingRequest::GetEquipmentInfo { id } => {
            let info = adapter.get_equipment_info(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(info)?))
        }
        ManufacturingRequest::GetAllEquipmentInfo => {
            let info_list = adapter.get_all_equipment_info().await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(info_list)?))
        }
        ManufacturingRequest::UpdateEquipmentStatus { id, status } => {
            let info = adapter.update_equipment_status(&id, status).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(info)?))
        }
        ManufacturingRequest::DeleteEquipmentInfo { id } => {
            let result = adapter.delete_equipment_info(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(result)?))
        }
        ManufacturingRequest::CreateSupplyChainOrder {
            supplier_id,
            supplier_name,
            item_id,
            item_name,
            quantity,
            unit_price,
            expected_delivery_date,
            remarks,
        } => {
            let order = adapter
                .create_supply_chain_order(
                    supplier_id,
                    supplier_name,
                    item_id,
                    item_name,
                    quantity,
                    unit_price,
                    expected_delivery_date,
                    remarks,
                )
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(order)?))
        }
        ManufacturingRequest::GetSupplyChainOrder { id } => {
            let order = adapter.get_supply_chain_order(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(order)?))
        }
        ManufacturingRequest::GetAllSupplyChainOrders => {
            let orders = adapter.get_all_supply_chain_orders().await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(orders)?))
        }
        ManufacturingRequest::UpdateSupplyChainOrderStatus { id, status } => {
            let order = adapter
                .update_supply_chain_order_status(&id, status)
                .await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(order)?))
        }
        ManufacturingRequest::DeleteSupplyChainOrder { id } => {
            let result = adapter.delete_supply_chain_order(&id).await?;
            Ok(ManufacturingResponse::Success(serde_json::to_value(result)?))
        }
        ManufacturingRequest::GetProductionPlanStats => {
            let stats = adapter.get_production_plan_stats().await?;
            Ok(ManufacturingResponse::Success(stats))
        }
        ManufacturingRequest::GetInventoryStats => {
            let stats = adapter.get_inventory_stats().await?;
            Ok(ManufacturingResponse::Success(stats))
        }
        ManufacturingRequest::GetEquipmentStats => {
            let stats = adapter.get_equipment_stats().await?;
            Ok(ManufacturingResponse::Success(stats))
        }
    }
}

