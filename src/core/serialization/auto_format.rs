// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Automatic serialization format selection
//! Provides intelligent format selection based on data characteristics

use crate::core::serialization::{SerializationFormat, serialize, deserialize};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::any::TypeId;

/// Automatic format selector
pub struct AutoFormatSelector {
    /// Threshold for choosing binary formats (in bytes)
    binary_threshold: usize,
    /// Threshold for choosing compact formats (in bytes)
    compact_threshold: usize,
}

impl Default for AutoFormatSelector {
    fn default() -> Self {
        Self {
            binary_threshold: 1024,      // 1KB
            compact_threshold: 65536,    // 64KB
        }
    }
}

impl AutoFormatSelector {
    /// Create new auto format selector
    pub fn new(binary_threshold: usize, compact_threshold: usize) -> Self {
        Self {
            binary_threshold,
            compact_threshold,
        }
    }
    
    /// Select format based on data size and type
    pub fn select_format<T: Serialize + std::fmt::Debug + 'static>(&self, data: &T) -> SerializationFormat {
        // 基于数据类型和大小选择最合适的格式
        let estimated_size = self.estimate_size(data);
        let data_type = TypeId::of::<T>();
        
        // 对于某些特定类型，直接选择合适的格式
        if self.is_binary_like_type(data_type) {
            return SerializationFormat::MessagePack;
        }
        
        if self.is_complex_struct_type(data_type) {
            // 复杂结构体优先考虑二进制格式
            if estimated_size > self.binary_threshold {
                return SerializationFormat::Bincode;
            }
        }
        
        // 基于大小选择格式
        if estimated_size < self.binary_threshold {
            SerializationFormat::Json
        } else if estimated_size < self.compact_threshold {
            // 中等大小数据，使用MessagePack平衡大小和速度
            SerializationFormat::MessagePack
        } else {
            // 大数据，使用Bincode获得最大压缩
            SerializationFormat::Bincode
        }
    }
    
    /// 估计数据大小（改进的启发式方法）
    fn estimate_size<T: Serialize + std::fmt::Debug + 'static>(&self, data: &T) -> usize {
        // 对于简单类型，使用更准确的估计
        // 注意：由于类型擦除，这里无法直接使用downcast_ref
        // 使用Debug表示的长度作为估计
        format!("{:?}", data).len()
    }
    
    /// 检查是否为二进制类类型
    fn is_binary_like_type(&self, type_id: TypeId) -> bool {
        // 检测常见的二进制类型
        type_id == TypeId::of::<Vec<u8>>() || 
        type_id == TypeId::of::<&[u8]>()
    }
    
    /// 检查是否为复杂结构体类型
    fn is_complex_struct_type(&self, _type_id: TypeId) -> bool {
        // 暂时返回false，后续可以添加更复杂的类型检测逻辑
        false
    }
}

/// Serialize with automatic format selection
pub fn auto_serialize<T: Serialize + std::fmt::Debug + 'static>(value: &T) -> Result<(Vec<u8>, SerializationFormat), crate::core::serialization::error::SerializationError> {
    let selector = AutoFormatSelector::default();
    let format = selector.select_format(value);
    let bytes = serialize(value, format.clone())?;
    Ok((bytes, format))
}

/// Deserialize with specified format
pub fn auto_deserialize<T: DeserializeOwned>(bytes: &[u8], format: SerializationFormat) -> Result<T, crate::core::serialization::error::SerializationError> {
    deserialize(bytes, format)
}

/// Smart serializer that automatically selects format
pub struct SmartSerializer {
    _selector: AutoFormatSelector,
}

impl SmartSerializer {
    /// Create new smart serializer
    pub fn new() -> Self {
        Self {
            _selector: AutoFormatSelector::default(),
        }
    }
    
    /// Serialize with automatic format selection
    pub fn serialize<T: Serialize + std::fmt::Debug + 'static>(&self, value: &T) -> Result<(Vec<u8>, SerializationFormat), crate::core::serialization::error::SerializationError> {
        auto_serialize(value)
    }
}
