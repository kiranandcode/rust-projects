use std::ops::Add;
/// a newtype representing world units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct WorldUnit(pub f32);
/// a newtype representing screen units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
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

impl Add for WorldUnit {
    type Output = WorldUnit;
    fn add(self, other : WorldUnit) -> WorldUnit {
        WorldUnit(self.0 + other.0)
    }
}

impl WorldBoundingBox {

    pub fn point_within_bounds(&self, x : WorldUnit, y: WorldUnit) -> bool {
        let self_x = (self.0);
        let self_y = (self.1);
        let self_w = (self.2); 
        let self_h = (self.3); 
 
            (x >= self_x) && (x <= self_x + self_w) &&
                (y >= self_y) && (y <= self_y + self_h)
    }


    pub fn check_intersect(boxa : &WorldBoundingBox, boxb : &WorldBoundingBox) -> bool {
        let WorldBoundingBox(boxa_x, boxa_y, boxa_w, boxa_h) = *boxa;
        let WorldBoundingBox(boxb_x, boxb_y, boxb_w, boxb_h) = *boxb;

        // check whether any vertex of the rendering box lies within the box
        boxa.point_within_bounds(boxb_x         , boxb_y         ) ||
        boxa.point_within_bounds(boxb_x + boxb_w, boxb_y         ) ||
        boxa.point_within_bounds(boxb_x         , boxb_y + boxb_h) ||
        boxa.point_within_bounds(boxb_x + boxb_w, boxb_y + boxb_h) ||


        // check whether any vertex of the rendering box lies within the box
        boxb.point_within_bounds(boxa_x         , boxa_y         ) ||
        boxb.point_within_bounds(boxa_x + boxa_w, boxa_y         ) ||
        boxb.point_within_bounds(boxa_x         , boxa_y + boxa_h) ||
        boxb.point_within_bounds(boxa_x + boxa_w, boxa_y + boxa_h)


    }
}

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
        // check whether any vertex of the box lies within the rendering box
        WorldBoundingBox::check_intersect(&self.world_bounding_box, bounding_box)
    }
}


#[cfg(test)]
mod test {
    use super::*;
     
    #[test]
    pub fn point_within_bounds_inside() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0), WorldUnit(1.0)); 
        assert!(unit_box.point_within_bounds(WorldUnit(0.5), WorldUnit(0.5)));
    }

    #[test]
    pub fn point_within_bounds_outside() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0), WorldUnit(1.0)); 
        assert!(!unit_box.point_within_bounds(WorldUnit(1.5), WorldUnit(1.5)));
    }


}
