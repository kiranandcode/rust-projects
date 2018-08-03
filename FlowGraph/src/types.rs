use std::ops::{Add, AddAssign, Mul, Sub, Div};
use std::cmp::Ordering;

// Using type synnonyms here to comprimise between safety and developer velocity.

pub type WorldWidth = WorldUnit;
pub type WorldHeight = WorldUnit;

pub type WorldX = WorldUnit;
pub type WorldY = WorldUnit;

pub type ScreenWidth = ScreenUnit;
pub type ScreenHeight = ScreenUnit;

pub type ScreenX = ScreenUnit;
pub type ScreenY = ScreenUnit;

pub type RenderX = RenderUnit;
pub type RenderY = RenderUnit;

pub type CurrentTime = TimeUnit;
pub type DeltaTime = TimeUnit;

/// a newtype representing time to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct TimeUnit(pub f64);
/// a newtype representing world units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct WorldUnit(pub f64);
/// a newtype representing screen units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct ScreenUnit(pub f64);
/// a newtype representing render units (0.0 - 1.0) to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct RenderUnit(pub f64);

/// a newtype representing world coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct WorldCoords(pub WorldX, pub WorldY);
/// a newtype representing screen coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ScreenCoords(pub ScreenX, pub ScreenY);
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct RenderCoords(pub RenderX, pub RenderY);

impl Add for WorldUnit {
    type Output = WorldUnit;
    fn add(self, other: WorldUnit) -> WorldUnit {
        WorldUnit(self.0 + other.0)
    }
}

impl Sub for WorldUnit {
    type Output = WorldUnit;
    fn sub(self, other: WorldUnit) -> WorldUnit {
        WorldUnit(self.0 - other.0)
    }
}

impl Mul for WorldUnit {
    type Output = WorldUnit;
    fn mul(self, other: WorldUnit) -> WorldUnit {
        WorldUnit(self.0 * other.0)
    }
}
impl Div for WorldUnit {
    type Output = WorldUnit;
    fn div(self, other: WorldUnit) -> WorldUnit {
        WorldUnit(self.0 / other.0)
    }
}
impl Add for ScreenUnit {
    type Output = ScreenUnit;
    fn add(self, other: ScreenUnit) -> ScreenUnit {
        ScreenUnit(self.0 + other.0)
    }
}

impl Sub for ScreenUnit {
    type Output = ScreenUnit;
    fn sub(self, other: ScreenUnit) -> ScreenUnit {
        ScreenUnit(self.0 - other.0)
    }
}

impl Mul for ScreenUnit {
    type Output = ScreenUnit;
    fn mul(self, other: ScreenUnit) -> ScreenUnit {
        ScreenUnit(self.0 * other.0)
    }
}

/// Represents a rectangle in screen space - immovable, but can be rescaled
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ScreenDimensions(pub ScreenWidth, pub ScreenHeight);
impl ScreenDimensions {
    pub fn set_width(&mut self, width: ScreenWidth) {
        assert!(width.0 > 0.0);
        self.0 = width;
    }

    pub fn set_height(&mut self, height: ScreenHeight) {
        assert!(height.0 > 0.0);
        self.1 = height;
    }

    pub fn set_dimensions(&mut self, width: ScreenWidth, height: ScreenHeight) {
        assert!(width.0 > 0.0);
        assert!(height.0 > 0.0);

        self.0 = width;
        self.1 = height;
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ScreenBoundingBox(pub ScreenX, pub ScreenY, pub ScreenWidth, pub ScreenHeight);




/// Represents a rectangle in world space - can be moved and scaled freely
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WorldBoundingBox(pub WorldX, pub WorldY, pub WorldWidth, pub WorldHeight);

impl WorldBoundingBox {
    pub fn point_within_bounds(&self, point: &WorldCoords) -> bool {
        let self_x = (self.0);
        let self_y = (self.1);
        let self_w = (self.2);
        let self_h = (self.3);
        let x = point.0;
        let y = point.1;

        (x >= self_x) && (x <= self_x + self_w) && (y >= self_y) && (y <= self_y + self_h)
    }

    pub fn check_intersect(boxa: &WorldBoundingBox, boxb: &WorldBoundingBox) -> bool {
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

    pub fn move_box(&mut self, dx: WorldUnit, dy: WorldUnit) {
        (self.0).0 += dx.0;
        (self.1).0 += dy.0;
    }

    pub fn scale_box(&mut self, sx: WorldUnit, sy: WorldUnit) {
        assert!((sx.0 > 0.0) && (sy.0 > 0.0));
        (self.2).0 *= sx.0;
        (self.3).0 *= sy.0;
    }

    pub fn scale_box_around_center(&mut self, sx: WorldUnit, sy: WorldUnit) {
        // offset + i/2 (scale * old_length) = base + 1/2 old_length

        let new_width = (self.2 * sx).0;
        let new_height = (self.3 * sy).0;
        let old_mid_x = (self.0).0 + (self.2).0 / 2.0;
        let old_mid_y = (self.1).0 + (self.3).0 / 2.0;
        (self.0).0 = old_mid_x - new_width / 2.0;
        (self.1).0 = old_mid_y - new_height / 2.0;
        (self.2).0 = new_width;
        (self.3).0 = new_height;
    }

    pub fn scale_box_around_point(&mut self, sx: WorldUnit, sy: WorldUnit, point: &WorldCoords) {
        let new_width = self.2 * sx;
        let new_height = self.3 * sy;
        let new_x = (self.0 - point.0) * sx + point.0;
        let new_y = (self.1 - point.1) * sy + point.1;
        // println!("Box was {:?}", self);
        self.0 = new_x;
        self.1 = new_y;
        self.2 = new_width;
        self.3 = new_height;
        // println!("Box is now {:?}", self);
    }

    pub fn new_between(point_a: WorldCoords, point_b: WorldCoords) -> Self {
        let (lower_x, upper_x) = if point_a.0 > point_b.0 {
            (point_b.0, point_a.0)
        } else {
            (point_a.0, point_b.0)
        };
        let (lower_y, upper_y) = if point_a.1 > point_b.1 {
            (point_b.1, point_a.1)
        } else {
            (point_a.1, point_b.1)
        };

        let width = upper_x - lower_x;
        let height = upper_y - lower_y;

        WorldBoundingBox(lower_x, lower_y, width, height)
    }

    pub fn new(point: WorldCoords, width: WorldWidth, height: WorldHeight) -> Self {
        WorldBoundingBox(point.0, point.1, width, height)
    }

    pub fn new_centered_at(point: WorldCoords, width: WorldWidth, height: WorldHeight) -> Self {
        WorldBoundingBox(
            WorldUnit((point.0).0 - width.0 / 2.0),
            WorldUnit((point.1).0 - height.0 / 2.0),
            width,
            height,
        )
    }

    // Constructs the smallest box bounding two other boxes
    pub fn union_boxes(box_0 : &WorldBoundingBox, box_1 : &WorldBoundingBox) -> WorldBoundingBox {
        let mut box_n = box_0.clone();
        box_n.union(box_1);
        box_n 
    }

    /// Updates a bounding box to contain the smallest possible box containing both boxes
    pub fn union(&mut self, other: &WorldBoundingBox) {
        let (o_x, o_y, o_w, o_h) = (
            (self.0).0,
            (self.1).0,
            (self.2).0,
            (self.3).0,
        );
        let (o_x, o_y, o_w, o_h) = (o_x as f64, o_y as f64, o_w as f64, o_h as f64);
        let (x, y, w, h) = (
            (other.0).0,
            (other.1).0,
            (other.2).0,
            (other.3).0,
        );
        let (x, y, w, h) = (x as f64, y as f64, w as f64, h as f64);



        let n_x = if o_x < x { o_x } else { x };
        let n_y = if o_y < y { o_y } else { y };
        let n_w = (if o_x + o_w > x + w { o_x + o_w } else { x + w }) - n_x;
        let n_h = (if o_y + o_h > y + h { o_y + o_h } else { y + h }) - n_y;

        self.0 = WorldUnit(n_x);
        self.1 = WorldUnit(n_y);
        self.2 = WorldUnit(n_w);
        self.3 = WorldUnit(n_h);
    }

    pub fn set_box_between(&mut self, point_a: WorldCoords, point_b: WorldCoords) {
        let (lower_x, upper_x) = if point_a.0 > point_b.0 {
            (point_b.0, point_a.0)
        } else {
            (point_a.0, point_b.0)
        };
        let (lower_y, upper_y) = if point_a.1 > point_b.1 {
            (point_b.1, point_a.1)
        } else {
            (point_a.1, point_b.1)
        };

        let width = upper_x - lower_x;
        let height = upper_y - lower_y;

        self.0 = lower_x;
        self.1 = lower_y;
        self.2 = width;
        self.3 = height;
    }

    pub fn set_box(&mut self, point: WorldCoords, width: WorldWidth, height: WorldHeight) {
        self.0 = point.0;
        self.1 = point.1;
        self.2 = width;
        self.3 = height;
    }

    pub fn set_width(&mut self, width: WorldWidth) {
        assert!(width.0 > 0.0);
        self.2 = width;
    }

    pub fn set_height(&mut self, height: WorldHeight) {
        assert!(height.0 > 0.0);
        self.3 = height;
    }

    pub fn set_dimensions(&mut self, width: WorldWidth, height: WorldHeight) {
        assert!(width.0 > 0.0);
        assert!(height.0 > 0.0);

        self.2 = width;
        self.3 = height;
    }
}

/// Represents a scroll direction
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ScrollDirection {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
pub struct GuiWidgetID(pub usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct BoxID(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct EdgeID(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct CharacterID(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct CharacterStateID(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct VariableID(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct VariableValueID(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct WorldObjectID(usize);

impl AddAssign<usize> for BoxID {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl AddAssign<usize> for EdgeID {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
pub enum ModelID {
    Box(BoxID),
    Edge(EdgeID),
}




#[derive(Debug, PartialEq, Eq)]
pub enum DrawPriority {
    High,   // Priority things that must be drawn above everything, including boxes
    Medium, // Priority for boxes - essentially the base 
    Low     // Priority for things below boxes - i.e edges
}

impl PartialOrd for DrawPriority {
    fn partial_cmp(&self, other: &DrawPriority) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DrawPriority {
    fn cmp(&self, other: &DrawPriority) -> Ordering {
        let self_value = match self {
            &DrawPriority::High => 0,
            &DrawPriority::Medium => 1,
            &DrawPriority::Low => 2,
        };

        let other_value = match self {
            &DrawPriority::High => 0,
            &DrawPriority::Medium => 1,
            &DrawPriority::Low => 2,
        };
        self_value.cmp(&other_value)
    }
}
