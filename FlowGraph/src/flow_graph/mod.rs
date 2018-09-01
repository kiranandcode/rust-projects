use render_window::RenderWindow;


use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;
use component_ui::*;
use component_ui::object::Object;
use component_ui::context::*;
use component_ui::id::*;

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

pub struct Node {
    
}
impl Object for Node {
    // used to customize when drawn
    fn draw_priority(&self) -> DrawPriority {DrawPriority::Low}

    // used to catch mouse events
    fn mouse_bounding_box(&self) -> Option<&WorldBoundingBox> {None}

    // used to decide whether to be included in a draw call
    fn render_bounding_box(&self) -> Option<WorldBoundingBox> {None}

    // called to be drawn
    fn draw(&mut self, context: &Context, root: ID, color_scheme: &ColorScheme)  {}

    // handling mouse over events
    fn motion(&mut self, coords: WorldCoords, ctx: &mut HandlerContext) -> bool {false}

    // handling update events
    fn update(&mut self, current_time: CurrentTime, elapsed_time: DeltaTime, ctx: &mut HandlerContext) {}

    // handling drag events - can be stolen by children
    fn drag_motion(&mut self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit, ctx: &mut HandlerContext) -> bool {false}

    // handling click events - can be stolen by children
    fn button_press(&mut self, button: ButtonEvent, ctx: &mut HandlerContext) -> bool {false}

    // handling key events - can be stolen by children
    fn key_press(&mut self, evnt: Key, ctx: &mut HandlerContext) -> bool { false }

    // used to send an arbitrary payload to the widget
    fn poke(&mut self, payload: &mut Any, ctx: &mut HandlerContext) -> bool { false }

    // called when the node is created
    fn create(&mut self, ctx: &mut HandlerContext) {}

    // called when the node is removed
    fn delete(&mut self, ctx: &mut HandlerContext) {}

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
