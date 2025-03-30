# Robust Binary File Downloader

A lightweight and performant Rust application for downloading binary data from an unreliable HTTP server. This proof-of-concept demonstrates how to handle edge cases when working with unstable services that may send incomplete responses.

## Problem Statement

This project addresses a specific challenge: downloading complete binary data from an HTTP server that frequently sends incomplete responses. The server has the following characteristics:

- Randomly generates binary data on startup
- Often returns partial data instead of the complete binary content
- Supports HTTP Range headers for partial content requests
- Provides a SHA-256 hash of the complete data for verification

## Features

- Minimal custom HTTP client implementation without external dependencies
- Automatic retry mechanism with Range headers to fetch missing data
- SHA-256 hash verification to ensure data integrity
- Configurable server URL and output file location
- Robust error handling for various failure scenarios

## Usage

### Running the Server

Start the Python server in a terminal:

```bash
python server.py
```

The server will output:
1. The length of the generated binary data
2. The SHA-256 hash of the data (needed for verification)

### Running the Downloader

The application accepts optional command-line arguments:

```bash
cargo run [SERVER_URL] [OUTPUT_FILENAME]
```

Parameters:
- `SERVER_URL` (optional): The URL of the server to download from
  - Default: `http://localhost:8080`
  - Format: `http://hostname:port`
  - Example: `http://example.com:8080`

- `OUTPUT_FILENAME` (optional): The name of the file to save the downloaded data
  - Default: `output.bin`
  - Example: `downloaded_data.bin`

Examples:
```bash
# Run with default settings
cargo run

# Specify a custom server URL
cargo run http://192.168.1.100:8080

# Specify both custom server URL and output filename
cargo run http://192.168.1.100:8080 important_data.bin
```

## Design Decisions

### Custom HTTP Client

A minimal HTTP client was implemented instead of using external libraries for:

1. **Performance**: Reduced overhead with direct TCP socket handling
2. **Simplicity**: Minimized dependencies for better maintainability
3. **Control**: Fine-grained control over connection handling and timeouts

### Range-Based Download Strategy

The application handles incomplete responses by:

1. Attempting a full download first
2. Tracking how much data was received
3. Using Range headers to request only the missing portions
4. Combining all received chunks to assemble the complete file

### Error Handling

Comprehensive error handling is implemented throughout the codebase to manage:
- Connection failures
- Invalid responses
- Timeout situations
- Missing content length headers
- Corrupt data

### Verification

The SHA-256 hash of the downloaded data is calculated and displayed, allowing you to verify its integrity against the hash provided by the server.

## Building from Source

```bash
# Clone the repository
git clone [repository-url]

# Navigate to the project directory
cd file_downloader

# Build the project
cargo build --release

# Run the application
./target/release/file_downloader
```
