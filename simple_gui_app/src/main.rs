extern crate gdk;
extern crate gtk;
extern crate rand;

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate relm_attributes;

mod drawing;

use relm::{Relm, Widget, Update};
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Inhibit
};

struct Model {

}


// Represents the messages sent to Widget::Update on an event occuring
#[derive(Msg)]
enum Msg {
    Quit
}


// this struct represents the Widget containing the GTK+ widget
struct Win {
    model: Model,
    window: Window
}

// to make win a relm widget, we need to implement Update and Widget traits
impl Update for Win {
    // the model used for the widget
    type Model = Model;
    // the model parameter used to initialize the model
    // these are passed in via the run method
    type ModelParam = ();
    // the type of messages sent to the update function
    type Msg = Msg;

    // return the initial model
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
        }
    }


    // when a messages is recieved, we can update the model
    // we can also update widgets
    // futures and streams can also be connected to send a message when a value is ready
    fn update(&mut self, event: Msg) {
            match event {
                Msg::Quit => gtk::main_quit(),
                _ => {
                   // let future  = create_future();
                   // relm.connect_exec_ignore_err((future,SomeEvent));
                }
            }
    }


    // the subscriptions method is optional
    // futures and streams can be connected in the subscriptions method
    fn subscriptions(&mut self, relm: &Relm<Self>) {

    }
}


impl Widget for Win {
    // specify the type of the root widget
    type Root = Window;

    // return the root widget
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // create the widgets
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a widget
        let window = Window::new(WindowType::Toplevel);

        // connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        window.show_all();

        Win {
            model,
            window: window
        }
    }

}


fn main() {
        Win::run(()).unwrap();
}
