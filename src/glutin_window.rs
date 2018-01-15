extern crate glutin;
extern crate gl;

use glutin::GlContext;
use std::collections::HashSet;
use input_state::InputState;
use error::*;

pub struct GlutinWindow(pub glutin::EventsLoop, pub glutin::GlWindow);

impl GlutinWindow {
    pub fn api(&self) -> glutin::Api {
        let &GlutinWindow(_, ref gl_window) = self;
        gl_window.get_api()
    }
}

pub fn create() -> Result<GlutinWindow> {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("App").with_dimensions(1024, 576);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop)?;

    unsafe {
        gl_window.make_current()?;
    }

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    Ok(GlutinWindow(events_loop, gl_window))
}

pub fn run<F>(w : &mut GlutinWindow, mut frame_func : F) -> Result<()> where F : FnMut(&InputState, f32) -> Result<()> {

    let &mut GlutinWindow(ref mut events_loop, ref gl_window) = w;

    let mut running = true;
    let mut t : f32 = 0.0;
    let mut input_state = InputState {
        buttons_down: HashSet::new(),
        buttons_down_last_frame: HashSet::new()
    };
    while running {
        input_state.buttons_down_last_frame = input_state.buttons_down.clone();
        events_loop.poll_events(|event| {
            running = handle_events(event, &gl_window, &mut input_state)
        });

        t += 0.05;

        frame_func(&input_state, t)?;

        gl_window.swap_buffers()?;
    }
    Ok(())
}

fn handle_events(event : glutin::Event, gl_window : &glutin::GlWindow, input_state : &mut InputState) -> bool {
    match event {
        glutin::Event::WindowEvent{ event, .. } => match event {
            glutin::WindowEvent::Closed => false,
            glutin::WindowEvent::Refresh => {
                println!("refresh");
                true
            },
            glutin::WindowEvent::Resized(w, h) => {
                gl_window.resize(w, h);
                true
            },
            glutin::WindowEvent::KeyboardInput{ device_id: _, input: inp } => 
                match inp {
                    glutin::KeyboardInput { virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                            scancode: _, state: _, modifiers: _ } =>
                        false,
                    glutin::KeyboardInput { virtual_keycode: _,
                                            scancode: s, state: glutin::ElementState::Pressed, modifiers: _ } => {
                        //println!("pressed: {}", s);
                        input_state.buttons_down.insert(s);
                        true
                    },
                    glutin::KeyboardInput { virtual_keycode: _,
                                            scancode: s, state: glutin::ElementState::Released, modifiers: _ } => {
                        input_state.buttons_down.remove(&s);
                        true
                    }
                },
            _ => true //println!("WindowEvent {:?}", event)
        },
        _ => true //println!("Event {:?}", event)
    }

}


