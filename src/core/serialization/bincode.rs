// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Bincode serialization implementation

use crate::core::serialization::error::SerializationError;
use crate::core::serialization::{Serializer, Deserializer};
use base64::engine::{general_purpose::STANDARD as BASE64_STANDARD, Engine};
use ::serde::Serialize;
use bincode::config;

/// Bincode serializer
#[derive(Clone)]
pub struct BincodeSerializer {
    config: config::Configuration,
}

impl BincodeSerializer {
    /// Create new Bincode serializer
    pub fn new() -> Self {
        Self {
            config: config::standard()
                .with_fixed_int_encoding()
                .with_little_endian()
                .with_variable_int_encoding(),
        }
    }

    /// Create new Bincode serializer with custom configuration
    pub fn with_config(config: config::Configuration) -> Self {
        Self {
            config,
        }
    }
}

impl Serializer for BincodeSerializer {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, SerializationError> {
        bincode::serde::encode_to_vec(value, self.config)
            .map_err(|e| SerializationError::FormatError(e.to_string()))
    }

    fn serialize_to_string<T: Serialize>(&self, value: &T) -> Result<String, SerializationError> {
        let bytes = self.serialize(value)?;
        Ok(BASE64_STANDARD.encode(&bytes))
    }
}

/// Bincode deserializer
#[derive(Clone)]
pub struct BincodeDeserializer {
    config: config::Configuration,
}

impl BincodeDeserializer {
    /// Create new Bincode deserializer
    pub fn new() -> Self {
        Self {
            config: config::standard()
                .with_fixed_int_encoding()
                .with_little_endian()
                .with_variable_int_encoding(),
        }
    }

    /// Create new Bincode deserializer with custom configuration
    pub fn with_config(config: config::Configuration) -> Self {
        Self {
            config,
        }
    }
}

impl Deserializer for BincodeDeserializer {
    fn deserialize<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, SerializationError> {
        bincode::serde::decode_from_slice(bytes, self.config)
            .map(|(v, _)| v)
            .map_err(|e| SerializationError::FormatError(e.to_string()))
    }

    fn deserialize_from_string<T: serde::de::DeserializeOwned>(&self, s: &str) -> Result<T, SerializationError> {
        let bytes = BASE64_STANDARD.decode(s).map_err(|e| SerializationError::InvalidData(e.to_string()))?;
        self.deserialize(&bytes)
    }
}
