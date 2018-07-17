use types::*;
use undo::{Modifiable, Modification};


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EdgeModel {
    id: EdgeID,
    start: BoxID,
    end: BoxID,
    tween_points: Vec<WorldCoords>
}


impl Modifiable for EdgeModel {
    fn update_state(&mut self, other: &Self) -> Modification {
        // TODO: Implement this
        Modification::Deleted
    }
}