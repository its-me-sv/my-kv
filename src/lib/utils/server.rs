use crate::handlers::client::handle_client;
use tokio::net::TcpListener;

use super::log::{log, Log};

pub async fn spawn_server(address: &str) -> TcpListener {
    TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| panic!("{}", log(Log::ServerBindIssue(address.to_string()))))
}

pub async fn handle_connections(server: TcpListener) {
    loop {
        match server.accept().await {
            Ok((client, addr)) => {
                println!("{}", log(Log::ClientConnected(addr)));
                handle_client(client).await;
            }
            Err(e) => eprintln!("{}", log(Log::UnableToAcceptClient(e.to_string()))),
        }
    }
}
