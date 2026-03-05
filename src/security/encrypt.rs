//! 加密与脱敏模块
//! 提供AES-256 GCM加密和敏感数据脱敏功能
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

/// 脱敏配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesensitizationConfig {
    pub phone: bool,
    pub id_card: bool,
    pub bank_card: bool,
    pub email: bool,
}

impl Default for DesensitizationConfig {
    fn default() -> Self {
        Self {
            phone: true,
            id_card: true,
            bank_card: true,
            email: true,
        }
    }
}

/// 自动加密配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoEncryptConfig {
    pub phone: bool,
    pub id_card: bool,
    pub bank_card: bool,
    pub email: bool,
    pub password: bool,
}

impl Default for AutoEncryptConfig {
    fn default() -> Self {
        Self {
            phone: true,
            id_card: true,
            bank_card: true,
            email: true,
            password: true,
        }
    }
}

/// 加密配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptConfig {
    pub key: Vec<u8>,
    pub desensitization: DesensitizationConfig,
    pub auto_encrypt: AutoEncryptConfig,
}

impl Default for EncryptConfig {
    fn default() -> Self {
        Self {
            key: EncryptionService::generate_key().to_vec(),
            desensitization: DesensitizationConfig::default(),
            auto_encrypt: AutoEncryptConfig::default(),
        }
    }
}

/// 加密错误类型
#[derive(Error, Debug)]
pub enum EncryptError {
    #[error("Invalid key length: expected 32 bytes, got {0}")]
    InvalidKeyLength(usize),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Nonce generation failed: {0}")]
    NonceGenerationFailed(String),
}

/// 加密服务
#[derive(Clone)]
pub struct EncryptionService {
    _config: Arc<EncryptConfig>,
    cipher: Aes256Gcm,
    rng: OsRng,
}

impl EncryptionService {
    pub fn new(config: EncryptConfig) -> Result<Self, EncryptError> {
        if config.key.len() != 32 {
            return Err(EncryptError::InvalidKeyLength(config.key.len()));
        }

        let key_array: [u8; 32] = config.key.clone().try_into().unwrap();
        let cipher = Aes256Gcm::new(&key_array.into());

        Ok(Self {
            _config: Arc::new(config),
            cipher,
            rng: OsRng,
        })
    }

    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut key);
        key
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptError> {
        let mut nonce_bytes = [0u8; 12];
        self.rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let cipher = self
            .cipher
            .encrypt(nonce, data.as_ref())
            .map_err(|e: aes_gcm::aead::Error| EncryptError::EncryptionFailed(e.to_string()))?;

        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&cipher);

        Ok(result)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, EncryptError> {
        if ciphertext.len() < 28 {
            return Err(EncryptError::DecryptionFailed(
                "Ciphertext too short".to_string(),
            ));
        }

        let nonce_bytes = &ciphertext[..12];
        let cipher = &ciphertext[12..];
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, cipher.as_ref())
            .map_err(|e| EncryptError::DecryptionFailed(e.to_string()))?;

        Ok(plaintext)
    }

    pub fn encrypt_string(&mut self, data: &str) -> Result<String, EncryptError> {
        let encrypted = self.encrypt(data.as_bytes())?;
        Ok(general_purpose::STANDARD.encode(encrypted))
    }

    pub fn decrypt_string(&self, data: &str) -> Result<String, EncryptError> {
        let decoded = general_purpose::STANDARD
            .decode(data)
            .map_err(|e| EncryptError::DecryptionFailed(e.to_string()))?;
        let decrypted = self.decrypt(&decoded)?;
        String::from_utf8(decrypted).map_err(|e| EncryptError::DecryptionFailed(e.to_string()))
    }
}

/// 脱敏服务
#[derive(Clone)]
pub struct DesensitizationService {
    config: Arc<DesensitizationConfig>,
}

impl DesensitizationService {
    pub fn new(config: DesensitizationConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn desensitize_phone(&self, phone: &str) -> String {
        if !self.config.phone || phone.len() < 7 {
            return phone.to_string();
        }

        let mut result = phone.to_string();
        for i in 3..(result.len() - 4) {
            result.replace_range(i..i + 1, "*");
        }
        result
    }

    pub fn desensitize_id_card(&self, id_card: &str) -> String {
        if !self.config.id_card || id_card.len() < 10 {
            return id_card.to_string();
        }

        let mut result = id_card.to_string();
        for i in 6..(result.len() - 4) {
            result.replace_range(i..i + 1, "*");
        }
        result
    }

    pub fn desensitize_bank_card(&self, bank_card: &str) -> String {
        if !self.config.bank_card || bank_card.len() < 10 {
            return bank_card.to_string();
        }

        let mut result = bank_card.to_string();
        for i in 6..(result.len() - 4) {
            result.replace_range(i..i + 1, "*");
        }
        result
    }

    pub fn desensitize_email(&self, email: &str) -> String {
        if !self.config.email {
            return email.to_string();
        }

        if let Some((prefix, domain)) = email.split_once('@') {
            if prefix.len() <= 2 {
                format!("{}@{}", prefix, domain)
            } else {
                let masked_prefix = format!("{}{}", &prefix[..2], "*".repeat(prefix.len() - 2));
                format!("{}@{}", masked_prefix, domain)
            }
        } else {
            email.to_string()
        }
    }

    pub fn auto_desensitize(&self, data_type: &str, data: &str) -> String {
        match data_type.to_lowercase().as_str() {
            "phone" => self.desensitize_phone(data),
            "id_card" => self.desensitize_id_card(data),
            "bank_card" => self.desensitize_bank_card(data),
            "email" => self.desensitize_email(data),
            _ => data.to_string(),
        }
    }
}
