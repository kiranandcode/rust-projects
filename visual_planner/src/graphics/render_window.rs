use std::ops::Add;
/// a newtype representing world units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd)]
pub struct WorldUnit(pub f32);
/// a newtype representing screen units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ScreenUnit(pub f32);

// I've made coordiates their own type as I figure they'll be a cohesive unit in the system

/// a newtype representing world coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd)]
pub struct WorldCoords(pub WorldUnit, pub WorldUnit);
/// a newtype representing screen coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ScreenCoords(pub ScreenUnit, pub ScreenUnit);

// Have I gone too far?
// Well, I guess I'll find out

type WorldWidth = WorldUnit;
type WorldHeight = WorldUnit;

type WorldX = WorldUnit;
type WorldY = WorldUnit;


type ScreenWidth = ScreenUnit;
type ScreenHeight = ScreenUnit;

type ScreenX = ScreenUnit;
type ScreenY = ScreenUnit;


pub struct WorldBoundingBox(pub WorldX, pub WorldY, pub WorldWidth, pub WorldHeight);
pub struct ScreenBoundingBox(pub ScreenX, pub ScreenY, pub ScreenWidth, pub ScreenHeight);

/// Represents a mapping between a virtual window in worldspace to the screen
/// Could be implemented as a 3d matrix, but would require pulling in
/// additional dependancies
pub struct RenderWindow {
    // world_dimensions - width, height
    world_bounding_box: WorldBoundingBox,
    // screen_dimensions - width height
    screen_bounding_box: ScreenBoundingBox
}

impl RenderWindow {
    // update screen_dim
    // update world_dim
    // world to screen
    pub fn new() -> RenderWindow {
        RenderWindow {
            world_bounding_box: WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(0.0), WorldUnit(0.0)),
            screen_bounding_box: ScreenBoundingBox(ScreenUnit(0.0), ScreenUnit(0.0), ScreenUnit(0.0), ScreenUnit(0.0)),
        }
    }

    pub fn is_in_view(&self, bounding_box: &WorldBoundingBox) -> bool {
        let self_x = (self.world_bounding_box.0).0;
        let self_y = (self.world_bounding_box.1).0;
        let self_w = (self.world_bounding_box.2).0; 
        let self_h = (self.world_bounding_box.3).0; 
        let bounding_x = (bounding_box.0).0;
        let bounding_y = (bounding_box.1).0;
        let bounding_w = (bounding_box.2).0;
        let bounding_h = (bounding_box.3).0;

        let point_within_screen_bounds = |x : f32, y : f32| -> bool {
            (x >= self_x) && (x <= self_x + self_w) &&
                (y >= self_y) && (y <= self_y + self_h)
        };
        let point_within_bounding_bounds = |x : f32, y : f32| -> bool {
            (x >= bounding_x) && (x <= bounding_x + bounding_w) &&
                (y >= bounding_y) && (y <= bounding_y + bounding_h)
        };



        // check whether any vertex of the box lies within the rendering box
        point_within_screen_bounds(bounding_x             , bounding_y             ) ||
        point_within_screen_bounds(bounding_x + bounding_w, bounding_y             ) ||
        point_within_screen_bounds(bounding_x             , bounding_y + bounding_h) ||
        point_within_screen_bounds(bounding_x + bounding_w, bounding_y + bounding_h) ||

        // check whether any vertex of the rendering box lies within the box
        point_within_bounding_bounds(self_x         , self_y         ) ||
        point_within_bounding_bounds(self_x + self_w, self_y         ) ||
        point_within_bounding_bounds(self_x         , self_y + self_h) ||
        point_within_bounding_bounds(self_x + self_w, self_y + self_h)
    }
}
