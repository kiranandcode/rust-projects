use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;

use std::ops::{IndexMut, Index};
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};
use std::convert::TryFrom;

use gtk::{Window, WindowExt, WidgetExt, ContainerExt};
use gdk::EventMask;

use super::id::*;
use super::Object;

// for variable mutablity purposes we split the definition of the graph into a seperate component
#[derive(Default)]
pub struct ObjectGraph {
    pub (in graph_component) root: ID,
    pub (in graph_component) children: Vec<Vec<ID>>,
    pub (in graph_component) parent: Vec<ID>
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
    pub (in graph_component) objects: &'a mut [(ID, Box<Object>)],
    pub (in graph_component) object_graph: &'a ObjectGraph,
    pub (in graph_component) idgen: &'a IDManager
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
