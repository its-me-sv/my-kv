use chrono::Utc;
use std::net::SocketAddr;

pub enum Log {
    ServerBindIssue(String),
    ServerBinded(SocketAddr),
    ClientConnected(SocketAddr),
    UnableToAcceptClient(String),
    ClientDisconnected(SocketAddr),
    ClientSent(SocketAddr, String),
    ReadError(SocketAddr, String),
}

pub fn log(kind: Log) -> String {
    use Log::*;
    match kind {
        ServerBindIssue(address) => {
            format!("[{}][SERVER] Couldn't bind to {}", Utc::now(), address)
        }
        ServerBinded(address) => {
            format!(
                "[{}][SERVER] Binded to IP {} at PORT {}",
                Utc::now(),
                address.ip(),
                address.port()
            )
        }
        ClientConnected(client_addr) => {
            format!(
                "[{}][SERVER] CLIENT[{}:{}] connected",
                Utc::now(),
                client_addr.ip(),
                client_addr.port()
            )
        }
        UnableToAcceptClient(error) => {
            format!(
                "[{}][SERVER] Couldn't accept connection from client due to error - {}",
                Utc::now(),
                error
            )
        }
        ClientDisconnected(client_addr) => {
            format!(
                "[{}][SERVER] CLIENT[{}:{}] disconnected",
                Utc::now(),
                client_addr.ip(),
                client_addr.port()
            )
        }
        ClientSent(client_addr, data) => {
            format!(
                "[{}][SERVER] CLIENT[{}:{}] sent {}",
                Utc::now(),
                client_addr.ip(),
                client_addr.port(),
                data
            )
        }
        ReadError(client_addr, error) => {
            format!(
                "[{}][SERVER] Read error - {} while recieving from CLIENT[{}:{}]",
                Utc::now(),
                error,
                client_addr.ip(),
                client_addr.port()
            )
        }
    }
}
