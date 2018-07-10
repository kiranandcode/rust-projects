use super::Model;
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
