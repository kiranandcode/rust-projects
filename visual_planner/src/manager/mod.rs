pub mod components;
pub mod draw_view;
pub mod object_manager;


use self::components::edge::EdgeModel;
use self::components::boxes::BoxModel;
use self::object_manager::ObjectManager;
use gui::manager::GuiManager;
use event::EventManagerBuilder;
use style_scheme::StyleScheme;
use render_window::RenderWindow;
use types::*;

use std::collections::hash_map::HashMap;
use std::ops::AddAssign;
use std::thread;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::hash::Hash;

use cairo::Context;



#[derive(Debug)]
pub struct ModelManager {
    box_models: Arc<ObjectManager<BoxID, BoxModel>>,
    edge_models: Arc<ObjectManager<EdgeID, EdgeModel>>,
    manager_thread_handle: JoinHandle<()>
}


impl ModelManager {

    pub fn new((event_builder, gui_manager): (&mut EventManagerBuilder, &mut GuiManager)) -> Self {
        let box_models = Arc::new(ObjectManager::new());
        let edge_models = Arc::new(ObjectManager::new());
        let (sender, receiver) = mpsc::channel();

        let channel = event_builder.get_gdk_channel();
        event_builder.set_model_manager_channel(sender);

        let manager_thread_handle = {
            let box_models = box_models.clone();
            let edge_models = edge_models.clone();
            let channel = channel;
            

            thread::spawn(move || {

                for event in receiver.iter() {
                    match event {

                    }
                }

            })
        };

        ModelManager {
            box_models,
            edge_models,
            manager_thread_handle
        }
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
