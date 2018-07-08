pub mod render_window;
pub mod style_scheme;
mod components;
use types::*;

use self::components::DrawableContainer;
pub use self::style_scheme::StyleScheme;
pub use self::render_window::RenderWindow;

use event::EventManagerBuilder;
use event::message::renderer::RendererMessage;
use event::message::gtk::GtkMessage;

use std::convert::AsRef;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread::JoinHandle;
use std::thread;
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
    BUTTON1_MOTION_MASK,

    EventConfigure,
    PROPERTY_CHANGE_MASK,

    EventScroll,
    SCROLL_MASK
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
   /// GTK drawing area on which the component will render all gui
   container: DrawingArea,
   /// Colorscheme used to render all objects
   style_scheme: Arc<RwLock<StyleScheme>>,
   /// Mapping from screen space to world space
   render_window: Arc<RwLock<RenderWindow>>,
   /// List of things to be drawn 
   draw_queue: Arc<RwLock<Vec<DrawableContainer>>>,
   // note: we need the rwlock as we don't know where the draw callback is called
   renderer_event_thread: JoinHandle<()>
}

impl AsRef<DrawingArea> for Renderer {
    fn as_ref(&self) -> &DrawingArea {
        &self.container
    }
}

impl Renderer {
    pub fn new(event_builder : &mut EventManagerBuilder, style_scheme: Arc<RwLock<StyleScheme>>) -> Renderer {
        let draw_queue : Arc<RwLock<Vec<DrawableContainer>>> = Arc::new(RwLock::new(Vec::new()));

        let drawing_area = DrawingArea::new();


        let render_window = Arc::new(RwLock::new(
                RenderWindow::new(
                    ScreenUnit(drawing_area.get_allocated_width() as f64), 
                    ScreenUnit(drawing_area.get_allocated_height() as f64)
                )
        ));

        let sender : Sender<GtkMessage> = event_builder.get_gdk_channel();

        drawing_area.add_events(BUTTON_PRESS_MASK.bits() as i32);
        drawing_area.add_events(BUTTON1_MOTION_MASK.bits() as i32);
        drawing_area.add_events(SCROLL_MASK.bits() as i32);

        {

                let drawing_area_ref = drawing_area.clone(); 
                drawing_area.connect_event(move |obj, event| { 

                if let Ok(ref result) = event.clone().downcast::<EventScroll>() {
                    let (x, y) = result.get_position();
                    let mut delta = 1.0;

                    let direction = result.get_direction();

                    let direction = match direction {
                        ::gdk::ScrollDirection::Up => {
                            delta = 1.0/1.1;     
                            Some(ScrollDirection::Up)
                        }
                        ::gdk::ScrollDirection::Down => {
                            delta = 1.1;    
                            Some(ScrollDirection::Down)
                        },
                        ::gdk::ScrollDirection::Smooth => {
                            let (x, y) = result.get_delta();
                            
                            if x > 0.0 {
                                delta = 1.0/1.1;
                                Some(ScrollDirection::Up)
                            } else {
                                delta = 1.1;
                                Some(ScrollDirection::Down)
                            }
                        }
                        _ => {
                            None
                        }
                    };

                    if let Some(dir) = direction {
                        sender.send(
                            GtkMessage::Scroll(
                                ScreenUnit(x as f64), 
                                ScreenUnit(y as f64),
                                dir,
                                delta
                            )
                        );
                    }


                    // slightly annoyying - have to queue redraw here? not the most logical
                    drawing_area_ref.queue_draw();
                }
                if let Ok(ref result) = event.clone().downcast::<EventButton>() {
                   println!("Could unwrap: {:?}", result.get_position()); 
                }
                if let Ok(ref result) = event.clone().downcast::<EventMotion>() {
                   println!("Motion unwrap: {:?}", result.get_position()); 
                } 
                if let Ok(ref result) = event.clone().downcast::<EventConfigure>() {
                    let (width, height) = result.get_size();

                    sender.send(
                        GtkMessage::RendererScreenResize(
                            ScreenUnit(width as f64), 
                            ScreenUnit(height as f64)
                        )
                    );
                }
                
                Inhibit(false) 

            });
        }


        {
            let draw_queue = draw_queue.clone();
            let style_scheme = style_scheme.clone();
            let render_window = render_window.clone();
            drawing_area.connect_draw(move |_, cr| {
                let style_scheme = style_scheme.read().unwrap();
                let render_window = render_window.read().unwrap();
                let draw_queue = draw_queue.read().unwrap();

                cr.set_source_rgb(0.3, 0.3, 0.3);
                cr.paint();

                let bounding_box = render_window.world_bounding_box();

                let start_x = (bounding_box.0).0; 
                let start_y = (bounding_box.1).0; 

                let end_x = (bounding_box.0 + bounding_box.2).0;
                let end_y = (bounding_box.1 + bounding_box.3).0;

                let mut x = 100.0 *  (start_x / 100.0).floor();
                let mut y = 100.0 *  (start_y / 100.0).floor();

                let mut point_1 = WorldCoords(WorldUnit(x), WorldUnit(start_y));
                let mut point_2 = WorldCoords(WorldUnit(x), WorldUnit(end_y));

                // cr.set_line_width(0.03);
                while x < end_x {
                    point_1.0 = WorldUnit(x);
                    point_2.0 = WorldUnit(x);

                    let ScreenCoords(ScreenUnit(x1), ScreenUnit(y1)) = render_window.world_to_screen(&point_1);
                    let ScreenCoords(ScreenUnit(x2), ScreenUnit(y2)) = render_window.world_to_screen(&point_2);

                    cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
                    cr.new_path();
                    cr.move_to(x1,y1);
                    cr.line_to(x2, y2);
                    cr.close_path();
                    cr.stroke();
                    x += 100.0;
                }

                let mut point_1 = WorldCoords(WorldUnit(start_x), WorldUnit(y));
                let mut point_2 = WorldCoords(WorldUnit(end_x), WorldUnit(y));

                while y < end_y {
                    point_1.1 = WorldUnit(y);
                    point_2.1 = WorldUnit(y);

                    let ScreenCoords(ScreenUnit(x1), ScreenUnit(y1)) = render_window.world_to_screen(&point_1);
                    let ScreenCoords(ScreenUnit(x2), ScreenUnit(y2)) = render_window.world_to_screen(&point_2);

                    cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
                    cr.new_path();
                    cr.move_to(x1,y1);
                    cr.line_to(x2, y2);
                    cr.close_path();
                    cr.stroke();
                    y += 100.0;
                }
 

                // main draw loop here
                // 1. draw background

                cr.rectangle(0.0, 0.0, 1.0, 1.0);
                cr.stroke();

                // 2. ask drawables to draw themselves

                for drawable in draw_queue.iter() {
                    drawable.draw(cr, &style_scheme, &render_window);
                }

                Inhibit(false)
            });
        }

        let (sender, receiver) : (Sender<RendererMessage>,Receiver<RendererMessage>) = mpsc::channel();
        
        event_builder.set_renderer_channel(sender);

        let renderer_event_thread = {
            let drawing_area = drawing_area.clone();
            let render_window = render_window.clone();
            thread::spawn(move || {
               for event in receiver.iter() {
                   match event {
                        RendererMessage::ResizeEvent(dimensions) => {
                            if let Ok(mut rw) = render_window.write() {
                                rw.update_screen_dimensions(dimensions);
                            }
                        },
                        RendererMessage::ScrollEvent(point, direction, delta) => {
                            println!("pos : {:?}, {:?}", point, direction);
                            if let Ok(mut rw) = render_window.write() {
                                rw.zoom_window(&point, direction, delta);
                            }
 
                        }
                   }
               }
            })
        };



        Renderer {
            container: drawing_area,
            render_window, 
            draw_queue,
            style_scheme,
            renderer_event_thread
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


