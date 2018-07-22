pub mod boxes;
pub mod edge;

pub use self::boxes::*;
pub use super::draw_view::{Drawable, DrawPriority};

use std::sync::Arc;

pub trait ToDrawable {
    fn to_drawable(&self) -> Arc<Drawable>;
}