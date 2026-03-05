//! 插件签名验证模块
//! 负责插件的签名生成和验证，确保插件的完整性和来源可信

use log::info;
use ring::signature::{self, KeyPair};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// 插件签名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSignature {
    /// 签名者
    pub signer: String,
    /// 签名时间
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 签名值
    pub signature: String,
    /// 公钥
    pub public_key: String,
}

/// 插件签名管理器
#[derive(Debug, Clone)]
pub struct GufPluginSignatureManager {
    /// 公钥存储
    public_keys: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, String>>>,
}

impl GufPluginSignatureManager {
    /// 创建新的签名管理器
    pub fn new() -> Self {
        Self {
            public_keys: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 初始化签名管理器
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化GUF插件签名管理器");
        Ok(())
    }

    /// 生成插件签名
    pub async fn sign_plugin(
        &self,
        plugin_path: &str,
        private_key_path: &str,
        signer: &str,
    ) -> Result<PluginSignature, String> {
        info!("为插件 {} 生成签名", plugin_path);

        // 读取插件文件
        let mut file = File::open(plugin_path).map_err(|e| format!("无法打开插件文件: {}", e))?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).map_err(|e| format!("无法读取插件文件: {}", e))?;

        // 读取私钥
        let mut key_file = File::open(private_key_path).map_err(|e| format!("无法打开私钥文件: {}", e))?;
        let mut private_key_pem = String::new();
        key_file.read_to_string(&mut private_key_pem).map_err(|e| format!("无法读取私钥文件: {}", e))?;

        // 解析私钥
        let private_key = signature::Ed25519KeyPair::from_pkcs8(
            private_key_pem.as_bytes()
        ).map_err(|e| format!("无法解析私钥: {}", e))?;

        // 生成签名
        let signature = private_key.sign(&content);
        let signature_str = BASE64.encode(signature.as_ref());

        // 获取公钥
        let public_key = private_key.public_key();
        let public_key_str = BASE64.encode(public_key.as_ref());

        // 创建签名对象
        let plugin_signature = PluginSignature {
            signer: signer.to_string(),
            timestamp: chrono::Utc::now(),
            signature: signature_str,
            public_key: public_key_str,
        };

        // 保存签名到插件目录
        let signature_path = Path::new(plugin_path).with_extension("sig");
        let mut sig_file = File::create(&signature_path).map_err(|e| format!("无法创建签名文件: {}", e))?;
        let sig_json = serde_json::to_string(&plugin_signature).map_err(|e| format!("无法序列化签名: {}", e))?;
        sig_file.write_all(sig_json.as_bytes()).map_err(|e| format!("无法写入签名文件: {}", e))?;

        info!("插件签名生成完成: {}", plugin_path);
        Ok(plugin_signature)
    }

    /// 验证插件签名
    pub async fn verify_plugin(&self, plugin_path: &str) -> Result<PluginSignature, String> {
        info!("验证插件 {} 的签名", plugin_path);

        // 读取插件文件
        let mut file = File::open(plugin_path).map_err(|e| format!("无法打开插件文件: {}", e))?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).map_err(|e| format!("无法读取插件文件: {}", e))?;

        // 读取签名文件
        let signature_path = Path::new(plugin_path).with_extension("sig");
        if !signature_path.exists() {
            return Err("签名文件不存在".to_string());
        }

        let mut sig_file = File::open(&signature_path).map_err(|e| format!("无法打开签名文件: {}", e))?;
        let mut sig_json = String::new();
        sig_file.read_to_string(&mut sig_json).map_err(|e| format!("无法读取签名文件: {}", e))?;

        // 解析签名
        let plugin_signature: PluginSignature = serde_json::from_str(&sig_json).map_err(|e| format!("无法解析签名: {}", e))?;

        // 验证签名
        let public_key_bytes = BASE64.decode(&plugin_signature.public_key).map_err(|e| format!("无法解码公钥: {}", e))?;
        let public_key = signature::UnparsedPublicKey::new(
            &signature::ED25519,
            &public_key_bytes
        );

        let signature_bytes = BASE64.decode(&plugin_signature.signature).map_err(|e| format!("无法解码签名: {}", e))?;
        public_key.verify(&content, &signature_bytes).map_err(|e| format!("签名验证失败: {}", e))?;

        // 检查签名时间是否过期（可选）
        let now = chrono::Utc::now();
        let signature_age = now.signed_duration_since(plugin_signature.timestamp);
        if signature_age > chrono::Duration::days(365) {
            info!("签名已超过一年，可能需要更新");
        }

        info!("插件签名验证通过: {}", plugin_path);
        Ok(plugin_signature)
    }

    /// 注册公钥
    pub async fn register_public_key(&self, signer: &str, public_key: &str) -> Result<(), String> {
        info!("注册签名者 {} 的公钥", signer);

        let mut public_keys = self.public_keys.write().map_err(|e| format!("无法获取公钥存储锁: {}", e))?;
        public_keys.insert(signer.to_string(), public_key.to_string());

        Ok(())
    }

    /// 获取公钥
    pub async fn get_public_key(&self, signer: &str) -> Result<Option<String>, String> {
        let public_keys = self.public_keys.read().map_err(|e| format!("无法获取公钥存储锁: {}", e))?;
        Ok(public_keys.get(signer).cloned())
    }

    /// 验证签名者
    pub async fn verify_signer(&self, signer: &str, public_key: &str) -> Result<bool, String> {
        let public_keys = self.public_keys.read().map_err(|e| format!("无法获取公钥存储锁: {}", e))?;
        
        match public_keys.get(signer) {
            Some(stored_key) => Ok(stored_key == public_key),
            None => Ok(false),
        }
    }
}
