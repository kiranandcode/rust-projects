extern crate server;
use server::ThreadPool;
use std::fs::File;
use std::time::Duration;
use std::thread;
use std::io::{
    Read,
    Write
};
use std::net::{
    TcpListener,
    TcpStream
};


fn main() {
    let listener = TcpListener::bind("127.0.01:8000").unwrap();
    let pool  = ThreadPool::new(4);
    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
            println!("Shutting down");
            break;
        }
        counter += 1;
        let stream = stream.unwrap();
        pool.execute(move || {
        handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); 
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}",status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    }
