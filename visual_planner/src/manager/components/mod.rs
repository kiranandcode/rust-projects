pub mod boxes;
pub mod edge;

use super::draw_view::Drawable;

use std::sync::Arc;

pub trait ToDrawable {
    fn to_drawable(&self) -> Arc<Drawable>;
}