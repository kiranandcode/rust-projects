
use super::*;
use types::*;
use manager::components::DialogBox;


use std::sync::{Arc, Mutex};

pub struct DialogView {
    dialog_box: DialogBox,
}

impl DialogView {
    pub fn new() -> Self {
        DialogView {
            dialog_box: DialogBox::new()
        }
    }
}


impl Drawable for DialogView {
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
        let bounding_box = self.dialog_box.get_bounding_box();
        let style_scheme =   style;

        if window.is_bounding_box_onscreen(bounding_box) {
            let ScreenCoords(ScreenUnit(upper_left_x), ScreenUnit(upper_left_y))  = window.world_to_screen(&WorldCoords(bounding_box.0, bounding_box.1));
            let ScreenCoords(ScreenUnit(lower_left_x), ScreenUnit(lower_left_y))  = window.world_to_screen(&WorldCoords(bounding_box.0, bounding_box.1 + bounding_box.3));
            let ScreenCoords(ScreenUnit(upper_right_x), ScreenUnit(upper_right_y))  = window.world_to_screen(&WorldCoords(bounding_box.0 + bounding_box.2, bounding_box.1));
            let ScreenCoords(ScreenUnit(lower_right_x), ScreenUnit(lower_right_y))  = window.world_to_screen(&WorldCoords(bounding_box.0 + bounding_box.2, bounding_box.1 + bounding_box.3));

                    cr.new_path();
                    cr.move_to(upper_left_x, upper_left_y);
                    cr.line_to(lower_left_x, lower_left_y);
                    cr.line_to(lower_right_x, lower_right_y);
                    cr.line_to(upper_right_x, upper_right_y);
                    cr.close_path();

                    cr.set_source_rgba(style_scheme.dialog_color.red, style_scheme.dialog_color.green, style_scheme.dialog_color.blue, style_scheme.dialog_color.alpha);
                    cr.fill();

                    cr.new_path();
                    cr.move_to(upper_left_x, upper_left_y);
                    cr.line_to(lower_left_x, lower_left_y);
                    cr.line_to(lower_right_x, lower_right_y);
                    cr.line_to(upper_right_x, upper_right_y);
                    cr.close_path();

                    cr.set_source_rgba(style_scheme.dialog_color.red, style_scheme.dialog_color.green, style_scheme.dialog_color.blue, style_scheme.dialog_color.alpha);
                    cr.fill()
        }
    }
}
