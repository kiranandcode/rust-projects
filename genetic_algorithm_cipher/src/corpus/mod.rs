extern crate futures;
extern crate hyper;
extern crate tokio_core;

use ::std::io::{Write, Read, Error, ErrorKind};
use ::std::fs::File;
use self::futures::{Future, Stream};
use self::hyper::Client;
use self::tokio_core::reactor::Core;


pub fn get_corpus() -> Result<String, Error> {
    println!("Loading local copy of corpus");
    match open_local_copy() {
        Ok(mut file) => {
            let mut result : String = String::new();

            file.read_to_string(&mut result)?;

            Ok(result)
        },
        Err(_e) => {
            let string = make_request()?;
            save_local_copy(&string)?;
            Ok(string)
        }
    }
}

fn open_local_copy() -> Result<File, Error> {
    File::open("corpus.txt")
}

fn save_local_copy(text : &String) -> Result<(), Error> {
    match open_local_copy() {
        Ok(file) => {
          write_local_copy(file, text)
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                  let file = File::create("corpus.txt")?;
                  write_local_copy(file, text)
                },
                _ => Err(e.into())
            }
        }
    }
}


fn write_local_copy(mut local_file : File, text : &String) -> Result<(), Error> {
    local_file.write_all(text.as_bytes())
}

fn make_request() -> Result<String,Error> {
    println!("Downloading Alice in Wonderland From Internet");
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = "http://www.gutenberg.org/files/11/11.txt".parse().expect("Could not parse the url");
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().concat2().map(|chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
        })
    });

    let result = core.run(work).expect("Error while running the task");

    Ok(result)
}
