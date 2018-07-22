use types::*;
use manager::components::boxes::BoxConstructor;


pub enum ModelManagerMessage {
    // used whenever a box is requested to be constructed
   BoxConstruct(BoxConstructor),
   // used whenever the dialog renderer has a frame update, and animated components need to be updated
   DialogUpdate(CurrentTime, DeltaTime)
}