use super::Model;
use renderer::components::Drawable;
use renderer::render_window::RenderWindow;
use renderer::style_scheme::StyleScheme;
use types::*;

use std::sync::{Arc, Mutex};
use cairo::Context;


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


impl Drawable for DialogBox {

    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {

        unimplemented!("Not Implemented!");
    }

}
 
