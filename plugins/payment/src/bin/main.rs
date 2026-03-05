//! GUF 支付集成插件示例

use anyhow::Result;
use guf_payment_plugin::{GufPaymentPlugin, PaymentRequest, RefundRequest, PaymentResponse};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== GUF Payment Plugin Example ===");

    // 创建插件实例
    let mut plugin = GufPaymentPlugin::new();
    println!("Created plugin: {}", plugin.manifest.name);
    println!("Version: {}", plugin.manifest.version);
    println!("Description: {}", plugin.manifest.description);
    println!("GUF Compatible: {}", plugin.manifest.guf_compatible);
    println!("GUF Version: {}", plugin.manifest.guf_version);

    // 初始化插件
    println!("\nInitializing plugin...");
    if let Err(e) = plugin.initialize().await {
        eprintln!("Failed to initialize plugin: {}", e);
        return Err(e);
    }
    println!("Plugin initialized successfully!");

    // 测试支付处理
    println!("\nTesting payment processing...");
    let payment_request = PaymentRequest {
        amount: 100.50,
        currency: "USD".to_string(),
        payment_method: "credit_card".to_string(),
        customer_id: "customer_123".to_string(),
        order_id: "order_456".to_string(),
        description: "Test payment".to_string(),
        metadata: serde_json::json!({
            "product_id": "prod_789",
            "quantity": 1,
            "customer_email": "test@example.com"
        }),
    };
    match plugin.process_payment(payment_request).await {
        Ok(response) => {
            println!("Payment response: {:?}", response);
            if response.success {
                println!("Payment processed successfully!");
                
                // 测试获取交易信息
                if let Some(transaction_id) = response.transaction_id {
                    println!("\nTesting transaction retrieval...");
                    match plugin.get_transaction(&transaction_id).await {
                        Ok(Some(transaction)) => {
                            println!("Transaction retrieved successfully: {:?}", transaction);
                            
                            // 测试退款处理
                            println!("\nTesting refund processing...");
                            let refund_request = RefundRequest {
                                transaction_id: transaction_id.clone(),
                                amount: Some(50.25),
                                reason: "Partial refund for test".to_string(),
                                metadata: serde_json::json!({
                                    "refund_reason": "Customer request",
                                    "refunded_items": ["prod_789"]
                                }),
                            };
                            match plugin.process_refund(refund_request).await {
                                Ok(refund_response) => {
                                    println!("Refund response: {:?}", refund_response);
                                    if refund_response.success {
                                        println!("Refund processed successfully!");
                                    } else {
                                        println!("Refund failed: {}", refund_response.message);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to process refund: {}", e);
                                }
                            }
                        }
                        Ok(None) => {
                            println!("Transaction not found");
                        }
                        Err(e) => {
                            eprintln!("Failed to retrieve transaction: {}", e);
                        }
                    }
                }
            } else {
                println!("Payment failed: {}", response.message);
            }
        }
        Err(e) => {
            eprintln!("Failed to process payment: {}", e);
        }
    }

    // 停止插件
    println!("\nStopping plugin...");
    if let Err(e) = plugin.stop().await {
        eprintln!("Failed to stop plugin: {}", e);
        return Err(e);
    }
    println!("Plugin stopped successfully!");

    println!("\n=== GUF Payment Plugin Example Complete ===");
    Ok(())
}
