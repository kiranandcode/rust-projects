extern crate mio;
extern crate http_muncher;

pub mod server;
pub mod client;

use server::WebSocketServer;


fn main() {
    let mut server = WebSocketServer::new("0.0.0.0:10000").unwrap();
    server.run();
}

