use crate::config::Config;
use crate::downloader::Downloader;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;

mod config;
mod downloader;
mod http;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from(std::env::args().collect::<Vec<String>>());

    let downloader = Downloader::new(config.server_url);

    let data = downloader.download_fully()?;

    let hash = Sha256::digest(&data);
    println!("SHA-256 hash of received data: {:x}", hash);

    fs::write(&config.output_filename, data)?;
    println!(
        "Binary contents successfully written to {}",
        config.output_filename
    );

    Ok(())
}
