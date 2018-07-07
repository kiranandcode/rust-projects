extern crate cairo;
extern crate gtk;
extern crate gdk;
mod graphics;

use graphics::gui::App;

fn main() {
    graphics::init();
    let app = App::new();
    graphics::run(&app);
}
