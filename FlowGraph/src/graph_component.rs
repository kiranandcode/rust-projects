use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;



use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};
use std::convert::TryFrom;

use gtk::{Window, WindowExt, WidgetExt, ContainerExt};
use gdk::EventMask;

/// - - - - - - - - - - - - - - - - - - - - -
///                   ID
/// - - - - - - - - - - - - - - - - - - - - -
type ID = usize;
static mut last_id: ID = 0;

pub fn get_id() -> ID {
    unsafe {
        let result = last_id;
        last_id += 1;
        result
    }
}



/// - - - - - - - - - - - - - - - - - - - - -
///              Color Scheme
/// - - - - - - - - - - - - - - - - - - - - -
pub struct ColorScheme {
    pub bg: Color,
    pub node_bg: Color,
    pub node_text: Color,
    pub node_fg: Color,
    pub node_fg_text: Color,
    pub node_bg_accent: Color,
    pub node_bg_highlight: Color,
}

const COLOR_SCHEME: ColorScheme = ColorScheme {
    bg: Color(231.0/255.0, 232.0/255.0, 236.0/255.0, Some(1.0)),
    node_bg: Color::WHITE,
    node_text: Color::BLACK,
    node_fg: Color(20.0/255.0, 177.0/255.0, 219.0/255.0, Some(1.0)),
    node_fg_text: Color::BLACK,
    node_bg_accent: Color(203.0/255.0, 203.0/255.0, 203.0/255.0, Some(1.0)),
    node_bg_highlight: Color(153.0/255.0, 153.0/255.0, 153.0/255.0, Some(0.7)),
};



/// - - - - - - - - - - - - - - - - - - - - -
///                  Nodes
/// - - - - - - - - - - - - - - - - - - - - -

trait Node {
    fn draw(&self, context:&Context);
    fn set_highlight(&self, highlight: bool);
    fn bounding_box(&self) -> Ref<WorldBoundingBox>;
    fn id(&self) -> ID;
}

struct SimpleNode {
    id: ID,
    bounding_box: RefCell<WorldBoundingBox>,
    is_highlighted: RefCell<bool>,
    parent_rc: Rc<GraphComponent>,
}
impl SimpleNode {
    pub fn new(parent_rc: Rc<GraphComponent>, coords: WorldCoords) -> Self {
        SimpleNode {
            id: get_id(),
            bounding_box: RefCell::new(WorldBoundingBox::new_centered_around(coords, WorldUnit(50.0), WorldUnit(100.0))),
            is_highlighted: RefCell::new(false),
            parent_rc
        }
    }
}

impl Node for SimpleNode {
    fn draw(&self, context: &Context) {
        let bounding_box = self.bounding_box.borrow_mut();
        context.color(COLOR_SCHEME.node_bg);
        context.rect(bounding_box.clone());
        if *self.is_highlighted.borrow() {
            context.fill_preserve();
            context.color(COLOR_SCHEME.node_bg_highlight);
            context.stroke_width(0.5);
            context.stroke();
        } else {
            context.fill();
        }

        context.color(COLOR_SCHEME.node_bg_accent);
        context.stroke_width(1.0);
        context.line(bounding_box.lower_left(), bounding_box.lower_right());
        context.stroke();
    }

    fn set_highlight(&self, highlight: bool) {
        println!("Being set to highlight {:?}", highlight);
        *self.is_highlighted.borrow_mut() = highlight;
        println!("Highlight done {:?}", highlight);
        let draw_bounding_box = self.bounding_box.borrow().clone();
        println!("Drawing {:?}", highlight);
        self.parent_rc.invalidate_region(draw_bounding_box);
        println!("Drawing done {:?}", highlight);
    }

    fn bounding_box(&self) -> Ref<WorldBoundingBox> {
        self.bounding_box.borrow()
    }
    fn id(&self) -> ID {
        self.id
    }
}


/// - - - - - - - - - - - - - - - - - - - - -
///                Main View
/// - - - - - - - - - - - - - - - - - - - - -
pub struct GraphComponent {
    self_rc: RefCell<Option<Rc<GraphComponent>>>,
    renderer: RefCell<Option<Rc<Renderer>>>,
    invalidated_region: RefCell<Option<WorldBoundingBox>>,
    nodes: RefCell<Vec<Rc<Node>>>,
    highlighted_node: RefCell<Option<Rc<Node>>>
}

impl Default for GraphComponent {
    fn default() -> Self {
        GraphComponent{
            self_rc: RefCell::new(None),
            renderer: RefCell::new(None),
            invalidated_region: RefCell::new(None),
            nodes: RefCell::new(Vec::new()),
            highlighted_node: RefCell::new(None)
        }
    }
}

impl GraphComponent {
    fn invalidate_region(&self, window: WorldBoundingBox) {
        println!("Invalidating region");
        if let Some(old_bounds) = self.invalidated_region.borrow_mut().as_mut() {
            println!("Locked Invalidating region");
            old_bounds.union(&window);
            println!("Invalidated region");
        } else {
            *self.invalidated_region.borrow_mut() = Some(window);
        }
    }

    fn remove_item(&self, id: ID) {
        self.nodes.borrow_mut().retain(|node| { node.id() != id });
    }

    fn get_node_at_point(&self, coords: &WorldCoords) -> Option<Rc<Node>> {
        let nodes = self.nodes.borrow();
        for node in nodes.iter() {
            if node.bounding_box().point_within_bounds(coords) {
                return Some(node.clone());
            }
        }
        None
    }

    fn add_node<T: Node + 'static>(&self, node: T) {
        self.nodes.borrow_mut().push(Rc::new(node));
        if let Some(renderer) = self.renderer.borrow().as_ref() {
            renderer.queue_draw();
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

        for node in self.nodes.borrow().iter() {
            node.draw(context);
        }
    }

    fn on_update(&self, current_time: CurrentTime, elapsed_time: DeltaTime) {
        println!("Update called");
        if let Some(old_bounds) = self.invalidated_region.borrow_mut().take() {
            println!("Holding invalidated region");
            if let Some(ref renderer) = self.renderer.borrow().as_ref() {
                println!("Drawing with renderer ");
                renderer.queue_draw_area(old_bounds);
            }
        }
    }

    fn on_setup(&self) {
        if let Some(renderer) = self.renderer.borrow().as_ref() {
            renderer.move_view_to(WorldCoords(WorldUnit(0.0), WorldUnit(0.0)));
            renderer.set_handle_drag(true);
        }
    }

    fn on_motion_notify(&self, coords: WorldCoords) {

        // highlight the current node
        if let Some(node) = self.get_node_at_point(&coords) {
            // unhighlight the previous node if it isn't the current node
            if let Some(ref old_node) = self.highlighted_node.borrow_mut().take() {
                if old_node.id() != node.id() {
                    node.set_highlight(false);
                }
            }

            node.set_highlight(true);
            *self.highlighted_node.borrow_mut() = Some(node);
        }
    }

    fn on_drag_motion_notify(&self, coords: WorldCoords, dx: WorldUnit, dy: WorldUnit) {
        println!("Recieved a drag motion event: {:?}", (coords, dx,dy));
    }

    fn on_button_press(&self, button: ButtonEvent) {
        println!("ButtonClick");
        if let Some(self_rc) = self.self_rc.borrow().clone() {
            let node = (SimpleNode::new(self_rc,button.pos));
            self.add_node(node);
        }
    }

    fn on_key_press(&self, evnt: Key) {
        if let Key::Char(chr) = evnt {
            if let Some(ref renderer) = self.renderer.borrow().as_ref() {
                if chr == ' ' {
                    renderer.set_handle_drag(false);
                } else {
                    renderer.set_handle_drag(true);
                }
            }
        }
    }

    fn register_renderer(&self, self_rc: Rc<Self>, renderer: Rc<Renderer>) {
        *self.renderer.borrow_mut() = Some(renderer);
        *self.self_rc.borrow_mut() = Some(self_rc);
    }
}
