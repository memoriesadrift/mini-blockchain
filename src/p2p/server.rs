use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

use tokio::net::TcpListener;

use crate::{config::MAX_PEERS, storage::BlockchainDB};

use super::listener::handle_peer;

pub async fn start(ip: &str, port: u16, db: Arc<BlockchainDB>) {
    let active_connections = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind(format!("{}:{}", ip, port))
        .await
        .unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let count = active_connections.fetch_add(1, Ordering::SeqCst);
        if count >= MAX_PEERS {
            active_connections.fetch_sub(1, Ordering::SeqCst);
            println!("Peer connection rejected: max peers reached");
            continue;
        }

        let db = db.clone();
        let active_connections = active_connections.clone();
        tokio::spawn(async move {
            println!("Connected: {}", active_connections.load(Ordering::Relaxed));
            handle_peer(stream, db).await;

            // Decrement on disconnect
            active_connections.fetch_sub(1, Ordering::Relaxed);
            println!("Disconnected: {}", active_connections.load(Ordering::Relaxed));
        });
    }
}
