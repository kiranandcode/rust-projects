extern crate http_muncher;
extern crate mio;

use std::io::Read;
use std::convert::AsRef;

use mio::net::{TcpStream};
use http_muncher::{Parser, ParserHandler};

pub struct HttpParser;

impl ParserHandler for HttpParser {}

pub struct WebSocketClient {
    socket: TcpStream,
    http_parser: Parser
}

impl AsRef<TcpStream> for WebSocketClient {
    fn as_ref(&self) -> &TcpStream {
        &self.socket
    }
}

impl WebSocketClient {

    pub fn new(socket: TcpStream) -> Self {
        WebSocketClient {
            socket: socket,
            http_parser: Parser::request()
        }
    }

    pub fn read(&mut self) {
        loop {
            let mut buf = [0; 2048];
            match self.socket.read(&mut buf) {
                Err(e) => {
                    println!("Error while reading socket: {:?}", e);
                    return;
                }
                Ok(len) => {
                    self.http_parser.parse(&mut HttpParser, &buf[0..len]);
                    if self.http_parser.is_upgrade() {

                        break;
                    }
                }
            }
        }
    }
}

