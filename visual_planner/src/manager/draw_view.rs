use renderer::*;
use types::*;
use cairo::Context;
use std::sync::{Arc, Mutex};

pub enum ModelChangeRequest {
    MoveTo(WorldCoords),
    MoveBy(WorldUnit, WorldUnit),
    SetText(String),
    SetSelected,
}

pub trait Drawable {
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow); 
}


pub struct DrawViewLocked {
    id: ModelID,
    drawable: Arc<Mutex<Drawable>>,
}


// click event constructed in dialog renderer
// click event sent to event bus
// event bus sends click event to dialog renderer
// dialog renderer forwards click event to manager
// manager handles click


// click event constructed in dialog renderer
// click event sent to event bus
// event bus sends click event to dialog renderer
// dialog renderer sends construct request to event bus
// event bus sends construct request to manager
// manager constructs entity, and responds with a register drawable message
// dialog renderer adds register drawable to self,

// click event constructed in dialog renderer
// click event sent to event bus
// event bus sends click event to dialog renderer
// dialog renderer sends delete click request to manager
// manager deletes entity and responds with a deregister drawable message
// dialog renderer removes drawable from self

// text event constructed in dialog renderer
// text event sent to event bus
// event bus send text event to manager
// manager edits element