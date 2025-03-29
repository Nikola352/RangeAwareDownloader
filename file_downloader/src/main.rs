use crate::config::Config;

mod config;

fn main() {
    let config = Config::from(std::env::args().collect());


}
