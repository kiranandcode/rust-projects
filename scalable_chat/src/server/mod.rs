extern crate mio;

use client::WebSocketClient;

use std::collections::HashMap;
use std::convert::AsRef;
use std::net::SocketAddr;

use mio::*;
use mio::net::{TcpListener};


const SERVER_TOKEN: Token = Token(0);

/// struct representing our server
pub struct WebSocketServer {
    poll: Poll,

    /// socket used to recieve input
    socket: TcpListener,                      
    /// used to keep track of which client is connected to which input
    clients: HashMap<Token, WebSocketClient>,       

    /// keeps count of number of clients
    token_counter: usize,                     
}


impl WebSocketServer {
    pub fn new<T : AsRef<str>>(address: T) -> Result<Self,String> {
        let poll = Poll::new().map_err(|e| e.to_string())?;

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

            // Main server event loop
            loop {
                
                self.poll.poll(&mut events, None).unwrap();

                for event in events.iter() {

                    match event.token() {
                        // when a new client connects, their token defaults to 0 - the server token
                        SERVER_TOKEN => {
                            let client_socket = match self.socket.accept()  {
                                Err(e) => {
                                        println!("Accept error: {}", e);
                                        return;
                                }
                                Ok((sock, _)) => sock
                            };

                            // We associate each user to the system with a "unique" id - simply by
                            // incrementing a counter.
                            self.token_counter += 1;
                            let new_token = Token(self.token_counter);

                            self.clients.insert(new_token, WebSocketClient::new(client_socket));

                            // once the stream has been secured, tell the event pool that we want
                            // to be updated when the input changes
                            self.poll.register
                                (self.clients[&new_token].as_ref(), 
                                 new_token, 
                                 Ready::readable(), 
                                 PollOpt::edge() | PollOpt::oneshot()).unwrap();
                        }
                        // if it's not a server token, this is probably a recurring user
                        #[warn(have_not_checked_whether_where_the_token_comes_from)]
                        token => {
                           if let Some(ref mut client) = self.clients.get_mut(&token)  {
                               // read their new input
                                client.read();
                                // re register them onto the event pool - so that if they have an
                                // event, we get notified.
                                self.poll.reregister(client.as_ref(),
                                                    token,
                                                    Ready::readable(),
                                                    PollOpt::edge() | PollOpt::oneshot()).unwrap();

                           }

                        }
                    }
                }

            }

        }
    }
}
