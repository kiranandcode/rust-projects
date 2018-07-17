pub mod components;
pub mod draw_view;


use self::components::edge::EdgeModel;
use self::components::boxes::BoxModel;
use gui::manager::GuiManager;
use event::EventManagerBuilder;
use style_scheme::StyleScheme;
use render_window::RenderWindow;
use types::*;

use std::collections::hash_map::HashMap;
use std::ops::AddAssign;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::hash::Hash;

use cairo::Context;


#[derive(Debug)]
pub struct ObjectManager<K,V> 
    where K : Eq  + Clone + Hash + Default + AddAssign<usize>,
          V : Clone {
    id_gen:  K,
    /// stores the true value of the models
    base: HashMap<K,V>,
    /// Stores the temporary value of a model
    temp: HashMap<K,V>,
}

impl<K,V> ObjectManager<K, V> 
    where K : Eq + Clone + Hash + Default + AddAssign<usize>, 
          V : Clone {
        pub fn new() -> Self {
            ObjectManager {
                id_gen: K::default(),
                base: HashMap::new(),
                temp: HashMap::new(),
            }
        }

        pub fn delete_model(&mut self, id: K) {
            
        }

        pub fn commit_changes(&mut self, id: K) {

        }

        pub fn register_model(&mut self, object: V) -> K {
            let old_id = self.id_gen.clone();
            self.id_gen += 1;

            // insert into model and temp

            old_id
        }
}


#[derive(Debug)]
pub struct ModelManager {
    box_models: ObjectManager<BoxID, BoxModel>,
    edge_models: ObjectManager<EdgeID, EdgeModel>,

}


impl ModelManager {

    pub fn new((event_builder, gui_manager): (&mut EventManagerBuilder, &mut GuiManager)) {

    }
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
