extern crate cairo;
extern crate gtk;
extern crate gdk;

mod input;
mod renderer;
mod gui;
mod types;
mod event;
mod manager;

use event::EventManager;
use gui::App;
use gtk::{Settings, SettingsExt, StyleContext, StyleContextExt};


fn main() {

    gui::init();

    println!("{:?}", Settings::get_default().unwrap().get_property_gtk_color_palette());

    let mut event_builder = EventManager::new();

    let app = App::new(&mut event_builder);

    let event_manager = event_builder.build();

    EventManager::start(event_manager);

    gui::run(&app);

    println!("Fin");
}
