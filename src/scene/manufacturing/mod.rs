// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 制造业场景模块
//! 提供制造业相关的业务逻辑和适配器

pub mod adapter;

/// 制造业场景错误
#[derive(thiserror::Error, Debug)]
pub enum ManufacturingError {
    /// 生产计划错误
    #[error("Production plan error: {0}")]
    ProductionPlanError(String),
    /// 库存错误
    #[error("Inventory error: {0}")]
    InventoryError(String),
    /// 质量控制错误
    #[error("Quality control error: {0}")]
    QualityControlError(String),
    /// 设备管理错误
    #[error("Equipment error: {0}")]
    EquipmentError(String),
    /// 供应链错误
    #[error("Supply chain error: {0}")]
    SupplyChainError(String),
    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

/// 生产计划状态
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProductionPlanStatus {
    /// 计划中
    Planning,
    /// 已批准
    Approved,
    /// 执行中
    Executing,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

/// 生产计划
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductionPlan {
    /// 计划ID
    pub id: String,
    /// 计划名称
    pub name: String,
    /// 产品ID
    pub product_id: String,
    /// 产品名称
    pub product_name: String,
    /// 计划数量
    pub planned_quantity: u32,
    /// 开始时间
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// 结束时间
    pub end_time: chrono::DateTime<chrono::Utc>,
    /// 状态
    pub status: ProductionPlanStatus,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 库存状态
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InventoryStatus {
    /// 正常
    Normal,
    /// 不足
    Insufficient,
    /// 过剩
    Excessive,
}

/// 库存项目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InventoryItem {
    /// 物品ID
    pub id: String,
    /// 物品名称
    pub name: String,
    /// 类型
    pub item_type: String,
    /// 当前数量
    pub current_quantity: u32,
    /// 最小库存
    pub min_quantity: u32,
    /// 最大库存
    pub max_quantity: u32,
    /// 单位
    pub unit: String,
    /// 状态
    pub status: InventoryStatus,
    /// 最后更新时间
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// 质量检测结果
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QualityTestResult {
    /// 通过
    Pass,
    /// 失败
    Fail,
    /// 待检测
    Pending,
}

/// 质量检测记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QualityTestRecord {
    /// 检测ID
    pub id: String,
    /// 产品ID
    pub product_id: String,
    /// 批次ID
    pub batch_id: String,
    /// 检测项目
    pub test_item: String,
    /// 检测结果
    pub result: QualityTestResult,
    /// 检测值
    pub test_value: String,
    /// 标准值
    pub standard_value: String,
    /// 检测时间
    pub test_time: chrono::DateTime<chrono::Utc>,
    /// 检测人员
    pub tester: String,
    /// 备注
    pub remarks: Option<String>,
}

/// 设备状态
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EquipmentStatus {
    /// 运行中
    Running,
    /// 待机
    Standby,
    /// 维护中
    Maintenance,
    /// 故障
    Fault,
    /// 停用
    Decommissioned,
}

/// 设备信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EquipmentInfo {
    /// 设备ID
    pub id: String,
    /// 设备名称
    pub name: String,
    /// 设备类型
    pub equipment_type: String,
    /// 型号
    pub model: String,
    /// 状态
    pub status: EquipmentStatus,
    /// 位置
    pub location: String,
    /// 购买时间
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    /// 上次维护时间
    pub last_maintenance: Option<chrono::DateTime<chrono::Utc>>,
    /// 下次维护时间
    pub next_maintenance: Option<chrono::DateTime<chrono::Utc>>,
    /// 责任人
    pub responsible_person: String,
}

/// 供应链订单状态
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SupplyChainOrderStatus {
    /// 待处理
    Pending,
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

/// 供应链订单
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SupplyChainOrder {
    /// 订单ID
    pub id: String,
    /// 供应商ID
    pub supplier_id: String,
    /// 供应商名称
    pub supplier_name: String,
    /// 物品ID
    pub item_id: String,
    /// 物品名称
    pub item_name: String,
    /// 数量
    pub quantity: u32,
    /// 单价
    pub unit_price: f64,
    /// 总金额
    pub total_amount: f64,
    /// 订单日期
    pub order_date: chrono::DateTime<chrono::Utc>,
    /// 预计到货日期
    pub expected_delivery_date: chrono::DateTime<chrono::Utc>,
    /// 实际到货日期
    pub actual_delivery_date: Option<chrono::DateTime<chrono::Utc>>,
    /// 状态
    pub status: SupplyChainOrderStatus,
    /// 备注
    pub remarks: Option<String>,
}

