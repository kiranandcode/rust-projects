extern crate cairo;
extern crate gtk;
mod graphics;

use graphics::visualizer::App;

fn main() {
    graphics::init();
    let app = App::new();
    graphics::run(&app);
}
