use std::ops::{Add,Sub,Mul};
use std::cmp;

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
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WorldCoords(pub WorldUnit, pub WorldUnit);
/// a newtype representing screen coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ScreenCoords(pub ScreenUnit, pub ScreenUnit);
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RenderCoords(pub RenderUnit, pub RenderUnit);

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

impl ScreenDimensions {
    fn set_width(&mut self, width : ScreenWidth) {
        assert!(width.0 > 0.0);
        self.0  = width;
    }

    fn set_height(&mut self, height : ScreenHeight) {
        assert!(height.0 > 0.0);
        self.1 = height;
    }

    fn set_dimensions(&mut self, width : ScreenWidth, height: ScreenHeight) {
        assert!(width.0 > 0.0);
        assert!(height.0 > 0.0);

        self.0  = width;
        self.1 = height;
    }

}

impl WorldBoundingBox {

    pub fn point_within_bounds(&self, point : &WorldCoords) -> bool {
        let self_x = (self.0);
        let self_y = (self.1);
        let self_w = (self.2); 
        let self_h = (self.3); 
        let x = point.0;
        let y = point.1;
 
            (x >= self_x) && (x <= self_x + self_w) &&
                (y >= self_y) && (y <= self_y + self_h)
    }


    pub fn check_intersect(boxa : &WorldBoundingBox, boxb : &WorldBoundingBox) -> bool {
        let WorldBoundingBox(boxa_x, boxa_y, boxa_w, boxa_h) = *boxa;
        let WorldBoundingBox(boxb_x, boxb_y, boxb_w, boxb_h) = *boxb;

        // check whether any vertex of the rendering box lies within the box
        boxa.point_within_bounds(&WorldCoords(boxb_x         , boxb_y         )) ||
        boxa.point_within_bounds(&WorldCoords(boxb_x + boxb_w, boxb_y         )) ||
        boxa.point_within_bounds(&WorldCoords(boxb_x         , boxb_y + boxb_h)) ||
        boxa.point_within_bounds(&WorldCoords(boxb_x + boxb_w, boxb_y + boxb_h)) ||


        // check whether any vertex of the rendering box lies within the box
        boxb.point_within_bounds(&WorldCoords(boxa_x         , boxa_y         )) ||
        boxb.point_within_bounds(&WorldCoords(boxa_x + boxa_w, boxa_y         )) ||
        boxb.point_within_bounds(&WorldCoords(boxa_x         , boxa_y + boxa_h)) ||
        boxb.point_within_bounds(&WorldCoords(boxa_x + boxa_w, boxa_y + boxa_h))
    }

    fn move_box(&mut self, dx : WorldUnit, dy : WorldUnit) {
        (self.0).0 += dx.0;
        (self.1).0 += dy.0;
    }

    pub fn scale_box(&mut self, sx : WorldUnit, sy : WorldUnit) {
        assert!((sx.0 > 0.0) && (sy.0 > 0.0));
        (self.2).0 *= sx.0;
        (self.3).0 *= sy.0;
    }

    fn scale_box_around_center(&mut self, sx : WorldUnit, sy: WorldUnit) {
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

    fn scale_box_around_point(&mut self, sx : WorldUnit, sy: WorldUnit, point : &WorldCoords) {
        let new_width = self.2 * sx;
        let new_height = self.3 * sy;
        let new_x = (self.0 - point.0) * sx + point.0;
        let new_y = (self.1 - point.1) * sy + point.1;
        self.0 = new_x;
        self.1 = new_y;
        self.2 = new_width;
        self.3 = new_height;

    }

    fn set_box_between(&mut self, point_a : WorldCoords, point_b : WorldCoords) {
        let (lower_x, upper_x) = if point_a.0 > point_b.0 {(point_b.0, point_a.0)} else {(point_a.0, point_b.0)} ;
        let (lower_y, upper_y) = if point_a.1 > point_b.1 {(point_b.1, point_a.1)} else {(point_a.1, point_b.1)} ;

        let width = upper_x - lower_x;
        let height = upper_y - lower_y;

        self.0 = lower_x;
        self.1 = lower_y;
        self.2 = width;
        self.3 = height;
    }


    fn set_box(&mut self, point : WorldCoords, width: WorldWidth, height: WorldHeight) {
        self.0 = point.0;
        self.1 = point.1;
        self.2 = width;
        self.3 = height;
    }

    fn set_width(&mut self, width : WorldWidth) {
        assert!(width.0 > 0.0);
        self.2  = width;
    }

    fn set_height(&mut self, height : WorldHeight) {
        assert!(height.0 > 0.0);
        self.3 = height;
    }

    fn set_dimensions(&mut self, width : WorldWidth, height: WorldHeight) {
        assert!(width.0 > 0.0);
        assert!(height.0 > 0.0);

        self.2  = width;
        self.3 = height;
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

    pub fn is_bounding_box_onscreen(&self, bounding_box: &WorldBoundingBox) -> bool {
        // check whether any vertex of the box lies within the rendering box
        WorldBoundingBox::check_intersect(&self.world_bounding_box, bounding_box)
    }

    pub fn is_point_onscreen(&self, world_coords: &WorldCoords) -> bool {
        self.world_bounding_box.point_within_bounds(world_coords)
    }

    pub fn screen_to_world(&self, screen_coords: &ScreenCoords) -> WorldCoords {
        let npx = (self.world_bounding_box.0).0 + ((screen_coords.0).0 / (self.screen_bounding_box.0).0) * (self.world_bounding_box.2).0;
        let npy = (self.world_bounding_box.1).0 + ((screen_coords.1).0 / (self.screen_bounding_box.1).0) * (self.world_bounding_box.3).0;
        WorldCoords(WorldUnit(npx), WorldUnit(npy))
    }

    pub fn world_to_screen(&self, world_coords: &WorldCoords) -> ScreenCoords {
        let npx = (((world_coords.0).0 - (self.world_bounding_box.0).0) / (self.world_bounding_box.2).0) * (self.screen_bounding_box.0).0;
        let npy = (((world_coords.1).0 - (self.world_bounding_box.1).0) / (self.world_bounding_box.3).0) * (self.screen_bounding_box.1).0;
        ScreenCoords(ScreenUnit(npx), ScreenUnit(npy))
    }

    pub fn screen_to_render(&self, screen_coords: &ScreenCoords) -> RenderCoords {
        let npx = (screen_coords.0).0 / (self.screen_bounding_box.0).0;
        let npy = (screen_coords.1).0 / (self.screen_bounding_box.1).0;
        RenderCoords(RenderUnit(npx), RenderUnit(npy))
    }

    pub fn world_to_render(&self, world_coords: &WorldCoords) -> RenderCoords {
        self.screen_to_render(&self.world_to_screen(world_coords))
    }

    /// Updates the screen dimensions maintaining the aspect ratio
    pub fn update_screen_dimensions(&mut self, screen_dimensions: ScreenDimensions) {
        if (self.screen_bounding_box.0).0 != (screen_dimensions.0).0  {
            let ratio = WorldUnit((screen_dimensions.0).0 / (screen_dimensions.1).0);
            let scaling = WorldUnit((screen_dimensions.0).0 / (self.screen_bounding_box.0).0);

            self.screen_bounding_box.set_dimensions(screen_dimensions.0, screen_dimensions.1);

            let new_width = self.world_bounding_box.3 * ratio * scaling;
            let new_height = self.world_bounding_box.3 * scaling;
            self.world_bounding_box.set_dimensions(new_width, new_height);
        } else {
            let ratio = WorldUnit((screen_dimensions.1).0 / (screen_dimensions.0).0);
            let scaling = WorldUnit((screen_dimensions.0).0 / (self.screen_bounding_box.0).0);

            self.screen_bounding_box.set_dimensions(screen_dimensions.0, screen_dimensions.1);

            let new_height = self.world_bounding_box.2 * ratio * scaling;
            let new_width = self.world_bounding_box.2 * scaling;
            self.world_bounding_box.set_dimensions(new_width, new_height);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
     
    #[test]
    pub fn point_within_bounds_inside() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        assert!(unit_box.point_within_bounds(&WorldCoords(WorldUnit(0.5), WorldUnit(0.5))));
    }

    #[test]
    pub fn point_within_bounds_outside() {
        let unit_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        assert!(!unit_box.point_within_bounds(&WorldCoords(WorldUnit(1.5), WorldUnit(1.5))));
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
        simple_box.scale_box_around_point(WorldUnit(2.0), WorldUnit(3.0), &WorldCoords(WorldUnit(0.5), WorldUnit(0.5)));
        // offset + i/2 (scale * old_length) = base + 1/2 old_length
        assert_eq!(simple_box, WorldBoundingBox(WorldUnit(-0.5), WorldUnit(-1.0), WorldUnit(2.0), WorldUnit(3.0))); 
    }


    #[test]
    pub fn scale_box_around_point_corner_works() {
        let mut simple_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        simple_box.scale_box_around_point(WorldUnit(2.0), WorldUnit(3.0), &WorldCoords(WorldUnit(1.0), WorldUnit(1.0)));
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

    #[test]
    pub fn update_screen_dimensions_simple_works() {
        let mut simple_world_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(1.0)); 
        let mut simple_screen_box = ScreenDimensions(ScreenUnit(10.0), ScreenUnit(10.0)); 
        let mut render_window = RenderWindow::new();

        render_window.world_bounding_box = simple_world_box;
        render_window.screen_bounding_box = simple_screen_box;

        render_window.update_screen_dimensions(ScreenDimensions(ScreenUnit(100.0), ScreenUnit(100.0)));
        assert_eq!(render_window.screen_bounding_box, ScreenDimensions(ScreenUnit(100.0), ScreenUnit(100.0)));
        assert_eq!(render_window.world_bounding_box, WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(10.0), WorldUnit(10.0)));
        
    }


    #[test]
    pub fn update_screen_dimensions_works() {
        let mut simple_world_box = WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(1.0), WorldUnit(5.0)); 
        let mut simple_screen_box = ScreenDimensions(ScreenUnit(10.0), ScreenUnit(50.0)); 
        let mut render_window = RenderWindow::new();

        render_window.world_bounding_box = simple_world_box;
        render_window.screen_bounding_box = simple_screen_box;

        render_window.update_screen_dimensions(ScreenDimensions(ScreenUnit(100.0), ScreenUnit(100.0)));
        assert_eq!(render_window.screen_bounding_box, ScreenDimensions(ScreenUnit(100.0), ScreenUnit(100.0)));
        assert_eq!(render_window.world_bounding_box, WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(50.0), WorldUnit(50.0)));
    }



}
