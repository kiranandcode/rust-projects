extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

pub mod server;
pub mod client;
pub mod key;

use server::WebSocketServer;


fn main() {
    let mut server = WebSocketServer::new("0.0.0.0:10000").unwrap();
    server.run();
}

