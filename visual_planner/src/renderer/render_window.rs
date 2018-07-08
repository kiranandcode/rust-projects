use types::*;
// Have I gone too far?
// Well, I guess I'll find out

// I've made coordiates their own type as I figure they'll be a cohesive unit in the system

type WorldWidth = WorldUnit;
type WorldHeight = WorldUnit;

type WorldX = WorldUnit;
type WorldY = WorldUnit;


type ScreenWidth = ScreenUnit;
type ScreenHeight = ScreenUnit;

type ScreenX = ScreenUnit;
type ScreenY = ScreenUnit;

type RenderX = RenderUnit;
type RenderY = RenderUnit;


const MAX_ZOOM_OUT : i32 = -100;
const MAX_ZOOM_IN  : i32 = 10;

/// Represents a rectangle in world space - can be moved and scaled freely
#[derive(Debug, PartialEq, PartialOrd)]
pub struct WorldBoundingBox(pub WorldX, pub WorldY, pub WorldWidth, pub WorldHeight);

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
        println!("Box was {:?}", self);
        self.0 = new_x;
        self.1 = new_y;
        self.2 = new_width;
        self.3 = new_height;
        println!("Box is now {:?}", self);
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
    // scale
    render_window_scale: i32,
    // screen_dimensions - width height
    screen_bounding_box: ScreenDimensions,
}

impl RenderWindow {
    // update screen_dim
    // world to screen
    pub fn new(screen_width: ScreenUnit, screen_height: ScreenUnit) -> RenderWindow {
        let width = screen_width.0;
        let height = screen_height.0;
        let x = -width/2.0;
        let y = -height/2.0;
        let dimensions = ScreenDimensions(screen_width, screen_height);

        println!("At creation xywh is {:?}, {:?}, {:?}, {:?}", x,y,width,height);
        RenderWindow {
            world_bounding_box: WorldBoundingBox(WorldUnit(x), WorldUnit(y), WorldUnit(width), WorldUnit(height)),
            render_window_scale: 0,
            screen_bounding_box: dimensions
        }
    }

    pub fn world_bounding_box(&self) -> &WorldBoundingBox {
        &self.world_bounding_box
    }

    pub fn screen_dimensions(&self) -> &ScreenDimensions {
        &self.screen_bounding_box
    }

    pub fn is_bounding_box_onscreen(&self, bounding_box: &WorldBoundingBox) -> bool {
        // check whether any vertex of the box lies within the rendering box
        WorldBoundingBox::check_intersect(&self.world_bounding_box, bounding_box)
    }

    pub fn is_point_onscreen(&self, world_coords: &WorldCoords) -> bool {
        self.world_bounding_box.point_within_bounds(world_coords)
    }

    pub fn screen_to_world_x(&self, coord_x: ScreenUnit) -> WorldUnit {
         WorldUnit((self.world_bounding_box.0).0 + (coord_x.0 / (self.screen_bounding_box.0).0) * (self.world_bounding_box.2).0)
    }

    pub fn screen_to_world_y(&self, coord_y: ScreenUnit) -> WorldUnit {
        WorldUnit((self.world_bounding_box.1).0 + (coord_y.0 / (self.screen_bounding_box.1).0) * (self.world_bounding_box.3).0)
    }
 

    pub fn screen_to_world(&self, screen_coords: &ScreenCoords) -> WorldCoords {
        let npx = (self.world_bounding_box.0).0 + ((screen_coords.0).0 / (self.screen_bounding_box.0).0) * (self.world_bounding_box.2).0;
        let npy = (self.world_bounding_box.1).0 + ((screen_coords.1).0 / (self.screen_bounding_box.1).0) * (self.world_bounding_box.3).0;
        WorldCoords(WorldUnit(npx), WorldUnit(npy))
    }

    pub fn world_to_screen_x(&self, coord_x: WorldUnit) -> ScreenUnit {
        ScreenUnit(((coord_x.0 - (self.world_bounding_box.0).0) / (self.world_bounding_box.2).0) * (self.screen_bounding_box.0).0)
    }

    pub fn world_to_screen_y(&self, coord_y: WorldUnit) -> ScreenUnit {
         ScreenUnit(((coord_y.0 - (self.world_bounding_box.1).0) / (self.world_bounding_box.3).0) * (self.screen_bounding_box.1).0)
    }
 

    pub fn world_to_screen(&self, world_coords: &WorldCoords) -> ScreenCoords {
        let npx = (((world_coords.0).0 - (self.world_bounding_box.0).0) / (self.world_bounding_box.2).0) * (self.screen_bounding_box.0).0;
        let npy = (((world_coords.1).0 - (self.world_bounding_box.1).0) / (self.world_bounding_box.3).0) * (self.screen_bounding_box.1).0;
        ScreenCoords(ScreenUnit(npx), ScreenUnit(npy))
    }

    pub fn screen_to_render_x(&self, coord_x: ScreenUnit) -> RenderUnit {
            RenderUnit(coord_x.0 / (self.screen_bounding_box.0).0)
    }

    pub fn screen_to_render_y(&self, coord_y: ScreenUnit) -> RenderUnit {
        RenderUnit(coord_y.0 / (self.screen_bounding_box.1).0)
    }

    pub fn screen_to_render(&self, screen_coords: &ScreenCoords) -> RenderCoords {
        let npx = (screen_coords.0).0 / (self.screen_bounding_box.0).0;
        let npy = (screen_coords.1).0 / (self.screen_bounding_box.1).0;
        RenderCoords(RenderUnit(npx), RenderUnit(npy))
    }

    pub fn world_to_render_x(&self, coord_x: WorldUnit) -> RenderUnit {
        self.screen_to_render_x(self.world_to_screen_x(coord_x))
    }

    pub fn world_to_render_y(&self, coord_y: WorldUnit) -> RenderUnit {
        self.screen_to_render_y(self.world_to_screen_y(coord_y))
    }

 

    pub fn world_to_render(&self, world_coords: &WorldCoords) -> RenderCoords {
        self.screen_to_render(&self.world_to_screen(world_coords))
    }

    pub fn zoom_window(&mut self, center: &ScreenCoords, direction: ScrollDirection, mut delta: f64) {
        println!("I am being asked to scale you!, delta: {}", delta);

        let point = self.screen_to_world(center);
        // self.render_window_scale = self.render_window_scale + WorldUnit(delta);
        let scale = WorldUnit(delta);
        let current_level = self.render_window_scale;
        match direction {
            ScrollDirection::Up => {
                if current_level < MAX_ZOOM_IN {
                    self.render_window_scale += 1;
                    self.world_bounding_box.scale_box_around_point(scale, scale, &point);
                }
            }
            ScrollDirection::Down => {
                if current_level > MAX_ZOOM_OUT {
                    self.render_window_scale -= 1;
                    self.world_bounding_box.scale_box_around_point(scale, scale, &point);
                }
            }
        }


    }

    /// Updates the screen dimensions maintaining the aspect ratio
    pub fn update_screen_dimensions(&mut self, screen_dimensions: ScreenDimensions) {
        if (self.screen_bounding_box.0).0 != (screen_dimensions.0).0  {

            let ratio = WorldUnit((screen_dimensions.0).0 / (screen_dimensions.1).0);

            let scaling = WorldUnit((screen_dimensions.1).0 / (self.screen_bounding_box.1).0);

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
        let mut render_window = RenderWindow::new(ScreenUnit(0.0), ScreenUnit(0.0));

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
        let mut render_window = RenderWindow::new(ScreenUnit(0.0), ScreenUnit(0.0));

        render_window.world_bounding_box = simple_world_box;
        render_window.screen_bounding_box = simple_screen_box;

        render_window.update_screen_dimensions(ScreenDimensions(ScreenUnit(100.0), ScreenUnit(100.0)));
        assert_eq!(render_window.screen_bounding_box, ScreenDimensions(ScreenUnit(100.0), ScreenUnit(100.0)));
        assert_eq!(render_window.world_bounding_box, WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(50.0), WorldUnit(50.0)));
    }

}
