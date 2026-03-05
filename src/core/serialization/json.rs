// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! JSON serialization implementation

use crate::core::serialization::error::SerializationError;
use crate::core::serialization::{Serializer, Deserializer};
use simd_json;

/// JSON serializer
#[derive(Debug, Clone)]
pub struct JsonSerializer {
    /// Pretty print
    pretty: bool,
}

impl JsonSerializer {
    /// Create new JSON serializer
    pub fn new() -> Self {
        Self {
            pretty: false,
        }
    }

    /// Create new pretty JSON serializer
    pub fn new_pretty() -> Self {
        Self {
            pretty: true,
        }
    }

    /// Set pretty print
    pub fn set_pretty(&mut self, pretty: bool) {
        self.pretty = pretty;
    }
}

impl Serializer for JsonSerializer {
    fn serialize<T: serde::Serialize>(&self, value: &T) -> Result<Vec<u8>, SerializationError> {
        if self.pretty {
            serde_json::to_vec_pretty(value).map_err(|e| e.into())
        } else {
            simd_json::to_vec(value).map_err(|e| e.into())
        }
    }

    fn serialize_to_string<T: serde::Serialize>(&self, value: &T) -> Result<String, SerializationError> {
        if self.pretty {
            serde_json::to_string_pretty(value).map_err(|e| e.into())
        } else {
            simd_json::to_string(value).map_err(|e| e.into())
        }
    }
}

/// JSON deserializer
#[derive(Debug, Clone)]
pub struct JsonDeserializer {
    /// Strict mode
    strict: bool,
}

impl JsonDeserializer {
    /// Create new JSON deserializer
    pub fn new() -> Self {
        Self {
            strict: true,
        }
    }

    /// Create new lenient JSON deserializer
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

impl Deserializer for JsonDeserializer {
    fn deserialize<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, SerializationError> {
        // 使用simd_json进行反序列化，提高性能
        // 需要先复制字节数组以获得可变引用
        let mut bytes_copy = bytes.to_vec();
        simd_json::from_slice(&mut bytes_copy).map_err(|e| e.into())
    }

    fn deserialize_from_string<T: serde::de::DeserializeOwned>(&self, s: &str) -> Result<T, SerializationError> {
        // 先将字符串转换为字节数组，再使用simd_json进行反序列化
        let mut bytes = s.as_bytes().to_vec();
        simd_json::from_slice(&mut bytes).map_err(|e| e.into())
    }
}
