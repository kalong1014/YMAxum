// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! HTTP headers handling

use std::collections::HashMap;
use std::str::FromStr;

/// HTTP headers
#[derive(Debug, Clone, Default)]
pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    /// Create new headers
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a header
    pub fn add(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_lowercase(), value.to_string());
    }

    /// Get a header value
    pub fn get(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    }

    /// Remove a header
    pub fn remove(&mut self, name: &str) {
        self.headers.remove(&name.to_lowercase());
    }

    /// Get all headers
    pub fn all(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Check if a header exists
    pub fn contains(&self, name: &str) -> bool {
        self.headers.contains_key(&name.to_lowercase())
    }

    /// Clear all headers
    pub fn clear(&mut self) {
        self.headers.clear();
    }

    /// Set content type
    pub fn set_content_type(&mut self, content_type: &str) {
        self.add("content-type", content_type);
    }

    /// Set content length
    pub fn set_content_length(&mut self, length: usize) {
        self.add("content-length", &length.to_string());
    }

    /// Set user agent
    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.add("user-agent", user_agent);
    }

    /// Set authorization header
    pub fn set_authorization(&mut self, auth_type: &str, token: &str) {
        self.add("authorization", &format!("{} {}", auth_type, token));
    }
}

impl FromStr for Headers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut headers = Headers::new();
        
        for line in s.lines() {
            if let Some((name, value)) = line.split_once(": ") {
                headers.add(name, value.trim());
            }
        }
        
        Ok(headers)
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        self.headers
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value))
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}
