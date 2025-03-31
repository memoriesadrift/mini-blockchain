use std::process::exit;
use std::sync::Arc;

use dotenv::dotenv;
use log::info;
use tokio::sync::broadcast;
use tokio_util::task::TaskTracker;
use tokio::signal::ctrl_c;

mod models;
mod config;
mod miner;
mod storage;
mod api;
mod p2p;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let (shutdown_tx, _) = broadcast::channel(1);
    let supervisor = TaskTracker::new();

    info!("Starting node...");
    let db = Arc::new(storage::BlockchainDB::new("./db"));

    supervisor.spawn(p2p::server::start("127.0.0.1", 8080, db.clone(), shutdown_tx.subscribe()));
    supervisor.spawn(api::start_api_server(3030, db.clone(), shutdown_tx.subscribe()));

    ctrl_c().await.unwrap();
    info!("SIGINT received, shutting down");
    let _ = shutdown_tx.send(());
    supervisor.close();
    supervisor.wait().await;
    info!("Goodbye!");
    exit(0);
}
