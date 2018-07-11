use types::*;
use event::message::gui::{GuiManagerMessage};

use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;


use gtk::{Widget, WidgetExt, IsA};
use gtk::Cast;

/// A non-thread-safe class to manage all gui-elements.
/// Note: this isn't thread safe, as it's meant to be run in sequence with gtk::main_iteration()
pub struct GuiManager {
    communication_channel: (Receiver<GuiManagerMessage>, Sender<GuiManagerMessage>),
    gui_widgets: HashMap<GuiWidgetID, Widget>,
    gui_widget_id: usize
}

impl GuiManager {

    pub fn new() -> Self {
        let (sender, receiver) = ::std::sync::mpsc::channel();
        GuiManager {
            communication_channel: (receiver, sender),
            gui_widgets: HashMap::new(),
            gui_widget_id: 0
        }
    }

    pub fn get_channel(&self) -> Sender<GuiManagerMessage> {
        self.communication_channel.1.clone()
    }

    pub fn register_widget<T>(&mut self, widget: T) -> GuiWidgetID
        where T : IsA<Widget> + Cast {
            let id = GuiWidgetID(self.gui_widget_id);
            self.gui_widget_id += 1;
            self.gui_widgets.insert(id, widget.upcast::<Widget>());
            id
        }

    pub fn run_iteration(&mut self) {
        let (ref mut receiver,_) = self.communication_channel; 
        for event in receiver.try_iter() {
            match event {
                GuiManagerMessage::RedrawEvent(guiID) => {
                    if let Some(ref widget) = self.gui_widgets.get(&guiID) {
                            widget.queue_draw();
                    }
                }
            }
        }
    }
}
