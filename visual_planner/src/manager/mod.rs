pub mod components;

use self::components::BoxModel;

use std::sync::{Arc, Mutex};

pub struct ModelID(usize, Arc<Mutex<ModelManager>>);



pub struct ModelManager {
    /// stores the true value of the models
    base_models: Vec<BoxModel>,
    /// Stores the temporary value of a model
    temp_models: Vec<BoxModel>,
    
}


impl ModelManager {

    // pub fn lookup_id(&self, id: ModelID) -> &BoxModel {
    //    &self.models[id.0]
    // }

    // pub fn lookup_id_mut(&mut self, id: ModelID) -> &mut BoxModel {
    //    &mut self.models[id.0]
    // }


    // pub fn reverse_lookup(&self, model: &BoxModel) -> ModelID {
    //     for (index, value) in self.models.iter().enumerate() {
    //         if value == model {
    //             return ModelID(index);
    //         }
    //     }

    //     panic!("reverse lookup called on un-registered model");
    // }
}
