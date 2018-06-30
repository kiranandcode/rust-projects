use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write, Error};
use std::thread;


fn handle_client(mut stream : TcpStream) -> Result<(),Error> {
    println!("Incoming connection from: {}",
             stream.peer_addr()?);
    let mut buf = [0;512];
    loop {
        let bytes_read = stream.read(&mut buf)?; 

        if bytes_read == 0 { return Ok(()); }
        stream.write(&buf[..bytes_read])?;
    }
}


fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888")
        .expect("Could not bind port 8888");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {eprintln!("Stream Error: {}", e);},
            Ok(stream) => {thread::spawn(move || {
                handle_client(stream)
                    .unwrap_or_else(
                        |e| eprintln!("Stream Error: {}", e));
            });}

        }
    }
    
}
