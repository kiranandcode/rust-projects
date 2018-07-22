use renderer::*;
use types::*;
use cairo::Context;

use std::cmp::{Ord, PartialOrd, PartialEq, Ordering};
use std::fmt::{Debug, Formatter, Error};
use std::sync::{Arc, Mutex, MutexGuard};
use std::ops::Deref;

pub enum ModelChangeRequest {
    CompleteMotion,
    MoveTo(WorldCoords),
    MoveBy(WorldUnit, WorldUnit),
    SetText(String),
    SetSelected,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DrawPriority {
    High,   // Priority things that must be drawn above everything, including boxes
    Medium, // Priority for boxes - essentially the base 
    Low     // Priority for things below boxes - i.e edges
}

impl PartialOrd for DrawPriority {
    fn partial_cmp(&self, other: &DrawPriority) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DrawPriority {
    fn cmp(&self, other: &DrawPriority) -> Ordering {
        let self_value = match self {
            &DrawPriority::High => 0,
            &DrawPriority::Medium => 1,
            &DrawPriority::Low => 2,
        };

        let other_value = match self {
            &DrawPriority::High => 0,
            &DrawPriority::Medium => 1,
            &DrawPriority::Low => 2,
        };
        self_value.cmp(&other_value)
    }
}

pub trait Drawable: Sync + Send {
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow); 
    fn bounding_box(&self) -> Option<MutexGuard<WorldBoundingBox>>;
    fn id(&self) -> ModelID;
    fn priority(&self) -> DrawPriority;
}


pub struct DrawView {
    id: ModelID,
    drawable: Arc<Drawable>,
    priority: DrawPriority
}

impl DrawView {
    pub fn new(drawable: Arc<Drawable>) -> Self {
        let priority = drawable.priority();
        DrawView {
            id: drawable.id(),
            drawable,
            priority
        }
    }

    pub fn matches_id(&self, id: &ModelID) -> bool {
        &self.id == id
    }

    pub fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
                self.drawable.draw(cr, style, window);
    }

    pub fn bounding_box(&self) -> Option<WorldBoundingBox> {
        self.drawable.bounding_box().map(|guard| guard.clone())
    }

    pub fn is_onscreen(&self, window: &RenderWindow) -> bool {
        if let Some(guard) = self.drawable.bounding_box() {
            window.is_bounding_box_onscreen(guard.deref())
        } else {
            false
        }
    }
}

impl Debug for Drawable {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "Drawable: {:?}, {:?}", self.id(), self.bounding_box())
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