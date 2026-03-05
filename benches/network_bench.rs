// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use criterion::{criterion_group, criterion_main, Criterion};
use ymaxum::core::network::client::Client;
use ymaxum::core::network::request::Request;

fn bench_http_requests(c: &mut Criterion) {
    let client = Client::new();
    
    c.bench_function("http_get_request", |b| {
        b.iter(|| {
            let request = Request::new(ymaxum::core::network::request::Method::GET, "/");
            tokio_test::block_on(async {
                client.send(&request, "http://example.com").await.unwrap();
            });
        });
    });
}

fn bench_https_requests(c: &mut Criterion) {
    let client = Client::new();
    
    c.bench_function("https_get_request", |b| {
        b.iter(|| {
            let request = Request::new(ymaxum::core::network::request::Method::GET, "/");
            tokio_test::block_on(async {
                client.send(&request, "https://example.com").await.unwrap();
            });
        });
    });
}

fn bench_connection_pool(c: &mut Criterion) {
    let client = Client::new();
    
    c.bench_function("connection_pool_reuse", |b| {
        b.iter(|| {
            tokio_test::block_on(async {
                for _ in 0..10 {
                    let request = Request::new(ymaxum::core::network::request::Method::GET, "/");
                    client.send(&request, "http://example.com").await.unwrap();
                }
            });
        });
    });
}

criterion_group!(benches, bench_http_requests, bench_https_requests, bench_connection_pool);
criterion_main!(benches);
