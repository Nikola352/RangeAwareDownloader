/// Represents a download response containing optional length information and the data.
pub struct DownloadResponse {
    /// Total length as specified by Content-Length HTTP header, 
    /// might not match the returned data length.
    pub length: Option<usize>,
    
    /// Raw response body bytes
    pub data: Vec<u8>,
}