use graphics::types::*;

/// A thread-safe wrapper for important messages sent by gtk
#[derive(Debug,PartialEq,PartialOrd,Clone)]
pub enum GtkMessage {
    RendererScreenResize(ScreenUnit, ScreenUnit)
}
