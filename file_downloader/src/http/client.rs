use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use crate::http::{HttpError, HttpResponse};

const WRITE_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(5);

/// Simple HTTP client for making requests to a specific host
pub struct HttpClient {
    host: String,
    port: u16,
}

impl HttpClient {
    /// Create a new client instance for the given host and port
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    /// Send an HTTP request without body and return the response
    pub fn send_request(
        &self,
        method: &str,
        path: &str,
        headers: Option<Vec<(&str, &str)>>,
    ) -> Result<HttpResponse, HttpError> {
        // Open a TCP connection
        let mut stream = TcpStream::connect((self.host.as_str(), self.port))
            .map_err(HttpError::ConnectionFailed)?;

        stream.set_write_timeout(Some(WRITE_TIMEOUT))?;
        stream.set_read_timeout(Some(READ_TIMEOUT))?;

        // Format HTTP request
        let mut request = format!(
            "{} {} HTTP/1.1\r\n\
            Host: {}:{}\r\n\
            Connection: close\r\n",
            method, path, self.host, self.port
        );

        if let Some(headers) = headers {
            for (name, value) in headers {
                request.push_str(&format!("{}: {}\r\n", name, value))
            }
        }

        request.push_str("\r\n");

        // Send HTTP request
        stream.write_all(request.as_bytes())?;
        stream.flush()?;

        // Read the raw response bytes using a buffer
        let mut response: Vec<u8> = Vec::new();
        let mut buffer = [0; 4096]; // 4KB buffer matches many system defaults
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            response.extend_from_slice(&buffer[..bytes_read]);
        }

        self.parse_response(response)
    }

    /// Parse raw HTTP response into structured data
    fn parse_response(&self, response: Vec<u8>) -> Result<HttpResponse, HttpError> {
        // Split header and body
        let split_position = response.windows(4).position(|x| x == b"\r\n\r\n");
        let (headers, body) = if let Some(position) = split_position {
            let (headers, body) = response.split_at(position + 4);
            (headers.to_vec(), body.to_vec())
        } else {
            return Err(HttpError::InvalidResponse);
        };

        // Parse status code
        let headers_str = String::from_utf8_lossy(&headers);
        let status_line = headers_str
            .lines()
            .next()
            .ok_or(HttpError::InvalidResponse)?;
        let status_code = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .ok_or(HttpError::InvalidResponse)?;

        if status_code >= 400 {
            return Err(HttpError::RequestFailed(status_code));
        }

        // Parse headers
        let headers: HashMap<String, String> = headers_str
            .lines()
            .skip(1)
            .filter_map(|line| {
                let mut parts = line.splitn(2, ':');
                match (parts.next(), parts.next()) {
                    (Some(name), Some(value)) => {
                        Some((name.trim().to_string(), value.trim().to_string()))
                    }
                    _ => None,
                }
            })
            .collect();

        Ok(HttpResponse {
            status_code,
            headers,
            body,
        })
    }
}
