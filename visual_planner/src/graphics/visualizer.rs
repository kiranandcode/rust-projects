use super::components::DrawableObject;
use super::RenderWindow;

use std::convert::AsRef;
use std::sync::{Arc, RwLock};

use gdk::{EventMask, EventType, EventButton, BUTTON_PRESS_MASK};
use gtk::{Window, WindowType, WindowExt, ContainerExt, WidgetExt, HeaderBar, HeaderBarExt, DrawingArea, Inhibit, main_quit};

pub enum Msg {
    
}

/// Window containing a visualizer
pub struct App {
   window: Window, 
   header: Header,
   content: Content
}


impl App {
    pub fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_title("GopViz - Visualizer");
        window.set_wmclass("app-name", "Gopviz");
        window.set_default_size(500, 500);


        // connect children
        window.set_titlebar(&header.container);
        window.add(&content.container);
        // params are self, envt
        window.connect_delete_event(move |_, _| {
            main_quit();
            // main_quit ends the gtk event loop, thus prompting our
            // app to close - there's no need to run the default 
            // handler
            Inhibit(false) 
        });
       
        App {
            window,
            header,
            content
        }
    }
}

impl AsRef<Window> for App {
    fn as_ref(&self) -> &Window {
        &self.window
    }
}


pub struct Header {
    container: HeaderBar
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title("GopViz - Visualizer");
        container.set_show_close_button(true);

        Header {
            container
        }
    }
}


pub struct Content {
   container: DrawingArea,
   render_window: Arc<RwLock<RenderWindow>>,
   draw_queue: Arc<Vec<DrawableObject>>
}

impl Content {
    fn new() -> Content {
        let drawing_area = DrawingArea::new();

        drawing_area.add_events(BUTTON_PRESS_MASK.bits() as i32);
        drawing_area.connect_event(|obj, event| { 
            //println!("event: {:?} {:?}", event, event.get_event_type()); 
            if let Ok(ref result) = event.clone().downcast::<EventButton>() {
               println!("Could unwrap: {:?}", result.get_position()); 
            }
            
            Inhibit(false) 

        });
        drawing_area.connect_draw(|_, cr| {
           // main draw loop here 
            // 1. draw background
            // 2. ask drawables to draw themselves
            
            Inhibit(false)
        });


        Content {
            container: drawing_area,
            render_window: Arc::new(RwLock::new(RenderWindow {})),
            draw_queue: Arc::new(Vec::new())
        }
    }
}


// reference cr drawing code:
//
//cr.set_dash(&[3., 2., 1.], 1.); 

// cr.scale(500f64, 500f64);

// cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
// cr.paint();

// cr.set_line_width(0.05);

// cr.set_source_rgb(0.3, 0.3, 0.3);
// cr.rectangle(0.0, 0.0, 1.0, 1.0);
// cr.stroke();

// cr.set_line_width(0.03);


// cr.arc(0.5, 0.5, 0.4, 0.0, ::std::f64::consts::PI * 2.);
// cr.stroke();


// let mouth_top = 0.68;
// let mouth_width = 0.38;
// let mouth_dx = 0.10;
// let mouth_dy = 0.10;


// cr.move_to(0.50 - mouth_width/2.0, mouth_top);
// cr.curve_to(0.50 - mouth_dx,     mouth_top + mouth_dy,
//             0.50 + mouth_dx,     mouth_top + mouth_dy,
//             0.50 + mouth_width/2.0, mouth_top);

// cr.stroke();

// let eye_y = 0.38;
// let eye_dx = 0.15;

// cr.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, ::std::f64::consts::PI * 2.);
// cr.fill();

// cr.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, ::std::f64::consts::PI * 2.);
// cr.fill();
//


