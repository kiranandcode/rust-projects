use renderer::{
    Renderer,
    RenderWindow, 
    StyleScheme
};
use event::{EventManager, EventManagerBuilder};

use std::convert::AsRef;
use std::sync::{
    Arc, 
    RwLock,
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


/// Window containing a visualizer
pub struct App {
   window: Window, 
   header: Header,
   content: Content,
   model: Model
}

pub struct Model {
    style_context: StyleContext
}

impl App {
    pub fn new(event_builder: &mut EventManagerBuilder) -> App {
        let style_context = StyleContext::new();
        let ref_style_context = Arc::new(RwLock::new(StyleScheme::from(&style_context)));

        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new(event_builder, ref_style_context.clone());

        window.set_title("GopViz - Visualizer");
        window.set_wmclass("app-name", "Gopviz");
        window.set_default_size(500, 500);


        // connect children
        window.set_titlebar(&header.container);
        window.add(content.as_ref());
        // params are self, envt
        window.connect_delete_event(move |_, _| {
            main_quit();
            // main_quit ends the gtk event loop, thus prompting our
            // app to close - there's no need to run the default 
            // handler
            Inhibit(false) 
        });


        let model = Model {
            style_context
        };
       
        App {
            window,
            header,
            content,
            model
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
    conversation_renderer: Renderer
}

impl Content {
    fn new(event_builder : &mut EventManagerBuilder, style_context: Arc<RwLock<StyleScheme>>) -> Self {

        Content {
            conversation_renderer: Renderer::new(event_builder, style_context)
        }

    }
}


impl AsRef<DrawingArea> for Content {
    fn as_ref(&self) -> &DrawingArea {
        self.conversation_renderer.as_ref()
    }
}
