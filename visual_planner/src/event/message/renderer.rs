use types::*;

pub enum DialogRendererMessage {
    ResizeEvent(ScreenDimensions),
    ScrollEvent(ScreenCoords, ScrollDirection, f64),
    WindowMoveEvent(ScreenUnit, ScreenUnit),
}
