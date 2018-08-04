#![feature(extern_prelude, try_from)]
extern crate gtk;
extern crate gdk;
extern crate cairo;
mod types;
mod render_window;
mod color;
mod drawing_context;
mod component_renderer;

use render_window::RenderWindow;


use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryFrom;

use gtk::{Window, WindowExt, WidgetExt, ContainerExt};
use gdk::EventMask;

fn main() {
    gtk::init();


    let window = Window::new(gtk::WindowType::Toplevel);

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        // main_quit ends the gtk event loop, thus prompting our
        // app to close - there's no need to run the default 
        // handler
        gtk::Inhibit(false) 
    });
    let renderer = ComponentRenderer::<SimpleComponent>::new_component_renderer();
    window.add(&renderer.get_drawing_area());

    window.show_all();

    gtk::main();
}




struct SimpleComponent {}
impl Component for SimpleComponent {
    fn new() -> Self {
        SimpleComponent {}
    }

    fn register_renderer(&self, self_rc: Rc<Self>, renderer: Rc<Renderer>) {}
}
