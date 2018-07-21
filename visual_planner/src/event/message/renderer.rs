use types::*;
use state::*;
use std::sync::Arc;
use manager::draw_view::Drawable;

#[derive(Debug,Clone)]
pub enum DialogRendererMessage {
    ResizeEvent(ScreenDimensions),
    ScrollEvent(ScreenCoords, ScrollDirection, f64),
    WindowMoveEvent(ScreenUnit, ScreenUnit),
    RegisterDrawable(Arc<Drawable>),
    RedrawRequest(WorldBoundingBox),
}


pub enum DialogStateMessage {
    ClickEvent(ScreenCoords),
    MotionEvent(ScreenCoords),
    SetDialogState(DialogInputState)
}