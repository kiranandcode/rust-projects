use render_window::RenderWindow;


use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;
use component_ui::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::env;

use gtk::{Window, WindowExt, WidgetExt, ContainerExt, Menu, MenuExt, MenuItem, MenuItemExt};
use gtk::{Application, ApplicationWindow, ApplicationWindowExt};
use gdk::EventMask;
use gio::{ApplicationFlags, ApplicationExt, ApplicationExtManual};


// main system for the thing
pub struct FlowGraph {
    // rendering components
    renderer: Rc<ComponentRenderer<ComponentUI>>,
    ui: Rc<ComponentUI>,
    // other gtk stuff

}


impl FlowGraph {
    pub fn new(window: &ApplicationWindow) -> Self {

        // generate a renderer
        let (renderer, ui) = FlowGraph::configure_renderer();

        // then connect up the rendering area to the window
        let render_area = renderer.get_drawing_area();
        window.add(&render_area);


        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            // main_quit ends the gtk event loop, thus prompting our
            // app to close - there's no need to run the default
            // handler
            gtk::Inhibit(false)
            // maybe also propagate to inner state - or handle here
        });



        // now we can also get a copy of the internal state
        {
            let state : &RefCell<Option<ComponentState>> = ui.get_state();
        }


        FlowGraph {
            renderer,
            ui
        }

    }

    fn configure_renderer() -> (Rc<ComponentRenderer<ComponentUI>>, Rc<ComponentUI>) {

        // first construct the renderer - this will manage the ui etc.
        let renderer = ComponentRenderer::<ComponentUI>::new_component_renderer();

        // and then construct the ui
        let ui = Rc::new(ComponentUI::new(COLOR_SCHEME));

        // register it to the renderer
        ComponentRenderer::set_component(&renderer, ui.clone());

        (renderer, ui)
    }

}
