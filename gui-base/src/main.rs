extern crate sdl2;
extern crate gl;

pub mod game;
pub mod shader;

use sdl2::event::Event;



fn main() {
    println!("Hello, world!");
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(4,5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl_ = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    let mut event_pump = sdl.event_pump().unwrap();


    unsafe {
        gl::Viewport(0,0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    'main:
    loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => continue
            }
        }

        window.gl_swap_window();

    }
}
