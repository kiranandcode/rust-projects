pub mod components;
pub mod draw_view;
pub mod object_manager;


use self::components::*;
use self::components::edge::EdgeModel;
use self::components::boxes::BoxModel;
use self::components::boxes::BoxConstructor;
use self::object_manager::ObjectManager;
use event::message::manager::ModelManagerMessage;
use gui::manager::GuiManager;
use event::EventManagerBuilder;
use event::message::GeneralMessage;
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
    // box_models: ObjectManager<BoxID, BoxModel>,
    // edge_models: ObjectManager<EdgeID, EdgeModel>,
    manager_thread_handle: JoinHandle<()>
}


impl ModelManager {

    pub fn new((event_builder, gui_manager): (&mut EventManagerBuilder, &mut GuiManager)) -> Self {
        let (sender, receiver) = mpsc::channel();

        let channel = event_builder.get_gdk_channel();
        event_builder.set_model_manager_channel(sender);

        let manager_thread_handle = {
            let channel = channel;
            

            thread::spawn(move || {
                let mut box_models :ObjectManager<BoxID, BoxModel> = ObjectManager::new();
                let mut edge_models :ObjectManager<EdgeID, EdgeModel> = ObjectManager::new();

                for event in receiver.iter() {
                    match event {
                           ModelManagerMessage::BoxConstruct(constructor_msg) => {
                               let result = match constructor_msg {
                                   BoxConstructor::DialogModel(center) => {
                                        let bounding_box = WorldBoundingBox::new_centered_at(center.clone(), DIALOG_BOX_WIDTH, DIALOG_BOX_HEIGHT);
                                        let (id, drawable, modification) = DialogBox::new(center, &mut box_models);
                                          // TODO: check drawing coordinates do not intersect with other components


                                        Some(drawable)
                                   } 
                                   _ => None
                               };

                               if let Some(drawable) = result {
                                   channel.send(GeneralMessage::ConstructResult(drawable));
                               }
                           } 

                            // This event is triggered by the timer (effectively the 10ms timeout in the dialog renderer)
                            // the model manager should propagate the update request to all of it's components, allowing
                            // those that want to update themselves to do so.
                            // the model manager should update all components, and send back a bounding box to indicate the 
                            // region that has changed
                           ModelManagerMessage::DialogUpdate(current_time, delta_time) => {
                               // the following bounding box acts as an accumulator to 
                               // produce a single bounding box encompassing all changes
                               let mut bounding_box : Option<WorldBoundingBox> = None;

                               // the following list contains all components which may have moved, and thus need to be checked for colissions
                               // TODO: Possibly reserve a capacity in advance?
                               let mut updated_components = Vec::new();

                               for (id, updatable) in box_models.temp_iter() {
                                   updatable.update(&current_time, &delta_time).map(|new_box| {
                                      // if it returns a bounding box, it may have changed position.
                                      // this means we need to queue it for colission checking
                                       updated_components.push(id.clone());

                                       if let Some(ref mut bbox) = bounding_box.take() {
                                           bbox.union(&new_box);
                                       } else {
                                           bounding_box = Some(new_box);
                                       }
                                   });
                               }

                               // now we want to go through the updated components
                                for i in 0..updated_components.len() {
                                    for j in i..updated_components.len() {
                                        // check if i,j are intersecting, and modify if so,
                                        unimplemented!("Implemented state change");
                                    }
                                }


                               for updateable in edge_models.temp_values() {
                                   updateable.update(&current_time, &delta_time).map(|new_box| {
                                    if let Some(ref mut bbox) = bounding_box.take() {
                                        bbox.union(&new_box);
                                    } else {
                                        bounding_box = Some(new_box);
                                    }
                                   });
 
                               }

                                if let Some(bbox) = bounding_box.take() {
                                   channel.send(GeneralMessage::DialogRedraw(bbox));
                                }
                           }
                    }
                }

            })
        };

        ModelManager {
            // box_models,
            // edge_models,
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
