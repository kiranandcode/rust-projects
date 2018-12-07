extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics,
    rotation: f64
}


// TODO: Extract to trait
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x,y) = ((args.width/2) as f64,
                     (args.height/2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            rectangle::Rectangle::new(RED).draw(
                [0.0,0.0, SCREEN_WIDTH as f64, 10.0],
                &c.draw_state, c.transform.trans(0.0,0.0), gl);


            let transform = c.transform.trans(x,y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);

        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }

    fn handle_input(&mut self, args: &Button) {
        println!("{:?}", args);
    }

    fn handle_mouse_press(&mut self, args: &MouseButton) {
        println!("{:?}", args);
    }

    fn handle_mouse(&mut self, args: &[f64;2]) {
        println!("{:?}", args);
    }

}


// TODO: Extract to function
fn main() {
    let opengl = OpenGL::V3_2;

    let mut window_settings = WindowSettings::new("Gop-Game", [SCREEN_WIDTH,SCREEN_HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true);
    window_settings.set_resizable(false);

    let mut window : Window = window_settings.build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }


        if let Some(u) = e.press_args() {
            match u {
                Button::Mouse(mb) => app.handle_mouse_press(&mb),
                _ => app.handle_input(&u)
            }
        }

        if let Some(m) = e.mouse_cursor_args() {
            app.handle_mouse(&m);
        }
    }
}
