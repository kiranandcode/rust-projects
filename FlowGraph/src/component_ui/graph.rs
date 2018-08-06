use types::*;

use super::id::*;
use super::Object;

// for variable mutablity purposes we split the definition of the graph into a seperate component
#[derive(Default)]
pub struct ObjectGraph {
    pub (in component_ui) root: ID,
    pub (in component_ui) children: Vec<Vec<ID>>,
    pub (in component_ui) parent: Vec<ID>
}

impl ObjectGraph {
    pub fn set_root(&mut self, id: ID) {
        self.root = id;
    }
    pub fn get_root(&self) -> ID {
        self.root
    }
}



pub struct ObjectAccessor<'a> {
    pub (in component_ui) objects: &'a mut [(ID, Box<Object>)],
    pub (in component_ui) object_graph: &'a ObjectGraph,
    pub (in component_ui) idgen: &'a IDManager
}
impl<'a> ObjectAccessor<'a> {
    pub fn new(objects: &'a mut [(ID,Box<Object>)], object_graph: &'a ObjectGraph, idgen: &'a IDManager) -> Self {
        ObjectAccessor {
            objects,
            idgen,
            object_graph
        }
    }

    pub fn get(&self, id: ID) -> Option<&Box<Object>> {
        if let Ok(r_id) = self.idgen.get(id) {
            Some(&self.objects[r_id].1)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, id: ID) -> Option<&mut Box<Object>> {
        if let Ok(r_id) = self.idgen.get(id) {
            Some(&mut self.objects[r_id].1)
        } else {
            None
        }
    }



    pub fn children(&self, id: ID) -> Option<&[ID]> {
        if let Ok(r_id) = self.idgen.get(id) {
            Some(&self.object_graph.children[r_id])
        } else {
            None
        }

    }
    pub fn parent(&self, id: ID) -> Option<ID> {
        if let Ok(r_id) = self.idgen.get(id) {
            Some(self.object_graph.parent[r_id])
        } else {
            None
        }
    }
}
