//! 序列化库性能测试
//! 测试各种序列化格式的性能表现

use ymaxum::core::serialization::{SerializationFormat, serialize, deserialize, auto_serialize};
use serde::{Serialize, Deserialize};
use std::time::Instant;

use std::io::Write;
use tempfile;

/// 测试数据结构
#[derive(Debug, Serialize, Deserialize)]
struct TestData {
    id: u32,
    name: String,
    values: Vec<f64>,
    nested: NestedData,
}

#[derive(Debug, Serialize, Deserialize)]
struct NestedData {
    items: Vec<String>,
    flags: Vec<bool>,
    map: std::collections::HashMap<String, u32>,
}

/// 生成测试数据
fn generate_test_data(size: usize) -> TestData {
    let mut values = Vec::with_capacity(size);
    for i in 0..size {
        values.push(i as f64);
    }

    let mut items = Vec::with_capacity(size / 10);
    for i in 0..(size / 10) {
        items.push(format!("item_{}", i));
    }

    let mut flags = Vec::with_capacity(size / 10);
    for i in 0..(size / 10) {
        flags.push(i % 2 == 0);
    }

    let mut map = std::collections::HashMap::new();
    for i in 0..(size / 20) {
        map.insert(format!("key_{}", i), i as u32);
    }

    TestData {
        id: size as u32,
        name: format!("test_data_{}", size),
        values,
        nested: NestedData {
            items,
            flags,
            map,
        },
    }
}

/// 测试序列化性能
fn test_serialization_performance(data: &TestData) {
    println!("=== 序列化性能测试 ===");
    
    // 测试 JSON 序列化
    let start = Instant::now();
    let json_bytes = serialize(data, SerializationFormat::Json).unwrap();
    let json_time = start.elapsed();
    println!("JSON 序列化: {:?}, 大小: {} bytes", json_time, json_bytes.len());

    // 测试 MessagePack 序列化
    let start = Instant::now();
    let msgpack_bytes = serialize(data, SerializationFormat::MessagePack).unwrap();
    let msgpack_time = start.elapsed();
    println!("MessagePack 序列化: {:?}, 大小: {} bytes", msgpack_time, msgpack_bytes.len());

    // 测试 Bincode 序列化
    let start = Instant::now();
    let bincode_bytes = serialize(data, SerializationFormat::Bincode).unwrap();
    let bincode_time = start.elapsed();
    println!("Bincode 序列化: {:?}, 大小: {} bytes", bincode_time, bincode_bytes.len());

    // 测试自动格式选择
    let start = Instant::now();
    let (auto_bytes, format) = auto_serialize(data).unwrap();
    let auto_time = start.elapsed();
    println!("自动格式选择: {:?}, 格式: {:?}, 大小: {} bytes", auto_time, format, auto_bytes.len());

    println!();
}

/// 测试反序列化性能
fn test_deserialization_performance(data: &TestData) {
    println!("=== 反序列化性能测试 ===");
    
    // 准备序列化数据
    let json_bytes = serialize(data, SerializationFormat::Json).unwrap();
    let msgpack_bytes = serialize(data, SerializationFormat::MessagePack).unwrap();
    let bincode_bytes = serialize(data, SerializationFormat::Bincode).unwrap();

    // 测试 JSON 反序列化
    let start = Instant::now();
    let _: TestData = deserialize(&json_bytes, SerializationFormat::Json).unwrap();
    let json_time = start.elapsed();
    println!("JSON 反序列化: {:?}", json_time);

    // 测试 MessagePack 反序列化
    let start = Instant::now();
    let _: TestData = deserialize(&msgpack_bytes, SerializationFormat::MessagePack).unwrap();
    let msgpack_time = start.elapsed();
    println!("MessagePack 反序列化: {:?}", msgpack_time);

    // 测试 Bincode 反序列化
    let start = Instant::now();
    let _: TestData = deserialize(&bincode_bytes, SerializationFormat::Bincode).unwrap();
    let bincode_time = start.elapsed();
    println!("Bincode 反序列化: {:?}", bincode_time);

    println!();
}

/// 测试大文件处理性能
fn test_large_file_performance() {
    println!("=== 大文件处理性能测试 ===");
    
    // 生成大型测试数据
    let large_data = generate_test_data(100000);
    
    // 测试序列化到文件
    let start = Instant::now();
    let temp_file = tempfile::tempfile().unwrap();
    let mut writer = std::io::BufWriter::new(temp_file);
    let json_bytes = serialize(&large_data, SerializationFormat::Json).unwrap();
    writer.write_all(&json_bytes).unwrap();
    let serialize_time = start.elapsed();
    println!("大文件序列化: {:?}, 大小: {} bytes", serialize_time, json_bytes.len());

    println!();
}

#[test]
fn test_serialization_performance_small() {
    let data = generate_test_data(100);
    test_serialization_performance(&data);
    test_deserialization_performance(&data);
}

#[test]
fn test_serialization_performance_medium() {
    let data = generate_test_data(1000);
    test_serialization_performance(&data);
    test_deserialization_performance(&data);
}

#[test]
fn test_serialization_performance_large() {
    let data = generate_test_data(10000);
    test_serialization_performance(&data);
    test_deserialization_performance(&data);
}

#[test]
fn test_large_file_processing() {
    test_large_file_performance();
}
