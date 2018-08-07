use std::ops::{DerefMut, Deref};
use std::any::Any;

use types::*;
use super::ComponentStateInner;
use super::ID;

//! Module encapsulating all contexts used by the system
//!
//! # Note: We will use the following terminology to refer to handling events
//!
//! A object can be sent events in two ways - builtins and listeners
//!  - a builtin is defined in the object's impl for the `Object` trait - these have to be static, and are recommended to be generic,
//!    - for example, a builtin may allow you to change the way the object renders, or some of it's internal properties
//!    - most of the builtins are called by the framework - i.e. mouse will be called when the mouse enters the object's mouse bounding region
//!    - the poke builtin is an additional builtin that isn't used by the framework, and can be modified to be specific to the component
//!
//!
//!  - a listener is defined externally to the object, and is implemented by calling the `add_listener` method on the ui-inner state
//!    - listeners can be more domain specific - such as responding to a button click
//!    - listeners are invoked from builtins - i.e, a builtin will be called, and in it's implementation it may choose to send a message to any listeners
//!
//! the builtins are provided a `HandlerContext<'a>` on invokation, providing them with:
//!     - `send_event`
//|           - used to propagate an object specific event to client listeners - example: ButtonReleased, ButtonPressed
//!     - `send_root_event`
//!           - used to send a global event to listeners on the root - it might be useful for erroring: i.e NumberFieldValueInvalid
//!     - `invalidate_region`
//!           - used to invalidate a region of screen area, queueing it for redraw upon the next update
//!
//! listeners are provided a `ListenerContext<'a>` on invokation, providing them with:
//!     - `send_event`
//!           - used to allow listeners to invoke other listeners (WARNING: may lead to infinite cycles), i.e invokation of A, sends message to B, which upon invokation sends a message to A
//!     - `send_root_event`
//!           - used to send an event to the root
//!     - `poke_up`
//!           - used to allow listeners to invoke builtins
//!                  - intended for use by UI developers - as it allows composing components, and propagating child events to the parent (i.e, a button may)

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

    /// use poke_up from a listener to send arbitrary payloads to be processed by parent objects
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

    pub fn send_event<A: Any>(&mut self, id: ID, a : A) {
        self.events.push((id, Box::new(a)));
    }


    pub fn send_root_event<A:Any>(&mut self, a: A) {
        self.events.push((self.inner.object_graph.root, Box::new(a)));
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
        let mut should_update = if let Some(bbox) = self.last_world_bounding_box.as_ref() {
            let bbox : &WorldBoundingBox = bbox;
            WorldBoundingBox::check_intersect(bbox, region)
        } else {
            // if we don't know what the world bounding box looks like, just always accept invalidations
            true
        };

        // first check if there is already an invalidated region
        if should_update {
            should_update = true;

            if let Some(invalid_region) = self.invalidated_region.as_mut()
            {
                // there is one, update it
                invalid_region.union(region);
                // also, update the should_update - no need to update again
                should_update = false;
            }
        }
        // if we still need to update, this means the current invalidated region is empty
        if should_update {
            *self.invalidated_region = Some(region.clone())
        }
    }
}
