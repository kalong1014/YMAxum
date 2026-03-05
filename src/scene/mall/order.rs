// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Order status
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub enum OrderStatus {
    /// Pending payment
    PendingPayment,
    /// Paid
    Paid,
    /// Pending shipment
    PendingShipment,
    /// Shipped
    Shipped,
    /// Pending receipt
    PendingReceipt,
    /// Completed
    Completed,
    /// Cancelled
    Cancelled,
    /// Refunding
    Refunding,
    /// Refunded
    Refunded,
}

/// Payment method
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    /// WeChat Pay
    WeChatPay,
    /// Alipay
    Alipay,
    /// Bank card
    BankCard,
    /// Cash on delivery
    CashOnDelivery,
}

/// Order product item
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    /// Product ID
    pub product_id: String,
    /// Product name
    pub product_name: String,
    /// Product category
    pub category: String,
    /// Product price
    pub price: f64,
    /// Purchase quantity
    pub quantity: u32,
    /// Product image
    pub image_url: Option<String>,
    /// Product description
    pub description: Option<String>,
}

/// Order information
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,
    /// Merchant ID
    pub merchant_id: String,
    /// User ID
    pub user_id: String,
    /// Order number
    pub order_no: String,
    /// Order product list
    pub items: Vec<OrderItem>,
    /// Order total amount
    pub total_amount: f64,
    /// Actual payment amount
    pub actual_amount: f64,
    /// Payment method
    pub payment_method: PaymentMethod,
    /// Payment time
    pub payment_time: Option<u64>,
    /// Order status
    pub status: OrderStatus,
    /// Shipping address
    pub shipping_address: String,
    /// Consignee name
    pub consignee: String,
    /// Consignee phone
    pub consignee_phone: String,
    /// Order remark
    pub remark: Option<String>,
    /// Created at
    pub created_at: u64,
    /// Updated at
    pub updated_at: u64,
    /// Completed at
    pub completed_at: Option<u64>,
    /// Cancelled at
    pub cancelled_at: Option<u64>,
    /// Cancel reason
    pub cancel_reason: Option<String>,
}

/// Order manager
pub struct OrderManager {
    /// Order map
    orders: Arc<RwLock<HashMap<String, Order>>>,
    /// Merchant order map (MerchantID -> Vec<OrderID>)
    merchant_orders: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// User order map (UserID -> Vec<OrderID>)
    user_orders: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Order counter
    order_counter: Arc<RwLock<u64>>,
}

impl OrderManager {
    /// Create new order manager
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
            merchant_orders: Arc::new(RwLock::new(HashMap::new())),
            user_orders: Arc::new(RwLock::new(HashMap::new())),
            order_counter: Arc::new(RwLock::new(0)),
        }
    }

    /// Generate order number
    async fn generate_order_no(&self) -> String {
        let mut counter = self.order_counter.write().await;
        *counter += 1;
        let now = chrono::Utc::now();
        format!("ORD{}{:06}", now.format("%Y%m%d%H%M%S"), *counter)
    }

    /// Create order
    pub async fn create_order(&self, order: Order) -> Result<Order, String> {
        let mut orders = self.orders.write().await;
        let mut merchant_orders = self.merchant_orders.write().await;
        let mut user_orders = self.user_orders.write().await;

        if orders.contains_key(&order.id) {
            return Err("Order already exists".to_string());
        }

        // 安全处理：对订单中的用户输入字段进行验证和转义，防止 XSS 攻击
        let mut new_order = order.clone();
        
        // 转义可能包含 HTML 的字段
        if let Some(remark) = &new_order.remark {
            new_order.remark = Some(ammonia::clean(remark));
        }
        
        // 对地址、收货人姓名和电话进行基本验证
        if new_order.shipping_address.len() > 500 {
            return Err("Shipping address too long".to_string());
        }
        
        if new_order.consignee.len() > 50 {
            return Err("Consignee name too long".to_string());
        }
        
        if new_order.consignee_phone.len() > 20 {
            return Err("Consignee phone too long".to_string());
        }

        // 对订单项中的用户输入字段进行验证和转义
        for item in &mut new_order.items {
            if let Some(description) = &item.description {
                item.description = Some(ammonia::clean(description));
            }
        }

        // Generate order number
        let order_no = self.generate_order_no().await;
        new_order.order_no = order_no;
        new_order.created_at = chrono::Utc::now().timestamp() as u64;
        new_order.updated_at = new_order.created_at;

        // Save order
        orders.insert(new_order.id.clone(), new_order.clone());

        // Update merchant order list
        if let Some(merchant_order_list) = merchant_orders.get_mut(&new_order.merchant_id) {
            merchant_order_list.push(new_order.id.clone());
        } else {
            merchant_orders.insert(new_order.merchant_id.clone(), vec![new_order.id.clone()]);
        }

        // Update user order list
        if let Some(user_order_list) = user_orders.get_mut(&new_order.user_id) {
            user_order_list.push(new_order.id.clone());
        } else {
            user_orders.insert(new_order.user_id.clone(), vec![new_order.id.clone()]);
        }

        info!(
            "Order created: {}, MerchantID: {}, UserID: {}",
            new_order.id, new_order.merchant_id, new_order.user_id
        );
        Ok(new_order)
    }

    /// Get order information
    pub async fn get_order(&self, order_id: &str) -> Option<Order> {
        let orders = self.orders.read().await;
        orders.get(order_id).cloned()
    }

    /// Update order status
    pub async fn update_order_status(
        &self,
        order_id: &str,
        status: OrderStatus,
    ) -> Result<Order, String> {
        let mut orders = self.orders.write().await;
        let order = orders
            .get_mut(order_id)
            .ok_or("Order does not exist".to_string())?;

        order.status = status.clone();
        order.updated_at = chrono::Utc::now().timestamp() as u64;

        // Update completion time or cancellation time
        match status {
            OrderStatus::Completed => {
                order.completed_at = Some(order.updated_at);
            }
            OrderStatus::Cancelled => {
                order.cancelled_at = Some(order.updated_at);
            }
            _ => {}
        }

        info!("Order status updated: {} -> {:?}", order_id, status);
        Ok(order.clone())
    }

    /// Pay order
    pub async fn pay_order(
        &self,
        order_id: &str,
        payment_method: PaymentMethod,
        actual_amount: f64,
    ) -> Result<Order, String> {
        let mut orders = self.orders.write().await;
        let order = orders
            .get_mut(order_id)
            .ok_or("Order does not exist".to_string())?;

        if order.status != OrderStatus::PendingPayment {
            return Err("Order status is incorrect, cannot pay".to_string());
        }

        let payment_method_clone = payment_method.clone();
        order.status = OrderStatus::Paid;
        order.payment_method = payment_method;
        order.actual_amount = actual_amount;
        order.payment_time = Some(chrono::Utc::now().timestamp() as u64);
        order.updated_at = order.payment_time.unwrap();

        info!(
            "Order paid: {}, Payment method: {:?}, Payment amount: {:.2}",
            order_id, payment_method_clone, actual_amount
        );
        Ok(order.clone())
    }

    /// Cancel order
    pub async fn cancel_order(
        &self,
        order_id: &str,
        reason: Option<&str>,
    ) -> Result<Order, String> {
        let mut orders = self.orders.write().await;
        let order = orders
            .get_mut(order_id)
            .ok_or("Order does not exist".to_string())?;

        // Only pending payment status can be cancelled
        if order.status != OrderStatus::PendingPayment {
            return Err("Order status is incorrect, cannot cancel".to_string());
        }

        order.status = OrderStatus::Cancelled;
        order.cancelled_at = Some(chrono::Utc::now().timestamp() as u64);
        order.cancel_reason = reason.map(|r| r.to_string());
        order.updated_at = order.cancelled_at.unwrap();

        info!("Order cancelled: {}, Reason: {:?}", order_id, reason);
        Ok(order.clone())
    }

    /// Get merchant order list
    pub async fn get_merchant_orders(
        &self,
        merchant_id: &str,
        status: Option<OrderStatus>,
    ) -> Vec<Order> {
        let orders = self.orders.read().await;
        let merchant_orders = self.merchant_orders.read().await;
        let mut result = Vec::new();

        if let Some(order_ids) = merchant_orders.get(merchant_id) {
            for order_id in order_ids {
                if let Some(order) = orders.get(order_id) {
                    if let Some(ref order_status) = status {
                        if order.status == *order_status {
                            result.push(order.clone());
                        }
                    } else {
                        result.push(order.clone());
                    }
                }
            }
        }

        result
    }

    /// Get user order list
    pub async fn get_user_orders(&self, user_id: &str, status: Option<OrderStatus>) -> Vec<Order> {
        let orders = self.orders.read().await;
        let user_orders = self.user_orders.read().await;
        let mut result = Vec::new();

        if let Some(order_ids) = user_orders.get(user_id) {
            for order_id in order_ids {
                if let Some(order) = orders.get(order_id) {
                    if let Some(ref order_status) = status {
                        if order.status == *order_status {
                            result.push(order.clone());
                        }
                    } else {
                        result.push(order.clone());
                    }
                }
            }
        }

        result
    }

    /// Get order statistics information
    pub async fn get_order_stats(&self, merchant_id: &str) -> HashMap<OrderStatus, u32> {
        let orders = self.orders.read().await;
        let merchant_orders = self.merchant_orders.read().await;
        let mut stats = HashMap::new();

        if let Some(order_ids) = merchant_orders.get(merchant_id) {
            for order_id in order_ids {
                if let Some(order) = orders.get(order_id) {
                    let count = stats.entry(order.status.clone()).or_insert(0);
                    *count += 1;
                }
            }
        }

        stats
    }
}

impl Default for OrderManager {
    fn default() -> Self {
        Self::new()
    }
}

