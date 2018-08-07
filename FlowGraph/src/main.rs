#![feature(extern_prelude, try_from)]
extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate gio;
#[macro_use] extern crate dependent_view;

mod types;
mod render_window;
mod color;
mod drawing_context;
mod component_renderer;
mod component_ui;
mod flow_graph;

use component_ui::*;
use flow_graph::*;


use gtk::{Window, WindowExt, WidgetExt, ContainerExt, Menu, MenuExt, MenuItem, MenuItemExt};
use gtk::{Application, ApplicationWindow, ApplicationWindowExt};
use gio::{ApplicationFlags, ApplicationExt};

const APP_ID: &str = "com.flowgraph.gopiandcode";
const TITLE : &str = "FlowGraph";

fn main() {
    gtk::init();

    match Application::new(Some(APP_ID), ApplicationFlags::empty()) {
        Ok(app) => {
            //  initialize the application
            app.connect_activate(|app| init(app));

            // run the application
            app.run(&[]);
        }
        Err(e) => panic!("Failed to construct gtk app: {:?}", e)
    }

}


fn init(app: &Application) {
    // construct the main window for the application
    let window = ApplicationWindow::new(app);
    window.set_default_size(1080, 720);
    window.set_title(TITLE);


    // initialize the underlying application
    // and connect up the ui

    let mut graph = FlowGraph::new(&window);

    window.show_all();
}
