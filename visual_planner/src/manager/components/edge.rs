use super::*;
use types::*;


pub struct Edge {
    id: EdgeID,
    start: BoxID,
    end: BoxID,
    tween_points: Vec<WorldCoords>
}