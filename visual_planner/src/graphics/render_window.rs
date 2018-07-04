
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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct WorldWidth(pub WorldUnit);
#[derive(Debug, PartialEq, PartialOrd)]
pub struct WorldHeight(pub WorldUnit);
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ScreenWidth(pub ScreenUnit);
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ScreenHeight(pub ScreenUnit);

/// Represents a mapping between a virtual window in worldspace to the screen
/// Could be implemented as a 3d matrix, but would require pulling in
/// additional dependancies
pub struct RenderWindow {
    // world_dimensions - width, height
    //world_dimensions: (WorldWidth, WorldHeight),
    // screen_dimensions - width height
    //screen_dimensions: (ScreenWidth, ScreenHeight),
}

impl RenderWindow {
    // update screen_dim
    // update world_dim
    // in view -> bool
    // world to screen
}
