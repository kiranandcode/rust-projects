pub mod message;
use self::message::gtk::GtkMessage;
use self::message::renderer::RendererMessage;
use graphics::types::*;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::mem;
use std::thread;

use gdk::Event;

pub struct EventManagerBuilder {
    renderer_channel: Option<Sender<message::renderer::RendererMessage>>, 
    gdk_pair: (Receiver<GtkMessage>, Sender<GtkMessage>),
}

impl EventManagerBuilder {
   pub fn new() -> Self {
       let (sender, receiver) = mpsc::channel();
        EventManagerBuilder {
           renderer_channel: None,
           gdk_pair: (receiver, sender)
        }
   }

   pub fn get_gdk_channel(&mut self) -> Sender<GtkMessage> {
        self.gdk_pair.1.clone()
   }

   pub fn set_renderer_channel(&mut self, renderer_channel : Sender<message::renderer::RendererMessage>) -> &mut Self {
       self.renderer_channel = Some(renderer_channel);
       self
   }

   pub fn build(self) -> EventManager {

        let (gdk_receiver, _) = self.gdk_pair;

        let renderer_channel = self.renderer_channel
                        .expect("Err: EventManagerBuilder::Build - can not build an event manager without a renderer_channel");

        EventManager {
            renderer_channel: Some(renderer_channel),
            gdk_receiver
        }
   }
}

pub struct EventManager {
    gdk_receiver: Receiver<GtkMessage>,
    renderer_channel: Option<Sender<message::renderer::RendererMessage>>, 
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
                for event in gdk_receiver.iter() {
                    println!("Got event e {:?}", event);

                    match event {
                        GtkMessage::RendererScreenResize(width, height) =>  {
                            if let Some(ref chnl) = renderer_channel {
                                chnl.send(RendererMessage::ResizeEvent(ScreenDimensions(width,height)));
                            }
                        }
                    }
                }
                println!("Event Manager main loop ended");
            });
        }

}
