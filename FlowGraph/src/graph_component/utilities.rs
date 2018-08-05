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
use super::object::*;
use super::graph::*;

pub fn add_node<O>(id_gen: &mut IDManager, object_graph: &mut ObjectGraph, objects: &mut Vec<(ID,Box<Object>)>, object: O, children: &[ID]) -> ID
where O: Object + 'static
{
    let raw_id = objects.len();
    let id = id_gen.new(raw_id);
    let root = object_graph.get_root();

    // first add it to the graph
    objects.push((id,Box::new(object)));
    object_graph.children.push(Vec::new());

    // by default everything is a child of the root
    object_graph.parent.push(root);
    // add it to the root's children
    if let Ok(p_id) = id_gen.get(root) {
        object_graph.children[p_id].push(id);
    }


    // for each of it's children (which must have been previously registered)
    for &child in children {
        // add it to the list of children
        object_graph.children[raw_id].push(child);
        let old_parent = if let Ok(raw_id) = id_gen.get(child) {
            let old_parent = object_graph.parent[raw_id];
            object_graph.parent[raw_id] = id;
            Some(old_parent)
        } else { None };

        if let Some(o_id) = old_parent {
            if let Ok(old_raw_id) = id_gen.get(o_id) {
                // remove the child from it's prior parent
                object_graph.children[old_raw_id].retain(|nid| *nid != id);
            }
        }
    }
    id
}


pub fn remove_node(id_gen: &mut IDManager, object_graph: &mut ObjectGraph, objects: &mut Vec<(ID,Box<Object>)>, id: ID, and_children: bool) {
        if id == object_graph.get_root() { return; }

        let r_id = id_gen.get(id).expect("Remove Node called on invalid id");

        // first, swap the last element into the place of the thing we want to remove
        objects.swap_remove(r_id);

        // if the length of the list after removal equals the index to be removed,
        // this means we compared and swapped the last element of the list,
        // which is equivalent to a plain remove
        // this is relevant as it means no additonal swappery has to be done
        let (parent, children) = if objects.len() == r_id {
            id_gen.remove(id);
            let children = object_graph.children.swap_remove(r_id);
            let parent = object_graph.parent.swap_remove(r_id);
            (parent, children)
        }
        else {
            // we swapped some element in the list, and now we need to update our references

            // grab the id of the thing we just removed
            let (repl_id, _) = objects[r_id];

            // update the ids to point to the correct location
            id_gen.swap_remove(id, repl_id);

            // remove parent and childrens of the replaced one, and set them to be the parent and children of the replaced node
            let children = object_graph.children.swap_remove(r_id);
            let parent = object_graph.parent.swap_remove(r_id);
            (parent, children)
        };

        // remove it from the children of the parent
        if let Ok(p_id) = id_gen.get(parent) {
            object_graph.children[p_id].retain(|n_id| *n_id != id);
        }

        // for each of the children, delete the references
        for child in children {
            // if remove the children,
            if and_children {
                if let Some(real_ind) = id_gen.get(child).ok() {
                    remove_node(id_gen, object_graph, objects, child, and_children);
                }
            } else {
                // otherwise, just update the parents of the node
                let root = object_graph.get_root();
                // first make set it's parent to the root
                if let Some(real_ind) = id_gen.get(child).ok() {
                    object_graph.parent[real_ind] = object_graph.get_root();
                }
                // then add it to the root's children
                if let Some(r_ind) = id_gen.get(root).ok() {
                    object_graph.children[r_ind].push(child);
                }
            }
        }
    }

