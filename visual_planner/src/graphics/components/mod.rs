use super::RenderWindow;

use std::sync::Mutex;

use cairo::Context;
use gtk::StyleContext;

// Note to me: the following drawable enum will be accepted by the
// visualizer's component struct.
// possibly this is an incorrect design decision.
// see: https://users.rust-lang.org/t/performance-implications-of-box-trait-vs-enum-delegation/11957/3

/// Represents a thread-safe drawable object
pub struct DrawableObject {
     // all drawable components used by the system
     object: Mutex<Drawable>
}

impl DrawableObject {
    pub fn draw(&self, cr : &Context, style: &StyleContext, window : &RenderWindow) {
        // 1. lock mutext
        // 2. match on object and call respective draw fn
    }
}

pub enum Drawable {

}

