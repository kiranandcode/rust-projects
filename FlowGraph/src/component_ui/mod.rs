//! ECS Based simple FlowGraph renderer - design "inspired" by Xi-Win.
//! https://github.com/google/xi-win/blob/master/xi-win-ui

pub mod id;
pub mod graph;
pub mod object;
pub mod utilities;
pub mod context;

use context::{HandlerContext, ListenerContext};
use id::{ID, IDManager};
use graph::{ObjectGraph, ObjectAccessor};
use object::Object;
use utilities::{add_node, remove_node};

use types::*;
use color::*;
use drawing_context::Context;
use component_renderer::{Renderer, Component};

use std::mem;
use std::ops::DerefMut;
use std::any::Any;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};
use std::cell::{RefCell };



// empty struct used to represent the base component of the system
// doesn't actually do anything, but rather used as the base
pub struct ComponentStateBase {}
impl Object for ComponentStateBase {
}


// As in Xi-window, represents the context handed to viewers
pub struct ComponentStateInner {
    pub (in component_ui) objects: Vec<(ID,Box<Object>)>,
    /// used to keep track of the graph structure
    pub (in component_ui) object_graph: ObjectGraph,
    /// used to manage ids
    pub (in component_ui) id_gen: IDManager
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

    fn add_node<O>(&mut self, object: O, children: &[ID],  events: &mut Vec<(ID, Box<Any>)>,invalidated_region: &mut Option<WorldBoundingBox>, last_world_bounding_box: &Option<WorldBoundingBox>) -> ID
    where O: Object + 'static
    {
        let mut handler_context = HandlerContext::new(self.object_graph.get_root(), self.object_graph.get_root(), events, invalidated_region, last_world_bounding_box);

        add_node(&mut self.id_gen, &mut self.object_graph, &mut self.objects, object, children, &mut handler_context)

    }

    /// removes a node from the graph.
    /// should be called from the componentstate rather than directly,
    /// as this one doesn't remove the listeners
    fn remove_node(&mut self, id: ID, and_children: bool, events: &mut Vec<(ID, Box<Any>)>,invalidated_region: &mut Option<WorldBoundingBox>, last_world_bounding_box: &Option<WorldBoundingBox>) -> Vec<ID> {
        let mut rem_id = Vec::new();

        let mut handler_context = HandlerContext::new(self.object_graph.get_root(), id, events, invalidated_region, last_world_bounding_box);
        // remove the node from the graph - place any id's removed by recursive calls into the rem_id array
        remove_node(&mut self.id_gen, &mut self.object_graph, &mut self.objects, id, and_children, &mut rem_id, &mut handler_context);

        // return the removed ids
        rem_id
    }


    fn draw(&mut self, context: &Context, color_scheme: &ColorScheme)  {
        fn draw_rec(accessor: &mut ObjectAccessor, context: &Context, id: ID, color_scheme: &ColorScheme) {
            // draws the object
            {
                let may_obj = accessor.get_mut(id);
                if let Some(obj) = may_obj {
                    obj.draw(context, id, color_scheme);
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
                draw_rec(accessor, context, child, color_scheme);
            }
        }


        // and all drawing starts at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        draw_rec(&mut accessor, context, root, color_scheme);
    }

    fn motion(&mut self, coords: WorldCoords, events: &mut Vec<(ID, Box<Any>)>, invalidated_region: &mut Option<WorldBoundingBox>, world_bbox: &Option<WorldBoundingBox>) {
        fn motion_rec(accessor: &mut ObjectAccessor, coords: &WorldCoords,  mut ctx: &mut HandlerContext) -> bool {
            let id = ctx.id;
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
                    ctx.id = child;
                    handled = motion_rec(accessor, coords, ctx);
                    if handled { break; }
                }

                // if the children did not handle the event
                if !handled {
                    // restore the id of the handler
                    ctx.id = id;
                    let may_obj = accessor.get_mut(id);
                    if let Some(obj) = may_obj {
                        // and let the parent try
                        handled = obj.motion(*coords, ctx)
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
        let mut handler = HandlerContext::new(root, root, events, invalidated_region, world_bbox);
        motion_rec(&mut accessor, &coords, &mut handler);
    }

    fn update(&mut self, current_time: CurrentTime, elapsed_time: DeltaTime,  events: &mut Vec<(ID, Box<Any>)>, invalidated_region: &mut Option<WorldBoundingBox>, world_bbox: &Option<WorldBoundingBox>) {
        fn update_rec(accessor: &mut ObjectAccessor, current_time: CurrentTime, elapsed_time: DeltaTime, ctx: &mut HandlerContext) {
            let id = ctx.id;
            // updates the object
            {
                let may_obj = accessor.get_mut(id);
                if let Some(obj) = may_obj {
                    obj.update(current_time, elapsed_time, ctx);
                }
            }

            let children = if let Some(children) = accessor.children(id) {
                children.iter().map(|id| *id).collect::<Vec<ID>>()
            } else { Vec::new() };

            // and update them
            for child in children {
                ctx.id = child;
                update_rec(accessor, current_time, elapsed_time, ctx);
            }
        }

        // and all events start at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        let mut ctx = HandlerContext::new(root, root, events, invalidated_region, world_bbox);
        update_rec(&mut accessor, current_time, elapsed_time, &mut ctx);
    }

    fn drag_motion(&mut self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit, events: &mut Vec<(ID, Box<Any>)>, invalidated_region: &mut Option<WorldBoundingBox>, world_bbox: &Option<WorldBoundingBox>) {
        fn drag_motion_rec(accessor: &mut ObjectAccessor, coords: &WorldCoords, dx: &WorldUnit, dy: &WorldUnit, ctx: &mut HandlerContext) -> bool {
            let mut should_recurse = false;
            let id = ctx.id;
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
                    ctx.id = child;
                    handled = drag_motion_rec(accessor, coords, dx, dy, ctx);
                    if handled { break; }
                }

                // if the children did not handle the event
                if !handled {
                    ctx.id = id;
                    let may_obj = accessor.get_mut(id);
                    if let Some(obj) = may_obj {
                        handled = obj.drag_motion(*coords, *dx, *dy, ctx);
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
        
        let mut ctx = HandlerContext::new(root, root, events, invalidated_region, world_bbox);
        drag_motion_rec(&mut accessor, &coords, &dx, &dy, &mut ctx);

    }

    fn button_press(&mut self, button: ButtonEvent, events: &mut Vec<(ID, Box<Any>)>, invalidated_region: &mut Option<WorldBoundingBox>, world_bbox: &Option<WorldBoundingBox>) {
        fn button_press_rec(accessor: &mut ObjectAccessor, button: &ButtonEvent, ctx: &mut HandlerContext) -> bool {
            let id = ctx.id;
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
                    ctx.id = child;
                    handled = button_press_rec(accessor, button, ctx);
                    if handled { break; }
                }

                // if the children did not handle the event
                if !handled {
                    ctx.id = id;
                    let may_obj = accessor.get_mut(id);
                    if let Some(obj) = may_obj {
                        handled = obj.button_press(*button, ctx);
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
        let mut ctx = HandlerContext::new(root, root, events, invalidated_region, world_bbox);
        button_press_rec(&mut accessor, &button, &mut ctx);
    }

    fn key_press(&mut self, evnt: Key, events: &mut Vec<(ID, Box<Any>)>, invalidated_region: &mut Option<WorldBoundingBox>, world_bbox: &Option<WorldBoundingBox>) {
        fn key_press_rec(accessor: &mut ObjectAccessor, key: &Key, ctx: &mut HandlerContext) -> bool {

            let id = ctx.id;
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
                ctx.id = child;
                handled = key_press_rec(accessor, key, ctx);
                if handled { break; }
            }

            // if the children did not handle the event
            if !handled {
                ctx.id = id;
                let may_obj = accessor.get_mut(id);
                if let Some(obj) = may_obj {
                    handled = obj.key_press(*key, ctx);
                }
            }

            handled
        }

        // and all events start at the root.
        let root = self.object_graph.get_root();
        let mut accessor = ObjectAccessor::new(&mut self.objects, &self.object_graph, &self.id_gen);
        let mut ctx = HandlerContext::new(root, root, events, invalidated_region, world_bbox);
        key_press_rec(&mut accessor, &evnt, &mut ctx);

    }

    fn poke_internal<A: Any>(&mut self, root: ID, id: ID, payload: &mut A, events: &mut Vec<(ID, Box<Any>)>, invalidated_region: &mut Option<WorldBoundingBox>, world_bbox: &Option<WorldBoundingBox>) -> bool {
        let mut ctx = HandlerContext::new(root, id, events, invalidated_region, world_bbox);
        if let Ok(raw_id) = self.id_gen.get(id) {
            self.objects[raw_id].1.poke(payload, &mut ctx)
        } else {
            false
        }
    }
}


/// - - - - - - - - - - - - - - - - - - - - -
///                Inner View
/// - - - - - - - - - - - - - - - - - - - - -
pub struct ComponentState {
    // a reference to the renderer - used to queue draws in update method
    renderer: Rc<Renderer>,
    // an accumulator used to keep track of invalidated regions, to be queued for redrawing on update
    invalidated_region: Option<WorldBoundingBox>,
    // a hint specifying the location of the rendering box the last time the draw function was called, used to identify whether a components
    // invalidation is worth redrawing 
    last_bounding_box: Option<WorldBoundingBox>,
    // represents the inner most state of the visualizer
    inner: ComponentStateInner,
    // represents the listeners for the system
    listeners: BTreeMap<ID, Vec<Box<FnMut(&mut Any, ListenerContext)>>>,
    // represents the event queue
    event_q: Vec<(ID, Box<Any>)>
}



impl ComponentState {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let inner = ComponentStateInner::new();
        ComponentState {
            listeners: Default::default(),
            inner,
            event_q: Vec::new(),
            renderer,
            invalidated_region: None,
            last_bounding_box: None
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

    pub fn add_root_listener<A,F>(&mut self, mut f: F)
    where A:Any, F: FnMut(&mut A, ListenerContext) + 'static {
        let root = self.inner.object_graph.root;
        let wrapper : Box<FnMut(&mut Any, ListenerContext)> = Box::new(move |a, ctx| {
            if let Some(arg) = a.downcast_mut() {
                f(arg,ctx)
            } else {
                println!("Type mismatch in listener args");
            }
        });
        self.listeners.entry(root).or_default().push(wrapper);
    }

    pub fn add_node<O>(&mut self, object: O, children: &[ID]) -> ID
    where O: Object + 'static
    {
        self.inner.add_node(object, children, &mut self.event_q, &mut self.invalidated_region, &self.last_bounding_box)
    }

    pub fn remove_node(&mut self, id: ID, and_children: bool) {
        // first, remove the node from the graph
        let removed = self.inner.remove_node(id, and_children, &mut self.event_q, &mut self.invalidated_region, &self.last_bounding_box);

        // then, remove listeners for all the removed nodes
        for rem_id in removed {
            self.listeners.remove(&rem_id);
        }

        // also remove any events for the node
        self.event_q.retain(|(oid, _)| *oid != id);
    }

    // remove any listeners for invalidated nodes
    fn clean_listeners(&mut self) {
        let mut to_remove = Vec::new();
        for key in self.listeners.keys() {
            let key = *key;
            if !self.inner.id_gen.valid(key) {
                to_remove.push(key);
            }
        }
        for rem_id in to_remove {
            self.listeners.remove(&rem_id);
        }
    }

    fn dispatch_events(&mut self) {
        let initial_len = self.inner.objects.len();
        let events = mem::replace(&mut self.event_q, Vec::new());
        for (id, mut event) in events {
            if let Some(listeners) = self.listeners.get_mut(&id) {
                for listener in listeners {
                    let ctx = ListenerContext::new(
                        id,
                        &mut self.inner,
                        &mut self.event_q,
                        &mut self.invalidated_region,
                        &self.last_bounding_box
                    );
                    listener(event.deref_mut(), ctx);
                }
            }
        }
        let final_len = self.inner.objects.len();
        // this probably means a node has been removed or added
        // so best to remove any old listeners
        if initial_len != final_len {
            self.clean_listeners();
        }
    }

    fn draw(&mut self, context: &Context, color_scheme: &ColorScheme) {
        // update the world_bounding_box
        self.last_bounding_box = Some(context.bounding_box().clone());

        // and draw yourself
        self.inner.draw(context, color_scheme);
    }

    fn motion(&mut self, coords: WorldCoords) {
        self.inner.motion(coords, &mut self.event_q, &mut self.invalidated_region, &self.last_bounding_box);
        self.dispatch_events();
    }

    fn update(&mut self, current_time: CurrentTime, elapsed_time: DeltaTime) {
        self.inner.update(current_time, elapsed_time, &mut self.event_q, &mut self.invalidated_region, &self.last_bounding_box);
        self.dispatch_events();

        // if there are any invalidated regions, update them
        if let Some(bbox) = self.invalidated_region.take() {
            self.renderer.queue_draw_area(bbox);
        }
    }

    fn drag_motion(&mut self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) {
        self.inner.drag_motion(coords, dx, dy, &mut self.event_q, &mut self.invalidated_region, &self.last_bounding_box);
        self.dispatch_events();
    }

    fn button_press(&mut self, button: ButtonEvent) {
        self.inner.button_press(button, &mut self.event_q, &mut self.invalidated_region, &self.last_bounding_box);
        self.dispatch_events();
    }

    fn key_press(&mut self, evnt: Key) {
        self.inner.key_press(evnt, &mut self.event_q, &mut self.invalidated_region, &self.last_bounding_box);
        self.dispatch_events();
    }

}


/// - - - - - - - - - - - - - - - - - - - - -
///                Main View
/// - - - - - - - - - - - - - - - - - - - - -
pub struct ComponentUI {
    renderer: RefCell<Option<Rc<Renderer>>>,
    // main inner view of the graph component
    // we wrap it in a option, to allow the componentstate to be accessed only when it has a renderer
    internal: RefCell<Option<ComponentState>>,
    color_scheme: ColorScheme,
}

impl Default for ComponentUI {
    fn default() -> Self {
        ComponentUI{
            renderer: RefCell::new(None),
            internal: RefCell::new(None),
            color_scheme: Default::default()
        }
    }
}

impl ComponentUI {
    pub fn new(color_scheme: ColorScheme) -> Self {
        ComponentUI{
            renderer: RefCell::new(None),
            internal: RefCell::new(None),
            color_scheme
        }
    }
    pub fn get_state(&self) -> &RefCell<Option<ComponentState>> {
        &self.internal
    }
}


impl Component for ComponentUI {
    fn on_draw(&self, context: &Context) {
        context.color(self.color_scheme.bg);
        context.paint();

        // debug drawing
        context.color(Color::BLACK);
        context.rect(WorldBoundingBox(WorldUnit(0.0), WorldUnit(0.0), WorldUnit(20.0), WorldUnit(20.0)));
        context.fill();

        if  let Some(internal) = self.internal.borrow_mut().as_mut() {
            internal.draw(context, &self.color_scheme);
        }
    }


    fn on_setup(&self) {
        if let Some(renderer) = self.renderer.borrow().as_ref() {
            renderer.move_view_to(WorldCoords(WorldUnit(0.0), WorldUnit(0.0)));
            renderer.set_handle_drag(false);
        }
    }

    fn on_motion_notify(&self, coords: WorldCoords) {
        // println!("Got a motion event {:?}", coords);
        if  let Some(internal) = self.internal.borrow_mut().as_mut() {
            internal.motion(coords);
        }
    }




    fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) {
        // println!("Got an update event {:?}", (current_time, elapsed_time));
        if  let Some(internal) = self.internal.borrow_mut().as_mut() {
            internal.update(current_time, elapsed_time);
        }
    }

    fn on_drag_motion_notify(&self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) {
        // println!("Got a drag event {:?}", (coords, dx, dy));
        if  let Some(internal) = self.internal.borrow_mut().as_mut() {
            internal.drag_motion(coords, dx, dy);
        }
    }

    fn on_button_press(&self, button: ButtonEvent) {
        // println!("Got a button press event {:?}", button);
        if let Some(internal) = self.internal.borrow_mut().as_mut() {
            internal.button_press(button);
        }
    }

    fn on_button_release(&self, button: ButtonEvent) {
        // println!("Got a button press event {:?}", button);
        if let Some(internal) = self.internal.borrow_mut().as_mut() {
            internal.button_press(button);
        }
    }

    fn on_key_press(&self, evnt: Key) {
        // TODO: Maybe movement keys
    }

    fn on_key_release(&self, evnt: Key) {
        // println!("Got a key press event {:?}", evnt);
        if let Some(internal) = self.internal.borrow_mut().as_mut() {
            internal.key_press(evnt);
        }
    }

    fn register_renderer(&self, renderer: Rc<Renderer>) {
        *self.renderer.borrow_mut() = Some(renderer.clone());
        // and once we have registered the renderer, we can construct our inner state
        *self.internal.borrow_mut() = Some(ComponentState::new(renderer));
    }
}
