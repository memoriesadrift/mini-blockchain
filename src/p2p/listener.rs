use std::sync::Arc;

use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use log::info;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

use crate::{p2p, storage::BlockchainDB};

pub async fn handle_peer(stream: TcpStream, db: Arc<BlockchainDB>) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    let mut handshake_completed = false;

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    // Send Hello to peer
    let hello = p2p::messages::HelloMessage {
        msg: "hello".to_string(),
        greeting: Some("hello there!".to_string())
    };
    let _ = write.send(Message::text(serde_json::to_string(&hello).unwrap())).await;

    // Listen for incoming messages
    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(val) => {
                let Ok(msg) = serde_json::from_str::<p2p::messages::Message>(&val) else {
                    let res = p2p::messages::ErrorMessage {
                        msg: "error".to_string(),
                        reason: "Unparsable message received".to_string()
                    };
                    let _ = write.send(Message::text(serde_json::to_string(&res).unwrap())).await;
                    break;
                };
                match msg {
                    p2p::messages::Message::Hello(data) => {
                        handshake_completed = true;
                        println!("{:#?}", data)

                    }
                    _ => unimplemented!()
                }
            }
            Message::Close(_) => break,
            _ => { /* Ignore other incoming data */}
        }
    }
}
