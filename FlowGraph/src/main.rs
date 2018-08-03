extern crate gtk;
extern crate gdk;
extern crate cairo;
mod types;
mod render_window;
mod color;
mod drawing_context;

use render_window::RenderWindow;

use types::*;
use color::*;
use drawing_context::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use std::cell::RefCell;

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
    let renderer = ComponentRenderer::new_component_renderer();
    window.add(&renderer.get_drawing_area());

    window.show_all();

    gtk::main();
}


struct ComponentRenderer<T: Renderer> {
    render_window: RefCell<render_window::RenderWindow>,
    handle_drag: RefCell<bool>,
    last_pos: RefCell<Option<ScreenCoords>>,
    drawing_area: gtk::DrawingArea,
    renderer: RefCell<T>
}


/// - - - - - - - - - - - - - - - - - - - - -
///                 Setup
/// - - - - - - - - - - - - - - - - - - - - -
impl<T:Renderer> ComponentRenderer<T> {

    pub fn new_component_renderer() -> Rc<ComponentRenderer<T>>  {
        let drawing_area = ComponentRenderer::generate_drawing_area();
        let self_rc = Rc::new(
            ComponentRenderer {
                render_window: RefCell::new(RenderWindow::new(ScreenUnit(200.0), ScreenUnit(200.0))),
                handle_drag: RefCell::new(true),
                last_pos: RefCell::new(None),
                drawing_area: drawing_area.clone()
            }
        );

        ComponentRenderer::connect_events(&self_rc, drawing_area);

        self_rc
    }

    pub fn get_drawing_area(&self) -> gtk::DrawingArea {
        self.drawing_area.clone()
    }

    fn generate_drawing_area() -> gtk::DrawingArea {
        let drawing_area = gtk::DrawingArea::new();
        let event_mask = gdk::POINTER_MOTION_MASK
            | gdk::BUTTON_PRESS_MASK | gdk::BUTTON_RELEASE_MASK
            | gdk::KEY_PRESS_MASK | gdk::KEY_RELEASE_MASK;

        drawing_area.set_can_focus(true);
        drawing_area.add_events(event_mask.bits() as i32);


        // establish a reasonable minimum view size
        drawing_area.set_size_request(800, 450);
        drawing_area        
    }


    fn connect_events(self_rc: &Rc<ComponentRenderer>, drawing_area: gtk::DrawingArea) {
        macro_rules! connect {
            ($connect:ident :> $action:ident) => {{
                let self_rc = self_rc.clone();
                drawing_area.$connect(
                    move |a, b| self_rc.$action(a, b)
                );
            }};
            ($connect:ident :5> $action:ident) => {{
                let self_rc = self_rc.clone();
                drawing_area.$connect(
                    move |a, b, c, d, e| self_rc.$action(a, b, c, d, e)
                );
            }}

        }

        connect!(connect_draw :> on_draw);
        connect!(connect_size_allocate :> on_layout);
        connect!(connect_button_press_event :> on_button_press);
        connect!(connect_button_release_event :> on_button_release);
        connect!(connect_key_press_event :> on_key_press);
        connect!(connect_key_release_event :> on_key_release);
        connect!(connect_motion_notify_event :> on_motion_notify);
        connect!(connect_drag_motion :5> on_drag_motion_notify);




        {
            let mut start_time = SystemTime::now();
            let mut last_time = 0.0;

            let self_rc = self_rc.clone();
            gtk::timeout_add(15, move || {
                let current_time = {
                    let current = SystemTime::now();
                    let since_the_epoch = current.duration_since(start_time).expect("Time Error");
                    ((since_the_epoch.as_secs() * 1000) as f64) + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0)
                };
                let elapsed_time = current_time - last_time;
                last_time = current_time;

                gtk::Continue(self_rc.on_update(TimeUnit(current_time), TimeUnit(elapsed_time)))
            });
        }

    }
}


/// - - - - - - - - - - - - - - - - - - - - -
///                 Actions
/// - - - - - - - - - - - - - - - - - - - - -
impl ComponentRenderer {

    pub fn on_layout(&self, drawing_area: &gtk::DrawingArea, layout: &gtk::Allocation) {
        let mut rw = self.render_window.borrow_mut();
        rw.update_screen_dimensions(ScreenDimensions(ScreenUnit(layout.width as f64), ScreenUnit(layout.height as f64)));
    }

    pub fn on_button_press(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventButton) -> gtk::Inhibit { gtk::Inhibit(false)}

    pub fn on_button_release(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventButton) -> gtk::Inhibit { gtk::Inhibit(false)}

    pub fn on_key_press(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventKey) -> gtk::Inhibit { gtk::Inhibit(false)}

    pub fn on_key_release(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventKey) -> gtk::Inhibit { gtk::Inhibit(false)}

    pub fn on_motion_notify(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventMotion) -> gtk::Inhibit { gtk::Inhibit(false)}

    pub fn on_drag_motion_notify(&self, drawing_area: &gtk::DrawingArea, context: &gdk::DragContext, x : i32, y: i32,  dt: u32) -> gtk::Inhibit {
        let x = ScreenUnit(x as f64);
        let y = ScreenUnit(y as f64);
        if let Some(p_xy) = self.last_pos.borrow_mut().take() {
            let ScreenCoords(px, py) = p_xy;

            let dx  = px - x;
            let dy  = py - y;


            if *self.handle_drag.borrow() {
                self.render_window.borrow_mut().move_window(&dx, &dy);
            } else {
            }
        }
        *self.last_pos.borrow_mut() = Some(ScreenCoords(x,y));
        gtk::Inhibit(true)
    }

    pub fn on_draw(&self, drawing_area: &gtk::DrawingArea, evnt: &cairo::Context) -> gtk::Inhibit {

        Context::new(evnt, &self.render_window.borrow());
        gtk::Inhibit(false)


    }

    pub fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) -> bool {
        true
    }
}


trait Renderer {
    fn set_handle_drag(&self, handle_drag: bool);
    fn queue_draw(&self, area: WorldBoundingBox);
}

trait Component {
    fn new() -> Self;
    fn register_renderer(&mut self, renderer: Rc<Renderer>);

    fn on_button_press(&mut self, evnt: &gdk::EventButton) -> gtk::Inhibit { gtk::Inhibit(false)}

    fn on_button_release(&mut self,  evnt: &gdk::EventButton) -> gtk::Inhibit { gtk::Inhibit(false)}

    fn on_key_press(&mut self,  evnt: &gdk::EventKey) -> gtk::Inhibit { gtk::Inhibit(false)}

    fn on_key_release(&mut self, evnt: &gdk::EventKey) -> gtk::Inhibit { gtk::Inhibit(false)}

    fn on_motion_notify(&mut self,  evnt: &gdk::EventMotion) -> gtk::Inhibit { gtk::Inhibit(false)}

    fn on_drag_motion_notify(&mut self, context: &gdk::DragContext, x : i32, y: i32,  dt: u32) -> gtk::Inhibit {
        gtk::Inhibit(false)
    }

    fn on_draw(&mut self, evnt: &Context) -> gtk::Inhibit {

        gtk::Inhibit(false)


    }

    fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) -> bool {
        true
    }
}



