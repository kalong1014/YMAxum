// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use criterion::{criterion_group, criterion_main, Criterion};
use ymaxum::core::serialization::{serialize, deserialize, SerializationFormat};
use serde::{Deserialize, Serialize};

// Large test struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LargeStruct {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub values: Vec<f64>,
    pub nested: NestedStruct,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NestedStruct {
    pub id: u64,
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub value: f64,
}

fn create_large_struct() -> LargeStruct {
    let items = (0..100).map(|i| Item {
        id: i,
        name: format!("Item {}", i),
        value: i as f64,
    }).collect();
    
    let nested = NestedStruct {
        id: 1,
        name: "Nested Struct".to_string(),
        items,
    };
    
    LargeStruct {
        id: 1,
        name: "Large Struct".to_string(),
        description: "This is a large test struct for serialization benchmarking".to_string(),
        tags: (0..50).map(|i| format!("tag-{}", i)).collect(),
        values: (0..1000).map(|i| i as f64).collect(),
        nested,
    }
}

fn bench_json_serialization(c: &mut Criterion) {
    let data = create_large_struct();
    
    c.bench_function("json_serialize", |b| {
        b.iter(|| {
            serialize(&data, SerializationFormat::Json).unwrap();
        });
    });
    
    let serialized = serialize(&data, SerializationFormat::Json).unwrap();
    c.bench_function("json_deserialize", |b| {
        b.iter(|| {
            deserialize::<LargeStruct>(&serialized, SerializationFormat::Json).unwrap();
        });
    });
}

fn bench_msgpack_serialization(c: &mut Criterion) {
    let data = create_large_struct();
    
    c.bench_function("msgpack_serialize", |b| {
        b.iter(|| {
            serialize(&data, SerializationFormat::MessagePack).unwrap();
        });
    });
    
    let serialized = serialize(&data, SerializationFormat::MessagePack).unwrap();
    c.bench_function("msgpack_deserialize", |b| {
        b.iter(|| {
            deserialize::<LargeStruct>(&serialized, SerializationFormat::MessagePack).unwrap();
        });
    });
}

fn bench_bincode_serialization(c: &mut Criterion) {
    let data = create_large_struct();
    
    c.bench_function("bincode_serialize", |b| {
        b.iter(|| {
            serialize(&data, SerializationFormat::Bincode).unwrap();
        });
    });
    
    let serialized = serialize(&data, SerializationFormat::Bincode).unwrap();
    c.bench_function("bincode_deserialize", |b| {
        b.iter(|| {
            deserialize::<LargeStruct>(&serialized, SerializationFormat::Bincode).unwrap();
        });
    });
}

fn bench_cbor_serialization(c: &mut Criterion) {
    let data = create_large_struct();
    
    c.bench_function("cbor_serialize", |b| {
        b.iter(|| {
            serialize(&data, SerializationFormat::Cbor).unwrap();
        });
    });
    
    let serialized = serialize(&data, SerializationFormat::Cbor).unwrap();
    c.bench_function("cbor_deserialize", |b| {
        b.iter(|| {
            deserialize::<LargeStruct>(&serialized, SerializationFormat::Cbor).unwrap();
        });
    });
}

criterion_group!(benches, bench_json_serialization, bench_msgpack_serialization, bench_bincode_serialization, bench_cbor_serialization);
criterion_main!(benches);
