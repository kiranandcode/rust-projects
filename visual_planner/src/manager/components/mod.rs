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


pub enum ModelBox {
    DialogModel(DialogBox),
    DecisionModel(DecisionBox),
    VariableModel(VariableBox),
    StateChangeModel(StateChangeBox),
    EntryModel(EntryBox)
}
