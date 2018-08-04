
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Color(pub f64, pub f64,pub f64, pub Option<f64>);

impl Color {
    pub fn red() -> Self {
        Color(1.0, 0.0, 0.0, Some(1.0))
    }
    pub fn green() -> Self {
        Color(0.0, 1.0, 0.0, Some(1.0))
    }
    pub fn blue() -> Self {
        Color(0.0, 0.0, 1.0, Some(1.0))
    }

    pub fn orange() -> Self {
        Color(1.0, 153.0/255.0, 51.0/255.0, Some(1.0))
    }

    pub fn yellow() -> Self {
        Color(1.0, 1.0, 51.0/255.0, Some(1.0))
    }


    pub fn light_green() -> Self {
        Color(153.0/255.0, 1.0, 51.0/255.0, Some(1.0))
    }

    pub fn light_blue() -> Self {
        Color(51.0/255.0, 1.0, 1.0, Some(1.0))
    }

    pub fn pink() -> Self {
        Color(1.0, 51.0/255.0, 1.0, Some(1.0))
    }

    pub fn purple() -> Self {
        Color(153.0/255.0, 51.0/255.0, 1.0, Some(1.0))
    }

    pub fn light_grey() -> Self {
        Color(224.0/255.0, 224.0/255.0, 224.0/255.0, Some(1.0))
    }

    pub fn mid_grey() -> Self {
        Color(192.0/255.0, 192.0/255.0, 192.0/255.0, Some(1.0))
    }

    pub fn dark_grey() -> Self {
        Color(128.0/255.0, 128.0/255.0, 128.0/255.0, Some(1.0))
    }

    pub fn black() -> Self {
        Color(0.0, 0.0, 0.0, Some(1.0))
    }

    pub fn white() -> Self {
        Color(1.0, 1.0, 1.0, Some(1.0))
    }

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
