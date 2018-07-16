use super::BoxBase;

use render_window::RenderWindow;
use style_scheme::StyleScheme;
use types::*;

use std::sync::{Arc, Mutex};
use cairo::Context;




#[derive(Debug, PartialEq, PartialOrd)]
pub struct VariableBox {


}


impl VariableBox {

    pub fn draw(&self, cr : &Context, style: &StyleScheme, window : &RenderWindow) {

        unimplemented!("Not Implemented!");
    }

}
 
