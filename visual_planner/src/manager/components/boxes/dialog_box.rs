use super::BoxBase;

use render_window::RenderWindow;
use style_scheme::StyleScheme;
use manager::draw_view::Drawable;
use manager::object_manager::ObjectManager;
use manager::components::boxes::BoxModel;
use types::*;
use undo::*;

use std::sync::{Arc, Mutex, MutexGuard};
use cairo::Context;

use types::*;

pub const DIALOG_BOX_WIDTH : WorldUnit = WorldUnit(200.0);
pub const DIALOG_BOX_HEIGHT : WorldUnit = WorldUnit(50.0);

#[derive(Debug)]
pub struct DialogBox {
    main_model: BoxBase

}


impl DialogBox {
    // pub fn new() -> Self {
    //     // TODO(Kiran): Fix this
    //     DialogBox {
    //         main_model: BoxBase {
    //             bounding_box: WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(50.0), WorldUnit(50.0))
    //         }
    //     }
    // }

    pub fn new(center: WorldCoords, manager: &mut ObjectManager<BoxID, BoxModel>) -> (BoxID, Arc<Drawable>, Modification) {
            manager.register_model(|id| {
                BoxModel::DialogModel(Arc::new(DialogBox {
                    main_model: BoxBase {
                            id,
                            // bounding_box: Mutex::new(WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(100.0), WorldUnit(100.0))),
                            bounding_box: Mutex::new(WorldBoundingBox::new_centered_at(center, DIALOG_BOX_WIDTH, DIALOG_BOX_HEIGHT)),
                    }
                }))
            })
        } 
}

impl Drawable for DialogBox {
    fn bounding_box(&self) -> Option<MutexGuard<WorldBoundingBox>> {
        self.main_model.bounding_box()
    }
    fn id(&self) -> ModelID {
        self.main_model.id()
    }

    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
        self.main_model.draw(cr, style, window);

        if let Ok(bounding_box) = self.main_model.bounding_box.lock() {

            let style_scheme =   style;

            if window.is_bounding_box_onscreen(&bounding_box) {
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
}
 
