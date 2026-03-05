// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use criterion::{criterion_group, criterion_main, Criterion};
use ymaxum::core::config::{load_from_file, Config, ConfigSource};
use ymaxum::core::config::loader::TomlConfig;
use std::fs::File;
use std::io::Write;

fn create_test_config() {
    let toml_content = r#"
[server]
port = 8080
host = "localhost"
enable_https = true

[database]
host = "localhost"
port = 3306
username = "root"
password = "password"
database = "ymaxum"

[logging]
level = "info"
file = "logs/app.log"
max_size = 10485760

[cache]
enabled = true
size = 10000
ttl = 3600

[security]
enable_csrf = true
enable_cors = true
allowed_origins = ["*"]
"#;
    
    let mut file = File::create("test_config.toml").unwrap();
    file.write_all(toml_content.as_bytes()).unwrap();
}

fn bench_config_loading(c: &mut Criterion) {
    create_test_config();
    
    c.bench_function("config_load_from_file", |b| {
        b.iter(|| {
            load_from_file("test_config.toml").unwrap();
        });
    });
}

fn bench_config_reading(c: &mut Criterion) {
    create_test_config();
    
    c.bench_function("config_get_value", |b| {
        b.iter(|| {
            let mut config = TomlConfig::new();
            config.load(ConfigSource::File("test_config.toml".to_string())).unwrap();
            let _port: u16 = config.get("server.port").unwrap();
            let _host: String = config.get("server.host").unwrap();
            let _enable_https: bool = config.get("server.enable_https").unwrap();
            let _db_host: String = config.get("database.host").unwrap();
            let _db_port: u16 = config.get("database.port").unwrap();
        });
    });
}

fn bench_config_writing(c: &mut Criterion) {
    c.bench_function("config_set_value", |b| {
        b.iter(|| {
            let mut config = TomlConfig::new();
            config.set("server.port", 8080).unwrap();
            config.set("server.host", "localhost").unwrap();
            config.set("server.enable_https", true).unwrap();
            config.set("database.host", "localhost").unwrap();
            config.set("database.port", 3306).unwrap();
        });
    });
}

criterion_group!(benches, bench_config_loading, bench_config_reading, bench_config_writing);
criterion_main!(benches);
