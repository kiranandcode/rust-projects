use types::*;
use super::id::*;
use drawing_context::*;

/// Represents an arbitrary renderable object in the system
pub trait Object {
    // used to customize when drawn
    fn draw_priority(&self) -> DrawPriority {DrawPriority::Low}

    // used to catch mouse events
    fn mouse_bounding_box(&mut self) -> Option<&WorldBoundingBox> {None}

    // used to decide whether to be included in a draw call
    fn render_bounding_box(&mut self) -> Option<WorldBoundingBox> {None}

    // called to be drawn
    fn draw(&mut self, context: &Context, root: ID)  {}

    // handling mouse over events
    fn motion(&mut self, coords: WorldCoords) -> bool {false}

    // handling update events
    fn update(&mut self, current_time: CurrentTime, elapsed_time: DeltaTime) {}

    // handling drag events - can be stolen by children
    fn drag_motion(&mut self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) -> bool {false}

    // handling click events - can be stolen by children
    fn button_press(&mut self, button: ButtonEvent) -> bool {false}

    // handling key events - can be stolen by children
    fn key_press(&mut self, evnt: Key) {}
}
