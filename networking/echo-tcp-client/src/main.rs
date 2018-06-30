use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888")
        .expect("Err: could not open stream");

    loop {
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();
        std::io::stdin().read_line(&mut input)
            .expect("Err: could not read string");

        stream.write(input.as_bytes())
            .expect("Err: could not send back data");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer)
            .expect("Err: could not read response from server");
        print!("{}", str::from_utf8(&buffer)
               .expect("Err: could not parse response from server"));
    }
    

}
