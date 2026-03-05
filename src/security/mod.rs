// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Security module
//! Provides security-related functionality including vulnerability scanning and intrusion detection

pub mod models;
pub mod intrusion_detection;
pub mod scanner_core;
pub mod scanner;
pub mod audit;
pub mod vulnerability_db;

#[cfg(test)]
pub mod tests;
pub mod benchmark;

pub use scanner::SecurityScanner;
pub use models::*;
pub use benchmark::run_security_benchmarks;
