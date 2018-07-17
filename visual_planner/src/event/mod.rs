pub mod message;
use self::message::GeneralMessage;
use self::message::renderer::{DialogRendererMessage, DialogStateMessage};
use self::message::gui::GuiManagerMessage;
use types::*;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::mem;
use std::thread;

use gdk::Event;



pub struct EventManagerBuilder {
    gdk_pair: (Receiver<GeneralMessage>, Sender<GeneralMessage>),
    gui_channel: Option<Sender<message::gui::GuiManagerMessage>>,

    dialog_renderer_channel: Option<Sender<message::renderer::DialogRendererMessage>>, 
    dialog_state_channel: Option<Sender<message::renderer::DialogStateMessage>>,

    model_manager_channel: Option<Sender<message::manager::ModelManagerMessage>>,
}

impl EventManagerBuilder {
   pub fn new() -> Self {
       let (sender, receiver) = mpsc::channel();
        EventManagerBuilder {
           gui_channel: None,
           gdk_pair: (receiver, sender),

           dialog_renderer_channel: None,
           dialog_state_channel: None,

           model_manager_channel: None
        }
   }

   pub fn get_gdk_channel(&mut self) -> Sender<GeneralMessage> {
        self.gdk_pair.1.clone()
   }

   pub fn set_dialog_renderer_channel(&mut self, renderer_channel : Sender<message::renderer::DialogRendererMessage>) -> &mut Self {
       self.dialog_renderer_channel = Some(renderer_channel);
       self
   }

   pub fn set_dialog_state_channel(&mut self, state_channel : Sender<message::renderer::DialogStateMessage>) -> &mut Self {
       self.dialog_state_channel = Some(state_channel);
       self
   }

   pub fn set_model_manager_channel(&mut self, model_channel: Sender<message::manager::ModelManagerMessage>) -> &mut Self {
       self.model_manager_channel = Some(model_channel);
       self
   }



   pub fn set_gui_channel(&mut self, gui_channel: Sender<message::gui::GuiManagerMessage>) -> &mut Self {
        self.gui_channel = Some(gui_channel);
        self
   }

   pub fn build(self) -> EventManager {

        let (gdk_receiver, _) = self.gdk_pair;

        let dialog_renderer_channel = self.dialog_renderer_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a dialog_renderer_channel");

        let dialog_state_channel = self.dialog_state_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a dialog_state_channel");

        let gui_channel = self.gui_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a gui_channel");
        
        let model_channel = self.model_manager_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a model_channel");

        EventManager {
            gui_channel: Some(gui_channel),
            gdk_receiver,
            dialog_renderer_channel: Some(dialog_renderer_channel),
            dialog_state_channel: Some(dialog_state_channel),
            model_manager_channel: Some(model_channel)
        }
   }
}

pub struct EventManager {
    gdk_receiver: Receiver<GeneralMessage>,
    gui_channel: Option<Sender<message::gui::GuiManagerMessage>>, 

    dialog_renderer_channel: Option<Sender<message::renderer::DialogRendererMessage>>, 
    dialog_state_channel: Option<Sender<message::renderer::DialogStateMessage>>,
    model_manager_channel: Option<Sender<message::manager::ModelManagerMessage>>,
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
                let gui_channel = event_manager.gui_channel;

                let dialog_renderer_channel = event_manager.dialog_renderer_channel;
                let dialog_state_channel = event_manager.dialog_state_channel;



                for event in gdk_receiver.iter() {
                    // println!("Got event {:?}", event);

                    match event {

                        // Renderer Channel
                        GeneralMessage::RendererScreenResize(width, height) =>  {
                            if let Some(ref chnl) = dialog_renderer_channel {
                                chnl.send(DialogRendererMessage::ResizeEvent(ScreenDimensions(width,height)));
                            }
                        }
                        GeneralMessage::RendererScroll(width, height, scroll_direction, delta) => {
                            if let Some(ref chnl) = dialog_renderer_channel {
                                    chnl.send(DialogRendererMessage::ScrollEvent(ScreenCoords(width,height), scroll_direction, delta));
                            }
                        }
                        GeneralMessage::WindowMove(x, y) => {
                            if let Some(ref chnl) = dialog_renderer_channel {
                                    chnl.send(DialogRendererMessage::WindowMoveEvent(x,y));
                            }
                        }



                        // State Channel
                        GeneralMessage::RendererClick(x, y) => {
                            // TODO(Kiran): Match on dialog state, and based on whether you hit something, change to selected
                            if let Some(ref chnl) = dialog_state_channel {
                                    chnl.send(
                                        DialogStateMessage::ClickEvent(ScreenCoords(x,y))
                                    );
                            }
                        }
                        GeneralMessage::RendererMotion(x, y) => {
                            if let Some(ref chnl) = dialog_state_channel {
                                    chnl.send(
                                        DialogStateMessage::MotionEvent(ScreenCoords(x,y))
                                    );
                            }
 
                        }
                        GeneralMessage::SetDialogInputState(msg) => {
                             if let Some(ref chnl) =  dialog_state_channel {
                                 chnl.send(DialogStateMessage::SetDialogState(msg));
                            }
                            
                        }

                
                        // GUI Channel
                        GeneralMessage::Redraw(id) => {
                            if let Some(ref chnl) =  gui_channel {
                                chnl.send(GuiManagerMessage::RedrawEvent(id));
                            }
                        }
                        GeneralMessage::SetCursor(id, cursor_name) => {
                            if let Some(ref chnl) = gui_channel {
                                chnl.send(GuiManagerMessage::SetCursorEvent(id, cursor_name)); 
                            }
                        }



                        // Manager Channel

                    }
                }
                println!("Event Manager main loop ended");
            });
        }

}
