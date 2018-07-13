#![allow(dead_code)]
extern crate cairo;
extern crate gtk;
extern crate gdk;

mod input;
mod renderer;
mod gui;
mod types;
mod event;
mod manager;
mod render_window;
mod style_scheme;
mod state;

use event::EventManager;
use gui::App;
use gui::manager::GuiManager;
use gtk::{Settings, SettingsExt, StyleContext, StyleContextExt};


fn main() {

    gui::init();

    println!("{:?}", Settings::get_default().unwrap().get_property_gtk_color_palette());

    let mut event_builder = EventManager::new();
    let mut gui_manager = GuiManager::new(&mut event_builder);

    let app = App::new((&mut event_builder, &mut gui_manager));

    let event_manager = event_builder.build();

    EventManager::start(event_manager);

    app.run();


    loop {
        gui_manager.run_iteration();
        ::gtk::main_iteration();
    }
    println!("Fin");
}
