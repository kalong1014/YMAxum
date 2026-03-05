// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! HTTP request handling

use std::collections::HashMap;
use std::str::FromStr;
use crate::core::network::headers::Headers;

/// HTTP method
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl Method {
    /// Get method as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::CONNECT => "CONNECT",
            Method::TRACE => "TRACE",
        }
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(()),
        }
    }
}

/// HTTP request
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: Method,
    /// Request path
    pub path: String,
    /// Query parameters
    pub query: HashMap<String, String>,
    /// Headers
    pub headers: Headers,
    /// Body
    pub body: Vec<u8>,
}

impl Request {
    /// Create new request
    pub fn new(method: Method, path: &str) -> Self {
        Self {
            method,
            path: path.to_string(),
            query: HashMap::new(),
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    /// Add query parameter
    pub fn add_query(&mut self, key: &str, value: &str) {
        self.query.insert(key.to_string(), value.to_string());
    }

    /// Add header
    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.add(name, value);
    }

    /// Set body
    pub fn set_body(&mut self, body: &[u8]) {
        self.body = body.to_vec();
        self.headers.set_content_length(body.len());
    }

    /// Set body from string
    pub fn set_body_string(&mut self, body: &str) {
        self.set_body(body.as_bytes());
    }

    /// Set content type
    pub fn set_content_type(&mut self, content_type: &str) {
        self.headers.set_content_type(content_type);
    }

    /// Get full URL with query parameters
    pub fn get_url(&self, base_url: &str) -> String {
        let mut url = format!("{}{}", base_url, self.path);
        
        if !self.query.is_empty() {
            url.push('?');
            url.push_str(
                &self.query
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&"),
            );
        }
        
        url
    }

    /// Convert request to HTTP message
    pub fn to_http_message(&self) -> String {
        let mut message = format!("{} {} HTTP/1.1\r\n", self.method.as_str(), self.path);
        
        if !self.query.is_empty() {
            message.push_str(&format!("Query: {}\r\n", 
                self.query
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&"))
            );
        }
        
        message.push_str(&self.headers.to_string());
        message.push_str("\r\n\r\n");
        
        if !self.body.is_empty() {
            message.push_str(std::str::from_utf8(&self.body).unwrap_or(""));
        }
        
        message
    }
}
