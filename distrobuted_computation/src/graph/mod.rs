use std::rc::Rc;
use std::cell::RefCell;
pub struct Node {
    data:  String,
    edges: Vec<Rc<RefCell<Node>>>
}

impl Node {
    fn new(s: String)  -> Self {
        Node{
                data: s.clone(),
                edges: Vec::new()
        }
    }
    fn first(&self) -> Rc<RefCell<Node>> {
        self.edges[0].clone()
    }
}