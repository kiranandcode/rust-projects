use types::*;


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EdgeModel {
    id: EdgeID,
    start: BoxID,
    end: BoxID,
    tween_points: Vec<WorldCoords>
}