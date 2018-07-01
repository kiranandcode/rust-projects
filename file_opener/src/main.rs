extern crate gio;
extern crate gtk;
extern crate gtk_sys;
#[macro_use] extern crate relm;
#[macro_use] extern crate relm_derive;
extern crate relm_attributes;


use gio::{ AppInfo, AppLaunchContext, CancellableExt, File, FileExt };
use gtk::{
    ButtonExt,
    DialogExt,
    FileChooserAction,
    FileChooserDialog,
    FileChooserExt,
    ContainerExt,
    Inhibit,
    LabelExt,
    OrientableExt,
    WidgetExt,
    WindowType,
    Window,
    Button
};
use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};
use gtk::Orientation::Vertical;

use relm::{Relm, Widget, Update};

use self::Msg::*;

const RESPONSE_ACCEPT : i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL : i32 = GTK_RESPONSE_CANCEL as i32;


pub struct Model {
    relm: Relm<Win>,
    text: String
}

#[derive(Msg)]
pub enum Msg {
    AppError(gtk::Error),
    AppOpened(()),
    FileRead((Vec<u8>, String)),
    OpenApp,
    OpenFile,
    Quit,
    ReadError(gtk::Error),
}

pub struct Win {
    window: Window,
    model: Model
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            text: String::new()
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AppError(error) => println!("Application Error: {}", error),
            Msg::AppOpened(()) => println!("Application opened"),
            Msg::FileRead((content, _)) => println!("Read: {}", String::from_utf8_lossy(&content)),
            Msg::OpenApp => self.open_app(),
            Msg::OpenFile => self.open_file(),
            Msg::Quit => gtk::main_quit(),
            Msg::ReadError(error) => println!("Read Error: {}", error),
        }

    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Window {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Model) -> Self {
        let mut window : Window = Window::new(WindowType::Toplevel);
        let mut vbox = gtk::Box::new(Vertical, 1);
        let open_file = gtk::Button::new_with_label("Open File");
        let open_application = gtk::Button::new_with_label("Open Application");

        {
            let string : &str = &model.text;
            vbox.add(&open_file);
            vbox.add(&gtk::Label::new(Some(string)));
            vbox.add(&open_application);
            window.add(&vbox);
        }


        connect!(relm, open_file, connect_clicked(_), Msg::OpenFile);
        connect!(relm, open_application, connect_clicked(_), Msg::OpenFile);
        connect!(relm, window, connect_delete_event(_, _), return (Quit, Inhibit(false)));

        window.show_all();

        Win {
            window: window,
            model: model
        }
    }
}

impl Win {

    fn open_app(&mut self) {
        let dialog = FileChooserDialog::new(Some("Open a file"), Some(&self.window), FileChooserAction::Open);
        dialog.add_button("Cancel", RESPONSE_CANCEL);
        dialog.add_button("Accept", RESPONSE_ACCEPT);

        let result = dialog.run();

        if result == RESPONSE_ACCEPT {
            if let Some(uri) = dialog.get_uri() {
               let app_launch_context = AppLaunchContext::new();
                let relm = &self.model.relm;
                let cancellable = connect_async_func_full!(
                    AppInfo::launch_default_for_uri_async(&uri, &app_launch_context),
                    relm, AppOpened, AppError);

                //cancellable.cancel();
            }
        }
        dialog.destroy();
    }

    fn open_file(&mut self) {
        let dialog = FileChooserDialog::new(Some("Open a file"), Some(&self.window), FileChooserAction::Open);
        dialog.add_button("Cancel", RESPONSE_CANCEL);
        dialog.add_button("Accept", RESPONSE_ACCEPT);

         let result = dialog.run();

        if result == RESPONSE_ACCEPT {
            if let Some(filename) = dialog.get_filename() {
                let file = File::new_for_path(filename);

                let relm = &self.model.relm;
                let cancellable = connect_async_full!(
                file,
                load_contents_async,
                relm, FileRead, ReadError);

                //cancellable.cancel();
            }
        }
        dialog.destroy();

    }
}



fn main() {
    Win::run(()).unwrap();
    println!("Hello, world!");
}
