pub mod visualizer;
mod components;
mod render_window;
mod style_scheme;

use self::render_window::*;
use self::style_scheme::StyleScheme;

use gtk::{Window, WidgetExt, main, init as gtk_init};
use std::convert::AsRef;

pub fn init() {
    if let Err(err) = gtk_init() {
        panic!("ERROR: While initializing gtk - {}", err);    
    };
}
pub fn run<T: AsRef<Window>>(window : T) {
    let window = window.as_ref();
    window.show_all();
    main();
}



