
use gtk::*;
use std::process;

// to structure a GUI program we'll be partioning the window into a header and a contents

pub struct App {
    pub window: Window,
    pub header: Header,
}

pub struct Header {
    pub container: HeaderBar
}

// the app structure will contain the overall structure of the UI in a hierarchy of data structures.
//
impl App {
    pub fn new() -> App {
        // first we need to create the top level window for our system
        let window = Window::new(WindowType::Toplevel);
        // also construct the header of the window
        let header = Header::new();

        // set the headerbar of the system
        window.set_titlebar(&header.container);
        // set the title of the window
        window.set_title("App Name");
        // set the window manager class - probably used to group simmilar layers together
        window.set_wmclass("app-name", "App Name");
        // set the app icon
        Window::set_default_icon_name("iconname");


        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {window, header}
    }
    pub fn run(&self) {
        self.window.show_all();
        main();
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title("App Name");

        container.set_show_close_button(true);

        Header {container}
    }
}

