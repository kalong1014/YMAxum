// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! CBOR serialization implementation

use crate::core::serialization::error::SerializationError;
use crate::core::serialization::{Serializer, Deserializer};
use base64::engine::{general_purpose::STANDARD as BASE64_STANDARD, Engine};
use serde_cbor;

/// CBOR serializer
#[derive(Debug, Clone)]
pub struct CborSerializer {
    /// Compact mode
    compact: bool,
}

impl CborSerializer {
    /// Create new CBOR serializer
    pub fn new() -> Self {
        Self {
            compact: true,
        }
    }

    /// Set compact mode
    pub fn set_compact(&mut self, compact: bool) {
        self.compact = compact;
    }
}

impl Serializer for CborSerializer {
    fn serialize<T: serde::Serialize>(&self, value: &T) -> Result<Vec<u8>, SerializationError> {
        serde_cbor::to_vec(value).map_err(|e| SerializationError::FormatError(e.to_string()))
    }

    fn serialize_to_string<T: serde::Serialize>(&self, value: &T) -> Result<String, SerializationError> {
        let bytes = self.serialize(value)?;
        Ok(BASE64_STANDARD.encode(&bytes))
    }
}

/// CBOR deserializer
#[derive(Debug, Clone)]
pub struct CborDeserializer {
    /// Strict mode
    strict: bool,
}

impl CborDeserializer {
    /// Create new CBOR deserializer
    pub fn new() -> Self {
        Self {
            strict: true,
        }
    }

    /// Create new lenient CBOR deserializer
    pub fn new_lenient() -> Self {
        Self {
            strict: false,
        }
    }

    /// Set strict mode
    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }
}

impl Deserializer for CborDeserializer {
    fn deserialize<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, SerializationError> {
        serde_cbor::from_slice(bytes).map_err(|e| SerializationError::FormatError(e.to_string()))
    }

    fn deserialize_from_string<T: serde::de::DeserializeOwned>(&self, s: &str) -> Result<T, SerializationError> {
        let bytes = BASE64_STANDARD.decode(s).map_err(|e| SerializationError::InvalidData(e.to_string()))?;
        self.deserialize(&bytes)
    }
}
