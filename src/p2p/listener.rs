use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use tokio::net::TcpStream;

pub async fn handle_peer(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    // TODO: Simple echo for now
    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}
