# Rust Networking Answers
1.  `use std::net::{TcpListener, TcpStream};`

2. See below:
```
let mut tcp_stream : TcpStream;
let mut buf = [0; 512];
let byte_count = stream.read(&mut buf)?;
// buf now contains byte_count no of bytes from the stream
```

3. See below:
```
let listener : TcpListener;    

for stream in listener.incoming() {
    match stream {
        Err(e) => // error handling
        Ok(stream) => {
            // Add stream code here

        }
    }
}
```

4. See `networking/echo_tcp_server/`

5. `nc "127.0.0.1" 8888`

6. `TcpStream::connect("127.0.0.1:8888")?`

7. See below:
```
let input : String;
let mut stream = TcpStream::connect("127.0.0.1:8888")?;
stream.write(input.as_bytes())?;
```

8. See below:
```
let mut buffer : Vec<u8> = Vec::new();
let mut reader = BufReader::new(&stream);
reader.read_until(b'\n', &mut buffer)?;
str::from_utf8(&buffer)
```

9. See `networking/echo_tcp_client/`.
