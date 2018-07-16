use manager::ModelManager;
use manager::components::*;


use std::cmp::{Ordering, Ord};
use std::sync::{Arc, Mutex};

// pub enum ModelID {
//     BoxID(BoxID),
//     EdgeID(EdgeID)
// }
 

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BoxID(usize);


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct EdgeID(usize);

