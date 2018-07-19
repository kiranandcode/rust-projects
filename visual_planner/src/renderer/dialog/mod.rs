pub mod state;
pub use super::{StyleScheme, RenderWindow};

use types::*;
use ::state::*;

use self::state::DialogStateManager;
use manager::ModelManager;
use event::EventManagerBuilder;
use event::message::renderer::DialogRendererMessage;
use event::message::GeneralMessage;
use gui::manager::GuiManager;
use manager::draw_view::DrawView;


use std::convert::AsRef;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread::JoinHandle;
use std::thread;
use std::sync::{
    Arc, 
    RwLock, Mutex };

use gdk::{
    EventMask, 
    Event,
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
use cairo::Context;


pub struct DialogRenderer {
   /// GTK drawing area on which the component will render all gui
   container: DrawingArea,
   /// Colorscheme used to render all objects
   style_scheme: Arc<RwLock<StyleScheme>>,
   /// Mapping from screen space to world space
   render_window: Arc<RwLock<RenderWindow>>,
   /// List of things to be drawn 
   draw_queue: Arc<RwLock<Vec<DrawView>>>,
   // note: we need the rwlock as we don't know where the draw callback is called
   renderer_event_thread: JoinHandle<()>,

   state_manager: DialogStateManager,

    // id for the drawing area
    drawable_id: GuiWidgetID,
}

impl AsRef<DrawingArea> for DialogRenderer {
    fn as_ref(&self) -> &DrawingArea {
        &self.container
    }
}

impl DialogRenderer {


    pub fn new((event_builder, gui_manager): (&mut EventManagerBuilder, &mut GuiManager), style_scheme: Arc<RwLock<StyleScheme>>) -> DialogRenderer {
        let mut draw_queue = Vec::new();
        // draw_queue.push(DrawableContainer::new(Box::new(DialogView::new()))); 
        let draw_queue : Arc<RwLock<Vec<DrawView>>> = Arc::new(RwLock::new(draw_queue));

        let drawing_area = DrawingArea::new();
        let drawable_id = gui_manager.register_widget(drawing_area.clone());
        let render_window = Arc::new(RwLock::new(
                RenderWindow::new(
                    ScreenUnit(drawing_area.get_allocated_width() as f64), 
                    ScreenUnit(drawing_area.get_allocated_height() as f64)
                )
        ));

        let state_manager = {
            let drawable_id = drawable_id.clone();
            let render_window = render_window.clone();

            DialogStateManager::new(drawable_id, render_window, event_builder)
        };



        let sender : Sender<GeneralMessage> = event_builder.get_gdk_channel();

        drawing_area.add_events(BUTTON_PRESS_MASK.bits() as i32);
        drawing_area.add_events(BUTTON1_MOTION_MASK.bits() as i32);
        drawing_area.add_events(SCROLL_MASK.bits() as i32);

        {
                let sender = sender.clone();
                let drawing_area_ref = drawing_area.clone(); 
                drawing_area.connect_event(move |obj, event| handle_drawing_area_events(event, || sender.clone()));
        }


        {
            let draw_queue = draw_queue.clone();
            let style_scheme = style_scheme.clone();
            let render_window = render_window.clone();
            drawing_area.connect_draw(move |_, cr| handle_drawing_area_draw(cr,  || style_scheme.clone(), || render_window.clone(), || draw_queue.clone()));
        }

        let (dialog_sender, receiver) : (Sender<DialogRendererMessage>,Receiver<DialogRendererMessage>) = mpsc::channel();
        
        event_builder.set_dialog_renderer_channel(dialog_sender);

        let renderer_event_thread = {
            let render_window = render_window.clone();
            let sender = sender.clone();
            let draw_queue = draw_queue.clone();

            thread::spawn(move || dialog_renderer_message_handler(receiver, sender, render_window, drawable_id, draw_queue))
        };



        DialogRenderer {
            container: drawing_area,
            render_window, 
            draw_queue,
            style_scheme,
            renderer_event_thread,
            state_manager,
            drawable_id
        }
    }
}



fn handle_drawing_area_events<F>(event : &Event, sender: F) -> Inhibit
    where F : Fn() -> Sender<GeneralMessage> {
            let sender = sender();
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
                            GeneralMessage::RendererScroll(
                                ScreenUnit(x as f64), 
                                ScreenUnit(y as f64),
                                dir,
                                delta
                            )
                        );
                    }

                }
                if let Ok(ref result) = event.clone().downcast::<EventButton>() {
                    let (x, y) = result.get_position();
                    sender.send(
                        GeneralMessage::RendererClick(
                            ScreenUnit(x as f64), 
                            ScreenUnit(y as f64)
                        )
                    );
 
                }
                if let Ok(ref result) = event.clone().downcast::<EventMotion>() {
                   
                    let (x, y) = result.get_position();

                    sender.send(
                        GeneralMessage::RendererMotion(
                            ScreenUnit(x as f64), 
                            ScreenUnit(y as f64)
                        )
                    );
 

                } 
                if let Ok(ref result) = event.clone().downcast::<EventConfigure>() {
                    let (width, height) = result.get_size();

                    sender.send(
                        GeneralMessage::RendererScreenResize(
                            ScreenUnit(width as f64), 
                            ScreenUnit(height as f64)
                        )
                    );
                }
                
                Inhibit(false) 
}

fn handle_drawing_area_draw<F,G,H>(cr : &Context, style_scheme : F, render_window : G, draw_queue : H) -> Inhibit 
    where F : Fn() -> Arc<RwLock<StyleScheme>>,
          G : Fn() -> Arc<RwLock<RenderWindow>>,
          H : Fn() -> Arc<RwLock<Vec<DrawView>>>
    {
            let style_scheme = style_scheme();
            let render_window = render_window();
            let draw_queue = draw_queue();

                let style_scheme = style_scheme.read().unwrap();
                let render_window = render_window.read().unwrap();
                let draw_queue = draw_queue.read().unwrap();

                cr.set_source_rgba(style_scheme.bg.red, style_scheme.bg.green, style_scheme.bg.blue, style_scheme.bg.alpha);
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

                    cr.set_source_rgba(style_scheme.bg_mid.red, style_scheme.bg_mid.green, style_scheme.bg_mid.blue, style_scheme.bg_mid.alpha);
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

                    cr.set_source_rgba(style_scheme.bg_mid.red, style_scheme.bg_mid.green, style_scheme.bg_mid.blue, style_scheme.bg_mid.alpha);
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
}

fn dialog_renderer_message_handler(receiver : Receiver<DialogRendererMessage>, sender : Sender<GeneralMessage>, render_window : Arc<RwLock<RenderWindow>>, drawable_id : GuiWidgetID, draw_queue : Arc<RwLock<Vec<DrawView>>>) {


               for event in receiver.iter() {
                   match event {
                        DialogRendererMessage::ResizeEvent(dimensions) => {
                            if let Ok(mut rw) = render_window.write() {
                                rw.update_screen_dimensions(dimensions);
                            }
                        },
                        DialogRendererMessage::ScrollEvent(point, direction, delta) => {
                            if let Ok(mut rw) = render_window.write() {
                                rw.zoom_window(&point, direction, delta);
                                sender.send(
                                    GeneralMessage::Redraw(drawable_id.clone())
                                );
                            }
                        },
                        DialogRendererMessage::WindowMoveEvent(x,y) => {
                            if let Ok(mut rw) = render_window.write() {
                                rw.move_window(&x,&y);

                                sender.send(
                                    GeneralMessage::Redraw(drawable_id.clone())
                                );
                            }
                        }
                        DialogRendererMessage::RegisterDrawable(drawable) => {
                            let drawable = DrawView::new(drawable);
                            let mut draw_queue = draw_queue.write().unwrap();
                            draw_queue.push(drawable);

                            sender.send(
                                GeneralMessage::Redraw(drawable_id.clone())
                            );
                        }
                   }
               }
            }