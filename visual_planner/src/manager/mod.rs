pub mod components;

use self::components::BoxModel;
use style_scheme::StyleScheme;
use render_window::RenderWindow;
use types::*;

use std::cmp::{Ordering, Ord};
use std::sync::{Arc, Mutex};
use std::collections::hash_map::HashMap;

use cairo::Context;

pub trait Drawable {
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow);
}



#[derive(Debug)]
pub struct ComponentID(usize, Arc<Mutex<ModelManager>>);

impl PartialEq for ComponentID {
    fn eq(&self, other : &ComponentID) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for ComponentID {
    fn partial_cmp(&self, other: &ComponentID) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Eq for ComponentID {}
impl Ord for ComponentID {
    fn cmp(&self, other: &ComponentID) -> Ordering {
        self.0.cmp(&other.0)
    }
}



pub enum ModelID {
    Component(ComponentID)
}
    


#[derive(Debug)]
pub struct ModelManager {
    /// stores the true value of the models
    base_models: HashMap<usize, BoxModel>,
    /// Stores the temporary value of a model
    temp_models: HashMap<usize, BoxModel>,
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
