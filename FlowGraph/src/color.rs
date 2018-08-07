
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct Color(pub f64, pub f64,pub f64, pub Option<f64>);

impl Color {
    pub const RED : Color = Color(1.0, 0.0, 0.0, Some(1.0));
    pub const GREEN : Color = Color(0.0, 1.0, 0.0, Some(1.0));

    pub const BLUE : Color = Color(0.0, 0.0, 1.0, Some(1.0));

    pub const ORANGE : Color = Color(1.0, 153.0/255.0, 51.0/255.0, Some(1.0));

    pub const YELLOW : Color = Color(1.0, 1.0, 51.0/255.0, Some(1.0));


    pub const LIGHT_GREEN : Color = Color(153.0/255.0, 1.0, 51.0/255.0, Some(1.0));

    pub const LIGHT_BLUE : Color = Color(51.0/255.0, 1.0, 1.0, Some(1.0));

    pub const PINK : Color = Color(1.0, 51.0/255.0, 1.0, Some(1.0));

    pub const PURPLE : Color = Color(153.0/255.0, 51.0/255.0, 1.0, Some(1.0));

    pub const LIGHT_GREY : Color = Color(224.0/255.0, 224.0/255.0, 224.0/255.0, Some(1.0));

    pub const MID_GREY : Color = Color(192.0/255.0, 192.0/255.0, 192.0/255.0, Some(1.0));

    pub const DARK_GREY : Color = Color(128.0/255.0, 128.0/255.0, 128.0/255.0, Some(1.0));

    pub const BLACK : Color = Color(0.0, 0.0, 0.0, Some(1.0));

    pub const WHITE : Color = Color(1.0, 1.0, 1.0, Some(1.0));


    pub fn multiply(colors: &[Color]) -> Color {
        let mut base = Color(1.0, 1.0, 1.0, Some(1.0));

        for color in colors {
            base.0 *= color.0;
            base.1 *= color.1;
            base.2 *= color.2;

            let alpha = if let Some(val) = color.3 {val} else {1.0};
            base.3.map(|val| val * alpha);
        }

        base
    }


    pub fn additive(colors: &[Color]) -> Color {
        let mut base = Color(1.0, 1.0, 1.0, Some(1.0));

        for color in colors {
            base.0 += color.0;
            base.1 += color.1;
            base.2 += color.2;
            let alpha = if let Some(val) = color.3 {val} else {1.0};
            base.3.map(|val| val * alpha);
        }
        base.0 /= colors.len() as f64;
        base.1 /= colors.len() as f64;
        base.2 /= colors.len() as f64;
        base.3.map(|val| val * (colors.len() as f64));

        base
    }

}


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
