use crate::vmath::vector2::Vector2;
use crate::vmath::vector3::Vector3;
use glfw::ffi::glfwGetTime;
use glfw::Context;
use std::process::exit;
use std::ptr::null;

static mut INITIALIZED: bool = false;

pub struct Engine {
    info: SWindowInfo,
    pub win: SWindow,
    pub input_info: SInputInfo,
    other_stuff: SOtherStuff,
    pub dt: f32,
}

pub struct SWindowInfo {
    pub size: Vector2<i32>,
    pub title: String,
}

pub struct SWindow {
    glfw_: glfw::Glfw,
    mouse_mode: glfw::CursorMode,
    pub window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

pub struct SInputInfo {
    pub move_speed: Vector3<f32>,
    pub rot_speed: Vector2<f32>,
    _move_speed: Vector3<f32>,
    _rot_speed: Vector2<f32>,
}

struct SOtherStuff {
    pub timelast: f64,
    pub timenow: f64,
    pub counter: f32,
    pub frame_count: i32,
    pub fps: i32,
}

impl Engine {
    pub fn initialize(wi: SWindowInfo) -> Engine {
        if unsafe { !INITIALIZED } {
            let mut glfw__ = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
            glfw__.window_hint(glfw::WindowHint::ContextVersionMajor(4));
            glfw__.window_hint(glfw::WindowHint::ContextVersionMinor(6));
            glfw__.window_hint(glfw::WindowHint::OpenGlProfile(
                glfw::OpenGlProfileHint::Core,
            ));

            let (mut window, events) = glfw__
                .create_window(
                    wi.size.x as u32,
                    wi.size.y as u32,
                    wi.title.as_str(),
                    glfw::WindowMode::Windowed,
                )
                .expect("Window creation failed");

            window.make_current();

            window.set_key_polling(true);
            window.set_size_polling(true);
            window.set_cursor_pos_polling(true);

            gl::load_with(|s| window.get_proc_address(s));
            unsafe {
                gl::Viewport(0, 0, wi.size.x, wi.size.y);
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Enable(gl::DEPTH_TEST);
                gl::Enable(gl::CULL_FACE);
                gl::CullFace(gl::BACK);
            }

            unsafe { INITIALIZED = true };

            Engine {
                info: wi,
                win: SWindow {
                    glfw_: glfw__,
                    mouse_mode: glfw::CursorMode::Normal,
                    window: window,
                    events: events,
                },
                input_info: SInputInfo {
                    move_speed: Vector3::new(0.0, 0.0, 0.0),
                    rot_speed: Vector2::new(0.0, 0.0),
                    _move_speed: Vector3::new(0.0, 0.0, 0.0),
                    _rot_speed: Vector2::new(0.0, 0.0),
                },
                other_stuff: SOtherStuff {
                    timelast: 0.0,
                    timenow: 0.0,
                    counter: 0.0,
                    fps: 0,
                    frame_count: 0,
                },
                dt: 0.0,
            }
        } else {
            eprintln!("Only one window can be created");
            exit(0);
        }
    }
    //user main loop
    pub fn update(&mut self) {
        self.win.glfw_.poll_events();
        self.win.window.swap_buffers();

        self.other_stuff.timenow = self.win.glfw_.get_time();
        self.dt = (self.other_stuff.timenow - self.other_stuff.timelast) as f32;
        self.other_stuff.counter += self.dt;
        self.other_stuff.timelast = self.other_stuff.timenow;
        self.other_stuff.frame_count += 1;
        if self.other_stuff.counter > 2.0 {
            self.other_stuff.fps =
                (1.0 / (self.other_stuff.counter / (self.other_stuff.frame_count as f32))) as i32;
            self.other_stuff.frame_count = 0;
            self.other_stuff.counter = 0.0;
            self.win.window.set_title(&format!(
                "{} | FPS : {}",
                self.info.title, self.other_stuff.fps
            ));
        }
    }
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn should_close(&self) -> bool {
        self.win.window.should_close()
    }

    //engine main loop
    pub fn run(&mut self) {
        while !self.win.window.should_close() {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            //rendering code
            self.win.glfw_.poll_events();
            self.win.window.swap_buffers();
        }
    }

    pub fn input(&mut self) {
        //input stuff
    }
}
