use std::sync::Arc;

use tokio::sync::broadcast;
use warp::Filter;
use log::info;

use crate::storage::BlockchainDB;

pub async fn start_api_server(
    port: u16, 
    db: Arc<BlockchainDB>,
    mut shutdown: broadcast::Receiver<()>
    ) {
    let routes = warp::path!("balance" / String)
        .map(|address: String| {
            format!("Balance for {}: 100\n", address) // Placeholder
        });
    let server = warp::serve(routes).run(([127, 0, 0, 1], port));
    tokio::select! {
        _ = server => {},
        _ = shutdown.recv() => {
            info!("API server received shutdown");
        }
    }
}
