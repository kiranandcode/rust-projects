use renderer::components::Drawable;
use renderer::render_window::RenderWindow;
use renderer::style_scheme::StyleScheme;
use types::*;

use std::sync::{Arc, Mutex};

use cairo::Context;



#[derive(Debug, PartialEq, PartialOrd)]
pub struct EntryBox {

}


impl Drawable for EntryBox {

    fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {

        unimplemented!("Not Implemented!");
    }

}
 
