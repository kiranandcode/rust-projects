use event::message::renderer::DialogRendererMessage;
use event::message::GeneralMessage;

pub enum DialogInputManagerState {
    SELECT,
    DRAG
}
pub struct DialogInputManager {
    state: DialogInputManagerState,
}

impl DialogInputManager {

   pub fn handle_message(message: GeneralMessage) -> DialogRendererMessage {
        unimplemented!("Has not been implemented");
   }


}
