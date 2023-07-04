use crate::handlers::client::handle_client;
use std::net::TcpListener;

use super::log::{log, Log};

pub fn spawn_server(address: &str) -> TcpListener {
    TcpListener::bind(address)
        .unwrap_or_else(|_| panic!("{}", log(Log::ServerBindIssue(address.to_string()))))
}

pub fn handle_connections(server: TcpListener) {
    for client_conn in server.incoming() {
        match client_conn {
            Ok(client) => {
                std::thread::spawn(move || {
                    println!("{}", log(Log::ClientConnected(client.peer_addr().unwrap())));
                    handle_client(client);
                });
            }
            Err(e) => eprintln!("{}", log(Log::UnableToAcceptClient(e.to_string()))),
        }
    }
}
