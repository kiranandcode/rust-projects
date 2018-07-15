use types::*;
use event::message::gui::{GuiManagerMessage};
use event::EventManagerBuilder;

use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;


use gtk::{Widget, WidgetExt, IsA};
use gdk::{Cursor, Display, Window, WindowExt, Screen, ScreenExt};
use gtk::Cast;

/// A non-thread-safe class to manage all gui-elements.
/// Note: this isn't thread safe, as it's meant to be run in sequence with gtk::main_iteration()
pub struct GuiManager {
    communication_channel: (Receiver<GuiManagerMessage>, Sender<GuiManagerMessage>),
    gui_widgets: HashMap<GuiWidgetID, GuiEntity>,
    gui_widget_id: usize
}

enum GuiEntity {
    Widget(Widget),
}

impl GuiManager {

    pub fn new(event_manager: &mut EventManagerBuilder) -> Self {
        let (sender, receiver) = ::std::sync::mpsc::channel();
        event_manager.set_gui_channel(sender.clone());
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
            self.gui_widgets.insert(id, GuiEntity::Widget(widget.upcast::<Widget>()));
            id
        }

    pub fn run_iteration(&mut self) {
        let (ref mut receiver,_) = self.communication_channel; 
        for event in receiver.try_iter() {
            match event {
                GuiManagerMessage::RedrawEvent(guiID) => {
                    if let Some(GuiEntity::Widget(ref widget)) = self.gui_widgets.get(&guiID) {
                            widget.queue_draw();
                    }
                }
                GuiManagerMessage::SetCursorEvent(guiID, cursor_type) => {
                    println!("Got setcursor event {:?} {:?}", guiID, cursor_type);
                    if let Some(GuiEntity::Widget(ref widget)) = self.gui_widgets.get(&guiID) {
                        // Screen::get_default().map(|scr| {
                            widget.get_window().map(|wnd| {
                                Display::get_default().map(|displ| {
                                        let curs = Cursor::new_from_name(&displ, &cursor_type);
                                        wnd.set_cursor(Some(&curs));
                                });
                            });
                        // });                    
                    }
                }
            }
        }
    }
}
