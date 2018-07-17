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
use types::*;

use std::sync::{Arc, Mutex};

use cairo::Context;



#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub (in manager) enum BoxModel {
    DialogModel(Arc<DialogBox>),
    DecisionModel(Arc<DecisionBox>),
    VariableModel(Arc<VariableBox>),
    StateChangeModel(Arc<StateChangeBox>),
    EntryModel(Arc<EntryBox>),
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



