use gtk::{StyleContext};
use gdk::RGBA;

// used to retrieve the color scheme for the app
pub struct StyleScheme {
    /// used for the main background of the system
    pub (in renderer) bg: RGBA,          

    /// used for the grid line colors
    pub (in renderer) bg_mid: RGBA,      

    /// used for  border color
    pub (in renderer) border: RGBA,

    /// used for  the color of the box
    pub (in renderer) dialog_color: RGBA,

    /// used for text
    pub (in renderer) dialog_box_text: RGBA,
}

impl StyleScheme {
    pub fn from(context: &StyleContext) -> StyleScheme {
        StyleScheme {
            bg: RGBA { red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0 },
            bg_mid: RGBA { red: 250.0/255.0, green: 224.0/255.0, blue: 55.0/255.0, alpha: 1.0  },      
            border: RGBA { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0  },
            dialog_color: RGBA { red: 0.2, green: 0.2, blue: 0.2, alpha: 1.0  },
            dialog_box_text: RGBA { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0  },
        }
    }
}
