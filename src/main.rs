use db::DBClient;

mod config;
mod models;
mod dtos;
mod error;
mod db;

use config::Config;
// use b::DBClient;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client : DBClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // dotenv().ok();
    // let config: = Config::init();
}
