use manager::ModelManager;
use manager::components::*;


use std::cmp::{Ordering, Ord};
use std::sync::{Arc, Mutex};

pub enum ModelID {
    BoxID(BoxID),
    EdgeID(EdgeID)
}
 

#[derive(Debug)]
pub struct BoxID(usize, Arc<Mutex<ModelManager>>);

impl PartialEq for BoxID { fn eq(&self, other : &BoxID) -> bool { self.0 == other.0 } }
impl PartialOrd for BoxID { fn partial_cmp(&self, other: &BoxID) -> Option<Ordering> { Some(self.0.cmp(&other.0)) } } 
impl Eq for BoxID {}
impl Ord for BoxID { fn cmp(&self, other: &BoxID) -> Ordering { self.0.cmp(&other.0) } }


#[derive(Debug)]
pub struct EdgeID(usize, Arc<Mutex<ModelManager>>);

impl PartialEq for EdgeID { fn eq(&self, other : &EdgeID) -> bool { self.0 == other.0 } }
impl PartialOrd for EdgeID { fn partial_cmp(&self, other: &EdgeID) -> Option<Ordering> { Some(self.0.cmp(&other.0)) } } 
impl Eq for EdgeID {}
impl Ord for EdgeID { fn cmp(&self, other: &EdgeID) -> Ordering { self.0.cmp(&other.0) } }

