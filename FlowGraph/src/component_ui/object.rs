use types::*;
use color::ColorScheme;
use super::id::ID;
use drawing_context::Context;
use std::any::Any;
use super::HandlerContext;

/// Represents an arbitrary renderable object in the system
pub trait Object {
    // used to customize when drawn
    fn draw_priority(&self) -> DrawPriority {DrawPriority::Low}

    // used to catch mouse events
    fn mouse_bounding_box(&self) -> Option<&WorldBoundingBox> {None}

    // used to decide whether to be included in a draw call
    fn render_bounding_box(&self) -> Option<WorldBoundingBox> {None}

    // called to be drawn
    fn draw(&mut self, context: &Context, root: ID, color_scheme: &ColorScheme)  {}

    // handling mouse over events
    fn motion(&mut self, coords: WorldCoords, ctx: &mut HandlerContext) -> bool {false}

    // handling update events
    fn update(&mut self, current_time: CurrentTime, elapsed_time: DeltaTime, ctx: &mut HandlerContext) {}

    // handling drag events - can be stolen by children
    fn drag_motion(&mut self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit, ctx: &mut HandlerContext) -> bool {false}

    // handling click events - can be stolen by children
    fn button_press(&mut self, button: ButtonEvent, ctx: &mut HandlerContext) -> bool {false}

    // handling key events - can be stolen by children
    fn key_press(&mut self, evnt: Key, ctx: &mut HandlerContext) -> bool { false }

    // used to send an arbitrary payload to the widget
    fn poke(&mut self, payload: &mut Any, ctx: &mut HandlerContext) -> bool { false }

    // called when the node is created
    fn create(&mut self, ctx: &mut HandlerContext) {}

    // called when the node is removed
    fn delete(&mut self, ctx: &mut HandlerContext) {}
}
