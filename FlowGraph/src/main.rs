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




struct SimpleComponent {
    renderer: RefCell<Option<Rc<Renderer>>>,
}
impl Default for SimpleComponent {
    fn default() -> Self {
        SimpleComponent{
            renderer: RefCell::new(None)
        }
    }
}
impl Component for SimpleComponent {
    fn on_draw(&self, context: &Context) -> bool {

        context.color(Color::red);
        context.paint();

        context.color(Color::blue);
        context.rect(WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(20.0), WorldUnit(20.0)));
        context.fill();

        true
    }

    fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) -> bool {
        true
    }

    fn on_setup(&self) {
        if let Some(renderer) = self.renderer.borrow().as_ref() {
            renderer.move_view_to(WorldCoords(WorldUnit(0.0), WorldUnit(0.0)));
            renderer.set_handle_drag(true);
        }
    }

    fn on_drag_motion_notify(&self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) -> bool {
        println!("Recieved a drag motion event: {:?}", (coords, dx,dy));
        true
    }

    fn on_button_press(&self, button: ButtonEvent) -> bool {
        println!("ButtonClick");
        true
    }

    fn on_key_press(&self, evnt: Key) -> bool {
        if let Key::Char(chr) = evnt {
            if let Some(ref renderer) = self.renderer.borrow().as_ref() {
                if chr == ' ' {
                    renderer.set_handle_drag(false);
                } else {
                    renderer.set_handle_drag(true);
                }
            }
        }
        true
    }

    fn register_renderer(&self, self_rc: Rc<Self>, renderer: Rc<Renderer>) {
        *self.renderer.borrow_mut() = Some(renderer);
    }
}
