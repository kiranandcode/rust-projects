use types::*;
use color::*;
use drawing_context::*;
use render_window::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryFrom;

use gtk::{Window, WindowExt, WidgetExt, ContainerExt};
use gdk::EventMask;


pub struct ComponentRenderer<T: Component> {
    render_window: RefCell<RenderWindow>,
    handle_drag: RefCell<bool>,
    last_pos: RefCell<Option<ScreenCoords>>,
    drawing_area: gtk::DrawingArea,
    renderer: Rc<T>
}


/// - - - - - - - - - - - - - - - - - - - - -
///                 Setup
/// - - - - - - - - - - - - - - - - - - - - -
impl<T:Component + 'static> ComponentRenderer<T> {

    pub fn new_component_renderer() -> Rc<ComponentRenderer<T>>  {
        let renderer = T::new();
        let drawing_area = ComponentRenderer::<T>::generate_drawing_area();
        let renderer = Rc::new(renderer);
        let self_rc = Rc::new(
            ComponentRenderer {
                render_window: RefCell::new(RenderWindow::new(ScreenUnit(200.0), ScreenUnit(200.0))),
                handle_drag: RefCell::new(true),
                last_pos: RefCell::new(None),
                drawing_area: drawing_area.clone(),
                renderer: renderer.clone()
            }
        );

        self_rc.renderer.register_renderer(renderer, self_rc.clone());


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


    fn connect_events(self_rc: &Rc<ComponentRenderer<T>>, drawing_area: gtk::DrawingArea) {
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
impl<T:Component> ComponentRenderer<T> {

    fn on_layout(&self, drawing_area: &gtk::DrawingArea, layout: &gtk::Allocation) {
        let mut rw = self.render_window.borrow_mut();
        rw.update_screen_dimensions(ScreenDimensions(ScreenUnit(layout.width as f64), ScreenUnit(layout.height as f64)));
    }

    fn on_button_press(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventButton) -> gtk::Inhibit {
        let (x,y) = evnt.get_position();
        let coords = self.render_window.borrow().screen_to_world_coords(&ScreenCoords(ScreenUnit(x), ScreenUnit(y)));
        let button_type = match evnt.get_button() {
            0 => ButtonType::Left,
            1 => ButtonType::Middle,
            2 => ButtonType::Right,
            _ => ButtonType::Left,
        };
        let button_press_type = match evnt.get_event_type() {
            gdk::EventType::ButtonPress => ButtonEventType::Click,
            gdk::EventType::ButtonPress => ButtonEventType::Release,
            gdk::EventType::DoubleButtonPress => ButtonEventType::DoubleClick,
            gdk::EventType::TripleButtonPress => ButtonEventType::TripleClick,
            _ => unreachable!()
        };

        gtk::Inhibit(self.renderer.on_button_press(ButtonEvent { pos: coords, button_type, button_press_type }))
    }

    fn on_button_release(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventButton) -> gtk::Inhibit {
        let (x,y) = evnt.get_position();
        let coords = self.render_window.borrow().screen_to_world_coords(&ScreenCoords(ScreenUnit(x), ScreenUnit(y)));
        let button_type = match evnt.get_button() {
            0 => ButtonType::Left,
            1 => ButtonType::Middle,
            2 => ButtonType::Right,
            _ => ButtonType::Left,
        };
        let button_press_type = match evnt.get_event_type() {
            gdk::EventType::ButtonPress => ButtonEventType::Click,
            gdk::EventType::ButtonPress => ButtonEventType::Release,
            gdk::EventType::DoubleButtonPress => ButtonEventType::DoubleClick,
            gdk::EventType::TripleButtonPress => ButtonEventType::TripleClick,
            _ => unreachable!()
        };


        gtk::Inhibit(self.renderer.on_button_release(ButtonEvent { pos: coords, button_type, button_press_type }))
    }

    fn on_key_press(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventKey) -> gtk::Inhibit {
        //println!("{:?} {:?}, {:?}, {:?}", evnt.get_keyval(), gdk::keyval_to_unicode(evnt.get_keyval()), evnt.get_state(), gdk::keyval_name(evnt.get_keyval()));
        if let Some(value) = TryFrom::try_from(evnt.get_keyval()).ok() {
            gtk::Inhibit(self.renderer.on_key_press(value))
        } else {
            gtk::Inhibit(false)
        }
    }

    fn on_key_release(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventKey) -> gtk::Inhibit {
        if let Some(value) = TryFrom::try_from(evnt.get_keyval()).ok() {
            gtk::Inhibit(self.renderer.on_key_press(value))
        } else {
            gtk::Inhibit(false)
        }
    }

    fn on_motion_notify(&self, drawing_area: &gtk::DrawingArea, evnt: &gdk::EventMotion) -> gtk::Inhibit {
        let (x, y) = evnt.get_position();
        let x = ScreenUnit(x as f64);
        let y = ScreenUnit(y as f64);
        let coords = self.render_window.borrow().screen_to_world_coords(&ScreenCoords(x,y));

        gtk::Inhibit(self.renderer.on_motion_notify(coords))
    }

    fn on_drag_motion_notify(&self, drawing_area: &gtk::DrawingArea, context: &gdk::DragContext, x : i32, y: i32,  dt: u32) -> gtk::Inhibit {
        let x = ScreenUnit(x as f64);
        let y = ScreenUnit(y as f64);
        if let Some(p_xy) = self.last_pos.borrow_mut().take() {
            let ScreenCoords(px, py) = p_xy;

            let dx  = px - x;
            let dy  = py - y;

            // the component renderer will handle all screen space stuff.
            // this includes moving the renderwindow around
            if *self.handle_drag.borrow() {
                self.render_window.borrow_mut().move_window(&dx, &dy);
            } else {
                let coords = self.render_window.borrow().screen_to_world_coords(&ScreenCoords(x,y));
                let dx = self.render_window.borrow().screen_to_world_distance_x(&dx);
                let dy = self.render_window.borrow().screen_to_world_distance_y(&dy);
                self.renderer.on_drag_motion_notify(coords, dx,dy);
            }
        }
        *self.last_pos.borrow_mut() = Some(ScreenCoords(x,y));
        gtk::Inhibit(true)
    }

    fn on_draw(&self, drawing_area: &gtk::DrawingArea, evnt: &cairo::Context) -> gtk::Inhibit {
        gtk::Inhibit(self.renderer.on_draw(&Context::new(evnt, &self.render_window.borrow())))
    }

    fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) -> bool {
        self.renderer.on_update(current_time, elapsed_time)
    }
}


pub trait Renderer {
    fn set_handle_drag(&self, handle_drag: bool);
    fn queue_draw(&self, area: WorldBoundingBox);
    fn move_view_to(&self, coords: WorldCoords);
}

impl<T:Component + 'static> Renderer for ComponentRenderer<T> {
    fn set_handle_drag(&self, handle_drag: bool) {
        *self.handle_drag.borrow_mut() = handle_drag;
    }
    fn queue_draw(&self, area: WorldBoundingBox) {
        self.drawing_area.queue_draw();
    }
    fn move_view_to(&self, coords :WorldCoords) {
        // self.render_window.borrow_mut().world_bounding_box.
    }
}



pub trait Component {
    fn new() -> Self;
    fn register_renderer(&self, self_rc: Rc<Self>, renderer: Rc<Renderer>);

    fn on_button_press(&self, evnt: ButtonEvent) -> bool { false }

    fn on_button_release(&self,  evnt: ButtonEvent) -> bool { false }

    fn on_key_press(&self,  evnt: Key) -> bool { false }

    fn on_key_release(&self, evnt: Key) -> bool { false }

    fn on_motion_notify(&self,  evnt: WorldCoords) -> bool { false }

    fn on_drag_motion_notify(&self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) -> bool {
        false
    }

    fn on_draw(&self, evnt: &Context) -> bool {
        false
    }

    fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) -> bool {
        true
    }
}
