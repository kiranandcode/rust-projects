//! ECS Based simple FlowGraph renderer - design "inspired" by Xi-Win.
//!

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
    pub fn add<O>(&mut self, object: O, children: &[ID]) -> ID
    where O: Object + 'static
    {
        add_node(&mut self.id_gen, &mut self.object_graph, &mut self.objects, object, children)
    }

    pub fn remove_node(&mut self, id: ID, and_children: bool) {
        remove_node(&mut self.id_gen, &mut self.object_graph, &mut self.objects, id, and_children);
    }


    pub fn draw(&mut self, context: &Context)  {
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

    pub fn motion(&self, coords: WorldCoords) {

    }

    pub fn update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) {
    }

    pub fn drag_motion(&self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) {
    }

    pub fn button_press(&self, button: ButtonEvent) {
    }

    pub fn key_press(&self, evnt: Key) {
    }


}


/// - - - - - - - - - - - - - - - - - - - - -
///                Inner View
/// - - - - - - - - - - - - - - - - - - - - -
pub struct ComponentState {
    inner: ComponentStateInner,
    listeners: BTreeMap<ID, Vec<Box<FnMut(&mut Any, ListenerContext)>>>
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
            inner
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
