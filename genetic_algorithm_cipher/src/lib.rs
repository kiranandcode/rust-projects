extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write, Read};
use std::path::Path;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;


pub fn get_corpus() -> Result<String, std::io::Error> {
    println!("Loading local copy of corpus");
    match open_local_copy() {
        Ok(mut file) => {
            let mut result : String = String::new();

            file.read_to_string(&mut result)?;

            Ok(result)
        },
        Err(e) => {
//            Ok("".to_string())
            let string = make_request()?;
            save_local_copy(&string)?;
            Ok(string)
        }
    }
}

fn open_local_copy() -> Result<std::fs::File, std::io::Error> {
    std::fs::File::open("corpus.txt")
}

fn save_local_copy(text : &String) -> Result<(), std::io::Error> {
    match open_local_copy() {
        Ok(mut file) => {
          write_local_copy(file, text)
        },
        Err(e) => {
            match e.kind() {
                NotFound => {
                  let mut file = std::fs::File::create("corpus.txt")?;
                  write_local_copy(file, text)
                },
                _ => Err(e.into())
            }
        }
    }
}


fn write_local_copy(mut local_file : std::fs::File, text : &String) -> Result<(), std::io::Error> {
    local_file.write_all(text.as_bytes())
}

pub fn make_request() -> Result<String,std::io::Error> {
    println!("Downloading Alice in Wonderland From Internet");
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let mut buf = String::new();

    let uri = "http://www.gutenberg.org/files/11/11.txt".parse().expect("Could not parse the url");
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

       let mut buf = String::new();
        res.body().concat2().map(|chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
            })
    });

    let result = core.run(work).expect("Error while running the task");

    Ok(result)
}
