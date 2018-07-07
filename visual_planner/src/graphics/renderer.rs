use super::components::DrawableContainer;
use super::{
    RenderWindow, 
    StyleScheme
};

use std::convert::AsRef;
use std::sync::{
    Arc, 
    RwLock,
    Mutex
};

use gdk::{
    EventMask, 
    EventType, 

    // the following two imports are for handling button clicks
    EventButton, 
    BUTTON_PRESS_MASK,

    // the following two imports are for handling drags
    EventMotion,
    BUTTON1_MOTION_MASK
};
use gtk::{
    Window,              // for the main app
    WindowType,          // Window::new(WindowType...
    WindowExt,           // window.set_title_bar 
    ContainerExt,        // window.add
    WidgetExt,           // 
    HeaderBar,           // for the header
    HeaderBarExt,        // header.set_show_close_button(true)
    DrawingArea,         // for cairo drawing
    Inhibit,             // returned from all callbacks to toggle default handling - Inhibit(false)
    main_quit,           // end the app
    StyleContext         // used for initializing the stylescheme
};


pub struct Renderer {
   /// GTK drawing area on which the component will render all graphics
   container: DrawingArea,
   /// Colorscheme used to render all objects
   style_scheme: Arc<RwLock<StyleScheme>>,
   /// Mapping from screen space to world space
   render_window: Arc<RwLock<RenderWindow>>,
   /// List of things to be drawn 
   draw_queue: Arc<RwLock<Vec<DrawableContainer>>> 
   // note: we need the rwlock as we don't know where the draw callback is called
}

impl AsRef<DrawingArea> for Renderer {
    fn as_ref(&self) -> &DrawingArea {
        &self.container
    }
}

impl Renderer {
    pub fn new(render_window : Arc<RwLock<RenderWindow>>, style_scheme: Arc<RwLock<StyleScheme>>) -> Renderer {
        let drawing_area = DrawingArea::new();
        let draw_queue : Arc<RwLock<Vec<DrawableContainer>>> = Arc::new(RwLock::new(Vec::new()));


        drawing_area.add_events(BUTTON_PRESS_MASK.bits() as i32);
        drawing_area.add_events(BUTTON1_MOTION_MASK.bits() as i32);
        drawing_area.connect_event(|obj, event| { 
            //println!("event: {:?} {:?}", event, event.get_event_type()); 
            if let Ok(ref result) = event.clone().downcast::<EventButton>() {
               println!("Could unwrap: {:?}", result.get_position()); 
            }
            if let Ok(ref result) = event.clone().downcast::<EventMotion>() {
               println!("Motion unwrap: {:?}", result.get_position()); 
            }
            
            Inhibit(false) 

        });
        {
            let draw_queue = draw_queue.clone();
            let style_scheme = style_scheme.clone();
            let render_window = render_window.clone();
            drawing_area.connect_draw(move |_, cr| {
                // main draw loop here 
                // 1. draw background
                cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
                cr.paint();

               

                cr.set_source_rgb(0.3, 0.3, 0.3);
                cr.rectangle(0.0, 0.0, 1.0, 1.0);
                cr.stroke();




                // 2. ask drawables to draw themselves
                let style_scheme = style_scheme.read().unwrap();
                let render_window = render_window.read().unwrap();
                let draw_queue = draw_queue.read().unwrap();
                
                for drawable in draw_queue.iter() {
                    drawable.draw(cr, &style_scheme, &render_window);
                }

                Inhibit(false)
            });
        }


        Renderer {
            container: drawing_area,
            render_window, 
            draw_queue,
            style_scheme
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


