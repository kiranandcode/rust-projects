use renderer::{
    RenderWindow,
    StyleScheme
};
use renderer::dialog::DialogRenderer;

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
    Widget,
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
    StyleContext,        // used for initializing the stylescheme
    Notebook,            // 
    NotebookExt,         //
    Box,
    BoxExt,
    Orientation,
    ListBox,
    ListBoxExt,
};


/// Window containing a visualizer
pub struct App {
   window: Window, 
   header: Header,
   content: Content,
   model: Model
}

pub struct Model {
    pub (in gui) style_context: StyleContext
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
    pub (in gui) container: HeaderBar
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
    pub (in gui) main_box: Box,

        pub (in gui) side_bar_panel: Box,
            pub (in gui) map_list: ListBox,
            pub (in gui) layer_list: ListBox,

        pub (in gui) main_tabs: Notebook,
            pub (in gui) conversation_renderer: DialogRenderer,

        pub (in gui) options_tabs: Notebook,
            pub (in gui) properties_list: ListBox,
            pub (in gui) variables_box: Box,
                pub (in gui) global_variables: ListBox,
                pub (in gui) local_variables: ListBox,
}

impl Content {
    fn new(event_builder : &mut EventManagerBuilder, style_context: Arc<RwLock<StyleScheme>>) -> Self {
        let main_box = Box::new(Orientation::Horizontal, 0);

            let side_bar_panel = Box::new(Orientation::Vertical, 0);

                let map_list = ListBox::new();
                let layer_list = ListBox::new();

                side_bar_panel.pack_start(&map_list, true, true, 0);
                side_bar_panel.pack_start(&layer_list, true, true, 0);


            let main_tabs = Notebook::new();
                let conversation_renderer = DialogRenderer::new(event_builder, style_context);
        
                main_tabs.add(conversation_renderer.as_ref());
                main_tabs.set_menu_label_text(conversation_renderer.as_ref(), "Dialog Editor");
                main_tabs.set_tab_label_text(conversation_renderer.as_ref(), "Dialog Editor");


            let options_tabs = Notebook::new();
                let properties_list = ListBox::new();

                let variables_box = Box::new(Orientation::Vertical, 0);
                    let local_variables = ListBox::new();
                    let global_variables = ListBox::new();

                    variables_box.pack_start(&global_variables, true, true, 0);
                    variables_box.pack_start(&local_variables, true, true, 0);


                options_tabs.add(&properties_list);
                options_tabs.set_menu_label_text(&properties_list, "Properties");
                options_tabs.set_tab_label_text(&properties_list, "Properties");

                options_tabs.add(&variables_box);
                options_tabs.set_menu_label_text(&variables_box, "Variables");
                options_tabs.set_tab_label_text(&variables_box, "Variables");


        main_box.pack_start(
            &side_bar_panel,
            false,
            true,
            0
        );

        main_box.pack_start(
            &main_tabs,
            true,
            true,
            0
        );

        main_box.pack_end(
            &options_tabs,
            false,
            false,
            0
        );

        Content {

            main_box,

                side_bar_panel,
                    map_list,
                    layer_list,

                main_tabs,
                    conversation_renderer,

            options_tabs,
            properties_list,
            variables_box,

            local_variables,
            global_variables,
        }

    }
}


impl AsRef<Box> for Content {
    fn as_ref(&self) -> &Box {
        &self.main_box
    }
}
