use color::*;

// - - - - - - - - - - - - - - - - - - - - -
//              Color Scheme
// - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct ColorScheme {
    pub bg: Color,
    pub node_bg: Color,
    pub node_text: Color,
    pub node_fg: Color,
    pub node_fg_text: Color,
    pub node_bg_accent: Color,
    pub node_bg_highlight: Color,
}

pub const COLOR_SCHEME: ColorScheme = ColorScheme {
    bg: Color(231.0/255.0, 232.0/255.0, 236.0/255.0, Some(1.0)),
    node_bg: Color::WHITE,
    node_text: Color::BLACK,
    node_fg: Color(20.0/255.0, 177.0/255.0, 219.0/255.0, Some(1.0)),
    node_fg_text: Color::BLACK,
    node_bg_accent: Color(203.0/255.0, 203.0/255.0, 203.0/255.0, Some(1.0)),
    node_bg_highlight: Color(153.0/255.0, 153.0/255.0, 153.0/255.0, Some(0.7)),
};

impl Default for ColorScheme {
    fn default() -> Self {
        COLOR_SCHEME.clone()
    }
}
