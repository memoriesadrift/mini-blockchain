use std::sync::Arc;

use log::info;

mod models;
mod config;
mod miner;
mod storage;
mod api;
mod p2p;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting node...");
    let db = Arc::new(storage::BlockchainDB::new("./db"));

    tokio::spawn(p2p::server::start("127.0.0.1", 8080, db.clone()));
    tokio::spawn(api::start_api_server(3030, db.clone()));

    loop { 
        tokio::time::sleep(std::time::Duration::from_secs(1)).await; 
    }
}
