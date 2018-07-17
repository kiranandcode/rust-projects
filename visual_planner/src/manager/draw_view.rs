use renderer::*;
use types::*;
use cairo::Context;
use std::sync::{Arc, Mutex, MutexGuard};
use std::ops::Deref;

pub enum ModelChangeRequest {
    CompleteMotion,
    MoveTo(WorldCoords),
    MoveBy(WorldUnit, WorldUnit),
    SetText(String),
    SetSelected,
}

pub trait Drawable {
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow); 
    fn bounding_box(&self) -> Option<MutexGuard<WorldBoundingBox>>;
    fn id(&self) -> ModelID;
}


pub struct DrawView {
    id: ModelID,
    drawable: Arc<Drawable>,
}

impl DrawView {
    pub fn new(drawable: Arc<Drawable>) -> Self {
        DrawView {
            id: drawable.id(),
            drawable
        }
    }

    pub fn matches_id(&self, id: &ModelID) -> bool {
        &self.id == id
    }

    pub fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
                self.drawable.draw(cr, style, window);
    }

    pub fn is_onscreen(&self, window: &RenderWindow) -> bool {
        if let Some(guard) = self.drawable.bounding_box() {
            window.is_bounding_box_onscreen(guard.deref())
        } else {
            false
        }
    }
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