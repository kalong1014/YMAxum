use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::core::state::AppState;
use crate::core::database::DatabaseManager;
use crate::core::cache::CacheManager;

/// 数据库连接性能测试
fn bench_database_connection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let app_state = Arc::new(AppState::new());
    let db_manager = DatabaseManager::new(app_state.clone());

    c.bench_function("database_connection_init", |b| {
        b.to_async(&rt).iter(|| async {
            // 注意：这里使用SQLite内存数据库进行测试，避免网络延迟影响
            db_manager.init_sqlite_pool("sqlite::memory:").await.unwrap();
        });
    });
}

/// 缓存操作性能测试
fn bench_cache_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let app_state = Arc::new(AppState::new());
    let cache_manager = CacheManager::new(app_state.clone());

    c.bench_function("cache_set_get", |b| {
        b.to_async(&rt).iter(|| async {
            // 设置缓存
            cache_manager.set("test_key", "test_value", Some(60)).await.unwrap();
            // 获取缓存
            cache_manager.get("test_key").await.unwrap();
        });
    });

    c.bench_function("cache_batch_operations", |b| {
        b.to_async(&rt).iter(|| async {
            // 准备测试数据
            let test_data = vec![
                ("key1", "value1", Some(60)),
                ("key2", "value2", Some(60)),
                ("key3", "value3", Some(60)),
                ("key4", "value4", Some(60)),
                ("key5", "value5", Some(60)),
            ];
            
            // 批量预热缓存
            cache_manager.warm_up(&test_data).await.unwrap();
            
            // 批量获取缓存
            let keys = vec!["key1", "key2", "key3", "key4", "key5"];
            cache_manager.get_batch(&keys).await.unwrap();
            
            // 批量删除缓存
            cache_manager.delete_batch(&keys).await.unwrap();
        });
    });
}

/// 限流性能测试
fn bench_rate_limiter(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let limiter = crate::core::middleware::RateLimiter::new_with_rate(1000, 100);

    c.bench_function("rate_limiter_acquire", |b| {
        b.to_async(&rt).iter(|| async {
            // 测试令牌获取
            limiter.try_acquire();
        });
    });
}

criterion_group!(benches, bench_database_connection, bench_cache_operations, bench_rate_limiter);
criterion_main!(benches);
