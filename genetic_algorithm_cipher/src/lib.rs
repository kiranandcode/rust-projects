extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write, Read};
use std::path::Path;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;


pub fn get_corpus() -> Result<String, std::io::Error> {
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

pub fn make_request() -> Result<(),std::io::Error> {
    println!("Making a request");
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = "http://www.gutenberg.org/files/11/11.txt".parse().expect("Could not parse the url");
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map(|_| {})
                .map_err(From::from)
        })
    });

    core.run(work).expect("Error while running the task");

    Ok({})

}
