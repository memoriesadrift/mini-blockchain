use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use log::info;

use tokio::{net::TcpListener, sync::broadcast};

use crate::{config::MAX_PEERS, storage::BlockchainDB};

use super::listener::handle_peer;

pub async fn start(
    ip: &str,
    port: u16,
    db: Arc<BlockchainDB>,
    mut shutdown: broadcast::Receiver<()>
    ) {
    let active_connections = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind(format!("{}:{}", ip, port))
        .await

        .unwrap();

    loop{
        let child_shutdown = shutdown.resubscribe();
        tokio::select! {
            Ok((stream,_)) = listener.accept() => {
                let count = active_connections.fetch_add(1, Ordering::SeqCst);
                if count >= MAX_PEERS {
                    active_connections.fetch_sub(1, Ordering::SeqCst);
                    println!("Peer connection rejected: max peers reached");
                    continue;
                }

                let db = db.clone();
                let active_connections = active_connections.clone();
                tokio::spawn(async move {
                    handle_peer(stream, db, child_shutdown).await;

                    // Decrement on disconnect (when future completes)
                    active_connections.fetch_sub(1, Ordering::Relaxed);
                    println!("Disconnected: {}", active_connections.load(Ordering::Relaxed));
                });
            }
            _ = shutdown.recv() => {
                info!("Websocket server received shutdown");
                break;
            }
        }
    }
}
