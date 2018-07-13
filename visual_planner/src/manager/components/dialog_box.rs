use super::Model;

use manager::{Drawable};
use render_window::RenderWindow;
use style_scheme::StyleScheme;
use types::*;

use std::sync::{Arc, Mutex};
use cairo::Context;


use types::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct DialogBox {
    main_model: Model

}


impl DialogBox {
    pub fn new() -> Self {
        // TODO(Kiran): Fix this
        DialogBox {
            main_model: Model {
                bounding_box: WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(50.0), WorldUnit(50.0))
            }
        }
    }
    pub fn get_bounding_box(&self) -> &WorldBoundingBox {
        &self.main_model.bounding_box
    }
}


impl Drawable for DialogBox {

    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {

        let bounding_box = self.model.bounding_box;
        let style_scheme =   style;

        if window.is_bounding_box_onscreen(bounding_box) {
            let ScreenCoords(ScreenUnit(upper_left_x), ScreenUnit(upper_left_y))  
                    = window.world_to_screen(&WorldCoords(bounding_box.0, bounding_box.1));
            let ScreenCoords(ScreenUnit(lower_left_x), ScreenUnit(lower_left_y))  
                    = window.world_to_screen(&WorldCoords(bounding_box.0, bounding_box.1 + bounding_box.3));
            let ScreenCoords(ScreenUnit(upper_right_x), ScreenUnit(upper_right_y))  
                    = window.world_to_screen(&WorldCoords(bounding_box.0 + bounding_box.2, bounding_box.1));
            let ScreenCoords(ScreenUnit(lower_right_x), ScreenUnit(lower_right_y))  
                    = window.world_to_screen(&WorldCoords(bounding_box.0 + bounding_box.2, bounding_box.1 + bounding_box.3));

                    cr.new_path();
                    cr.move_to(upper_left_x, upper_left_y);
                    cr.line_to(lower_left_x, lower_left_y);
                    cr.line_to(lower_right_x, lower_right_y);
                    cr.line_to(upper_right_x, upper_right_y);
                    cr.close_path();

                    cr.set_source_rgba(
                            style_scheme.dialog_color.red, 
                            style_scheme.dialog_color.green, 
                            style_scheme.dialog_color.blue, 
                            style_scheme.dialog_color.alpha);
                    cr.fill();

                    cr.new_path();
                    cr.move_to(upper_left_x, upper_left_y);
                    cr.line_to(lower_left_x, lower_left_y);
                    cr.line_to(lower_right_x, lower_right_y);
                    cr.line_to(upper_right_x, upper_right_y);
                    cr.close_path();

                    cr.set_source_rgba(
                        style_scheme.dialog_color.red, 
                        style_scheme.dialog_color.green, 
                        style_scheme.dialog_color.blue, 
                        style_scheme.dialog_color.alpha);
                    cr.fill();


        }

    }
}
 
