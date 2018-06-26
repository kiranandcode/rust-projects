extern crate http_muncher;
extern crate mio;

use std::io::Read;
use std::convert::AsRef;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::str;

use mio::net::{TcpStream};
use http_muncher::{Parser, ParserHandler};

pub struct HttpParser {
    current_key: Option<String>,
    headers: Rc<RefCell<HashMap<String,String>>>
}

impl ParserHandler for HttpParser {
    fn on_header_field(&mut self, parser: &mut Parser, s: &[u8]) -> bool {
        self.current_key = Some(str::from_utf8(s).unwrap().to_string());
        true
    }


    /// Called for each HTTP header value part.
    fn on_header_value(&mut self, parser: &mut Parser, s: &[u8]) -> bool {
        self.headers.borrow_mut()
            .insert(self.current_key.clone().unwrap(),
                str::from_utf8(s).unwrap().to_string());
        true
    }

    /// Notified when all available headers have been processed.
    fn on_headers_complete(&mut self, parser: &mut Parser) -> bool {
        false
    }



}

pub struct WebSocketClient {
    socket: TcpStream,
    http_parser: (Parser,HttpParser),
    headers: Rc<RefCell<HashMap<String,String>>>
}

impl AsRef<TcpStream> for WebSocketClient {
    fn as_ref(&self) -> &TcpStream {
        &self.socket
    }
}

impl WebSocketClient {

    pub fn new(socket: TcpStream) -> Self {
        let headers = Rc::new(RefCell::new(HashMap::new()));

        
        WebSocketClient {
            socket: socket,
            headers: headers.clone(),
            http_parser: (Parser::request(), HttpParser { current_key: None, headers: headers.clone() }),
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
                    let (ref mut parser, ref mut handler) = self.http_parser;

                    parser.parse(handler, &buf[0..len]);
                    if parser.is_upgrade() {

                        break;
                    }
                }
            }
        }
    }
}

