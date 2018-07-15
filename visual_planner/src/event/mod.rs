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
    dialog_widget_id: Option<GuiWidgetID>
}

impl EventManagerBuilder {
   pub fn new() -> Self {
       let (sender, receiver) = mpsc::channel();
        EventManagerBuilder {
           renderer_channel: None,
           gui_channel: None,
           dialog_widget_id: None,
           gdk_pair: (receiver, sender),
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

   pub fn set_dialog_widget_id(&mut self, id : GuiWidgetID) -> &mut Self {
       self.dialog_widget_id = Some(id);
       self
   }

   pub fn build(self) -> EventManager {

        let (gdk_receiver, _) = self.gdk_pair;

        let renderer_channel = self.renderer_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a renderer_channel");

        let gui_channel = self.gui_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a gui_channel");
        let widget_id = self.dialog_widget_id;

        EventManager {
            renderer_channel: Some(renderer_channel),
            gui_channel: Some(gui_channel),
            gdk_receiver,
            dialog_input_state: DialogInputState::NORMAL,
            dialog_widget_id: widget_id
        }
   }
}

pub struct EventManager {
    gdk_receiver: Receiver<GeneralMessage>,
    renderer_channel: Option<Sender<message::renderer::DialogRendererMessage>>, 
    gui_channel: Option<Sender<message::gui::GuiManagerMessage>>, 
    dialog_input_state: DialogInputState,
    dialog_widget_id: Option<GuiWidgetID>
    
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
                let dialog_widget_id = event_manager.dialog_widget_id;

                let mut prev_input_pos : Option<ScreenCoords> = None;

                for event in gdk_receiver.iter() {
                    // println!("Got event {:?}", event);

                    match event {
                        GeneralMessage::RendererScreenResize(width, height) =>  {
                            if let Some(ref chnl) = renderer_channel {
                                chnl.send(DialogRendererMessage::ResizeEvent(ScreenDimensions(width,height)));
                            }
                        }
                        GeneralMessage::RendererScroll(width, height, scroll_direction, delta) => {
                            if let Some(ref chnl) = renderer_channel {
                                chnl.send(DialogRendererMessage::ScrollEvent(ScreenCoords(width,height), scroll_direction, delta));
                            }
                        }
                        GeneralMessage::RendererClick(x, y) => {
                            // TODO(Kiran): Match on dialog state, and based on whether you hit something, change to selected

                            // Every distinct drag event is distinguished by one
                            // initializing click event, and then several motion
                            // events. To avoid multiple distinct drags coalescing
                            // into one single large drag (with a jump inbetween)
                            // we have to reset the prev input pos each time we see a click
                            prev_input_pos = None;

                            match dialog_input_state {
                                DialogInputState::NORMAL => (),
                                _ => {
                                    unimplemented!("Has not been implemented!");
                                }
                            }
                        }
                        GeneralMessage::RendererMotion(x, y) => {
                            // TODO(Kiran): Match on dialog state, 
                            // if normal, then move renderwindow, 
                            // if selected move selected component
                            match dialog_input_state {
                                DialogInputState::NEW => (
                                    // When in create new state, then dragging doesn't do anything???
                                ),
                                DialogInputState::NORMAL => {
                                   if let Some(p_xy) = prev_input_pos.take() {
                                       let ScreenCoords(px, py) = p_xy;
                                       let dx =  px - x;
                                       let dy =  py - y;

                                        println!("dx,dy: {:?} {:?}", dx, dy);
                                            if let Some(ref chnl) = renderer_channel {
                                                chnl.send(DialogRendererMessage::WindowMoveEvent(dx, dy));
                                            }
                                        prev_input_pos = Some(ScreenCoords(x,y));
                                   }  
                                    prev_input_pos = Some(ScreenCoords(x,y));
                                }
                            }
                        }
                        GeneralMessage::Redraw(id) => {
                            if let Some(ref chnl) =  gui_channel {
                                chnl.send(GuiManagerMessage::RedrawEvent(id));
                            }
                        }
                        GeneralMessage::SetDialogInputState(msg) => {
                            if dialog_input_state != msg {
                                dialog_input_state = msg;
                            } else {
                                dialog_input_state =   DialogInputState::NORMAL;
                            }
                            println!("Setting the dialog input state to {:?}", dialog_input_state); 
                            if let Some(ref id) = dialog_widget_id {
                                match dialog_input_state {
                                    DialogInputState::NORMAL => {
                                            if let Some(ref chnl) = gui_channel {
                                                chnl.send(GuiManagerMessage::SetCursorEvent(
                                                    id.clone(),
                                                    "default"
                                                ));
                                            }
                                    }
                                    DialogInputState::NEW => {
                                            if let Some(ref chnl) = gui_channel {
                                                 chnl.send(GuiManagerMessage::SetCursorEvent(
                                                    id.clone(),
                                                    "cell"
                                                ));
                                            }
                                    }
                                }
                            }
                        }

                    }
                }
                println!("Event Manager main loop ended");
            });
        }

}
