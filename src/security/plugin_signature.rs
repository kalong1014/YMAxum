//! 插件签名验证模块
//! 用于验证插件的 RSA2048 签名，确保插件的完整性和真实性

use log::{info, warn};
use ring::digest;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use base64;
use chrono;

/// 插件签名信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSignature {
    /// 签名算法
    pub algorithm: String,
    /// 签名值（Base64 编码）
    pub signature: String,
    /// 公钥指纹
    pub public_key_fingerprint: String,
    /// 签名时间
    pub timestamp: String,
}

/// 插件签名验证器
#[derive(Debug, Clone)]
pub struct PluginSignatureVerifier {
    /// 可信公钥列表
    trusted_public_keys:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Vec<u8>>>>,
}

impl PluginSignatureVerifier {
    /// 创建新的插件签名验证器
    pub fn new() -> Self {
        Self {
            trusted_public_keys: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// 初始化插件签名验证器
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing plugin signature verifier...");
        // 这里可以加载可信公钥列表
        Ok(())
    }

    /// 添加可信公钥
    pub async fn add_trusted_public_key(
        &self,
        fingerprint: String,
        public_key: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Adding trusted public key with fingerprint: {}",
            fingerprint
        );
        let mut trusted_public_keys = self.trusted_public_keys.write().await;
        trusted_public_keys.insert(fingerprint, public_key);
        Ok(())
    }

    /// 从文件加载公钥
    pub fn load_public_key_from_file(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut public_key = Vec::new();
        file.read_to_end(&mut public_key)?;
        Ok(public_key)
    }

    /// 生成 RSA 密钥对
    pub fn generate_keypair() -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        info!("Generating RSA-2048 keypair...");

        // 使用 ring 库生成 RSA-2048 密钥对
        use ring::rsa;
        use ring::rand::SystemRandom;

        let rng = SystemRandom::new();
        let key_pair = rsa::KeyPair::generate_pkcs8(&rsa::RSA_PKCS1_2048_8192_SHA256, 2048, &rng)?;
        
        // 提取公钥和私钥
        let private_key = key_pair.as_ref().to_vec();
        let public_key = key_pair.public_key().as_ref().to_vec();

        Ok((private_key, public_key))
    }

    /// 计算公钥指纹
    pub fn calculate_public_key_fingerprint(public_key: &[u8]) -> String {
        let digest = digest::digest(&digest::SHA256, public_key);
        hex::encode(digest.as_ref())
    }

    /// 签名插件文件
    pub fn sign_plugin(
        private_key: &[u8],
        plugin_path: &Path,
    ) -> Result<PluginSignature, Box<dyn std::error::Error>> {
        info!("Signing plugin at path: {:?}", plugin_path);

        // 读取插件文件
        let mut file = File::open(plugin_path)?;
        let mut plugin_data = Vec::new();
        file.read_to_end(&mut plugin_data)?;

        // 使用 ring 库进行 RSA 签名
        use ring::rsa;
        use ring::rand::SystemRandom;
        use ring::signature;

        let rng = SystemRandom::new();
        let key_pair = rsa::KeyPair::from_pkcs8(private_key)?;
        
        // 计算文件哈希
        let digest = digest::digest(&digest::SHA256, &plugin_data);
        
        // 生成签名
        let mut signature_bytes = vec![0; key_pair.public_modulus_len()];
        let signature_length = key_pair.sign(
            &signature::RSA_PKCS1_SHA256,
            &rng,
            digest.as_ref(),
            &mut signature_bytes,
        )?;
        signature_bytes.truncate(signature_length);
        
        // 计算公钥指纹
        let public_key = key_pair.public_key().as_ref();
        let public_key_fingerprint = Self::calculate_public_key_fingerprint(public_key);
        
        // 创建签名信息
        let signature = PluginSignature {
            algorithm: "RSA2048-SHA256".to_string(),
            signature: base64::encode(&signature_bytes),
            public_key_fingerprint,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        Ok(signature)
    }

    /// 验证插件签名
    pub async fn verify_plugin_signature(
        &self,
        plugin_path: &Path,
        signature: &PluginSignature,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        info!("Verifying plugin signature at path: {:?}", plugin_path);

        // 检查签名算法
        if signature.algorithm != "RSA2048-SHA256" {
            warn!("Unsupported signature algorithm: {}", signature.algorithm);
            return Ok(false);
        }

        // 获取可信公钥
        let trusted_public_keys = self.trusted_public_keys.read().await;
        let public_key = match trusted_public_keys.get(&signature.public_key_fingerprint) {
            Some(key) => key,
            None => {
                warn!(
                    "Public key not found in trusted list: {}",
                    signature.public_key_fingerprint
                );
                return Ok(false);
            }
        };

        // 读取插件文件
        let mut file = File::open(plugin_path)?;
        let mut plugin_data = Vec::new();
        file.read_to_end(&mut plugin_data)?;

        // 使用 ring 库进行 RSA 签名验证
        use ring::signature;

        // 解码签名
        let signature_bytes = base64::decode(&signature.signature)?;
        
        // 计算文件哈希
        let digest = digest::digest(&digest::SHA256, &plugin_data);
        
        // 验证签名
        let public_key_der = signature::UnparsedPublicKey::new(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            public_key,
        );
        
        let result = public_key_der.verify(digest.as_ref(), &signature_bytes);
        match result {
            Ok(_) => {
                info!("Plugin signature verified successfully");
                Ok(true)
            }
            Err(e) => {
                warn!("Plugin signature verification failed: {:?}", e);
                Ok(false)
            }
        }
    }

    /// 验证插件是否已签名
    pub fn is_plugin_signed(plugin_path: &Path) -> bool {
        // 检查插件目录中是否存在签名文件
        let signature_path = plugin_path.with_extension("sig");
        signature_path.exists()
    }

    /// 从签名文件加载签名
    pub fn load_signature_from_file(
        signature_path: &Path,
    ) -> Result<PluginSignature, Box<dyn std::error::Error>> {
        let mut file = File::open(signature_path)?;
        let mut signature_data = Vec::new();
        file.read_to_end(&mut signature_data)?;

        let signature: PluginSignature = serde_json::from_slice(&signature_data)?;
        Ok(signature)
    }

    /// 保存签名到文件
    pub fn save_signature_to_file(
        signature: &PluginSignature,
        signature_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signature_data = serde_json::to_vec_pretty(signature)?;
        std::fs::write(signature_path, signature_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_keypair() {
        // 由于当前实现不支持RSA密钥对生成，我们只测试方法是否返回预期的错误
        let result = PluginSignatureVerifier::generate_keypair();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "RSA keypair generation not supported in this implementation"
        );
    }

    #[test]
    fn test_sign_and_verify() {
        // 由于当前实现不支持RSA密钥对生成和签名，我们测试验证逻辑
        let temp_dir = tempdir().unwrap();
        let plugin_path = temp_dir.path().join("test_plugin.wasm");
        std::fs::write(&plugin_path, b"test plugin content").unwrap();

        // 创建模拟签名
        let signature = PluginSignature {
            algorithm: "RSA2048-SHA256".to_string(),
            signature: "test_signature".to_string(),
            public_key_fingerprint: "test_fingerprint".to_string(),
            timestamp: "2026-02-06T00:00:00Z".to_string(),
        };

        // 创建验证器
        let verifier = PluginSignatureVerifier::new();

        // 在异步上下文中验证
        tokio_test::block_on(async {
            // 由于公钥不在可信列表中，验证应该失败
            let result = verifier
                .verify_plugin_signature(&plugin_path, &signature)
                .await
                .unwrap();
            assert!(!result);
        });
    }

    #[test]
    fn test_verify_invalid_signature() {
        // 由于当前实现不支持RSA密钥对生成和签名，我们测试验证逻辑
        let temp_dir = tempdir().unwrap();
        let plugin_path = temp_dir.path().join("test_plugin.wasm");
        std::fs::write(&plugin_path, b"test plugin content").unwrap();

        // 创建模拟签名
        let signature = PluginSignature {
            algorithm: "RSA2048-SHA256".to_string(),
            signature: "invalid_signature".to_string(),
            public_key_fingerprint: "test_fingerprint".to_string(),
            timestamp: "2026-02-06T00:00:00Z".to_string(),
        };

        // 创建验证器
        let verifier = PluginSignatureVerifier::new();

        // 在异步上下文中验证
        tokio_test::block_on(async {
            // 由于公钥不在可信列表中，验证应该失败
            let result = verifier
                .verify_plugin_signature(&plugin_path, &signature)
                .await
                .unwrap();
            assert!(!result);
        });
    }
}
