// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! HTTP response handling

use crate::core::network::headers::Headers;

/// HTTP status code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusCode {
    /// Status code
    pub code: u16,
    /// Status message
    pub message: &'static str,
}

impl StatusCode {
    /// 100 Continue
    pub const CONTINUE: StatusCode = StatusCode { code: 100, message: "Continue" };
    /// 101 Switching Protocols
    pub const SWITCHING_PROTOCOLS: StatusCode = StatusCode { code: 101, message: "Switching Protocols" };
    /// 200 OK
    pub const OK: StatusCode = StatusCode { code: 200, message: "OK" };
    /// 201 Created
    pub const CREATED: StatusCode = StatusCode { code: 201, message: "Created" };
    /// 202 Accepted
    pub const ACCEPTED: StatusCode = StatusCode { code: 202, message: "Accepted" };
    /// 203 Non-Authoritative Information
    pub const NON_AUTHORITATIVE_INFORMATION: StatusCode = StatusCode { code: 203, message: "Non-Authoritative Information" };
    /// 204 No Content
    pub const NO_CONTENT: StatusCode = StatusCode { code: 204, message: "No Content" };
    /// 205 Reset Content
    pub const RESET_CONTENT: StatusCode = StatusCode { code: 205, message: "Reset Content" };
    /// 206 Partial Content
    pub const PARTIAL_CONTENT: StatusCode = StatusCode { code: 206, message: "Partial Content" };
    /// 300 Multiple Choices
    pub const MULTIPLE_CHOICES: StatusCode = StatusCode { code: 300, message: "Multiple Choices" };
    /// 301 Moved Permanently
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode { code: 301, message: "Moved Permanently" };
    /// 302 Found
    pub const FOUND: StatusCode = StatusCode { code: 302, message: "Found" };
    /// 303 See Other
    pub const SEE_OTHER: StatusCode = StatusCode { code: 303, message: "See Other" };
    /// 304 Not Modified
    pub const NOT_MODIFIED: StatusCode = StatusCode { code: 304, message: "Not Modified" };
    /// 307 Temporary Redirect
    pub const TEMPORARY_REDIRECT: StatusCode = StatusCode { code: 307, message: "Temporary Redirect" };
    /// 308 Permanent Redirect
    pub const PERMANENT_REDIRECT: StatusCode = StatusCode { code: 308, message: "Permanent Redirect" };
    /// 400 Bad Request
    pub const BAD_REQUEST: StatusCode = StatusCode { code: 400, message: "Bad Request" };
    /// 401 Unauthorized
    pub const UNAUTHORIZED: StatusCode = StatusCode { code: 401, message: "Unauthorized" };
    /// 402 Payment Required
    pub const PAYMENT_REQUIRED: StatusCode = StatusCode { code: 402, message: "Payment Required" };
    /// 403 Forbidden
    pub const FORBIDDEN: StatusCode = StatusCode { code: 403, message: "Forbidden" };
    /// 404 Not Found
    pub const NOT_FOUND: StatusCode = StatusCode { code: 404, message: "Not Found" };
    /// 405 Method Not Allowed
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode { code: 405, message: "Method Not Allowed" };
    /// 406 Not Acceptable
    pub const NOT_ACCEPTABLE: StatusCode = StatusCode { code: 406, message: "Not Acceptable" };
    /// 407 Proxy Authentication Required
    pub const PROXY_AUTHENTICATION_REQUIRED: StatusCode = StatusCode { code: 407, message: "Proxy Authentication Required" };
    /// 408 Request Timeout
    pub const REQUEST_TIMEOUT: StatusCode = StatusCode { code: 408, message: "Request Timeout" };
    /// 409 Conflict
    pub const CONFLICT: StatusCode = StatusCode { code: 409, message: "Conflict" };
    /// 410 Gone
    pub const GONE: StatusCode = StatusCode { code: 410, message: "Gone" };
    /// 411 Length Required
    pub const LENGTH_REQUIRED: StatusCode = StatusCode { code: 411, message: "Length Required" };
    /// 412 Precondition Failed
    pub const PRECONDITION_FAILED: StatusCode = StatusCode { code: 412, message: "Precondition Failed" };
    /// 413 Payload Too Large
    pub const PAYLOAD_TOO_LARGE: StatusCode = StatusCode { code: 413, message: "Payload Too Large" };
    /// 414 URI Too Long
    pub const URI_TOO_LONG: StatusCode = StatusCode { code: 414, message: "URI Too Long" };
    /// 415 Unsupported Media Type
    pub const UNSUPPORTED_MEDIA_TYPE: StatusCode = StatusCode { code: 415, message: "Unsupported Media Type" };
    /// 416 Range Not Satisfiable
    pub const RANGE_NOT_SATISFIABLE: StatusCode = StatusCode { code: 416, message: "Range Not Satisfiable" };
    /// 417 Expectation Failed
    pub const EXPECTATION_FAILED: StatusCode = StatusCode { code: 417, message: "Expectation Failed" };
    /// 426 Upgrade Required
    pub const UPGRADE_REQUIRED: StatusCode = StatusCode { code: 426, message: "Upgrade Required" };
    /// 500 Internal Server Error
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode { code: 500, message: "Internal Server Error" };
    /// 501 Not Implemented
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode { code: 501, message: "Not Implemented" };
    /// 502 Bad Gateway
    pub const BAD_GATEWAY: StatusCode = StatusCode { code: 502, message: "Bad Gateway" };
    /// 503 Service Unavailable
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode { code: 503, message: "Service Unavailable" };
    /// 504 Gateway Timeout
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode { code: 504, message: "Gateway Timeout" };
    /// 505 HTTP Version Not Supported
    pub const HTTP_VERSION_NOT_SUPPORTED: StatusCode = StatusCode { code: 505, message: "HTTP Version Not Supported" };

    /// Get status code by number
    pub fn from_code(code: u16) -> Option<StatusCode> {
        match code {
            100 => Some(StatusCode::CONTINUE),
            101 => Some(StatusCode::SWITCHING_PROTOCOLS),
            200 => Some(StatusCode::OK),
            201 => Some(StatusCode::CREATED),
            202 => Some(StatusCode::ACCEPTED),
            203 => Some(StatusCode::NON_AUTHORITATIVE_INFORMATION),
            204 => Some(StatusCode::NO_CONTENT),
            205 => Some(StatusCode::RESET_CONTENT),
            206 => Some(StatusCode::PARTIAL_CONTENT),
            300 => Some(StatusCode::MULTIPLE_CHOICES),
            301 => Some(StatusCode::MOVED_PERMANENTLY),
            302 => Some(StatusCode::FOUND),
            303 => Some(StatusCode::SEE_OTHER),
            304 => Some(StatusCode::NOT_MODIFIED),
            307 => Some(StatusCode::TEMPORARY_REDIRECT),
            308 => Some(StatusCode::PERMANENT_REDIRECT),
            400 => Some(StatusCode::BAD_REQUEST),
            401 => Some(StatusCode::UNAUTHORIZED),
            402 => Some(StatusCode::PAYMENT_REQUIRED),
            403 => Some(StatusCode::FORBIDDEN),
            404 => Some(StatusCode::NOT_FOUND),
            405 => Some(StatusCode::METHOD_NOT_ALLOWED),
            406 => Some(StatusCode::NOT_ACCEPTABLE),
            407 => Some(StatusCode::PROXY_AUTHENTICATION_REQUIRED),
            408 => Some(StatusCode::REQUEST_TIMEOUT),
            409 => Some(StatusCode::CONFLICT),
            410 => Some(StatusCode::GONE),
            411 => Some(StatusCode::LENGTH_REQUIRED),
            412 => Some(StatusCode::PRECONDITION_FAILED),
            413 => Some(StatusCode::PAYLOAD_TOO_LARGE),
            414 => Some(StatusCode::URI_TOO_LONG),
            415 => Some(StatusCode::UNSUPPORTED_MEDIA_TYPE),
            416 => Some(StatusCode::RANGE_NOT_SATISFIABLE),
            417 => Some(StatusCode::EXPECTATION_FAILED),
            426 => Some(StatusCode::UPGRADE_REQUIRED),
            500 => Some(StatusCode::INTERNAL_SERVER_ERROR),
            501 => Some(StatusCode::NOT_IMPLEMENTED),
            502 => Some(StatusCode::BAD_GATEWAY),
            503 => Some(StatusCode::SERVICE_UNAVAILABLE),
            504 => Some(StatusCode::GATEWAY_TIMEOUT),
            505 => Some(StatusCode::HTTP_VERSION_NOT_SUPPORTED),
            _ => None,
        }
    }
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct Response {
    /// Status code
    pub status: StatusCode,
    /// Headers
    pub headers: Headers,
    /// Body
    pub body: Vec<u8>,
}

impl Response {
    /// Create new response
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    /// Create OK response
    pub fn ok() -> Self {
        Self::new(StatusCode::OK)
    }

    /// Create error response
    pub fn error(status: StatusCode, message: &str) -> Self {
        let mut response = Self::new(status);
        response.set_body_string(message);
        response.set_content_type("text/plain");
        response
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

    /// Get body as string
    pub fn body_as_string(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    /// Convert response to HTTP message
    pub fn to_http_message(&self) -> String {
        let mut message = format!("HTTP/1.1 {} {}\r\n", self.status.code, self.status.message);
        message.push_str(&self.headers.to_string());
        message.push_str("\r\n\r\n");
        
        if !self.body.is_empty() {
            message.push_str(std::str::from_utf8(&self.body).unwrap_or(""));
        }
        
        message
    }
}
