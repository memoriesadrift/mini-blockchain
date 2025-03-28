use tokio::net::TcpListener;

use super::listener::handle_peer;

pub async fn start(ip: &str, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", ip, port))
        .await
        .unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_peer(stream));
    }
}
