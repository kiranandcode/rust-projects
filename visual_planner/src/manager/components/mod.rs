mod dialog_box;
mod decision_box;
mod variable_box;
mod state_change_box;
mod entry_box;
mod box_edge;

pub use super::id::*;
pub use self::dialog_box::*;
pub use self::decision_box::*;
pub use self::variable_box::*;
pub use self::state_change_box::*;
pub use self::entry_box::*;
pub use self::box_edge::*;


use style_scheme::StyleScheme;
use render_window::RenderWindow;
use types::*;

use std::sync::{Arc, Mutex};

use cairo::Context;


#[derive(Debug, PartialEq, PartialOrd)]
pub enum BoxModel {
    DialogModel(DialogBox),
    DecisionModel(DecisionBox),
    VariableModel(VariableBox),
    StateChangeModel(StateChangeBox),
    EntryModel(EntryBox)
}


/// Generic struct containing all components required to render a model
#[derive(Debug)]
pub struct BoxBase {
    pub (in manager) id: BoxID,
    pub (in manager) bounding_box: Mutex<WorldBoundingBox>,
    pub (in manager) in_edges: Vec<EdgeID>,
    pub (in manager) out_edges: Vec<EdgeID>
}

impl BoxModel {

    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
        match self {
            BoxModel::DialogModel(ref dialogBox) => dialogBox.draw(cr, style, window),
            BoxModel::DecisionModel(ref decisionBox) => decisionBox.draw(cr, style, window),
            BoxModel::VariableModel(ref variableBox) => variableBox.draw(cr, style, window),
            BoxModel::StateChangeModel(ref stateChangeBox) => stateChangeBox.draw(cr, style, window),
            BoxModel::EntryModel(ref entryBox) => entryBox.draw(cr, style, window),
        }
    }

}
