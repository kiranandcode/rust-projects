use types::*;
use super::{ToDrawable, DrawPriority};
use style_scheme::StyleScheme;
use render_window::RenderWindow;
use manager::draw_view::Drawable;
use undo::{Modifiable, Modification};

use std::sync::{Arc, Mutex, MutexGuard};

use cairo::Context;



#[derive(Debug, Clone)]
pub struct EdgeModel(Arc<EdgeModelInternal>);


impl EdgeModel {
    pub fn update(&self, current_time: &CurrentTime, delta_time: &DeltaTime) -> Option<WorldBoundingBox> {
        self.0.update(current_time, delta_time)
    }
}

impl Modifiable for EdgeModel {
    fn update_state(&mut self, other: &Self) -> Modification {
        // TODO: Implement this
        Modification::Deleted
    }
}


impl ToDrawable for EdgeModel {
    fn to_drawable(&self) -> Arc<Drawable> {
        self.0.clone()
    }
}

/// Internal struct used to represent an edge
/// Must be constructed with an ID and a start and end boxID
#[derive(Debug)]
pub struct EdgeModelInternal {
    // Immutable fields of EdgeModel
    id: EdgeID,
    start: BoxID,
    end: BoxID,

    // Mutable Points
    tween_points: Mutex<Vec<WorldCoords>>,
    bounding_box: Mutex<WorldBoundingBox>
}

impl EdgeModelInternal {
    pub fn update(&self, current_time: &CurrentTime, delta_time: &DeltaTime) -> Option<WorldBoundingBox> {
        None
    }
}



impl Drawable for EdgeModelInternal {
    fn priority(&self) -> DrawPriority {
        DrawPriority::Low
    }
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
        // DEBUG Drawing
        if let Ok(bounding_box) = self.bounding_box.lock() {

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
                                style_scheme.dialog_box_text.red, 
                                style_scheme.dialog_box_text.green, 
                                style_scheme.dialog_box_text.blue, 
                                style_scheme.dialog_box_text.alpha);
                        cr.stroke();

           }
        }
 
    }

    fn bounding_box(&self) -> Option<MutexGuard<WorldBoundingBox>> {
        self.bounding_box.lock().ok()
    }


    fn id(&self) -> ModelID {
        ModelID::Edge(self.id.clone())
    }


}