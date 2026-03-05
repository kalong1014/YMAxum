// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件签名和验证
//! 提供插件的数字签名、验证和可信白名单管理

use log::{error, info};
use rand::rngs::OsRng;
use rsa::{
    Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    sha2::{Digest, Sha256},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// 签名算法
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SignAlgorithm {
    /// RSA2048算法
    RSA2048,
    /// ECDSA算法
    ECDSA,
    /// 未知算法
    Unknown,
}

/// 签名信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SignatureInfo {
    /// 签名算法
    pub algorithm: SignAlgorithm,
    /// 签名者信息
    pub signer: String,
    /// 签名时间戳
    pub timestamp: u64,
    /// 签名数据
    pub signature: Vec<u8>,
    /// 插件哈希值
    pub plugin_hash: Vec<u8>,
    /// 签名ID
    pub signature_id: String,
}

/// 可信白名单条目
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TrustedEntry {
    /// 条目ID
    pub id: String,
    /// 签名者信息
    pub signer: String,
    /// 公钥哈希值
    pub pubkey_hash: Vec<u8>,
    /// 是否启用
    pub enabled: bool,
    /// 添加时间
    pub added_at: u64,
    /// 过期时间
    pub expires_at: u64,
}

/// 可信白名单
pub struct TrustedWhitelist {
    /// 可信条目映射
    entries: Arc<RwLock<HashMap<String, TrustedEntry>>>,
    /// 白名单文件路径
    whitelist_path: String,
}

impl TrustedWhitelist {
    /// 创建新的可信白名单实例
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            whitelist_path: "data/plugins/trusted_whitelist.json".to_string(),
        }
    }

    /// 获取白名单文件路径
    pub fn whitelist_path(&self) -> &str {
        &self.whitelist_path
    }

    /// 加载可信白名单
    pub fn load(&self) -> Result<(), String> {
        std::fs::create_dir_all("data/plugins").map_err(|e| format!("创建插件目录失败: {}", e))?;

        // 检查文件是否存在
        if !Path::new(&self.whitelist_path).exists() {
            // 文件不存在，创建默认文件
            let default_entries = Vec::<TrustedEntry>::new();
            let default_data = serde_json::to_string_pretty(&default_entries)
                .map_err(|e| format!("序列化默认白名单失败: {}", e))?;

            std::fs::write(&self.whitelist_path, default_data)
                .map_err(|e| format!("写入默认白名单文件失败: {}", e))?;
            return Ok(());
        }

        // 文件存在，读取并解析
        let file_content = std::fs::read_to_string(&self.whitelist_path)
            .map_err(|e| format!("读取白名单文件失败: {}", e))?;

        let entries: Vec<TrustedEntry> = serde_json::from_str(&file_content)
            .map_err(|e| format!("解析白名单文件失败: {}", e))?;

        // 清空现有条目并添加加载的条目
        let mut entries_map = self
            .entries
            .write()
            .map_err(|e| format!("获取白名单锁失败: {}", e))?;
        entries_map.clear();

        for entry in entries {
            entries_map.insert(entry.id.clone(), entry);
        }

        info!("加载可信白名单成功，共 {} 个条目", entries_map.len());
        Ok(())
    }

    /// 保存可信白名单
    pub fn save(&self) -> Result<(), String> {
        // 获取所有条目
        let entries_map = self
            .entries
            .read()
            .map_err(|e| format!("获取白名单锁失败: {}", e))?;

        let entries: Vec<TrustedEntry> = entries_map.values().cloned().collect();

        // 序列化条目
        let data = serde_json::to_string_pretty(&entries)
            .map_err(|e| format!("序列化白名单失败: {}", e))?;

        // 写入文件
        std::fs::write(&self.whitelist_path, data)
            .map_err(|e| format!("写入白名单文件失败: {}", e))?;

        info!("保存可信白名单成功，共 {} 个条目", entries.len());
        Ok(())
    }

    /// 添加可信白名单条目
    pub fn add_entry(&self, entry: TrustedEntry) -> Result<(), String> {
        let mut entries = self
            .entries
            .write()
            .map_err(|e| format!("获取白名单失败: {}", e))?;
        entries.insert(entry.id.clone(), entry);
        self.save()
    }

    /// 移除可信白名单条目
    pub fn remove_entry(&self, entry_id: &str) -> Result<(), String> {
        let mut entries = self
            .entries
            .write()
            .map_err(|e| format!("获取白名单失败: {}", e))?;
        entries.remove(entry_id);
        self.save()
    }

    /// 检查签名者是否在可信白名单中
    pub fn is_trusted(&self, signer: &str) -> bool {
        let entries = self.entries.read();
        if let Ok(entries) = entries {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            entries.values().any(|e| {
                e.signer == signer && 
                e.enabled && 
                e.expires_at > now
            })
        } else {
            error!("获取白名单失败，无法验证签名者");
            false
        }
    }

    /// 添加可信签名者
    pub fn add_trusted_signer(&self, signer: &str, pubkey_hash: &[u8], expires_days: u64) -> Result<(), String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let entry = TrustedEntry {
            id: Uuid::new_v4().to_string(),
            signer: signer.to_string(),
            pubkey_hash: pubkey_hash.to_vec(),
            enabled: true,
            added_at: now,
            expires_at: now + (expires_days * 86400),
        };
        
        self.add_entry(entry)
    }

    /// 启用/禁用可信签名者
    pub fn set_signer_enabled(&self, entry_id: &str, enabled: bool) -> Result<(), String> {
        let mut entries = self.entries.write()
            .map_err(|e| format!("获取白名单锁失败: {}", e))?;
        
        if let Some(entry) = entries.get_mut(entry_id) {
            entry.enabled = enabled;
            self.save()
        } else {
            Err(format!("白名单条目不存在: {}", entry_id))
        }
    }

    /// 获取所有可信白名单条目
    pub fn get_all_entries(&self) -> Vec<TrustedEntry> {
        let entries = self.entries.read();
        if let Ok(entries) = entries {
            entries.values().cloned().collect()
        } else {
            error!("获取白名单失败，无法获取所有条目");
            Vec::new()
        }
    }
}

impl Default for TrustedWhitelist {
    fn default() -> Self {
        Self::new()
    }
}

/// 插件签名器
pub struct PluginSigner {
    /// 私钥
    private_key: Option<Vec<u8>>,
    /// 公钥
    public_key: Option<Vec<u8>>,
    /// 签名算法
    algorithm: SignAlgorithm,
    /// 可信白名单
    whitelist: TrustedWhitelist,
}

impl PluginSigner {
    /// 创建新的插件签名器实例
    pub fn new() -> Self {
        Self {
            private_key: None,
            public_key: None,
            algorithm: SignAlgorithm::RSA2048,
            whitelist: TrustedWhitelist::new(),
        }
    }

    /// 获取签名算法
    pub fn get_algorithm(&self) -> &SignAlgorithm {
        &self.algorithm
    }

    /// 加载私钥
    pub fn load_private_key<P: AsRef<Path>>(&mut self, key_path: P) -> Result<(), String> {
        let mut file = File::open(key_path).map_err(|e| format!("打开私钥文件失败: {}", e))?;

        let mut key_data = Vec::new();
        file.read_to_end(&mut key_data)
            .map_err(|e| format!("读取私钥文件失败: {}", e))?;

        self.private_key = Some(key_data);
        Ok(())
    }

    /// 加载公钥
    pub fn load_public_key<P: AsRef<Path>>(&mut self, key_path: P) -> Result<(), String> {
        let mut file = File::open(key_path).map_err(|e| format!("打开公钥文件失败: {}", e))?;

        let mut key_data = Vec::new();
        file.read_to_end(&mut key_data)
            .map_err(|e| format!("读取公钥文件失败: {}", e))?;

        self.public_key = Some(key_data);
        Ok(())
    }

    /// 签名插件
    pub fn sign_plugin<P: AsRef<Path>>(&self, plugin_path: P) -> Result<SignatureInfo, String> {
        if self.private_key.is_none() {
            return Err("私钥未加载".to_string());
        }

        let plugin_path = plugin_path.as_ref();
        info!("签名插件: {:?}", plugin_path);

        // 计算插件哈希值
        let plugin_hash = self.calculate_plugin_hash(plugin_path)?;

        // 签名数据
        let signature = self.sign_data(&plugin_hash)?;

        // 获取当前时间戳
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {}", e))?
            .as_secs();

        // 保存签名信息
        let signature_info = SignatureInfo {
            algorithm: self.algorithm.clone(),
            signer: "YMAxum Framework".to_string(),
            timestamp,
            signature,
            plugin_hash,
            signature_id: Uuid::new_v4().to_string(),
        };

        Ok(signature_info)
    }

    /// 验证插件签名
    pub fn verify_signature<P: AsRef<Path>>(
        &self,
        plugin_path: P,
        signature_info: &SignatureInfo,
    ) -> Result<bool, String> {
        if self.public_key.is_none() {
            return Err("公钥未加载".to_string());
        }

        let plugin_path = plugin_path.as_ref();
        info!(
            "验证插件签名: {:?}, 签名ID: {}",
            plugin_path, signature_info.signature_id
        );

        // 检查签名算法
        if signature_info.algorithm != SignAlgorithm::RSA2048 {
            error!("不支持的签名算法: {:?}", signature_info.algorithm);
            return Ok(false);
        }

        // 检查签名时间戳
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("获取时间戳失败: {}", e))?
            .as_secs();
        
        // 检查签名是否过期（假设签名有效期为30天）
        const SIGNATURE_EXPIRY_DAYS: u64 = 30;
        const SECONDS_PER_DAY: u64 = 86400;
        
        if current_time - signature_info.timestamp > SIGNATURE_EXPIRY_DAYS * SECONDS_PER_DAY {
            error!("签名已过期，签名时间: {}, 当前时间: {}", signature_info.timestamp, current_time);
            return Ok(false);
        }

        // 检查签名者是否在可信白名单中
        if self.whitelist.is_trusted(&signature_info.signer) {
            info!("签名者 {} 在可信白名单中，跳过验证", signature_info.signer);
            return Ok(true);
        }

        // 计算插件哈希值
        let plugin_hash = self.calculate_plugin_hash(plugin_path)?;

        // 验证哈希值是否匹配
        if plugin_hash != signature_info.plugin_hash {
            error!("插件哈希值不匹配");
            return Ok(false);
        }

        // 验证签名
        let verified = self.verify_data(&signature_info.plugin_hash, &signature_info.signature)?;
        if !verified {
            error!("签名验证失败");
            return Ok(false);
        }

        info!("插件签名验证成功: {:?}", plugin_path);
        Ok(true)
    }

    /// 计算插件哈希值
    fn calculate_plugin_hash<P: AsRef<Path>>(&self, plugin_path: P) -> Result<Vec<u8>, String> {
        let plugin_path = plugin_path.as_ref();

        let mut file = File::open(plugin_path).map_err(|e| format!("打开插件文件失败: {}", e))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0; 4096];

        loop {
            let bytes_read = file
                .read(&mut buffer)
                .map_err(|e| format!("读取插件文件失败: {}", e))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let hash = hasher.finalize();
        Ok(hash.to_vec())
    }

    /// 签名数据
    fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let private_key_bytes = self
            .private_key
            .as_ref()
            .ok_or_else(|| "私钥未加载".to_string())?;

        let private_key = RsaPrivateKey::from_pkcs8_der(private_key_bytes)
            .map_err(|e| format!("解析私钥失败: {}", e))?;

        let mut rng = OsRng;
        let signature = private_key
            .sign_with_rng(&mut rng, Pkcs1v15Sign::new::<Sha256>(), data)
            .map_err(|e| format!("签名失败: {}", e))?;

        Ok(signature.to_vec())
    }

    /// 验证签名
    fn verify_data(&self, data: &[u8], signature: &[u8]) -> Result<bool, String> {
        let public_key_bytes = self
            .public_key
            .as_ref()
            .ok_or_else(|| "公钥未加载".to_string())?;

        let public_key_pem = String::from_utf8(public_key_bytes.clone())
            .map_err(|e| format!("公钥编码转换失败: {}", e))?;

        let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)
            .map_err(|e| format!("解析公钥失败: {}", e))?;

        public_key
            .verify(Pkcs1v15Sign::new::<Sha256>(), data, signature)
            .map_err(|e| format!("验证签名失败: {}", e))?;

        Ok(true)
    }

    /// 获取可信白名单
    pub fn get_whitelist(&self) -> &TrustedWhitelist {
        &self.whitelist
    }
}

impl Default for PluginSigner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_signer() {
        let signer = PluginSigner::new();

        // 测试白名单功能，不涉及文件系统操作
        let whitelist = signer.get_whitelist();

        // 直接测试 is_trusted 方法的逻辑，不添加条目
        // 测试未知签名者
        assert!(!whitelist.is_trusted("unknown_signer"));

        // 测试签名者验证逻辑
        assert!(!whitelist.is_trusted(""));
    }
}

