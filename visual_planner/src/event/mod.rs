pub mod message;
use self::message::GeneralMessage;
use self::message::renderer::DialogRendererMessage;
use self::message::gui::GuiManagerMessage;
use types::*;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::mem;
use std::thread;

use gdk::Event;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum DialogInputState {
    NORMAL,
    NEW,
}

pub struct EventManagerBuilder {
    renderer_channel: Option<Sender<message::renderer::DialogRendererMessage>>, 
    gui_channel: Option<Sender<message::gui::GuiManagerMessage>>,
    gdk_pair: (Receiver<GeneralMessage>, Sender<GeneralMessage>),
}

impl EventManagerBuilder {
   pub fn new() -> Self {
       let (sender, receiver) = mpsc::channel();
        EventManagerBuilder {
           renderer_channel: None,
           gui_channel: None,
           gdk_pair: (receiver, sender)
        }
   }

   pub fn get_gdk_channel(&mut self) -> Sender<GeneralMessage> {
        self.gdk_pair.1.clone()
   }

   pub fn set_renderer_channel(&mut self, renderer_channel : Sender<message::renderer::DialogRendererMessage>) -> &mut Self {
       self.renderer_channel = Some(renderer_channel);
       self
   }

   pub fn set_gui_channel(&mut self, gui_channel: Sender<message::gui::GuiManagerMessage>) -> &mut Self {
        self.gui_channel = Some(gui_channel);
        self
   }

   pub fn build(self) -> EventManager {

        let (gdk_receiver, _) = self.gdk_pair;

        let renderer_channel = self.renderer_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a renderer_channel");

        let gui_channel = self.gui_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a gui_channel");

        EventManager {
            renderer_channel: Some(renderer_channel),
            gui_channel: Some(gui_channel),
            gdk_receiver,
            dialog_input_state: DialogInputState::NORMAL
        }
   }
}

pub struct EventManager {
    gdk_receiver: Receiver<GeneralMessage>,
    renderer_channel: Option<Sender<message::renderer::DialogRendererMessage>>, 
    gui_channel: Option<Sender<message::gui::GuiManagerMessage>>, 
    dialog_input_state: DialogInputState
    
}


impl EventManager {
        pub fn new() -> EventManagerBuilder {
            EventManagerBuilder::new()
        }

        /// Starts the event manager 
        pub fn start(event_manager: EventManager) {
            thread::spawn(move || {
                // main loop, recieve gdk events, send to corresponding components
                let gdk_receiver = event_manager.gdk_receiver;
                let renderer_channel = event_manager.renderer_channel;
                let gui_channel = event_manager.gui_channel;
                let mut dialog_input_state = event_manager.dialog_input_state;

                for event in gdk_receiver.iter() {
                    // println!("Got event {:?}", event);

                    match event {
                        GeneralMessage::RendererScreenResize(width, height) =>  {
                            if let Some(ref chnl) = renderer_channel {
                                chnl.send(DialogRendererMessage::ResizeEvent(ScreenDimensions(width,height)));
                            }
                        }
                        GeneralMessage::Redraw(id) => {
                            if let Some(ref chnl) =  gui_channel {
                                chnl.send(GuiManagerMessage::RedrawEvent(id));
                            }
                        }
                        GeneralMessage::Scroll(width, height, scroll_direction, delta) => {
                            if let Some(ref chnl) = renderer_channel {
                                chnl.send(DialogRendererMessage::ScrollEvent(ScreenCoords(width,height), scroll_direction, delta));
                            }
                        }
                        GeneralMessage::SetDialogInputState(msg) => {
                            dialog_input_state = msg;
                        }
                    }
                }
                println!("Event Manager main loop ended");
            });
        }

}
