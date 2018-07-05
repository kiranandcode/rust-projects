use std::ops::{Add,Sub,Mul};
/// a newtype representing world units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct WorldUnit(pub f32);
/// a newtype representing screen units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct ScreenUnit(pub f32);
/// a newtype representing render units (0.0 - 1.0) to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct RenderUnit(pub f32);

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


/// Represents a rectangle in world space - can be moved and scaled freely
#[derive(Debug, PartialEq, PartialOrd)]
pub struct WorldBoundingBox(pub WorldX, pub WorldY, pub WorldWidth, pub WorldHeight);

/// Represents a rectangle in screen space - immovable, but can be rescaled
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ScreenDimensions(pub ScreenWidth, pub ScreenHeight);

impl Add for WorldUnit {
    type Output = WorldUnit;
    fn add(self, other : WorldUnit) -> WorldUnit {
        WorldUnit(self.0 + other.0)
    }
}

impl Sub for WorldUnit {
    type Output = WorldUnit;
    fn sub(self, other : WorldUnit) -> WorldUnit {
        WorldUnit(self.0 - other.0)
    }
}

impl Mul for WorldUnit {
    type Output = WorldUnit;
    fn mul(self, other : WorldUnit) -> WorldUnit {
        WorldUnit(self.0 * other.0)
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

    pub fn move_box(&mut self, dx : WorldUnit, dy : WorldUnit) {
        (self.0).0 += dx.0;
        (self.1).0 += dy.0;
    }

    pub fn scale_box(&mut self, sx : WorldUnit, sy : WorldUnit) {
        assert!((sx.0 > 0.0) && (sy.0 > 0.0));
        (self.2).0 *= sx.0;
        (self.3).0 *= sy.0;
    }

    pub fn scale_box_around_center(&mut self, sx : WorldUnit, sy: WorldUnit) {
        // offset + i/2 (scale * old_length) = base + 1/2 old_length
       
        let new_width = (self.2 * sx).0;
        let new_height = (self.3 * sy).0;
        let old_mid_x = (self.0).0 + (self.2).0/2.0;
        let old_mid_y = (self.1).0 + (self.3).0/2.0;
        (self.0).0 = old_mid_x - new_width/2.0;
        (self.1).0 = old_mid_y - new_height/2.0;
        (self.2).0 = new_width;
        (self.3).0 = new_height;
    }

    pub fn scale_box_around_point(&mut self, sx : WorldUnit, sy: WorldUnit, point : WorldCoords) {
        let new_width = self.2 * sx;
        let new_height = self.3 * sy;
        let new_x = (self.0 - point.0) * sx + point.0;
        let new_y = (self.1 - point.1) * sy + point.1;
        self.0 = new_x;
        self.1 = new_y;
        self.2 = new_width;
        self.3 = new_height;

    }

    pub fn set_box_between(&mut self, point_a : WorldCoords, point_b : WorldCoords) {
    }


    pub fn set_box(&mut self, point : WorldCoords, width: WorldWidth, height: WorldHeight) {

    }
}

/// Represents a mapping between a virtual window in worldspace to the screen
/// Could be implemented as a 3d matrix, but would require pulling in
/// additional dependancies
pub struct RenderWindow {
    // world_dimensions - width, height
    world_bounding_box: WorldBoundingBox,
    // screen_dimensions - width height
    screen_bounding_box: ScreenDimensions
}

impl RenderWindow {
    // update screen_dim
    // world to screen
    pub fn new() -> RenderWindow {
        RenderWindow {
            world_bounding_box: WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(0.0), WorldUnit(0.0)),
            screen_bounding_box: ScreenDimensions(ScreenUnit(0.0), ScreenUnit(0.0)),
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
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        assert!(unit_box.point_within_bounds(WorldUnit(0.5), WorldUnit(0.5)));
    }

    #[test]
    pub fn point_within_bounds_outside() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        assert!(!unit_box.point_within_bounds(WorldUnit(1.5), WorldUnit(1.5)));
    }

    #[test]
    pub fn check_intersect_outside() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        let moved_box = WorldBoundingBox(WorldUnit(2.0), WorldUnit(2.0), WorldUnit(1.0), WorldUnit(1.0));
        assert!(!WorldBoundingBox::check_intersect(&unit_box, &moved_box));
    }

    #[test]
    pub fn check_intersect_edge() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        let moved_box = WorldBoundingBox(WorldUnit(1.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0));
        assert!(WorldBoundingBox::check_intersect(&unit_box, &moved_box));
    }

    #[test]
    pub fn check_intersect_corner() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        let moved_box = WorldBoundingBox(WorldUnit(1.0), WorldUnit(1.0), WorldUnit(1.0), WorldUnit(1.0));
        assert!(WorldBoundingBox::check_intersect(&unit_box, &moved_box));
    }

    #[test]
    pub fn check_intersect_inside() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        let moved_box = WorldBoundingBox(WorldUnit(0.2), WorldUnit(0.2), WorldUnit(0.6), WorldUnit(0.6));
        assert!(WorldBoundingBox::check_intersect(&unit_box, &moved_box));
    }

    #[test]
    pub fn check_intersect_intersect() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        let moved_box = WorldBoundingBox(WorldUnit(0.2), WorldUnit(0.2), WorldUnit(1.0), WorldUnit(1.0));
        assert!(WorldBoundingBox::check_intersect(&unit_box, &moved_box));
    }

    #[test]
    pub fn move_box_works() {
        let mut unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        unit_box.move_box(WorldUnit(1.0), WorldUnit(1.0));
        assert_eq!(unit_box, WorldBoundingBox(WorldUnit(1.0), WorldUnit(1.0), WorldUnit(1.0), WorldUnit(1.0))); 
    }

    #[test]
    pub fn scale_box_works() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(2.0)); 
        simple_box.scale_box(WorldUnit(2.0), WorldUnit(3.0));
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(2.0), WorldUnit(6.0))); 
    }

    #[test]
    #[should_panic]
    pub fn scale_box_checks() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(2.0)); 
        simple_box.scale_box(WorldUnit(0.0), WorldUnit(3.0));
    }

    #[test]
    pub fn scale_box_around_center_works() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        simple_box.scale_box_around_center(WorldUnit(2.0), WorldUnit(3.0));
        // offset + i/2 (scale * old_length) = base + 1/2 old_length
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(-0.5), WorldUnit(-1.0), WorldUnit(2.0), WorldUnit(3.0))); 
    }

    #[test]
    pub fn scale_box_around_point_center_works() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        simple_box.scale_box_around_point(WorldUnit(2.0), WorldUnit(3.0), WorldCoords(WorldUnit(0.5), WorldUnit(0.5)));
        // offset + i/2 (scale * old_length) = base + 1/2 old_length
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(-0.5), WorldUnit(-1.0), WorldUnit(2.0), WorldUnit(3.0))); 
    }


    #[test]
    pub fn scale_box_around_point_corner_works() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        simple_box.scale_box_around_point(WorldUnit(2.0), WorldUnit(3.0), WorldCoords(WorldUnit(1.0), WorldUnit(1.0)));
        // offset + i/2 (scale * old_length) = base + 1/2 old_length
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(-1.0), WorldUnit(-2.0), WorldUnit(2.0), WorldUnit(3.0))); 
    }


    #[test]
    pub fn set_box_between_works() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        simple_box.set_box_between(WorldCoords(WorldUnit(1.0), WorldUnit(1.0)), WorldCoords(WorldUnit(3.0), WorldUnit(3.0)));
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(1.0), WorldUnit(1.0), WorldUnit(2.0), WorldUnit(2.0))); 
    }

    #[test]
    pub fn set_box_between_order_irrelevant() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        simple_box.set_box_between(WorldCoords(WorldUnit(3.0), WorldUnit(3.0)), WorldCoords(WorldUnit(1.0), WorldUnit(1.0)));
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(1.0), WorldUnit(1.0), WorldUnit(2.0), WorldUnit(2.0))); 
    }




    #[test]
    pub fn set_box_works() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        simple_box.set_box(WorldCoords(WorldUnit(2.0), WorldUnit(2.0)), WorldUnit(3.0), WorldUnit(2.0));
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(2.0), WorldUnit(2.0), WorldUnit(3.0), WorldUnit(2.0))); 
    }

}
