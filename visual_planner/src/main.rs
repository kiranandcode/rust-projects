extern crate cairo;
extern crate gtk;
extern crate gdk;
mod graphics;
mod event;

use event::EventManager;
use graphics::gui::App;

fn main() {
    graphics::init();
    let mut event_builder = EventManager::new();
    let app = App::new(&mut event_builder);
    let event_manager = event_builder.build();

    EventManager::start(event_manager);
    graphics::run(&app);
}
