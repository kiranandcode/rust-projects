use types::*;
use state::*;
use event::{EventManagerBuilder};
use event::message::renderer::DialogStateMessage;
use event::message::GeneralMessage;
use render_window::RenderWindow;

use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

pub struct DialogStateManager {
   state_event_thread: JoinHandle<()>
}


impl DialogStateManager {


    pub fn new(drawable_id: GuiWidgetID, render_window: Arc<RwLock<RenderWindow>>,  event_builder: &mut EventManagerBuilder) -> Self {

        let (sender, receiver) = mpsc::channel();
        let chnl = event_builder.get_gdk_channel();
        event_builder.set_dialog_state_channel(sender);

        let handle = thread::spawn(move || {

            let mut prev_input_pos : Option<ScreenCoords> = None;
            let mut state = DialogInputState::NORMAL;

            for event in receiver.iter() {

                match event {
                    DialogStateMessage::ClickEvent(ScreenCoords(x,y)) => {
                        // Every distinct drag event is distinguished by one
                        // initializing click event, and then several motion
                        // events. To avoid multiple distinct drags coalescing
                        // into one single large drag (with a jump inbetween)
                        // we have to reset the prev input pos each time we see a click
                        prev_input_pos = None;

                        match state {
                            DialogInputState::NORMAL => (),
                            _ => {
                                unimplemented!("Has not been implemented!");
                            }
                        }
                        
                    },
                    DialogStateMessage::MotionEvent(ScreenCoords(x,y)) => {
                            // TODO(Kiran): Match on dialog state, 
                        // if normal, then move renderwindow, 
                        // if selected move selected component
                        match state {
                            DialogInputState::NEW => (
                                // When in create new state, then dragging doesn't do anything???
                            ),
                            DialogInputState::NORMAL => {
                                if let Some(p_xy) = prev_input_pos.take() {
                                    
                                    let ScreenCoords(px, py) = p_xy;
                                    let dx =  px - x;
                                    let dy =  py - y;
                                    chnl.send(GeneralMessage::WindowMove(dx,dy));
                                }  
                                prev_input_pos = Some(ScreenCoords(x,y));
                            }
                        }
                        
                    },
                    DialogStateMessage::SetDialogState(msg) => {
                        if state != msg {
                            state = msg;
                        } else {
                            state = DialogInputState::NORMAL;
                        }
                        match state {
                            DialogInputState::NORMAL => {
                                chnl.send(
                                    GeneralMessage::SetCursor(drawable_id.clone(), "default")
                                );

                            }
                            DialogInputState::NEW => {
                                chnl.send(
                                    GeneralMessage::SetCursor(drawable_id.clone(), "cell")
                                );
                            }
                        }
                    

                    },
                }

            }
        });


        DialogStateManager {
            state_event_thread: handle
        }

    }
}