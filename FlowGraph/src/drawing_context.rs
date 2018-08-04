use types::*;
use color::*;
use render_window::RenderWindow;
use std::rc::Rc;
use std::cell::RefCell;

/// Wrapper struct that provides a processing like interface to Cairo's drawing api
pub struct Context<'a>{
    cr: &'a cairo::Context,
    rw: &'a RenderWindow,
    bbox: WorldBoundingBox,
    font: RefCell<Option<cairo::ScaledFont>>,
    is_in_path: RefCell<bool>,
    added_point: RefCell<bool>
}

impl<'a> Context<'a> {
    pub fn new(context: &'a cairo::Context, render_window: &'a RenderWindow) -> Self {
        let (x,y, w, h) = context.clip_extents();
        let x = render_window.screen_to_world_x(ScreenUnit(x));
        let y = render_window.screen_to_world_y(ScreenUnit(y));

        let w = render_window.screen_to_world_distance_x(&ScreenUnit(w));
        let h = render_window.screen_to_world_distance_y(&ScreenUnit(h));


        Context{
            cr: context,
            rw: render_window,
            bbox: WorldBoundingBox(x,y,w,h),
            font: RefCell::new(None),
            is_in_path: RefCell::new(false),
            added_point: RefCell::new(false)
        }
    }

    /// sets the colour used to draw
    pub fn color(&self, color:Color) {
        let alpha = if let Some(alpha) = color.3 { alpha } else { 1.0 };
        self.cr.set_source_rgba(color.0, color.1, color.2, alpha);
    }

    /// sets the stroke width
    pub fn stroke_width(&self, width : f64) {
        self.cr.set_line_width(width);
    }

    /// fills a path
    ///
    /// example use:
    /// ```ignore
    ///
    /// context.begin_shape();
    /// context.vertex(WorldCoords(0.0, 0.0))
    /// context.vertex(WorldCoords(1.0, 0.0))
    /// context.vertex(WorldCoords(1.0, 1.0))
    /// context.vertex(WorldCoords(0.0, 1.0))
    /// context.end_shape();
    ///
    /// context.fill();
    /// ```
    pub fn fill(&self) {
        self.cr.fill();
    }

    /// fills the current path, preserving the context
    pub fn fill_preserve(&self) {
        self.cr.fill_preserve();
    }

    /// fills the entire clipping region
    /// much like processing's background()
    pub fn paint(&self) {
        self.cr.paint();
    }

    pub fn push_matrix(&self) {
        self.cr.save();
    }

    pub fn pop_matrix(&self) {
        self.cr.restore();
    }

    pub fn translate(&self, dx: WorldUnit, dy: WorldUnit) {
        let ScreenUnit(x) = self.rw.world_to_screen_distance_x(&dx);
        let ScreenUnit(y) = self.rw.world_to_screen_distance_y(&dy);

        self.cr.translate(x,y);
    }

    pub fn scale(&self, sx: WorldUnit, sy: WorldUnit) {
        let ScreenUnit(x) = self.rw.world_to_screen_distance_x(&sx);
        let ScreenUnit(y) = self.rw.world_to_screen_distance_y(&sy);

        self.cr.scale(x,y);
    }

    pub fn rotate(&self, angle: f64) {
        self.cr.rotate(angle);
    }




    // paints the current color within the clipping region with a constant alpha value
    pub fn paint_with_alpha(&self, alpha: f64) {
        self.cr.paint_with_alpha(alpha);
    }

    /// adds a stroke around the current path, clearing it
    pub fn stroke(&self) {
        self.cr.stroke();
    }

    /// adds a stroke around the current path, preserving it
    pub fn stroke_preserve(&self) {
        self.cr.stroke_preserve();
    }


    /// sets the font size
    pub fn font_size(&self, size: f64) {
        self.cr.set_font_size(size);
        *self.font.borrow_mut() = Some(self.cr.get_scaled_font());
    }

    /// Immediate text rendering function
    ///
    /// # Note
    /// intended for fast text editing etc. If some text is repeated frequently,
    /// create a text object instead. (TODO: Yet to be implemented)
    pub fn text(&self, coords: WorldCoords, text: &str) {
        let need_update = self.font.borrow().is_none();
        if need_update {
            *self.font.borrow_mut() = Some(self.cr.get_scaled_font());
        }

        let ScreenCoords(ScreenUnit(x), ScreenUnit(y)) = self.rw.world_to_screen_coords(&coords);

        if let &Some(ref font) = &self.font.borrow().as_ref() {
            let (glyphs, clusters) = font.text_to_glyphs(x, y, text);
            self.cr.show_text_glyphs(text, &glyphs, &clusters, cairo::enums::TextClusterFlags::None);
        }
    }


    /// Starts a shape path
    /// Add vertexes with vertex
    /// Complete path with end_shape()
    /// then fill using fill()
    pub fn begin_shape(&self) {

        if *self.is_in_path.borrow_mut() {
            eprintln!("Beginning a new path while in a path - sounds like a bad idea.");
        }

        self.cr.new_path();
        *self.is_in_path.borrow_mut() = true;
        *self.added_point.borrow_mut() = false;
    }

    /// Completes a shape path
    /// Automatically joins the last and start points
    pub fn end_shape(&self) {
        self.cr.close_path();
        *self.is_in_path.borrow_mut() = false;
        *self.added_point.borrow_mut() = false;
    }

    /// draws a line between two points
    pub fn line(&self, start: WorldCoords, end: WorldCoords) {
        self.begin_shape();
        self.vertex(start);
        self.vertex(end);
        self.end_shape();
    }

    /// calculates the area it would take to draw a shape
    pub fn text_bounding_box(&self, text: &str) -> WorldBoundingBox {
        let need_update = self.font.borrow().is_none();
        if need_update {
            *self.font.borrow_mut() = Some(self.cr.get_scaled_font());
        }

        let extents = self.cr.text_extents(text);
        let x = ScreenUnit(extents.x_bearing);
        let y = ScreenUnit(extents.y_bearing);
        let w = ScreenUnit(extents.width);
        let h = ScreenUnit(extents.height);

        let screen_b_box = ScreenBoundingBox(x,y,w,h);
        self.rw.screen_to_world_bounding_box(screen_b_box)
    }

    /// Adds an arc to the current path
    pub fn arc(&self, center: WorldCoords, radius: WorldX, from_angle: f64, to_angle: f64) {
        let ScreenCoords(ScreenUnit(x), ScreenUnit(y))= self.rw.world_to_screen_coords(&center);
        let ScreenUnit(radius) = self.rw.world_to_screen_x(radius);

        self.cr.arc(x, y, radius, from_angle, to_angle);
    }

    /// Adds a vertex to the current shape
    pub fn vertex(&self, coords: WorldCoords) {
        {
            if !*self.is_in_path.borrow() {
                eprintln!("Drawing vertex without starting a path. Usually a bad idea");
            }
        }
        *self.is_in_path.borrow_mut() = true;
        let ScreenCoords(ScreenUnit(x), ScreenUnit(y))= self.rw.world_to_screen_coords(&coords);
        {
            let added_point = self.added_point.borrow();
            if *added_point {
                self.cr.line_to(x,y);
            } else {
                self.cr.move_to(x,y);
            }
        }
        *self.added_point.borrow_mut() = true;
    }

    pub fn curve(&self, start_point: WorldCoords, start_control: WorldCoords, end_control: WorldCoords, end_point: WorldCoords) {
        let ScreenCoords(s_px, s_py) = self.rw.world_to_screen_coords(&start_point);
        let ScreenCoords(s_cx, s_cy) = self.rw.world_to_screen_coords(&start_control);
        let ScreenCoords(e_px, e_py) = self.rw.world_to_screen_coords(&end_point);
        let ScreenCoords(e_cx, e_cy) = self.rw.world_to_screen_coords(&end_control);
        let in_path = *self.is_in_path.borrow();
        self.cr.move_to(s_px.0, s_py.0);
        self.cr.curve_to(s_cx.0, s_cy.0, e_cx.0, e_cy.0, e_px.0, e_py.0);
        if !in_path {
            self.cr.close_path();
        }
    }


    /// draws a rectangle
    pub fn rect(&self, rect: WorldBoundingBox) {
        if self.rw.is_bounding_box_onscreen(&rect) {
            let screen_box = self.rw.world_to_screen_bounding_box(&rect);
            self.cr.rectangle(
                (screen_box.0).0,
                (screen_box.1).0,
                (screen_box.2).0,
                (screen_box.3).0);
        }
    }


    // draws an ellipse
    pub fn ellipse(&self, rect: WorldBoundingBox) {
        if WorldBoundingBox::check_intersect(&self.bbox, &rect) {
            self.push_matrix();
            self.translate(rect.0 + rect.2 / WorldUnit(2.0), rect.1 + rect.3/WorldUnit(2.0));
            self.scale(rect.2/WorldUnit(2.0), rect.3 / WorldUnit(2.0));
            self.arc(WorldCoords(WorldUnit(0.0), WorldUnit(0.0)), WorldUnit(1.0), 0.0, 2.0 * std::f64::consts::PI);
            self.pop_matrix();
        }
    }
}
