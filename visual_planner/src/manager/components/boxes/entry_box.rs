use super::*;

use render_window::RenderWindow;
use style_scheme::StyleScheme;
use manager::draw_view::Drawable;
use types::*;

use std::sync::{Arc, Mutex, MutexGuard};

use cairo::Context;



#[derive(Debug)]
pub struct EntryBox {
    main_model: BoxBase
}

impl EntryBox {

    pub fn update(&self, current_time: &CurrentTime, delta_time: &DeltaTime) -> Option<WorldBoundingBox> {
        None
    }
}


impl Drawable for EntryBox {
    fn priority(&self) -> DrawPriority {
        self.main_model.priority()
    }
 
    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {
        unimplemented!("Not Implemented!");
    } 
    fn bounding_box(&self) -> Option<MutexGuard<WorldBoundingBox>> {
       self.main_model.bounding_box() 
    }
    fn id(&self) -> ModelID {
        self.main_model.id()
    }
}



