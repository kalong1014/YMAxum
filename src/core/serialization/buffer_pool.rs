// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Buffer pool implementation for serialization
//! Provides shared buffer reuse to reduce memory allocation overhead

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;
use lazy_static::lazy_static;

/// Thread-safe buffer pool for reusing Vec<u8> buffers
#[derive(Debug, Clone)]
pub struct BufferPool {
    small_pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    medium_pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    large_pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    max_size_per_pool: usize,
    small_threshold: usize,
    medium_threshold: usize,
    max_buffer_capacity: usize,
}

impl BufferPool {


    /// Get a buffer from the appropriate pool based on estimated size
    pub fn get(&self, estimated_size: usize) -> Vec<u8> {
        let pool = match estimated_size {
            s if s <= self.small_threshold => &self.small_pool,
            m if m <= self.medium_threshold => &self.medium_pool,
            _ => &self.large_pool,
        };
        pool.lock().unwrap().pop_front().unwrap_or_else(|| Vec::with_capacity(estimated_size))
    }

    /// Return a buffer to the appropriate pool for reuse
    pub fn put(&self, mut buffer: Vec<u8>) {
        let capacity = buffer.capacity();
        let pool = match capacity {
            s if s <= self.small_threshold => &self.small_pool,
            m if m <= self.medium_threshold => &self.medium_pool,
            _ if capacity <= self.max_buffer_capacity => &self.large_pool,
            _ => return, // Too large, don't pool
        };

        let mut pool_lock = pool.lock().unwrap();
        if pool_lock.len() < self.max_size_per_pool {
            // Clear the buffer
            buffer.clear();
            pool_lock.push_back(buffer);
        }
    }

    /// Get the current size of all pools
    pub fn size(&self) -> usize {
        self.small_pool.lock().unwrap().len() + 
        self.medium_pool.lock().unwrap().len() + 
        self.large_pool.lock().unwrap().len()
    }

    /// Create a new buffer pool with default values
    pub fn new() -> Self {
        Self::new_with_capacity(128, 1024, 65536, 1024 * 1024)
    }

    /// Create a new buffer pool with specified capacity
    pub fn new_with_capacity(max_size_per_pool: usize, small_threshold: usize, medium_threshold: usize, max_buffer_capacity: usize) -> Self {
        Self {
            small_pool: Arc::new(Mutex::new(VecDeque::with_capacity(max_size_per_pool))),
            medium_pool: Arc::new(Mutex::new(VecDeque::with_capacity(max_size_per_pool))),
            large_pool: Arc::new(Mutex::new(VecDeque::with_capacity(max_size_per_pool))),
            max_size_per_pool,
            small_threshold,
            medium_threshold,
            max_buffer_capacity,
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_BUFFER_POOL: BufferPool = BufferPool::new();
}

/// Get the global buffer pool
pub fn get_global_buffer_pool() -> &'static BufferPool {
    &GLOBAL_BUFFER_POOL
}

/// Get a buffer with estimated size
pub fn get_buffer(estimated_size: usize) -> Vec<u8> {
    get_global_buffer_pool().get(estimated_size)
}

/// Return a buffer to the pool
pub fn return_buffer(buffer: Vec<u8>) {
    get_global_buffer_pool().put(buffer);
}
