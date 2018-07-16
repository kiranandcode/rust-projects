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


use style_scheme::StyleScheme;
use render_window::RenderWindow;
use types::*;

use std::sync::{Arc, Mutex};

use cairo::Context;


#[derive(Debug, PartialEq, PartialOrd)]
pub enum BoxType {
    DialogModel,
    DecisionModel,
    VariableModel,
    StateChangeModel,
    EntryModel,
}

trait Box {
    fn try_cast<T>(&self, box_type: BoxType) -> Option<T>;
}

/// Generic struct containing all components required to render a model
#[derive(Debug)]
pub struct BoxBase {
    pub (in manager) id: BoxID,
    pub (in manager) box_type: BoxType,
    pub (in manager) bounding_box: Mutex<WorldBoundingBox>,
}


