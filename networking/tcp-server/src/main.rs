// rust primitives for TCP messages
use std::net::{TcpStream,TcpListener};
use std::thread;
use std::io::{Read, Write};
use std::error::Error;


fn handle_client(mut stream: TcpStream) -> Result<(), Box<Error>> {
    println!("Incoming connection from: {}", 

             // Peer_addr returns the address of the connected client
             stream.peer_addr()?);
    
    // allocate static buffer for recieving information
    let mut buf = [0; 512];

    loop {
        // tcp stream allows bidirectional connection
        // first we read from the stream to a buffer
        let bytes_read = stream.read(&mut buf)?;

        // empty input terminates connection
        if bytes_read == 0 {
            return Ok(());
        }

        // then we just simply respond with the recieved data
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888")
            .expect("Could not bind to port 8888");


    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("error: {:?}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
