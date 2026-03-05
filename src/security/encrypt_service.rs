//! 加密服务模块
//! 提供AES-256 GCM加密功能，用于敏感数据加密存储
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use std::sync::Arc;
use thiserror::Error;

/// 加密错误
#[derive(Error, Debug)]
pub enum EncryptError {
    #[error("加密失败: {0}")]
    EncryptFailed(String),
    #[error("解密失败: {0}")]
    DecryptFailed(String),
    #[error("密钥无效")]
    InvalidKey,
    #[error("数据无效")]
    InvalidData,
}

/// 加密服务
pub struct EncryptService {
    /// 加密密钥 (32字节，256位)
    key: [u8; 32],
    /// 加密算法实例
    cipher: Aes256Gcm,
}

impl EncryptService {
    /// 创建新的加密服务实例
    pub fn new(key: [u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(&key.into());
        Self { key, cipher }
    }

    /// 从字符串密钥创建加密服务实例
    pub fn from_string(key_str: &str) -> Result<Self, EncryptError> {
        let key_bytes = key_str.as_bytes();
        let mut key = [0u8; 32];

        let len = key_bytes.len().min(32);
        key[..len].copy_from_slice(&key_bytes[..len]);

        Ok(Self::new(key))
    }

    /// 加密数据
    pub fn encrypt(&self, plaintext: &str) -> Result<String, EncryptError> {
        use rand::RngCore;
        use rand::rngs::OsRng;

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| EncryptError::EncryptFailed(e.to_string()))?;

        // 将nonce和密文一起编码
        let mut combined = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);

        let result = STANDARD.encode(combined);
        Ok(result)
    }

    /// 解密数据
    pub fn decrypt(&self, ciphertext: &str) -> Result<String, EncryptError> {
        let combined = STANDARD
            .decode(ciphertext)
            .map_err(|e| EncryptError::DecryptFailed(e.to_string()))?;

        if combined.len() < 12 {
            return Err(EncryptError::InvalidData);
        }

        let nonce_bytes = &combined[..12];
        let ciphertext_bytes = &combined[12..];
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext_bytes.as_ref())
            .map_err(|e| EncryptError::DecryptFailed(e.to_string()))?;

        String::from_utf8(plaintext).map_err(|e| EncryptError::DecryptFailed(e.to_string()))
    }

    /// 获取加密密钥
    pub fn key(&self) -> &[u8; 32] {
        &self.key
    }
}

impl Clone for EncryptService {
    fn clone(&self) -> Self {
        Self::new(self.key)
    }
}

/// 加密服务管理器（单例模式）
pub struct EncryptServiceManager {
    /// 加密服务实例
    service: Arc<EncryptService>,
}

impl EncryptServiceManager {
    /// 创建新的加密服务管理器实例
    pub fn new(key: [u8; 32]) -> Self {
        let service = Arc::new(EncryptService::new(key));
        Self { service }
    }

    /// 从字符串密钥创建加密服务管理器实例
    pub fn from_string(key_str: &str) -> Result<Self, EncryptError> {
        let service = Arc::new(EncryptService::from_string(key_str)?);
        Ok(Self { service })
    }

    /// 获取加密服务实例
    pub fn service(&self) -> Arc<EncryptService> {
        self.service.clone()
    }
}

impl Clone for EncryptServiceManager {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let service = EncryptService::new(key);

        let plaintext = "hello world";
        let encrypted = service.encrypt(plaintext).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_long_text() {
        let key = [0u8; 32];
        let service = EncryptService::new(key);

        let plaintext = "This is a long text that should be encrypted and decrypted correctly.";
        let encrypted = service.encrypt(plaintext).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_special_chars() {
        let key = [0u8; 32];
        let service = EncryptService::new(key);

        let plaintext = "特殊字符!@#$%^&*()_+-={}[]|\\:;\"'<>,.?/~`";
        let encrypted = service.encrypt(plaintext).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_from_string() {
        let key_str = "my-secret-key-32-bytes-long!!";
        let service = EncryptService::from_string(key_str).unwrap();

        let plaintext = "hello world";
        let encrypted = service.encrypt(plaintext).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_service_manager() {
        let key = [0u8; 32];
        let manager = EncryptServiceManager::new(key);
        let service = manager.service();

        let plaintext = "hello world";
        let encrypted = service.encrypt(plaintext).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }
}
