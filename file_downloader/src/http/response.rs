use std::collections::HashMap;

/// HTTP response container with status code, headers and raw body
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}
