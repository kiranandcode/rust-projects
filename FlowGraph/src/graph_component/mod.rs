//! ECS Based simple FlowGraph renderer - design "inspired" by Xi-Win.
//! https://github.com/google/xi-win/blob/master/xi-win-ui

pub mod id;
pub mod graph;
pub mod object;
pub mod color_scheme;
pub mod utilities;

use id::*;
use graph::*;
use object::*;
use color_scheme::*;
use utilities::*;

use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;

use std::ops::DerefMut;
use std::mem;
use std::cmp::{Ord, Ordering};
use std::iter;
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


// empty struct used to represent the base component of the system
// doesn't actually do anything, but rather used as the base
pub struct ComponentStateBase {}
impl Object for ComponentStateBase {}


// As in Xi-window, represents the context handed to viewers
pub struct ComponentStateInner {
    pub (in graph_component) objects: Vec<(ID,Box<Object>)>,
    /// used to keep track of the graph structure
    pub (in graph_component) object_graph: ObjectGraph,
    /// used to manage ids
    pub (in graph_component) id_gen: IDManager
}




impl ComponentStateInner {
    pub fn new() -> Self {
        let mut graph = ObjectGraph::default();
        let mut objects : Vec<(ID, Box<Object>)>= Vec::new();
        let mut id_manager = IDManager::default();

        // setup the base root
        let id = id_manager.new(0);
        objects.push((id, Box::new(ComponentStateBase{})));
        graph.set_root(id);
        graph.parent.push(id);
        graph.children.push(Vec::new());

        ComponentStateInner {
            id_gen: id_manager,
            objects: objects,
            object_graph: graph
        }

    }
    fn add_node<O>(&mut self, object: O, children: &[ID]) -> ID
    where O: Object + 'static
    {
        add_node(&mut self.id_gen, &mut self.object_graph, &mut self.objects, object, children)
    }

    /// removes a node from the graph.
    /// should be called from the componentstate rather than directly,
    /// as this one doesn't remove the listeners
    fn remove_node(&mut self, id: ID, and_children: bool) -> Vec<ID> {
        // allocate a vec to hold the removed ids
        let mut rem_id = Vec::new();

        // remove the node from the graph - place any id's removed by recursive calls into the rem_id array
        remove_node(&mut self.id_gen, &mut self.object_graph, &mut self.objects, id, and_children, &mut rem_id);

        // return the removed ids
        rem_id
    }


    fn draw(&mut self, context: &Context)  {
        fn draw_rec(accessor: &mut ObjectAccessor, context: &Context, id: ID) {
            // draws the object
            {
                let may_obj = accessor.get_mut(id);
                if let Some(obj) = may_obj {
                    obj.draw(context, id);
                }
            }

            let mut children : Vec<(ID, DrawPriority)> =
            {
                if let Some(o_child) = accessor.children(id) {
                   o_child 
                        .iter()
                    // note, pre-retrieving all the draw priorities here
                    // is better than calculating them on the fly during the
                    // comparison method
                        .map(|id| (*id, accessor.get(*id).map(|obj| obj.draw_priority()).unwrap_or(DrawPriority::Low)))

                        .collect::<Vec<(ID, DrawPriority)>>()
                } else {Vec::new()}
            };

            // then sort the children by draw priority
            children.sort_unstable_by(|(id_a, priority_a), (id_b, priority_b)| {
                priority_a.cmp(priority_b)
            });


            // and draw them
            for (child,_) in children {
                draw_rec(accessor, context, child);
            }
        }


        // and all drawing starts at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        draw_rec(&mut accessor, context, root);
    }

    pub fn motion(&mut self, coords: WorldCoords) {
        fn motion_rec(accessor: &mut ObjectAccessor, coords: &WorldCoords, id: ID) -> bool {
            let mut should_recurse = false;
            // find out whether the node contains the mouse
            {
                let may_obj = accessor.get(id);
                if let Some(obj) = may_obj {
                    if let Some(bbox) = obj.mouse_bounding_box() {
                        should_recurse = bbox.point_within_bounds(coords);
                    } else {
                        // if the node doesn't have a bounding box, we have to recurse
                        // IMPORTANT. this is used by the base view to allow it to always try handling it's children
                        // do not remove
                        should_recurse = true;
                    }
                }
            }

            if should_recurse {
                let mut children : Vec<(ID, DrawPriority)> =
                {
                    if let Some(o_child) = accessor.children(id) {
                        o_child 
                            .iter()
                        // note, pre-retrieving all the draw priorities here
                        // is better than calculating them on the fly during the
                        // comparison method
                            .map(|id| (*id, accessor.get(*id).map(|obj| obj.draw_priority()).unwrap_or(DrawPriority::Low)))
                            .collect::<Vec<(ID, DrawPriority)>>()
                    } else {Vec::new()}
                };

                // then sort the children by draw priority
                children.sort_unstable_by(|(id_a, priority_a), (id_b, priority_b)| {
                    priority_a.cmp(priority_b)
                });
                // we want to handle events in reverse order - i.e if a is drawn on top of b, we want a to handle the event first
                children.reverse();


                let mut handled = false;
                // and draw them
                for (child,_) in children {
                    handled = motion_rec(accessor, coords, child);
                    if handled { break; }
                }

                // if the children did not handle the event
                if !handled {
                        let may_obj = accessor.get_mut(id);
                        if let Some(obj) = may_obj {
                            handled = obj.motion(*coords)
                        }
                }

                handled

            } else {
                // return whether it handled the event
                false
            }
        }

        // and all events start at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        motion_rec(&mut accessor, &coords, root);
    }

    pub fn update(&mut self, current_time: CurrentTime, elapsed_time: DeltaTime) {
        fn update_rec(accessor: &mut ObjectAccessor, current_time: CurrentTime, elapsed_time: DeltaTime, id: ID) {
            // updates the object
            {
                let may_obj = accessor.get_mut(id);
                if let Some(obj) = may_obj {
                    obj.update(current_time, elapsed_time);
                }
            }

            let children = if let Some(children) = accessor.children(id) {
                children.iter().map(|id| *id).collect::<Vec<ID>>()
            } else { Vec::new() };

            // and update them
            for child in children {
                update_rec(accessor, current_time, elapsed_time, child);
            }
        }

        // and all events start at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        update_rec(&mut accessor, current_time, elapsed_time, root);
    }

    pub fn drag_motion(&mut self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) {
        fn drag_motion_rec(accessor: &mut ObjectAccessor, coords: &WorldCoords, dx: &WorldUnit, dy: &WorldUnit, id: ID) -> bool {
            let mut should_recurse = false;
            // find out whether the node contains the mouse
            {
                let may_obj = accessor.get(id);
                if let Some(obj) = may_obj {
                    if let Some(bbox) = obj.mouse_bounding_box() {
                        should_recurse = bbox.point_within_bounds(coords);
                    } else {
                        // if the node doesn't have a bounding box, we have to recurse
                        // IMPORTANT. this is used by the base view to allow it to always try handling it's children
                        // do not remove
                        should_recurse = true;
                    }
                }
            }

            if should_recurse {
                let mut children : Vec<(ID, DrawPriority)> =
                {
                    if let Some(o_child) = accessor.children(id) {
                        o_child
                            .iter()
                        // note, pre-retrieving all the draw priorities here
                        // is better than calculating them on the fly during the
                        // comparison method
                            .map(|id| (*id, accessor.get(*id).map(|obj| obj.draw_priority()).unwrap_or(DrawPriority::Low)))
                            .collect::<Vec<(ID, DrawPriority)>>()
                    } else {Vec::new()}
                };

                // then sort the children by draw priority
                children.sort_unstable_by(|(id_a, priority_a), (id_b, priority_b)| {
                    priority_a.cmp(priority_b)
                });
                // we want to handle events in reverse order - i.e if a is drawn on top of b, we want a to handle the event first
                children.reverse();


                let mut handled = false;
                // and draw them
                for (child,_) in children {
                    handled = drag_motion_rec(accessor, coords, dx, dy, child);
                    if handled { break; }
                }

                // if the children did not handle the event
                if !handled {
                        let may_obj = accessor.get_mut(id);
                        if let Some(obj) = may_obj {
                            handled = obj.drag_motion(*coords, *dx, *dy);
                        }
                }

                handled

            } else {
                // return whether it handled the event
                false
            }
        }

        // and all events start at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        drag_motion_rec(&mut accessor, &coords, &dx, &dy, root);

    }

    pub fn button_press(&mut self, button: ButtonEvent) {
        fn button_press_rec(accessor: &mut ObjectAccessor, button: &ButtonEvent, id: ID) -> bool {
            let mut should_recurse = false;
            // find out whether the node contains the mouse
            {
                let may_obj = accessor.get(id);
                if let Some(obj) = may_obj {
                    if let Some(bbox) = obj.mouse_bounding_box() {
                        should_recurse = bbox.point_within_bounds(&button.pos);
                    } else {
                        // if the node doesn't have a bounding box, we have to recurse
                        // IMPORTANT. this is used by the base view to allow it to always try handling it's children
                        // do not remove
                        should_recurse = true;
                    }
                }
            }

            if should_recurse {
                let mut children : Vec<(ID, DrawPriority)> =
                {
                    if let Some(o_child) = accessor.children(id) {
                        o_child
                            .iter()
                        // note, pre-retrieving all the draw priorities here
                        // is better than calculating them on the fly during the
                        // comparison method
                            .map(|id| (*id, accessor.get(*id).map(|obj| obj.draw_priority()).unwrap_or(DrawPriority::Low)))
                            .collect::<Vec<(ID, DrawPriority)>>()
                    } else {Vec::new()}
                };

                // then sort the children by draw priority
                children.sort_unstable_by(|(id_a, priority_a), (id_b, priority_b)| {
                    priority_a.cmp(priority_b)
                });
                // we want to handle events in reverse order - i.e if a is drawn on top of b, we want a to handle the event first
                children.reverse();


                let mut handled = false;
                // and draw them
                for (child,_) in children {
                    handled = button_press_rec(accessor, button, child);
                    if handled { break; }
                }

                // if the children did not handle the event
                if !handled {
                        let may_obj = accessor.get_mut(id);
                        if let Some(obj) = may_obj {
                            handled = obj.button_press(*button);
                        }
                }

                handled

            } else {
                // return whether it handled the event
                false
            }
        }

        // and all events start at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        button_press_rec(&mut accessor, &button, root);
    }

    pub fn key_press(&mut self, evnt: Key) {
        fn key_press_rec(accessor: &mut ObjectAccessor, key: &Key, id: ID) -> bool {

                let mut children : Vec<ID> =
                {
                    if let Some(o_child) = accessor.children(id) {
                        o_child
                            .iter()
                            .map(|id| *id)
                            .collect::<Vec<ID>>()
                    } else {Vec::new()}
                };


                let mut handled = false;
                // go through the children, and check whether they handle the event
                for child in children {
                    handled = key_press_rec(accessor, key, child);
                    if handled { break; }
                }

                // if the children did not handle the event
                if !handled {
                        let may_obj = accessor.get_mut(id);
                        if let Some(obj) = may_obj {
                            handled = obj.key_press(*key);
                        }
                }

                handled
        }

        // and all events start at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        key_press_rec(&mut accessor, &evnt, root);

    }

}


/// - - - - - - - - - - - - - - - - - - - - -
///                Inner View
/// - - - - - - - - - - - - - - - - - - - - -
pub struct ComponentState {
    inner: ComponentStateInner,
    listeners: BTreeMap<ID, Vec<Box<FnMut(&mut Any, ListenerContext)>>>,
    event_q: Vec<(ID, Box<Any>)>
}


pub struct ListenerContext<'a> {
    id: ID,
    inner: &'a ComponentStateInner
}

impl ComponentState {
    pub fn new() -> Self {
        let inner = ComponentStateInner::new();
        ComponentState {
            listeners: Default::default(),
            inner,
            event_q: Vec::new()
        }
    }

    pub fn add_listener<A,F>(&mut self, node: ID, mut f: F)
    where A:Any, F: FnMut(&mut A, ListenerContext) + 'static {
        let wrapper : Box<FnMut(&mut Any, ListenerContext)> = Box::new(move |a, ctx| {
            if let Some(arg) = a.downcast_mut() {
                f(arg,ctx)
            } else {
                println!("Type mismatch in listener args");
            }
        });
        self.listeners.entry(node).or_default().push(wrapper);
    }


    fn add_node<O>(&mut self, object: O, children: &[ID]) -> ID
    where O: Object + 'static
    {
        self.inner.add_node(object, children)
    }

    fn remove_node(&mut self, id: ID, and_children: bool) {
        // first, remove the node from the graph
        let removed = self.inner.remove_node(id, and_children);
        // then, remove listeners for all the removed nodes
        for rem_id in removed {
            self.listeners.remove(&rem_id);
        }
        // also remove any events for the node
        self.event_q.retain(|(oid, _)| *oid != id);
    }

    fn dispatch_events(&mut self) {
        let events = mem::replace(&mut self.event_q, Vec::new());
        for (id, mut event) in events {
            if let Some(listeners) = self.listeners.get_mut(&id) {
                for listener in listeners {
                    let ctx = ListenerContext {
                        id,
                        inner: &mut self.inner
                    };
                    listener(event.deref_mut(), ctx);
                }
            }
        }
    }

}


/// - - - - - - - - - - - - - - - - - - - - -
///                Main View
/// - - - - - - - - - - - - - - - - - - - - -
pub struct GraphComponent {
    renderer: RefCell<Option<Rc<Renderer>>>,
    // main inner view of the graph component
    // internal: RefCell<ComponentState>
}

impl Default for GraphComponent {
    fn default() -> Self {
        GraphComponent{
            renderer: RefCell::new(None),
        }
    }
}



impl Component for GraphComponent {
    fn on_draw(&self, context: &Context) {
        context.color(COLOR_SCHEME.bg);
        context.paint();

        context.color(Color::BLACK);
        context.rect(WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(20.0), WorldUnit(20.0)));
        context.fill();
    }


    fn on_setup(&self) {
        if let Some(renderer) = self.renderer.borrow().as_ref() {
            renderer.move_view_to(WorldCoords(WorldUnit(0.0), WorldUnit(0.0)));
            renderer.set_handle_drag(true);
        }
    }

    fn on_motion_notify(&self, coords: WorldCoords) {

    }

    fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) {
    }

    fn on_drag_motion_notify(&self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) {
    }

    fn on_button_press(&self, button: ButtonEvent) {
    }

    fn on_key_press(&self, evnt: Key) {
    }

    fn register_renderer(&self, self_rc: Rc<Self>, renderer: Rc<Renderer>) {
        *self.renderer.borrow_mut() = Some(renderer);
    }
}
