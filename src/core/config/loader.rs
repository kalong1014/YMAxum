// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Configuration loaders

use std::fs::File;
use std::io::{Read, Write};
use std::env;
use crate::core::config::error::ConfigError;
use crate::core::config::{Config, ConfigSource};
use crate::core::config::watcher::ConfigWatcher;

/// 继承策略
#[derive(Debug, Clone, PartialEq)]
pub enum InheritanceStrategy {
    /// 优先级策略：按父配置顺序优先
    Priority,
    /// 合并策略：合并所有父配置
    Merge,
}

/// 配置缓存项
#[derive(Debug, Clone)]
pub struct ConfigCacheItem {
    /// 配置数据
    data: serde_json::Value,
    /// 加载时间
    loaded_at: std::time::Instant,
    /// 缓存有效期
    ttl: Option<std::time::Duration>,
}

/// Base configuration implementation
#[derive(Debug, Clone)]
pub struct BaseConfig {
    /// Configuration data
    data: serde_json::Value,
    /// Watcher
    watcher: Option<ConfigWatcher>,
    /// Configuration version
    version: String,
    /// Parent configurations for inheritance
    parents: Vec<Box<BaseConfig>>,
    /// Inheritance strategy
    inheritance_strategy: InheritanceStrategy,
    /// Configuration history
    history: Vec<(String, serde_json::Value)>,
    /// Configuration cache
    cache: std::cell::RefCell<std::collections::HashMap<String, ConfigCacheItem>>,
    /// Cache enabled flag
    cache_enabled: bool,
}

impl BaseConfig {
    /// Create new base configuration
    pub fn new() -> Self {
        Self {
            data: serde_json::Value::Object(serde_json::Map::new()),
            watcher: None,
            version: "1.0.0".to_string(),
            parents: Vec::new(),
            inheritance_strategy: InheritanceStrategy::Priority,
            history: Vec::new(),
            cache: std::cell::RefCell::new(std::collections::HashMap::new()),
            cache_enabled: true,
        }
    }

    /// Get nested value
    fn get_nested(&self, key: &str) -> Option<serde_json::Value> {
        // First try to get from cache if enabled
        if self.cache_enabled {
            let cache = self.cache.borrow();
            if let Some(cache_item) = cache.get(key) {
                // Check if cache is still valid
                if let Some(ttl) = cache_item.ttl {
                    if cache_item.loaded_at.elapsed() < ttl {
                        return Some(cache_item.data.clone());
                    }
                } else {
                    // Cache without TTL is always valid
                    return Some(cache_item.data.clone());
                }
            }
        }

        // First try to get from current config
        let mut current = &self.data;
        let mut found = true;
        
        for part in key.split('.') {
            match current.get(part) {
                Some(value) => current = value,
                None => {
                    found = false;
                    break;
                }
            }
        }
        
        if found {
            // Cache the result if enabled
            if self.cache_enabled {
                let mut cache = self.cache.borrow_mut();
                cache.insert(key.to_string(), ConfigCacheItem {
                    data: current.clone(),
                    loaded_at: std::time::Instant::now(),
                    ttl: Some(std::time::Duration::from_secs(300)), // 5 minutes cache
                });
            }
            return Some(current.clone());
        }
        
        // If not found, try to get from parent configs based on strategy
        match self.inheritance_strategy {
            InheritanceStrategy::Priority => {
                // Try each parent in order
                for parent in &self.parents {
                    if let Some(value) = parent.get_nested(key) {
                        // Cache the result if enabled
                        if self.cache_enabled {
                            let mut cache = self.cache.borrow_mut();
                            cache.insert(key.to_string(), ConfigCacheItem {
                                data: value.clone(),
                                loaded_at: std::time::Instant::now(),
                                ttl: Some(std::time::Duration::from_secs(300)), // 5 minutes cache
                            });
                        }
                        return Some(value);
                    }
                }
                None
            },
            InheritanceStrategy::Merge => {
                // For merge strategy, collect all values from parents and merge them
                let mut values = Vec::new();
                for parent in &self.parents {
                    if let Some(value) = parent.get_nested(key) {
                        values.push(value);
                    }
                }
                
                if !values.is_empty() {
                    // Merge the values
                    let merged_value = self.merge_values(values);
                    // Cache the result if enabled
                    if self.cache_enabled {
                        let mut cache = self.cache.borrow_mut();
                        cache.insert(key.to_string(), ConfigCacheItem {
                            data: merged_value.clone(),
                            loaded_at: std::time::Instant::now(),
                            ttl: Some(std::time::Duration::from_secs(300)), // 5 minutes cache
                        });
                    }
                    return Some(merged_value);
                }
                None
            }
        }
    }
    
    /// Merge multiple values according to merge strategy
    fn merge_values(&self, values: Vec<serde_json::Value>) -> serde_json::Value {
        if values.is_empty() {
            return serde_json::Value::Null;
        }
        
        if values.len() == 1 {
            return values[0].clone();
        }
        
        // Start with the first value
        let mut result = values[0].clone();
        
        // Merge subsequent values into the result
        for value in values.iter().skip(1) {
            result = self.merge_two_values(result, value.clone());
        }
        
        result
    }
    
    /// Merge two values
    fn merge_two_values(&self, target: serde_json::Value, source: serde_json::Value) -> serde_json::Value {
        match (target, source) {
            (serde_json::Value::Object(mut target_map), serde_json::Value::Object(source_map)) => {
                // Merge objects
                for (key, source_value) in source_map {
                    if let Some(target_value) = target_map.get(&key) {
                        // Recursively merge nested objects
                        let merged_value = self.merge_two_values(target_value.clone(), source_value);
                        target_map.insert(key, merged_value);
                    } else {
                        // Add new key
                        target_map.insert(key, source_value);
                    }
                }
                serde_json::Value::Object(target_map)
            }
            (serde_json::Value::Array(mut target_array), serde_json::Value::Array(source_array)) => {
                // Merge arrays (concatenate)
                target_array.extend(source_array);
                serde_json::Value::Array(target_array)
            }
            // For other types, source overrides target
            (_, source) => source,
        }
    }

    /// Set nested value
    fn set_nested(&mut self, key: &str, value: serde_json::Value) {
        // Record current value for history
        if let Some(current_value) = self.get_nested(key) {
            self.history.push((key.to_string(), current_value));
        }
        
        let mut parts: Vec<&str> = key.split('.').collect();
        let last = parts.pop().unwrap();
        
        let mut current = &mut self.data;
        for part in parts {
            if !current.is_object() {
                *current = serde_json::Value::Object(serde_json::Map::new());
            }
            current = current.as_object_mut().unwrap().entry(part).or_insert(serde_json::Value::Object(serde_json::Map::new()));
        }
        
        if current.is_object() {
            current.as_object_mut().unwrap().insert(last.to_string(), value);
        }
        
        // Clear cache for this key and any parent keys
        if self.cache_enabled {
            let mut cache = self.cache.borrow_mut();
            // Clear exact key
            cache.remove(key);
            // Clear parent keys (e.g., if key is "a.b.c", clear "a" and "a.b")
            let mut current_key = String::new();
            for part in key.split('.').take_while(|&p| p != last) {
                current_key = if current_key.is_empty() {
                    part.to_string()
                } else {
                    format!("{}.{}", current_key, part)
                };
                cache.remove(&current_key);
            }
        }
    }

    /// Remove nested value
    fn remove_nested(&mut self, key: &str) -> bool {
        // Record current value for history
        if let Some(current_value) = self.get_nested(key) {
            self.history.push((key.to_string(), current_value));
        }
        
        let mut parts: Vec<&str> = key.split('.').collect();
        let last = parts.pop().unwrap();
        
        let mut current = &mut self.data;
        for part in &parts {
            match current.get(*part) {
                Some(value) if value.is_object() => current = current.as_object_mut().unwrap().get_mut(*part).unwrap(),
                _ => return false,
            }
        }
        
        let result = if current.is_object() {
            current.as_object_mut().unwrap().remove(last).is_some()
        } else {
            false
        };
        
        // Clear cache for this key and any parent keys
        if self.cache_enabled && result {
            let mut cache = self.cache.borrow_mut();
            // Clear exact key
            cache.remove(key);
            // Clear parent keys (e.g., if key is "a.b.c", clear "a" and "a.b")
            let mut current_key = String::new();
            for part in key.split('.').take_while(|&p| p != last) {
                current_key = if current_key.is_empty() {
                    part.to_string()
                } else {
                    format!("{}.{}", current_key, part)
                };
                cache.remove(&current_key);
            }
        }
        
        result
    }
    
    /// Set configuration version
    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }
    
    /// Get configuration version
    pub fn get_version(&self) -> &str {
        &self.version
    }
    
    /// Set parent configuration for inheritance (single parent)
    pub fn set_parent(&mut self, parent: BaseConfig) {
        self.parents.clear();
        self.parents.push(Box::new(parent));
    }
    
    /// Add parent configuration for inheritance (multiple parents)
    pub fn add_parent(&mut self, parent: BaseConfig) {
        self.parents.push(Box::new(parent));
    }
    
    /// Get parent configurations
    pub fn get_parents(&self) -> &Vec<Box<BaseConfig>> {
        &self.parents
    }
    
    /// Set inheritance strategy
    pub fn set_inheritance_strategy(&mut self, strategy: InheritanceStrategy) {
        self.inheritance_strategy = strategy;
    }
    
    /// Get inheritance strategy
    pub fn get_inheritance_strategy(&self) -> &InheritanceStrategy {
        &self.inheritance_strategy
    }
    
    /// Enable or disable cache
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
        if !enabled {
            // Clear cache when disabling
            let mut cache = self.cache.borrow_mut();
            cache.clear();
        }
    }
    
    /// Check if cache is enabled
    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }
    
    /// Clear cache
    pub fn clear_cache(&mut self) {
        let mut cache = self.cache.borrow_mut();
        cache.clear();
    }
    
    /// Get cache size
    pub fn get_cache_size(&self) -> usize {
        let cache = self.cache.borrow();
        cache.len()
    }
    
    /// 异步加载配置
    pub async fn load_async(&mut self, source: ConfigSource) -> Result<(), ConfigError> {
        match source {
            ConfigSource::File(path) => {
                // 异步读取文件，使用更大的缓冲区
                let content = tokio::fs::read_to_string(&path)
                    .await
                    .map_err(|e| ConfigError::Other(e.to_string()))?;
                
                let extension = path.split('.').last().unwrap_or("");
                match extension {
                    "toml" => {
                        // 使用toml的快速解析
                        let toml_value: toml::Value = toml::from_str(&content)?;
                        self.data = serde_json::to_value(toml_value).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                    }
                    "json" => {
                        // 使用serde_json的快速解析
                        self.data = serde_json::from_str(&content)?;
                    }
                    "yaml" | "yml" => {
                        // 使用serde_yaml的快速解析
                        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                        self.data = serde_json::to_value(yaml_value).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                    }
                    _ => return Err(ConfigError::InvalidFormat("Unknown format".to_string())),
                }
                Ok(())
            }
            ConfigSource::Env => {
                let mut map = serde_json::Map::new();
                // 批量处理环境变量
                for (key, value) in env::vars() {
                    map.insert(key, serde_json::Value::String(value));
                }
                self.data = serde_json::Value::Object(map);
                Ok(())
            }
            ConfigSource::CommandLine => {
                // TODO: Implement command line argument parsing
                Ok(())
            }
            ConfigSource::Memory => {
                // Already in memory
                Ok(())
            }
        }
    }

    /// 并行加载多个配置文件
    pub async fn parallel_load(&mut self, sources: Vec<ConfigSource>) -> Result<(), ConfigError> {
        if sources.is_empty() {
            return Ok(());
        }
        
        // 限制并发数，避免过多的并发任务
        let max_concurrent = std::cmp::min(sources.len(), 8);
        let mut source_iter = sources.into_iter();
        
        // 批量处理
        loop {
            let mut current_handles = Vec::with_capacity(max_concurrent);
            
            // 收集一批任务
            for _ in 0..max_concurrent {
                if let Some(source) = source_iter.next() {
                    let handle = tokio::spawn(async move {
                        let mut config = BaseConfig::new();
                        config.load_async(source).await.map(|_| config.data)
                    });
                    current_handles.push(handle);
                } else {
                    break;
                }
            }
            
            if current_handles.is_empty() {
                break;
            }
            
            // 等待这批任务完成并合并结果
            for handle in current_handles {
                let config_data = handle.await.map_err(|e| ConfigError::Other(e.to_string()))??;
                // 合并到原始配置
                self.merge_config(&config_data);
            }
        }
        
        Ok(())
    }
    
    /// 合并配置数据
    fn merge_config(&mut self, other: &serde_json::Value) {
        if let serde_json::Value::Object(other_map) = other {
            if let serde_json::Value::Object(self_map) = &mut self.data {
                for (key, value) in other_map {
                    self_map.insert(key.clone(), value.clone());
                }
            }
        }
    }
    
    /// Rollback to previous version
    pub fn rollback(&mut self, steps: usize) -> Result<(), ConfigError> {
        if steps > self.history.len() {
            return Err(ConfigError::Other("Not enough history to rollback".to_string()));
        }
        
        for _ in 0..steps {
            if let Some((key, value)) = self.history.pop() {
                self.set_nested(&key, value);
            }
        }
        
        Ok(())
    }
    
    /// Get configuration history
    pub fn get_history(&self) -> &Vec<(String, serde_json::Value)> {
        &self.history
    }
}

impl Config for BaseConfig {
    fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<T, ConfigError> {
        match self.get_nested(key) {
            Some(value) => serde_json::from_value(value).map_err(|e| ConfigError::TypeError(e.to_string())),
            None => Err(ConfigError::KeyNotFound(key.to_string())),
        }
    }

    fn get_with_default<T: for<'de> serde::Deserialize<'de> + Default>(&self, key: &str) -> T {
        self.get(key).unwrap_or_default()
    }

    fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), ConfigError> {
        let json_value = serde_json::to_value(value).map_err(|e| ConfigError::TypeError(e.to_string()))?;
        self.set_nested(key, json_value);
        Ok(())
    }

    fn remove(&mut self, key: &str) -> Result<(), ConfigError> {
        if self.remove_nested(key) {
            Ok(())
        } else {
            Err(ConfigError::KeyNotFound(key.to_string()))
        }
    }

    fn contains(&self, key: &str) -> bool {
        self.get_nested(key).is_some()
    }

    fn load(&mut self, source: ConfigSource) -> Result<(), ConfigError> {
        // 同步加载
        match source {
            ConfigSource::File(path) => {
                let mut file = File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                let extension = path.split('.').last().unwrap_or("");
                match extension {
                    "toml" => {
                        let toml_value: toml::Value = toml::from_str(&content)?;
                        self.data = serde_json::to_value(toml_value).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                    }
                    "json" => {
                        self.data = serde_json::from_str(&content)?;
                    }
                    "yaml" | "yml" => {
                        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                        self.data = serde_json::to_value(yaml_value).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                    }
                    _ => return Err(ConfigError::InvalidFormat("Unknown format".to_string())),
                }
                Ok(())
            }
            ConfigSource::Env => {
                let mut map = serde_json::Map::new();
                for (key, value) in env::vars() {
                    map.insert(key, serde_json::Value::String(value));
                }
                self.data = serde_json::Value::Object(map);
                Ok(())
            }
            ConfigSource::CommandLine => {
                // TODO: Implement command line argument parsing
                Ok(())
            }
            ConfigSource::Memory => {
                // Already in memory
                Ok(())
            }
        }
    }

    fn save(&self, source: ConfigSource) -> Result<(), ConfigError> {
        match source {
            ConfigSource::File(path) => {
                let mut file = File::create(&path)?;
                
                let extension = path.split('.').last().unwrap_or("");
                let content = match extension {
                    "toml" => {
                        let toml_value: toml::Value = serde_json::from_value(self.data.clone()).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                        toml::to_string(&toml_value).map_err(|e| ConfigError::ParseError(e.to_string()))?
                    }
                    "json" => {
                        serde_json::to_string_pretty(&self.data).map_err(|e| ConfigError::ParseError(e.to_string()))?
                    }
                    "yaml" | "yml" => {
                        let yaml_value: serde_yaml::Value = serde_json::from_value(self.data.clone()).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                        serde_yaml::to_string(&yaml_value).map_err(|e| ConfigError::ParseError(e.to_string()))?
                    }
                    _ => return Err(ConfigError::InvalidFormat("Unknown format".to_string())),
                };
                
                file.write_all(content.as_bytes())?;
                Ok(())
            }
            ConfigSource::Env => {
                // Environment variables are read-only
                Err(ConfigError::Other("Cannot save to environment variables".to_string()))
            }
            ConfigSource::CommandLine => {
                // Command line arguments are read-only
                Err(ConfigError::Other("Cannot save to command line arguments".to_string()))
            }
            ConfigSource::Memory => {
                // Already in memory
                Ok(())
            }
        }
    }

    fn watch(&mut self, source: ConfigSource, callback: Box<dyn Fn() + Send + Sync>) -> Result<(), ConfigError> {
        match source {
            ConfigSource::File(path) => {
                let watcher = ConfigWatcher::new(path, callback);
                self.watcher = Some(watcher);
                Ok(())
            }
            _ => Err(ConfigError::WatchError("Only file sources can be watched".to_string())),
        }
    }
}

/// TOML configuration
#[derive(Debug, Clone)]
pub struct TomlConfig {
    base: BaseConfig,
}

impl TomlConfig {
    /// Create new TOML configuration
    pub fn new() -> Self {
        Self {
            base: BaseConfig::new(),
        }
    }
}

impl Config for TomlConfig {
    fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<T, ConfigError> {
        self.base.get(key)
    }
    
    fn get_with_default<T: for<'de> serde::Deserialize<'de> + Default>(&self, key: &str) -> T {
        self.base.get_with_default(key)
    }
    
    fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), ConfigError> {
        self.base.set(key, value)
    }
    
    fn remove(&mut self, key: &str) -> Result<(), ConfigError> {
        self.base.remove(key)
    }
    
    fn contains(&self, key: &str) -> bool {
        self.base.contains(key)
    }
    
    fn load(&mut self, source: ConfigSource) -> Result<(), ConfigError> {
        self.base.load(source)
    }
    
    fn save(&self, source: ConfigSource) -> Result<(), ConfigError> {
        self.base.save(source)
    }
    
    fn watch(&mut self, source: ConfigSource, callback: Box<dyn Fn() + Send + Sync>) -> Result<(), ConfigError> {
        self.base.watch(source, callback)
    }
}

/// JSON configuration
#[derive(Debug, Clone)]
pub struct JsonConfig {
    base: BaseConfig,
}

impl JsonConfig {
    /// Create new JSON configuration
    pub fn new() -> Self {
        Self {
            base: BaseConfig::new(),
        }
    }
}

impl Config for JsonConfig {
    fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<T, ConfigError> {
        self.base.get(key)
    }
    
    fn get_with_default<T: for<'de> serde::Deserialize<'de> + Default>(&self, key: &str) -> T {
        self.base.get_with_default(key)
    }
    
    fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), ConfigError> {
        self.base.set(key, value)
    }
    
    fn remove(&mut self, key: &str) -> Result<(), ConfigError> {
        self.base.remove(key)
    }
    
    fn contains(&self, key: &str) -> bool {
        self.base.contains(key)
    }
    
    fn load(&mut self, source: ConfigSource) -> Result<(), ConfigError> {
        self.base.load(source)
    }
    
    fn save(&self, source: ConfigSource) -> Result<(), ConfigError> {
        self.base.save(source)
    }
    
    fn watch(&mut self, source: ConfigSource, callback: Box<dyn Fn() + Send + Sync>) -> Result<(), ConfigError> {
        self.base.watch(source, callback)
    }
}

/// YAML configuration
#[derive(Debug, Clone)]
pub struct YamlConfig {
    base: BaseConfig,
}

impl YamlConfig {
    /// Create new YAML configuration
    pub fn new() -> Self {
        Self {
            base: BaseConfig::new(),
        }
    }
}

impl Config for YamlConfig {
    fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<T, ConfigError> {
        self.base.get(key)
    }
    
    fn get_with_default<T: for<'de> serde::Deserialize<'de> + Default>(&self, key: &str) -> T {
        self.base.get_with_default(key)
    }
    
    fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), ConfigError> {
        self.base.set(key, value)
    }
    
    fn remove(&mut self, key: &str) -> Result<(), ConfigError> {
        self.base.remove(key)
    }
    
    fn contains(&self, key: &str) -> bool {
        self.base.contains(key)
    }
    
    fn load(&mut self, source: ConfigSource) -> Result<(), ConfigError> {
        self.base.load(source)
    }
    
    fn save(&self, source: ConfigSource) -> Result<(), ConfigError> {
        self.base.save(source)
    }
    
    fn watch(&mut self, source: ConfigSource, callback: Box<dyn Fn() + Send + Sync>) -> Result<(), ConfigError> {
        self.base.watch(source, callback)
    }
}

/// Environment variables configuration
#[derive(Debug, Clone)]
pub struct EnvConfig {
    base: BaseConfig,
}

impl EnvConfig {
    /// Create new environment variables configuration
    pub fn new() -> Self {
        Self {
            base: BaseConfig::new(),
        }
    }
}

impl Config for EnvConfig {
    fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<T, ConfigError> {
        self.base.get(key)
    }

    fn get_with_default<T: for<'de> serde::Deserialize<'de> + Default>(&self, key: &str) -> T {
        self.base.get_with_default(key)
    }

    fn set<T: serde::Serialize>(&mut self, _key: &str, _value: T) -> Result<(), ConfigError> {
        // Environment variables are read-only
        Err(ConfigError::Other("Cannot set environment variables".to_string()))
    }

    fn remove(&mut self, _key: &str) -> Result<(), ConfigError> {
        // Environment variables are read-only
        Err(ConfigError::Other("Cannot remove environment variables".to_string()))
    }

    fn contains(&self, key: &str) -> bool {
        self.base.contains(key)
    }

    fn load(&mut self, source: ConfigSource) -> Result<(), ConfigError> {
        self.base.load(source)
    }

    fn save(&self, _source: ConfigSource) -> Result<(), ConfigError> {
        // Environment variables are read-only
        Err(ConfigError::Other("Cannot save to environment variables".to_string()))
    }

    fn watch(&mut self, _source: ConfigSource, _callback: Box<dyn Fn() + Send + Sync>) -> Result<(), ConfigError> {
        // Environment variables cannot be watched
        Err(ConfigError::WatchError("Cannot watch environment variables".to_string()))
    }
}
