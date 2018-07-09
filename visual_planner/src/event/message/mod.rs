pub mod renderer;
use types::*;

/// A thread-safe wrapper for all messages sent 
#[derive(Debug,PartialEq,PartialOrd,Clone)]
pub enum GeneralMessage {
    RendererScreenResize(ScreenUnit, ScreenUnit),
    Scroll(ScreenUnit, ScreenUnit, ScrollDirection, f64)
}
