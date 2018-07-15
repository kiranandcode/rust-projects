use types::*;
use state::*;

pub enum DialogRendererMessage {
    ResizeEvent(ScreenDimensions),
    ScrollEvent(ScreenCoords, ScrollDirection, f64),
    WindowMoveEvent(ScreenUnit, ScreenUnit),
    ClickEvent(ScreenCoords),
    MotionEvent(ScreenCoords),
    SetDialogState(DialogInputState)
}
