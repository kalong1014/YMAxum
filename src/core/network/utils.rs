// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Network utility functions

use std::net::IpAddr;
use std::str::FromStr;

/// Validate IP address
pub fn validate_ip_address(ip: &str) -> bool {
    IpAddr::from_str(ip).is_ok()
}

/// Validate URL
pub fn validate_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

/// Parse query string into key-value pairs
pub fn parse_query_string(query: &str) -> std::collections::HashMap<String, String> {
    let mut params = std::collections::HashMap::new();
    
    for param in query.split('&') {
        if let Some((key, value)) = param.split_once('=') {
            params.insert(key.to_string(), value.to_string());
        }
    }
    
    params
}

/// Build query string from key-value pairs
pub fn build_query_string(params: &std::collections::HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}

/// Sanitize URL path
pub fn sanitize_path(path: &str) -> String {
    // Remove any leading or trailing slashes
    let path = path.trim_matches('/');
    
    // Ensure path starts with a single slash
    if path.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", path)
    }
}

/// Get content type from file extension
pub fn get_content_type_from_extension(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        _ => "application/octet-stream",
    }
}

/// Get file extension from path
pub fn get_file_extension(path: &str) -> Option<&str> {
    path.rsplit('.').next()
}

/// Generate random string for CSRF token
pub fn generate_csrf_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    (0..32).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

/// Validate CSRF token
pub fn validate_csrf_token(token: &str) -> bool {
    token.len() == 32 && token.chars().all(|c| c.is_alphanumeric())
}
