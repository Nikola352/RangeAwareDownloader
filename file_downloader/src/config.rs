const DEFAULT_SERVER_URL: &str = "http://localhost:8080";
const DEFAULT_FILENAME: &str = "output.bin";

pub struct Config {
    pub server_url: String,
    pub output_filename: String,
}

impl From<Vec<String>> for Config {
    fn from(arguments: Vec<String>) -> Self {
        let server_url = if arguments.len() > 1 {
            arguments[1].clone()
        } else {
            DEFAULT_SERVER_URL.to_string()
        };

        let filename = if arguments.len() > 2 {
            arguments[2].clone()
        } else {
            DEFAULT_FILENAME.to_string()
        };

        Config {
            server_url,
            output_filename: filename
        }
    }
}