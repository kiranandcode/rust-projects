mod dialog_box;
mod decision_box;
mod variable_box;
mod state_change_box;
mod entry_box;

pub use self::dialog_box::*;
pub use self::decision_box::*;
pub use self::variable_box::*;
pub use self::state_change_box::*;
pub use self::entry_box::*;

use types::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum BoxModel {
    DialogModel(DialogBox),
    DecisionModel(DecisionBox),
    VariableModel(VariableBox),
    StateChangeModel(StateChangeBox),
    EntryModel(EntryBox)
}


/// Generic struct containing all components required to render a model
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Model {
    pub (in manager) bounding_box: WorldBoundingBox,
}
