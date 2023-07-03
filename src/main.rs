use server::utils::{
    log::{log, Log},
    server::{handle_connections, spawn_server},
};

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:8080";
    let server = spawn_server(address).await;

    println!("{}", log(Log::ServerBinded(server.local_addr().unwrap())));

    handle_connections(server).await;
}
