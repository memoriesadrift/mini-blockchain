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

    tokio::spawn(p2p::server::start("127.0.0.1", 8080));
    tokio::spawn(api::start_api_server(3030));
    info!("p2p websocket server running...");

    loop { 
        tokio::time::sleep(std::time::Duration::from_secs(1)).await; 
    }
}
