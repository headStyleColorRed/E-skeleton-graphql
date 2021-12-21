mod server;
mod schema;

extern crate log;

#[tokio::main]
async fn main() {
    let address: [u8; 4] = [127, 0, 0, 1];
    let port: u16 = 8080;

    log::info!("Server running on port {}", port);
    server::start((address, port)).await;
}
