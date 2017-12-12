extern crate gl;
extern crate sdl2;

use std::process;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::video::GLProfile;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2 Example", 800, 600)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let ctx = window.gl_create_context().unwrap();
    window.gl_make_current(&ctx).unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let mut events = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(0);
                }, 
                _ => {}
            }
        }
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();

        std::thread::sleep(Duration::from_millis(16));
    };

}
