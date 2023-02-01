extern crate gl;

use sdl2::event::Event;
use sdl2::video;
use sdl2::{EventPump, Sdl, VideoSubsystem};

pub struct Window {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    window: video::Window,
    event_pump: EventPump,
    gl_context: video::GLContext,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem
            .window("Shader", 500, 500)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        unsafe {
            gl::Viewport(0, 0, 500, 500);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        Self {
            sdl_context,
            video_subsystem,
            window,
            event_pump,
            gl_context,
        }
    }

    pub fn run<F>(self: &mut Self, update: &mut F)
    where
        F: FnMut(),
    {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::Window {
                        timestamp,
                        window_id,
                        win_event,
                    } => match win_event {
                        sdl2::event::WindowEvent::Resized(width, height) => unsafe {
                            gl::Viewport(0, 0, width as i32, height as i32);
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
            update();
            self.window.gl_swap_window();
        }
    }
}
