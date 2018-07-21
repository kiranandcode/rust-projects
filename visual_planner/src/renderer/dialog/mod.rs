pub mod state;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::Continue;
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

use cairo::{
    ImageSurface,
    Format
};
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
    SCROLL_MASK,
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
    StyleContext,         // used for initializing the stylescheme
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
        // draw queue - the list of Items to be drawn by the worker threads
        // TODO: THIS DOESN'T need to be thread safe, remove it.
        let draw_queue : Arc<RwLock<Vec<DrawView>>> = Arc::new(RwLock::new(Vec::new()));


        let drawing_area = DrawingArea::new();
        // Register it into our gui manager 
        // while we don't use the gui manager to queue redraws anymore, it is still used for handy utilities such as changing the cursor
        let drawable_id = gui_manager.register_widget(drawing_area.clone());


        // While GTK's Cairo context provides a userspace to screen space conversion mechanism,
        // as most of our drawing is done on a seperate worker thread, we need our own render window.
        // + the cairo context can't be shared between threads, while this can.
        let render_window = Arc::new(RwLock::new(
                RenderWindow::new(
                    ScreenUnit(drawing_area.get_allocated_width() as f64), 
                    ScreenUnit(drawing_area.get_allocated_height() as f64)
                )
        ));



        // This channel will be used by the whole system to send info to the worker thread.
        let (dialog_sender, receiver) : (Sender<DialogRendererMessage>,Receiver<DialogRendererMessage>) = mpsc::channel();


        let state_manager = {
            let drawable_id = drawable_id.clone();
            let render_window = render_window.clone();

            DialogStateManager::new(drawable_id, render_window, event_builder)
        };



        // These channels are used to communicate between the worker drawing threads and the gtk renderer
        // the worker threads recieve messages from the rest of the system, and draw anything that needs to be drawn onto a buffer.
        // the buffers are then sent via this channel to the refresh callback (called at 30 fps), which upon activation, paints the changes
        // onto the main buffer, and queues a redraw.
        let (buffer_sender, buffer_receiver) = mpsc::channel::<((Box<[u8]>, i32, i32, i32), (f64, f64))>();

        // the main drawing buffer - this is updated at 30 fps by the refresher callback, and whenever a section of the screen is invalidated,
        // it is used to redraw any lost regions
        let mut draw_buffer : Rc<RefCell<ImageSurface>>  = {
            let (width, height) = (drawing_area.get_allocated_width(), drawing_area.get_allocated_height());
            let draw_buffer = ImageSurface::create(Format::Rgb24, width, height).expect("Could not create an image buffer");
            Rc::new(RefCell::new(draw_buffer))
        };

        let sender : Sender<GeneralMessage> = event_builder.get_gdk_channel();


        // We want to handle 
        //  - button press events
        drawing_area.add_events(BUTTON_PRESS_MASK.bits() as i32);
        //  - button movement events
        drawing_area.add_events(BUTTON1_MOTION_MASK.bits() as i32);
        //  - scroll events
        drawing_area.add_events(SCROLL_MASK.bits() as i32);




        // Input handling callback
        // The following callback is intended to recieve all input events to this drawing area,
        // it forwards general inputs (mouse presses, etc) to the event bus where they can be handled by the whole system
        // localized events (such as screen resizes can be handled inhouse) and don't need to be propagated upper into the system
        // TODO: Remove the sending of the resize event.
        {
                let draw_buffer = draw_buffer.clone();
                let sender = sender.clone();
                let dialog_sender = dialog_sender.clone();
                let drawing_area_ref = drawing_area.clone();
                let render_window = render_window.clone();
                let draw_queue = draw_queue.clone();
                let style_scheme = style_scheme.clone();



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
                            sender.send( GeneralMessage::RendererScroll( ScreenUnit(x as f64), ScreenUnit(y as f64), dir, delta));
                        }

                    }


                    // Just propagate button clicks upwards - we don't know enough to deal with them
                    if let Ok(ref result) = event.clone().downcast::<EventButton>() {
                        let (x, y) = result.get_position();
                        sender.send( GeneralMessage::RendererClick( ScreenUnit(x as f64), ScreenUnit(y as f64)));
                    }

                    if let Ok(ref result) = event.clone().downcast::<EventMotion>() {
                        let (x, y) = result.get_position();
                        sender.send( GeneralMessage::RendererMotion( ScreenUnit(x as f64), ScreenUnit(y as f64)));
                    } 

                    // on resize event - it's slightly expensive as it tries to do a screen draw on the main thread.
                    if let Ok(ref result) = event.clone().downcast::<EventConfigure>() {
                        let (width, height) = result.get_size();
                        let mut draw_buffer = draw_buffer.borrow_mut();
                        let dimensions = ScreenDimensions(ScreenUnit(width as f64), ScreenUnit(height as f64));



                        // when a resize event occurs, as it isn't necassarily connected to the refresh cycle (we need to send the request to 
                        // the worker thread, and then wait for the refresh callback to be called to update the main buffer), which leads to a 
                        // brief black screens this is sufficient - but I don't settle for that trash.
                        // so, to avoid this, whenever a resize occurs, I'm going to try and bypass the refresh cycle, and (slightly expensively)
                        // redraw the screen from the update screen
                        let update_success = if let (Ok(ref mut rw), Ok(ref draw_queue), Ok(ref style_scheme)) = (render_window.try_write(), draw_queue.try_read(), style_scheme.try_read()) {
                            // first update the shared state - the render_window
                            rw.update_screen_dimensions(dimensions.clone());


                            let bounding_box = rw.world_bounding_box();  
                            let ((data, width, height, stride), (x, y)) = render_screen(draw_queue, rw, bounding_box, style_scheme);
                            if let Ok(surface) = ImageSurface::create_for_data(data, |data| {}, Format::Rgb24, width, height, stride) {
                                *draw_buffer = surface;
                                drawing_area_ref.queue_draw();
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
        
                        if !update_success {
                            *draw_buffer = ImageSurface::create(Format::Rgb24, width as i32, height as i32).expect("Could not create an image buffer");
                        }

                        // now, send the resize event straight to the worker thread for instant updates.
                        dialog_sender.send(DialogRendererMessage::ResizeEvent(dimensions)).expect("Didn't get anything");


                    }
                            
                            Inhibit(false) 
                        });
                }



                // GTK Drawing handler - called whenever gtk needs to redraw anything 
                // we are passed a cr context that has already been clipped to the region that needs to be drawn. 
                // this callback simply fills the region that needs to be painted with the corresponding data in the main buffer.
        // the buffer would have been drawn to already by the worker thread, and updated by the 30fps refresh callback.
        {
            let draw_buffer = draw_buffer.clone();
            // let draw_queue = draw_queue.clone();
            // let style_scheme = style_scheme.clone();
            // let render_window = render_window.clone();
            // drawing_area.connect_draw(move |_, cr| handle_drawing_area_draw(cr,  || style_scheme.clone(), || render_window.clone(), || draw_queue.clone()));
            drawing_area.connect_draw(move |_, cr| {
                let db = draw_buffer.borrow();
                cr.set_source_surface(&*db, 0.0, 0.0);
                cr.paint();

                Inhibit(true)
            });
        }

        
        event_builder.set_dialog_renderer_channel(dialog_sender);



        // Worker Drawing thread
        // This thread is the real workhorse of this system.
        // It primarily keeps a personal draw queue which contains every drawable entity in the system.
        // Upon updating, it 

        let renderer_event_thread = {
            let render_window = render_window.clone();
            let draw_queue = draw_queue.clone();
            let style_scheme = style_scheme.clone();

            // thread::spawn(move || dialog_renderer_message_handler(receiver, sender, render_window, drawable_id, draw_queue))
            thread::spawn(move || {


               for event in receiver.iter() {
                   match event {
                       // on getting a resize event, we need to redraw the entire screen, 
                        DialogRendererMessage::ResizeEvent(dimensions) => {
                            if let Ok(mut rw) = render_window.write() {
                                // first update the shared state - the render_window
                                rw.update_screen_dimensions(dimensions);
                            }

                            // Only one writer is allowed at a time, so as we no longer need to modify the shared state, 
                            // let's get a read only reference instead
                            if let (Ok(ref rw), Ok(ref draw_queue), Ok(ref style_scheme)) = (render_window.read(), draw_queue.read(), style_scheme.read()) {
                                // now, redraw the entire screen as the entire screen has been invalidated.
                                let bounding_box = rw.world_bounding_box();  
                                let ((data, width, height, stride), (x, y)) = render_screen(draw_queue, rw, bounding_box, style_scheme);
                                buffer_sender.send(((data, width, height, stride), (x, y)));
                            }
                        },
                        DialogRendererMessage::ScrollEvent(point, direction, delta) => {
                            if let Ok(mut rw) = render_window.write() {
                                // first update the shared state, the render_window
                                rw.zoom_window(&point, direction, delta);

                            }

                            // Only one writer is allowed at a time, so as we no longer need to modify the shared state, 
                            // let's get a read only reference instead
                            if let (Ok(ref rw), Ok(ref draw_queue), Ok(ref style_scheme)) = (render_window.read(), draw_queue.read(), style_scheme.read()) {
                                // now, redraw the entire screen as the entire screen has been invalidated.
                                let bounding_box = rw.world_bounding_box();  
                                let ((data, width, height, stride), (x, y)) = render_screen(draw_queue, rw, bounding_box, style_scheme);
                                buffer_sender.send(((data, width, height, stride), (x, y)));
 

                                // sender.send(
                                //     GeneralMessage::Redraw(drawable_id.clone())
                                // );
                            }
                        },
                        DialogRendererMessage::WindowMoveEvent(x,y) => {

                            if let Ok(mut rw) = render_window.write() {
                                // update the shared state
                                rw.move_window(&x,&y);
                            }

                            // Only one writer is allowed at a time, so as we no longer need to modify the shared state, 
                            // let's get a read only reference instead
                            if let (Ok(ref rw), Ok(ref draw_queue), Ok(ref style_scheme)) = (render_window.read(), draw_queue.read(), style_scheme.read()) {
                                // now, redraw the entire screen as the entire screen has been invalidated.
                                let bounding_box = rw.world_bounding_box();  
                                let ((data, width, height, stride), (x, y)) = render_screen(draw_queue, rw, bounding_box, style_scheme);
                                buffer_sender.send(((data, width, height, stride), (x, y)));
                            }

                        }
                        DialogRendererMessage::RegisterDrawable(drawable) => {
                            let drawable = DrawView::new(drawable);

                            // first, update the shared state - important for the dialog state
                            if let Ok(mut draw_queue) = draw_queue.write() {
                                draw_queue.push(drawable);
                            }


                            // now, get a read only copy of the relevant components
                            if let (Ok(ref rw), Ok(ref draw_queue), Ok(ref style_scheme)) = (render_window.read(), draw_queue.read(), style_scheme.read()) {
                                if let Some(ref bounding_box) = draw_queue.last().and_then(|draw_view| draw_view.bounding_box()) {
                                    // this time, we only need to redraw the area in which the new object has been placed
                                    let ((data, width, height, stride), (x, y)) = render_screen(draw_queue, rw, bounding_box, style_scheme);
                                    buffer_sender.send(((data, width, height, stride), (x, y)));
                                }
                            }
                        }
                        // whenever a visual component changes, we get a redraw request
                        DialogRendererMessage::RedrawRequest(bounding_box) => {
                            // first we need to check whether the updated region actually intersects the world area
                            if let Ok(ref rw) = render_window.read() {
                                // if it does, do the expensive call of catching a reference to the draw queue and the style scheme to do the update
                                if WorldBoundingBox::check_intersect(&bounding_box, rw.world_bounding_box()) {
                                    if let (Ok(ref draw_queue), Ok(ref style_scheme)) = ( draw_queue.read(), style_scheme.read()) {
                                        let ((data, width, height, stride), (x, y)) = render_screen(draw_queue, rw, &bounding_box, style_scheme);
                                        buffer_sender.send(((data, width, height, stride), (x, y)));
                                    }
                                }
                            }
                        }
                   }
               }
            })
        };





        // Refresh callback
        // The following callback is called at 30fps, and paints any surfaces drawn by the worker thread to the main buffer.
        {
        
            let draw_buffer = draw_buffer.clone();
            let drawing_area = drawing_area.clone();

            // Small reusable buffer for the refresh thread.
            // let mut invalidated_regions = Vec::with_capacity(10);

            // called at 30 fps
            ::gtk::timeout_add(1000/60, move || {

                    // invalidated_regions.clear();

                    // get a reference to the main drawing buffer - note: this is thread safe, as all gtk functions are called from the same thread
                    // so there are no race conditions.
                    let draw_buffer = draw_buffer.borrow();
                    let cr = Context::new(&*draw_buffer);

                    // grab any drawn surfaces that have been sent by the worker thread
                    let requests = buffer_receiver.try_iter().collect::<Vec<((Box<[u8]>, i32, i32, i32), (f64, f64))>>();

                    if requests.len() > 0 {
                        let mut max = None;
                        let mut i = 0;

                        // as an optimization, we're going to preprocess the draw queue.
                        // first, we're going to find the smallest area that encompasses all things that need to be draw
                        while i < requests.len() {
                            if let Some((x,y,w,h)) = max {
                                let (o_x, o_y, o_w, o_h) = ((requests[i].1).0, (requests[i].1).1, (requests[i].0).1, (requests[i].0).2);
                                let (o_x, o_y, o_w, o_h) = (o_x as f64, o_y as f64, o_w as f64, o_h as f64);


                                let n_x  = if o_x < x { o_x } else { x };
                                let n_y  = if o_y < y { o_y } else { y };
                                let n_w = (if o_x + o_w > x + w { o_x + o_w } else {x + w}) - n_x;
                                let n_h = (if o_y + o_h > y + h { o_y + o_h } else {y + h}) - n_y;

                                max = Some((n_x, n_y, n_w, n_h));

                            } else {
                                max = Some(((requests[i].1).0, (requests[i].1).1, (requests[i].0).1 as f64, (requests[i].0).2 as f64));
                            }
                           i += 1; 
                        }

                        
                        for ((data, width, height, stride), (x, y)) in  requests {
                            if let Ok(surface) = ImageSurface::create_for_data(data, |data| {}, Format::Rgb24, width, height, stride) {
                                cr.set_source_surface(&surface, x as f64, y);
                                cr.paint();
                                

                                // Note: the following is needed to ensure that the surface is dropped ASAP and isn't held around by the context
                                cr.set_source_rgb(0.0, 0.0, 0.0);
                                // invalidated_regions.push((x as i32, y as i32, width as i32, height as i32));
                            }
                        }

                        // for (x,y, width, height) in invalidated_regions.iter() {
                        //     drawing_area.queue_draw_area(*x, *y, *width, *height);
                        // }
                        if let Some((x,y,w,h)) = max {
                            drawing_area.queue_draw_area(x as i32,y as i32,w as i32,h as i32);
                        }

                    }
                Continue(true)
            });
        }



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

fn generate_buffer(width : i32 , height : i32) -> (Vec<u8>, i32) {
    let mut image = ImageSurface::create(Format::Rgb24, width, height).expect("Could not create an image buffer");

    {
        let cr = Context::new(&image);
        cr.set_source_rgb(0.0, 1.0, 0.0);
        cr.paint();
    }
    let buf = image.get_data().expect("Could not retrieve image buffer from surface").to_vec();

    (buf, image.get_stride())
}


/// Given a bounding box representing the invalidated region, draws the world for that region, and
/// produces a package that can be sent  to the refresh thread to update the invalidated region
fn render_screen(draw_queue: &Vec<DrawView>, render_window: &RenderWindow, invalidated_region: &WorldBoundingBox, style_scheme: &StyleScheme) -> ((Box<[u8]>, i32, i32, i32), (f64, f64)) {

    let bounding_box = invalidated_region; 
    let (ScreenUnit(x), ScreenUnit(y), ScreenDimensions(ScreenUnit(w), ScreenUnit(h))) = render_window.world_bounding_box_to_screen(&bounding_box);
    let mut surface = ImageSurface::create(Format::Rgb24, w as i32, h as i32).expect("Could not create image surface");

    {
        let cr = Context::new(&surface);
        let render_window = RenderWindow::new_from_parts(bounding_box.0, bounding_box.1, bounding_box.2, bounding_box.3, ScreenUnit(w), ScreenUnit(h));
        
        cr.set_source_rgba(style_scheme.bg.red, style_scheme.bg.green, style_scheme.bg.blue, style_scheme.bg.alpha);
        cr.paint();


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

        // cr.rectangle(0.0, 0.0, 1.0, 1.0);
        // cr.stroke();

        // 2. ask drawables to draw themselves

        for drawable in draw_queue.iter() {
            // if drawable.is_onscreen(&render_window) {
                drawable.draw(&cr, &style_scheme, &render_window);
            // }
        }


    }

    let data = surface.get_data().expect("Could not retrieve buffer data").to_vec().into_boxed_slice();
    let stride = surface.get_stride();

    ((data, w as i32, h as i32, stride), (x, y))
}


