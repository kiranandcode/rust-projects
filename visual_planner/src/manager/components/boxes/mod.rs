pub mod dialog_box;
pub mod decision_box;
pub mod variable_box;
pub mod state_change_box;
pub mod entry_box;

pub use super::*;
pub use self::dialog_box::*;
pub use self::decision_box::*;
pub use self::variable_box::*;
pub use self::state_change_box::*;
pub use self::entry_box::*;


use undo::{Modifiable, Modification};
use style_scheme::StyleScheme;
use render_window::RenderWindow;
use manager::draw_view::Drawable;
use types::*;

use std::sync::{Arc, Mutex, MutexGuard};

use cairo::Context;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BoxConstructor {
    DialogModel,
    DecisionModel,
    VariableModel,
    StateChangeModel,
    EntryModel,
}



#[derive(Debug, Clone)]
pub enum BoxModel {
    DialogModel(Arc<DialogBox>),
    DecisionModel(Arc<DecisionBox>),
    VariableModel(Arc<VariableBox>),
    StateChangeModel(Arc<StateChangeBox>),
    EntryModel(Arc<EntryBox>),
}

impl BoxModel {
    pub fn to_drawable(&self) -> Arc<Drawable> {
        match self {
            BoxModel::DialogModel(value) => value.clone(),
            BoxModel::DecisionModel(value) => value.clone(),
            BoxModel::VariableModel(value) => value.clone(),
            BoxModel::StateChangeModel(value) => value.clone(),
            BoxModel::EntryModel(value) => value.clone(),
        }
    }
}


impl Modifiable for BoxModel {
    fn update_state(&mut self, other: &Self) -> Modification {
        // TODO: Implement this
        Modification::Deleted
    }
}


/// Generic struct containing all components required to render a model
#[derive(Debug)]
pub struct BoxBase {
    pub (in manager) id: BoxID,
    pub (in manager) bounding_box: Mutex<WorldBoundingBox>,
}




impl Drawable for BoxBase {
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
        ModelID::Box(self.id.clone())
    }

}


