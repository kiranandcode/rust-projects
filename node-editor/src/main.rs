extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::*;

mod support;



struct Node {
    id: usize,
    name: ImString,
    pos: ImVec2,
    size: ImVec2,
    value: f32,
    color: [f32; 3],
    inputs_count: usize,
    outputs_count: usize
}
impl Node {
    fn get_input_slot_pos(&self, slot_no: usize) -> ImVec2 {
        ImVec2::new(
            self.pos.x,
            self.pos.y + self.size.y * ((slot_no + 1) as f32)/((self.inputs_count + 1) as f32)

        )
    }

    fn get_output_slot_pos(&self, slot_no: usize) -> ImVec2 {
        ImVec2::new(
            self.pos.x + self.size.x,
            self.pos.y +  self.size.y * ((slot_no + 1) as f32)/((self.outputs_count + 1) as f32))
    }
}
struct NodeLink {
    input_id: usize,
    input_slot: usize,
    output_id: usize,
    output_slot: usize,

}
impl NodeLink {
    fn new(input_id: usize, input_slot: usize, output_id: usize, output_slot: usize) -> Self {
        NodeLink {
            input_id,
            input_slot,
            output_id,
            output_slot
        }
    }
}

const CLEAR_COLOR: [f32; 4] = [0.2, 0.3, 0.5, 1.0];
const GRID_SZ: f32 = 64.0;
const NODE_SLOT_RADIUS: f32 = 4.0;

fn fmodf(value: f32, other: f32) -> f32 {
    value - (value / other).floor() * other
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fmodf_works_1() {
        assert_eq!(fmodf(10.0, 5.0), 0.0);
    }
    #[test]
    fn fmodf_works_2() {
        assert_eq!(fmodf(12.0, 5.0), 2.0);
    }
    #[test]
    fn fmodf_works_3() {
        assert!((fmodf(12.3, 5.0) -  2.3).abs() < 0.0001);
    }

    #[test]
    fn fmodf_works_4() {
        assert_eq!(fmodf(5.3, 10.0), 5.3);
    }


}

fn add(a: ImVec2, b: ImVec2) -> ImVec2 {
    ImVec2::new(
        a.x + b.x,
        a.y + b.y
    )
}



fn main() {
    let GRID_COLOR: ImColor = ImColor::from((200.0/255.0, 200.0/255.0, 200.0/255.0, 40.0/255.0));
    let NODE_WINDOW_PADDING: ImVec2 = ImVec2::new(8.0, 8.0);

    let mut nodes : Vec<Node> = Vec::new();
    let mut links : Vec<NodeLink> = Vec::new();
    let mut initialized = false;
    let mut scrolling = ImVec2::new(0.0, 0.0);
    let mut show_grid = true;
    let mut selected_node: Option<usize> = None;

    support::run("hello_world.rs".to_owned(), CLEAR_COLOR, |ui| {
        if !initialized {
            nodes.push(Node{
                id: 0,
                name: unsafe { ImString::from_utf8_unchecked(b"MainTex".to_vec()) },
                pos: ImVec2::new(40.0, 50.0),
                size: ImVec2::new(0.0, 0.0),
                value: 0.5,
                color: [255.0/255.0, 100.0/255.0, 100.0/255.0],
                inputs_count: 1,
                outputs_count: 1
            });

            nodes.push(Node{
                id: 1,
                name: unsafe {ImString::from_utf8_unchecked(b"BumpMap".to_vec())},
                pos: ImVec2::new(40.0, 150.0),
                size: ImVec2::new(0.0, 0.0),
                value: 0.42,
                color: [200.0/255.0, 100.0/255.0, 100.0/255.0],
                inputs_count: 1,
                outputs_count: 1
            });

            nodes.push(Node{
                id: 2,
                name: unsafe { ImString::from_utf8_unchecked(b"Combine".to_vec()) },
                pos: ImVec2::new(270.0, 80.0),
                size: ImVec2::new(0.0, 0.0),
                value: 0.42,
                color: [0.0, 200.0/255.0, 100.0/255.0],
                inputs_count: 2,
                outputs_count: 2
            });
            links.push(NodeLink::new(0,0,2,0));
            links.push(NodeLink::new(1,0,2,1));
            initialized = true;
        }


        // draw a list of nodes on the left side
        ui.window(im_str!("Node Editor")) .title_bar(true) .position((0.0, 0.0), ImGuiCond::Appearing)
            .build(|| {

                let mut open_context_menu = false;
                let mut node_hovered_in_list = None;
                let mut node_hovered_in_scene = None;
                let mut any_hovered = false;
                let mut any_active = false;

                ui.child_frame(im_str!("node_list"), ImVec2::new(100.0, 0.0))
                    .build(|| {
                        ui.text(im_str!("Nodes"));
                        ui.separator();
                        for node in nodes.iter() {
                            ui.with_id(ImId::from(node.id as i32), || {
                                let node_is_selected = if let Some(id) = selected_node {id == node.id} else { false };

                                if ui.selectable(&node.name, node_is_selected, ImGuiSelectableFlags::empty(), node.size.clone()) {
                                    selected_node = Some(node.id);
                                }

                                if ui.is_item_hovered() {
                                    node_hovered_in_list = Some(node.id);
                                    open_context_menu |= ui.imgui().is_mouse_clicked(ImMouseButton::Right);
                                }

                                any_active = any_active || ui.is_item_active();
                                any_hovered = any_hovered || ui.is_item_hovered();
                            });
                        }
                    });
                ui.same_line(0.0);

                ui.group(|| {

                    ui.text(
                        format!("Hold the middle mouse button to scroll ({:.2}, {:.2})", scrolling.x, scrolling.y)
                    );

                    ui.checkbox(im_str!("Show grid"), &mut show_grid);
                    ui.with_style_vars(
                        &[
                            StyleVar::FramePadding(ImVec2::new(1.0, 1.0)),
                            StyleVar::WindowPadding(ImVec2::new(0.0, 0.0))
                        ], || {
                            ui.with_color_var(
                                ImGuiCol::ChildBg,
                                ImVec4::new(60.0/255.0, 60.0/255.0, 70.0/255.0, 200.0/255.0),
                                || {
                                    ui.child_frame(im_str!("scrolling_region"), ImVec2::new(0.0, 0.0))
                                        .show_borders(true)
                                        .show_scrollbar(false)
                                        .movable(false)
                                        .build(|| {
                                            ui.push_item_width(120.0);

                                            let mut offset = ui.get_cursor_screen_pos();
                                            offset.0 += scrolling.x;
                                            offset.1 += scrolling.y;
                                            let draw_list = ui.get_window_draw_list();

                                            if show_grid {

                                                let win_pos = ui.get_cursor_screen_pos();
                                                let canvas_sz = ui.get_window_size();
                                                let mut x = fmodf(scrolling.x, GRID_SZ);
                                                while x < canvas_sz.0 {
                                                    draw_list.add_line(
                                                        ImVec2::new(
                                                            x + win_pos.0,
                                                            0.0 + win_pos.1
                                                        ),

                                                        ImVec2::new(
                                                            x + win_pos.0,
                                                            canvas_sz.1 + win_pos.1
                                                        ),
                                                        GRID_COLOR
                                                    ).build();
                                                    x += GRID_SZ;

                                                }

                                                let mut y = fmodf(scrolling.y, GRID_SZ);
                                                while y < canvas_sz.1 {
                                                    draw_list.add_line(
                                                        ImVec2::new(
                                                            0.0 + win_pos.0,
                                                            y + win_pos.1
                                                        ),

                                                        ImVec2::new(
                                                            canvas_sz.0 + win_pos.0,
                                                            y + win_pos.1
                                                        ),
                                                        GRID_COLOR
                                                    ).build();
                                                    y += GRID_SZ;
                                                }
                                            }



                                            let offset = ImVec2::new(offset.0, offset.1);
                                            draw_list.channels_split(2, |channels| {

                                                channels.set_current(0);

                                                for link in links.iter() {
                                                    let link: &NodeLink = link;
                                                    let node_inp : &Node = &nodes[link.input_id];
                                                    let node_out : &Node = &nodes[link.output_id];

                                                    let p1 = add(offset, node_inp.get_output_slot_pos(link.input_slot));
                                                    let p2 = add(offset, node_out.get_input_slot_pos(link.output_slot));

                                                    draw_list.add_bezier_curve(
                                                        p1,
                                                        add(p1, ImVec2::new(50.0, 0.0)),
                                                        add(p2, ImVec2::new(-50.0, 0.0)),
                                                        p2,
                                                        ImColor::from((200.0/255.0, 200.0/255.0, 100.0/255.0, 255.0/255.0)),
                                                    ).build();
                                                }




                                                for (i_n, node) in nodes.iter_mut().enumerate() {
                                                    let node: &mut Node = node;
                                                    ui.with_id(ImId::from(node.id as i32), || {
                                                        let node_rect_min = add(offset, node.pos);

                                                        // display node contents first
                                                        channels.set_current(1); // foreground

                                                        ui.set_cursor_screen_pos(add(node_rect_min, NODE_WINDOW_PADDING));
                                                        ui.group(|| {
                                                            ui.text(format!("{:?}", node.name));
                                                            ui .slider_float(im_str!("##value"),&mut node.value, 0.0, 1.0)
                                                                .display_format(im_str!("Alpha %.2f"))
                                                                .build();
                                                            ui.color_edit(im_str!("##color"), EditableColor::Float3(&mut node.color))
                                                                .build();
                                                        });

                                                        let node_widgets_active = !any_active && ui.is_item_active();


                                                        any_active = any_active || ui.is_item_active();
                                                        any_hovered = any_hovered || ui.is_item_hovered();
                                                        let size : ImVec2 = {
                                                            let size = ui.get_item_rect_size();
                                                            add(add(ImVec2::new(size.0, size.1), NODE_WINDOW_PADDING), NODE_WINDOW_PADDING)
                                                        };
                                                        node.size = size;
                                                        let node_rect_max = add(node_rect_min, node.size);

                                                        // display node box
                                                        channels.set_current(0); // background
                                                        ui.set_cursor_screen_pos(node_rect_min);
                                                        ui.invisible_button(im_str!("node"), node.size);
                                                        any_active = any_active || ui.is_item_active();
                                                        any_hovered = any_hovered || ui.is_item_hovered();

                                                        if ui.is_item_hovered() {
                                                            node_hovered_in_scene = Some(node.id);
                                                            open_context_menu |= ui.imgui().is_mouse_clicked(ImMouseButton::Right);
                                                        }

                                                        let node_moving_active = ui.is_item_active();
                                                        if  node_widgets_active || node_moving_active {
                                                            selected_node = Some(node.id);
                                                        }
                                                        if node_moving_active && ui.imgui().is_mouse_dragging(ImMouseButton::Left) {
                                                            let delta = ui.imgui().mouse_delta();
                                                            let delta = ImVec2::new(delta.0, delta.1);
                                                            node.pos = add(node.pos, delta);
                                                        }

                                                        let node_bg_color =
                                                            (node_hovered_in_list == Some(node.id)) ||
                                                            (node_hovered_in_scene == Some(node.id) ||
                                                             (node_hovered_in_list == None && selected_node == Some(node.id)));

                                                        let node_bg_color = if node_bg_color {
                                                            ImColor::from((75.0/255.0, 75.0/255.0, 75.0/255.0))
                                                        } else {
                                                            ImColor::from((60.0/255.0, 60.0/255.0, 60.0/255.0))
                                                        };

                                                        draw_list.add_rect_filled_multicolor(
                                                            node_rect_max,
                                                            node_rect_min,
                                                            node_bg_color,
                                                            node_bg_color,
                                                            node_bg_color,
                                                            node_bg_color
                                                        );
                                                        any_active = any_active || ui.is_item_active();
                                                        any_hovered = any_hovered || ui.is_item_hovered();

                                                        draw_list.add_rect(
                                                            node_rect_min,
                                                            node_rect_max,
                                                            ImColor::from((100.0, 100.0, 100.0, 255.0))
                                                        ).build();

                                                        any_active = any_active || ui.is_item_active();
                                                        any_hovered = any_hovered || ui.is_item_hovered();

                                                        for i in 0..node.inputs_count {
                                                            draw_list.add_circle(
                                                                add(offset, node.get_input_slot_pos(i)),
                                                                NODE_SLOT_RADIUS,
                                                                ImColor::from((150.0/255.0, 150.0/255.0, 150.0/255.0, 150.0/255.0))
                                                            ).build();

                                                            any_active = any_active || ui.is_item_active();
                                                            any_hovered = any_hovered || ui.is_item_hovered();
                                                        }

                                                        for i in 0..node.outputs_count {
                                                            draw_list.add_circle(
                                                                add(offset, node.get_output_slot_pos(i)),
                                                                NODE_SLOT_RADIUS,
                                                                ImColor::from((150.0/255.0, 150.0/255.0, 150.0/255.0, 150.0/255.0))
                                                            ).build();

                                                            any_active = any_active || ui.is_item_active();
                                                            any_hovered = any_hovered || ui.is_item_hovered();
                                                        }
                                                    });


                                                }

                                            });
                                            // channels have been merged
                                            if
                                                !any_hovered &&
                                                ui.is_window_hovered() &&
                                                ui.imgui().is_mouse_clicked(ImMouseButton::Right) {
                                                    selected_node = None;
                                                    node_hovered_in_list = None;
                                                    node_hovered_in_list = None;
                                                    open_context_menu = true;
                                                }
                                            if open_context_menu {
                                                ui.open_popup(im_str!("context_menu"));
                                                if node_hovered_in_list.is_some() {
                                                    selected_node = node_hovered_in_list;
                                                }
                                                if node_hovered_in_scene.is_some() {
                                                    selected_node = node_hovered_in_scene;
                                                }
                                            }

                                            ui.with_style_var(StyleVar::WindowPadding(ImVec2::new(8.0, 8.0)), || {
                                                ui.popup(im_str!("context_menu"), || {
                                                    if let Some(ind) = selected_node {
                                                        let node = &nodes[ind];
                                                        ui.text(format!("Node {:?}",node.name));
                                                        ui.separator();
                                                        ui.menu_item(im_str!("Rename..")).build();
                                                        ui.menu_item(im_str!("Delete")).build();
                                                        ui.menu_item(im_str!("Copy")).build();

                                                    } else {
                                                        if ui.menu_item(im_str!("Add")).build() {
                                                            let mouse_pos = ui.imgui().mouse_pos();
                                                            let mouse_pos = ImVec2::new(mouse_pos.0, mouse_pos.1);
                                                            let next_id = nodes.len();
                                                            nodes.push(
                                                                Node {
                                                                    id: next_id,
                                                                    size: ImVec2::new(0.0, 0.0),
                                                                    value: 0.0,
                                                                    name: unsafe { ImString::from_utf8_unchecked(b"New Node".to_vec()) },
                                                                    pos: mouse_pos,
                                                                    color: [100.0/255.0, 100.0/255.0, 200.0/255.0],
                                                                    inputs_count: 2,
                                                                    outputs_count: 2
                                                                }
                                                            )
                                                        }
                                                    }
                                                });
                                            });


                                        });
                                    if ui.is_item_hovered() &&
                                        !any_active &&
                                        ui.imgui().is_mouse_dragging(ImMouseButton::Middle)
                                    {
                                        let delta = ui.imgui().mouse_delta();
                                        let delta = ImVec2::new(delta.0, delta.1);
                                        scrolling = add(scrolling, delta);
                                    }

                                }
                            );

                        });

                });
            });


        // ui.window(im_str!("Color button examples"))
        //     .position((20.0, 20.0), ImGuiCond::Appearing)
        //     .size((700.0, 80.0), ImGuiCond::Appearing)
        //     .resizable(false)
        //     .build(|| {
        //         let ex1 = ui.radio_button(im_str!("Example 1: Basics"), &mut state.example, 1);
        //         let ex2 = ui.radio_button(im_str!("Example 2: Alpha component"), &mut state.example, 2);
        //         if ex1 || ex2 {
        //             state.reset();
        //         }
        //     });

        // match state.example {
        //     1 => example_1(&mut state, ui),
        //     2 => example_2(ui),
        //     _ => (),
        // }


        true
    });
}

// fn hello_world<'a>(ui: &Ui<'a>) -> bool {
//     ui.window(im_str!("Hello world"))
//         .size((300.0, 100.0), ImGuiCond::FirstUseEver)
//         .build(|| {
//             ui.text(im_str!("Hello world!"));
//             ui.text(im_str!("こんにちは世界！"));
//             ui.text(im_str!("This...is...imgui-rs!"));
//             ui.separator();
//             let mouse_pos = ui.imgui().mouse_pos();
//             ui.text(im_str!(
//                 "Mouse Position: ({:.1},{:.1})",
//                 mouse_pos.0,
//                 mouse_pos.1
//             ));
//         });


//     true
// }

// fn example_1(state: &mut State, ui: &Ui) {
//     ui.window(im_str!("Example 1: Basics"))
//         .size((700.0, 300.0), ImGuiCond::Appearing)
//         .position((20.0, 120.0), ImGuiCond::Appearing)
//         .build(|| {
//             ui.text_wrapped(im_str!(
//                 "Color button is a widget that displays a color value as a clickable rectangle. \
//                  It also supports a tooltip with detailed information about the color value. \
//                  Try hovering over and clicking these buttons!"
//             ));
//             ui.text(state.notify_text);

//             ui.text("This button is black:");
//             if ui
//                 .color_button(im_str!("Black color"), (0.0, 0.0, 0.0, 1.0))
//                 .build()
//             {
//                 state.notify_text = "*** Black button was clicked";
//             }

//             ui.text("This button is red:");
//             if ui
//                 .color_button(im_str!("Red color"), (1.0, 0.0, 0.0, 1.0))
//                 .build()
//             {
//                 state.notify_text = "*** Red button was clicked";
//             }

//             ui.text("This button is BIG because it has a custom size:");
//             if ui
//                 .color_button(im_str!("Green color"), (0.0, 1.0, 0.0, 1.0))
//                 .size((100.0, 50.0))
//                 .build()
//             {
//                 state.notify_text = "*** BIG button was clicked";
//             }

//             ui.text("This button doesn't use the tooltip at all:");
//             if ui
//                 .color_button(im_str!("No tooltip"), (0.0, 0.0, 1.0, 1.0))
//                 .tooltip(false)
//                 .build()
//             {
//                 state.notify_text = "*** No tooltip button was clicked";
//             }
//         });
// }

// fn example_2(ui: &Ui) {
//     ui.window(im_str!("Example 2: Alpha component"))
//         .size((700.0, 320.0), ImGuiCond::Appearing)
//         .position((20.0, 140.0), ImGuiCond::Appearing)
//         .build(|| {
//             ui.text_wrapped(im_str!(
//                 "The displayed color is passed to the button as four float values between \
//                  0.0 - 1.0 (RGBA). If you don't care about the alpha component, it can be \
//                  disabled and it won't show up in the tooltip"
//             ));

//             ui.text("This button ignores the alpha component:");
//             ui.color_button(im_str!("Red color"), (1.0, 0.0, 0.0, 0.5))
//                 .alpha(false)
//                 .build();

//             ui.spacing();
//             ui.spacing();
//             ui.spacing();
//             ui.text_wrapped(im_str!(
//                 "If you *do* care about the alpha component, you can choose how it's \
//                  displayed in the button and the tooltip"
//             ));

//             ui.separator();
//             ui.text_wrapped(im_str!(
//                 "ColorPreview::Opaque (default) doesn't show the alpha component at all"
//             ));
//             ui.color_button(im_str!("Red + ColorPreview::Opaque"), (1.0, 0.0, 0.0, 0.5))
//                 .preview(ColorPreview::Opaque)
//                 .build();

//             ui.separator();
//             ui.text_wrapped(im_str!(
//                 "ColorPreview::HalfAlpha divides the color area into two halves and uses a \
//                  checkerboard pattern in one half to illustrate the alpha component"
//             ));
//             ui.color_button(
//                 im_str!("Red + ColorPreview::HalfAlpha"),
//                 (1.0, 0.0, 0.0, 0.5),
//             ).preview(ColorPreview::HalfAlpha)
//                 .build();

//             ui.separator();
//             ui.text_wrapped(im_str!(
//                 "ColorPreview::Alpha uses a checkerboard pattern in the entire color area to \
//                  illustrate the alpha component"
//             ));
//             ui.color_button(im_str!("Red + ColorPreview::Alpha"), (1.0, 0.0, 0.0, 0.5))
//                 .preview(ColorPreview::Alpha)
//                 .build();
//         });
// }
