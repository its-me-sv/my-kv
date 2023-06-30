use chrono::Utc;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "127.0.0.1:8080";

fn main() {
    let server = TcpListener::bind(ADDRESS)
        .unwrap_or_else(|_| panic!("[SERVER][{}] Couldn't bind to {}", Utc::now(), ADDRESS));
    println!("[SERVER][{}] Binded to {}", Utc::now(), ADDRESS);

    for client_conn in server.incoming() {
        match client_conn {
            Ok(client) => {
                std::thread::spawn(move || {
                    println!(
                        "[SERVER][{}] Client {:#?} connected",
                        Utc::now(),
                        client.peer_addr().unwrap()
                    );
                    handle_client(client);
                });
            }
            Err(e) => eprintln!(
                "[SERVER][{}] Error: {} | Couldn't accept conncetion from client",
                Utc::now(),
                e
            ),
        }
    }
}

fn handle_client(mut client: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match client.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!(
                        "[SERVER][{}] Client {:#?} disconnected",
                        Utc::now(),
                        client.peer_addr().unwrap()
                    );
                    break;
                }
                let request = String::from_utf8_lossy(buffer.as_slice()).to_string();
                println!(
                    "[SERVER][{}] Client {:#?} sent {}",
                    Utc::now(),
                    client.peer_addr().unwrap(),
                    request
                );
                client.write_all(&buffer[..size]).unwrap();
            }
            Err(e) => {
                eprintln!(
                    "[SERVER][{}] Error: {} | Read error occurred",
                    Utc::now(),
                    e
                );
                println!(
                    "[SERVER][{}] Client {:#?} disconnected",
                    Utc::now(),
                    client.peer_addr().unwrap()
                );
                break;
            }
        }
    }
}
