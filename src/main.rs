use server::utils::{
    log::{log, Log},
    server::{handle_connections, spawn_server},
};

fn main() {
    let address = "127.0.0.1:8080";
    let server = spawn_server(address);

    println!("{}", log(Log::ServerBinded(server.local_addr().unwrap())));

    handle_connections(server);
}
