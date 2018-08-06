use std::ops::{DerefMut, Deref};
use std::any::Any;

use types::*;
use super::ComponentStateInner;
use super::ID;


pub struct ListenerContext<'a> {
    id: ID,
    inner: &'a mut ComponentStateInner,
    events: &'a mut Vec<(ID, Box<Any>)>,
    // holds the currently invalidated region
    invalidated_region: &'a mut Option<WorldBoundingBox>,
    // hold the worldbounding box from the last call to draw
    last_world_bounding_box: &'a Option<WorldBoundingBox>
}

impl<'a> ListenerContext<'a> {
    pub fn new(id: ID, inner: &'a mut ComponentStateInner, events: &'a mut Vec<(ID, Box<Any>)>, invalidated_region: &'a mut Option<WorldBoundingBox>, last_world_bounding_box: &'a Option<WorldBoundingBox>) -> Self {
        ListenerContext {
            id,
            inner,
            events,
            invalidated_region,
            last_world_bounding_box
        }
    }
    pub fn poke_up<A: Any>(&mut self, payload: &mut A) -> bool {
        let mut node = self.id;
        let root = self.inner.object_graph.root;
        loop {
            if let Ok(raw_id) = self.inner.id_gen.get(node) {
                let parent = self.inner.object_graph.parent[raw_id];
                if parent == node {
                    return false;
                }
                node = parent;
                if self.inner.poke_internal(root, node, payload, self.events, self.invalidated_region, self.last_world_bounding_box) {
                    return true;
                }
            } else {
                return false;
            }
        }
    }
}

impl<'a> Deref for ListenerContext<'a> {
    type Target = ComponentStateInner;

    fn deref(&self) -> &ComponentStateInner {
        self.inner
    }
}

impl<'a> DerefMut for ListenerContext<'a> {

    fn deref_mut(&mut self) -> &mut ComponentStateInner {
        self.inner
    }
}

pub struct HandlerContext<'a> {
    // the root id - used to allow nodes to send events directly to the top
    root: ID,
    // the id of the object handling the response
    pub (in component_ui) id: ID,
    // the the events bus, used to allow nodes to post messages
    events: &'a mut Vec<(ID, Box<Any>)>,
    // holds the currently invalidated region
    invalidated_region: &'a mut Option<WorldBoundingBox>,
    // hold the worldbounding box from the last call to draw
    last_world_bounding_box: &'a Option<WorldBoundingBox>
}

impl<'a> HandlerContext<'a> {
    pub fn new(root: ID, id: ID, events_ref: &'a mut Vec<(ID, Box<Any>)>, invalidated_region: &'a mut Option<WorldBoundingBox>, world_bbox: &'a Option<WorldBoundingBox>) -> Self {
        HandlerContext {
            root,
            id,
            events: events_ref,
            invalidated_region,
            last_world_bounding_box: world_bbox
        }
    }
    pub fn send_event<A: Any>(&mut self, a : A) {
        self.events.push((self.id, Box::new(a)));
    }
    pub fn send_root_event<A:Any>(&mut self, a: A) {
        self.events.push((self.root, Box::new(a)));
    }
    pub fn invalidate_region(&mut self, region: &WorldBoundingBox) {
        let should_update = if let Some(bbox) = self.last_world_bounding_box.as_ref() {
            let bbox : &WorldBoundingBox = bbox;
            WorldBoundingBox::check_intersect(bbox, region)
        } else {
            // if we don't know what the world bounding box looks like, just always accept invalidations
            true
        };

        if should_update {
            self.invalidated_region.as_mut().map(|bbox| {
                bbox.union(region);
            });
        }
    }
}
