// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! High-performance serialization library
//! Provides optimized serialization and deserialization for various formats

pub mod json;
pub mod msgpack;
pub mod bincode;
pub mod cbor;
pub mod error;
pub mod buffer_pool;
pub mod large_file;
pub mod auto_format;

/// Serialization format
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationFormat {
    /// JSON format
    Json,
    /// MessagePack format
    MessagePack,
    /// Bincode format
    Bincode,
    /// CBOR format
    Cbor,
}

/// Serializer trait
pub trait Serializer {
    /// Serialize value to bytes
    fn serialize<T: serde::Serialize>(&self, value: &T) -> Result<Vec<u8>, error::SerializationError>;
    
    /// Serialize value to string
    fn serialize_to_string<T: serde::Serialize>(&self, value: &T) -> Result<String, error::SerializationError>;
}

/// Deserializer trait
pub trait Deserializer {
    /// Deserialize from bytes
    fn deserialize<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, error::SerializationError>;
    
    /// Deserialize from string
    fn deserialize_from_string<T: serde::de::DeserializeOwned>(&self, s: &str) -> Result<T, error::SerializationError>;
}

/// Serialize value to specific format
pub fn serialize<T: serde::Serialize>(value: &T, format: SerializationFormat) -> Result<Vec<u8>, error::SerializationError> {
    match format {
        SerializationFormat::Json => {
            let serializer = json::JsonSerializer::new();
            serializer.serialize(value)
        }
        SerializationFormat::MessagePack => {
            let serializer = msgpack::MessagePackSerializer::new();
            serializer.serialize(value)
        }
        SerializationFormat::Bincode => {
            let serializer = bincode::BincodeSerializer::new();
            serializer.serialize(value)
        }
        SerializationFormat::Cbor => {
            let serializer = cbor::CborSerializer::new();
            serializer.serialize(value)
        }
    }
}

/// Deserialize from specific format
pub fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8], format: SerializationFormat) -> Result<T, error::SerializationError> {
    match format {
        SerializationFormat::Json => {
            let deserializer = json::JsonDeserializer::new();
            deserializer.deserialize(bytes)
        }
        SerializationFormat::MessagePack => {
            let deserializer = msgpack::MessagePackDeserializer::new();
            deserializer.deserialize(bytes)
        }
        SerializationFormat::Bincode => {
            let deserializer = bincode::BincodeDeserializer::new();
            deserializer.deserialize(bytes)
        }
        SerializationFormat::Cbor => {
            let deserializer = cbor::CborDeserializer::new();
            deserializer.deserialize(bytes)
        }
    }
}

/// Serialize with automatic format selection
pub use auto_format::auto_serialize;

/// Deserialize with specified format
pub use auto_format::auto_deserialize;
