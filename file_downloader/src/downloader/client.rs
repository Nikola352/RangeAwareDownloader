use crate::downloader::{DownloadError, DownloadResponse};
use crate::http::{HttpClient, HttpResponse};
use std::ops::Range;

/// Handles downloading binary data from a server, supporting both full and partial downloads.
pub struct Downloader {
    client: HttpClient,
    url_path: String,
}

impl Downloader {
    /// Creates a new Downloader instance from a server URL.
    /// URL format should be "http://host:port/path".
    /// Defaults to localhost:80 if not specified.
    pub fn new(server_url: String) -> Self {
        let (host, port, path) = Self::parse_url(server_url);
        Self {
            client: HttpClient::new(host, port),
            url_path: path,
        }
    }

    /// Parses a URL into (host, port, path) components.
    fn parse_url(url: String) -> (String, u16, String) {
        let stripped = url.trim_start_matches("http://");

        let mut parts = stripped.splitn(2, '/');
        let host_port = parts.next().unwrap_or("127.0.0.1:8080").trim();
        let mut path = parts.next().unwrap_or("").trim().to_string();
        path.insert(0, '/');

        let mut host_port_parts = host_port.splitn(2, ":");
        let host = host_port_parts
            .next()
            .unwrap_or("127.0.0.1")
            .trim()
            .to_string(); // default to localhost
        let port = host_port_parts
            .next()
            .and_then(|p| p.parse().ok())
            .unwrap_or(80); // default HTTP port

        (host, port, path)
    }

    /// Downloads the complete file by making multiple requests if needed.
    /// Returns an error if the server doesn't provide content length.
    pub fn download_fully(&self) -> Result<Vec<u8>, DownloadError> {
        let response = self.get()?;

        let total_length = response.length.ok_or(DownloadError::MissingLength)?;

        let mut data = response.data;

        while data.len() < total_length {
            let response = self.get_range(Range {
                start: data.len(),
                end: total_length,
            })?;
            data.extend_from_slice(&response.data)
        }

        Ok(data)
    }

    /// Makes a GET request for the entire resource.
    fn get(&self) -> Result<DownloadResponse, DownloadError> {
        let response = self.client.send_request("GET", &self.url_path, None)?;
        self.validate_response(&response)?;
        Ok(DownloadResponse {
            length: self.get_content_length(&response),
            data: response.body,
        })
    }

    /// Makes a GET request for a specific byte range of the resource.
    fn get_range(&self, range: Range<usize>) -> Result<DownloadResponse, DownloadError> {
        let range_header = format!("bytes={}-{}", range.start, range.end);
        let headers: Vec<(&str, &str)> = Vec::from([("Range", range_header.as_str())]);
        let response = self
            .client
            .send_request("GET", &self.url_path, Some(headers))?;
        self.validate_response(&response)?;
        Ok(DownloadResponse {
            length: self.get_content_length(&response),
            data: response.body,
        })
    }

    /// Checks if the response status code indicates success.
    fn validate_response(&self, response: &HttpResponse) -> Result<(), DownloadError> {
        if response.status_code > 299 {
            return Err(DownloadError::InvalidResponse(format!(
                "Non-ok response status: {}",
                response.status_code
            )));
        }
        Ok(())
    }

    /// Extracts content length from response headers if present.
    fn get_content_length(&self, response: &HttpResponse) -> Option<usize> {
        response
            .headers
            .get("Content-Length")
            .and_then(|s| s.parse().ok())
    }
}
