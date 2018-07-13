use event::message::renderer::DialogRendererMessage;
use event::message::GeneralMessage;

pub enum DialogInputManagerState {
    NORMAL,
    NEW,
}
pub struct DialogInputManager {
    state: DialogInputManagerState,
}

impl DialogInputManager {
    pub fn new() -> Self  {
        DialogInputManager {
            state: DialogInputManagerState::NORMAL
        }
    }

   pub fn handle_message(&mut self, message: GeneralMessage) -> Option<DialogRendererMessage> {
       let state = self.state;
        match message {

            _ => panic!("Invalid message passed to dialog input manager")
       }


   }


}
