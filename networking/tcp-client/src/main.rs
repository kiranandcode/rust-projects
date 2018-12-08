use std::net::TcpStream;
use std::net::TcpListener;
use std::str;
use std::io::{self, BufRead, BufReader, Write};


fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:8888")
        .expect("Could not connect to provided server");

    loop {
        let mut input = String::new();
        let mut buffer:Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input)
            .expect("could not read from stdin");
        stream.write(input.as_bytes())
            .expect("Failed to send response to server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");

        println!("{}", str::from_utf8(&buffer)
                 .expect("Could not write response from server into string"));
    
    }
}
