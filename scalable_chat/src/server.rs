extern crate mio;

use std::collections::HashMap;
use std::convert::AsRef;
use std::net::SocketAddr;

use mio::*;
use mio::net::{TcpListener, TcpStream};


const SERVER_TOKEN: Token = Token(0);
const CLIENT_TOKEN: Token = Token(1);

// struct representing our server
struct WebSocketServer {
    poll: Poll,
    socket: TcpListener,                      // socket used to recieve input
    clients: HashMap<Token, TcpStream>,       // used to keep track of which client is connected to which input
    token_counter: usize                      // keeps count of number of clients
}

impl WebSocketServer {
    pub fn new<T : AsRef<str>>(address: T) -> Result<Self,String> {
        let mut poll = Poll::new().map_err(|e| e.to_string())?;

        let address = address.as_ref().parse::<SocketAddr>().map_err(|e| e.to_string())?;

        // Setup server side socket
        let server_socket = TcpListener::bind(&address).unwrap();
        poll.register(&server_socket, SERVER_TOKEN, Ready::readable(), PollOpt::edge()).map_err(|e| e.to_string())?;


        // Testing - Setup client side socket
        // let client_socket = TcpStream::connect(&address).unwrap();
        // poll.register(&client_socket, CLIENT, Ready::readable(), PollOpt::edge()).unwrap();
        
        Ok(WebSocketServer {
            poll: poll,
            socket: server_socket,
            clients: HashMap::new(),
            token_counter: 0
        })
    }


    pub fn run(&mut self) {
        loop {

            let mut events = Events::with_capacity(1024);

            loop {
                self.poll.poll(&mut events, None).unwrap();
                for event in events.iter() {
                    match event.token() {
                        SERVER_TOKEN => {
                            let _ = self.socket.accept();
                        }
                        CLIENT_TOKEN => {
                            return;
                        }
                        _ => unreachable!()
                    }
                }

            }

        }
    }
}
