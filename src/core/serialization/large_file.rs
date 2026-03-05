// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Large file serialization implementation
//! Provides optimized serialization and deserialization for large files

use crate::core::serialization::error::SerializationError;
use crate::core::serialization::SerializationFormat;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;

/// 优化的缓冲区大小 (8MB)
const BUFFER_SIZE: usize = 8 * 1024 * 1024;

/// Serialize large data to file
pub fn serialize_to_file<T: serde::Serialize>(
    value: &T, 
    path: &Path, 
    format: SerializationFormat
) -> Result<(), SerializationError> {
    let file = File::create(path).map_err(|e| SerializationError::InvalidData(e.to_string()))?;
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, file);
    
    match format {
        SerializationFormat::Json => {
            serde_json::to_writer(&mut writer, value).map_err(|e| e.into())
        }
        SerializationFormat::MessagePack => {
            rmp_serde::encode::write(&mut writer, value).map_err(|e| SerializationError::FormatError(e.to_string()))
        }
        SerializationFormat::Bincode => {
            bincode::serde::encode_into_std_write(value, &mut writer, bincode::config::standard())
                .map(|_| ())
                .map_err(|e| SerializationError::FormatError(e.to_string()))
        }
        SerializationFormat::Cbor => {
            serde_cbor::to_writer(&mut writer, value).map_err(|e| SerializationError::FormatError(e.to_string()))
        }
    }
}

/// Deserialize large data from file
pub fn deserialize_from_file<T: serde::de::DeserializeOwned>(
    path: &Path, 
    format: SerializationFormat
) -> Result<T, SerializationError> {
    let file = File::open(path).map_err(|e| SerializationError::InvalidData(e.to_string()))?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    
    match format {
        SerializationFormat::Json => {
            serde_json::from_reader(&mut reader).map_err(|e| e.into())
        }
        SerializationFormat::MessagePack => {
            rmp_serde::decode::from_read(&mut reader).map_err(|e| SerializationError::FormatError(e.to_string()))
        }
        SerializationFormat::Bincode => {
            bincode::serde::decode_from_std_read(&mut reader, bincode::config::standard()).map_err(|e| SerializationError::FormatError(e.to_string()))
        }
        SerializationFormat::Cbor => {
            serde_cbor::from_reader(&mut reader).map_err(|e| SerializationError::FormatError(e.to_string()))
        }
    }
}

/// Stream-based serializer for large data
pub struct StreamSerializer {
    format: SerializationFormat,
}

impl StreamSerializer {
    /// Create new stream serializer
    pub fn new(format: SerializationFormat) -> Self {
        Self {
            format,
        }
    }
    
    /// Serialize to writer
    pub fn serialize_to_writer<T: serde::Serialize, W: Write>(
        &self, 
        value: &T, 
        writer: &mut W
    ) -> Result<(), SerializationError> {
        // 为不同格式选择最优的序列化方式
        match self.format {
            SerializationFormat::Json => {
                serde_json::to_writer(writer, value).map_err(|e| e.into())
            }
            SerializationFormat::MessagePack => {
                rmp_serde::encode::write(writer, value).map_err(|e| SerializationError::FormatError(e.to_string()))
            }
            SerializationFormat::Bincode => {
                bincode::serde::encode_into_std_write(value, writer, bincode::config::standard())
                    .map(|_| ())
                    .map_err(|e| SerializationError::FormatError(e.to_string()))
            }
            SerializationFormat::Cbor => {
                serde_cbor::to_writer(writer, value).map_err(|e| SerializationError::FormatError(e.to_string()))
            }
        }
    }
    
    /// Serialize to writer with buffered wrapper
    pub fn serialize_to_writer_buffered<T: serde::Serialize, W: Write>(
        &self, 
        value: &T, 
        writer: W
    ) -> Result<(), SerializationError> {
        let mut buffered_writer = BufWriter::with_capacity(BUFFER_SIZE, writer);
        self.serialize_to_writer(value, &mut buffered_writer)
    }
}

/// Stream-based deserializer for large data
pub struct StreamDeserializer {
    format: SerializationFormat,
}

impl StreamDeserializer {
    /// Create new stream deserializer
    pub fn new(format: SerializationFormat) -> Self {
        Self {
            format,
        }
    }
    
    /// Deserialize from reader
    pub fn deserialize_from_reader<T: serde::de::DeserializeOwned, R: Read>(
        &self, 
        reader: &mut R
    ) -> Result<T, SerializationError> {
        match self.format {
            SerializationFormat::Json => {
                serde_json::from_reader(reader).map_err(|e| e.into())
            }
            SerializationFormat::MessagePack => {
                rmp_serde::decode::from_read(reader).map_err(|e| SerializationError::FormatError(e.to_string()))
            }
            SerializationFormat::Bincode => {
                bincode::serde::decode_from_std_read(reader, bincode::config::standard()).map_err(|e| SerializationError::FormatError(e.to_string()))
            }
            SerializationFormat::Cbor => {
                serde_cbor::from_reader(reader).map_err(|e| SerializationError::FormatError(e.to_string()))
            }
        }
    }
    
    /// Deserialize from reader with buffered wrapper
    pub fn deserialize_from_reader_buffered<T: serde::de::DeserializeOwned, R: Read>(
        &self, 
        reader: R
    ) -> Result<T, SerializationError> {
        let mut buffered_reader = BufReader::with_capacity(BUFFER_SIZE, reader);
        self.deserialize_from_reader(&mut buffered_reader)
    }
}
