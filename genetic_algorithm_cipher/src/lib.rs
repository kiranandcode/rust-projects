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
            Ok("".to_string())
        }
    }
}

fn open_local_copy() -> Result<std::fs::File, std::io::Error> {
    std::fs::File::open("corpus.txt")
}

pub fn make_request() -> Result<String,std::io::Error> {
    println!("Making a request");
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
