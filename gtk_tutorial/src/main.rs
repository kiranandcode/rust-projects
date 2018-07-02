extern crate gtk;
use std::process;

mod  tut_2;
mod tut_3;
mod tut_3_t;

fn main() {
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK application");
        process::exit(1);
    }

    let app = tut_3_t::App::new();

    app.run();
}
