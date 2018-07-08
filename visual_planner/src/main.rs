extern crate cairo;
extern crate gtk;
extern crate gdk;

mod input;
mod renderer;
mod gui;
mod types;
mod event;

use event::EventManager;
use gui::App;


fn main() {
    gui::init();
    let mut event_builder = EventManager::new();

    let app = App::new(&mut event_builder);

    let event_manager = event_builder.build();

    EventManager::start(event_manager);

    gui::run(&app);
}
