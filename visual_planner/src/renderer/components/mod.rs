pub mod dialog_view;

use super::{RenderWindow, StyleScheme};

use std::sync::Mutex;

use cairo::Context;
use gtk::StyleContext;

// Note to me: the following drawablecontainer will be accepted by the
// visualizer's component struct.
// possibly this is an incorrect design decision.
// see: https://users.rust-lang.org/t/performance-implications-of-box-trait-vs-enum-delegation/11957/3

/// Represents a thread-safe drawable object
pub struct DrawableContainer {
     // using a trait object, maybe not the best???
     object: Mutex<Box<Drawable>>
}

impl DrawableContainer {
    pub fn new(drawable: Box<Drawable>) -> Self {
        DrawableContainer {
            object: Mutex::new(drawable) 
        }
    }
    pub fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
        // 1. lock mutext
        // 2. match on object and call respective draw fn
        match self.object.lock() {
            Ok(mut object) => {
                object.draw(cr, style, window); 
            }
            Err(err) => eprintln!("Error: {:?}", err)
        }
    }
}

pub trait Drawable {
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow);
}

