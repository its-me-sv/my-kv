use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::utils::log::{log, Log};

pub async fn handle_client(mut client: TcpStream) {
    let mut buffer = [0; 1024];
    let client_addr = client.peer_addr().unwrap();

    loop {
        match client.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    println!("{}", log(Log::ClientDisconnected(client_addr)));
                    break;
                }
                let request = String::from_utf8_lossy(buffer.as_slice()).to_string();
                println!("{}", log(Log::ClientSent(client_addr, request)));
                client.write_all(&buffer[..size]).await.unwrap();
            }
            Err(e) => {
                eprintln!("{}", log(Log::ReadError(client_addr, e.to_string())));
                println!("{}", log(Log::ClientDisconnected(client_addr)));
                break;
            }
        }
    }
}
