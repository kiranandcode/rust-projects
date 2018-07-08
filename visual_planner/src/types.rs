use std::ops::{Add,Sub,Mul};
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
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WorldCoords(pub WorldX, pub WorldY);
/// a newtype representing screen coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ScreenCoords(pub ScreenX, pub ScreenY);
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RenderCoords(pub RenderX, pub RenderY);


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

/// Represents a rectangle in screen space - immovable, but can be rescaled
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ScreenDimensions(pub ScreenWidth, pub ScreenHeight);
impl ScreenDimensions {
    pub fn set_width(&mut self, width : ScreenWidth) {
        assert!(width.0 > 0.0);
        self.0  = width;
    }

    pub fn set_height(&mut self, height : ScreenHeight) {
        assert!(height.0 > 0.0);
        self.1 = height;
    }

    pub fn set_dimensions(&mut self, width : ScreenWidth, height: ScreenHeight) {
        assert!(width.0 > 0.0);
        assert!(height.0 > 0.0);

        self.0  = width;
        self.1 = height;
    }

}

/// Represents a scroll direction
#[derive(Debug,PartialEq,PartialOrd,Clone)]
pub enum ScrollDirection {
    Up,
    Down
}


