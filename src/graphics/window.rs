use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window{
    pub fn new(width: u32, height: u32, title: &str) -> Window{
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
                .create_window(width, height, title, glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|symbol| self.window_handle.get_proc_address(symbol) as *const _);
    }

    pub fn should_close(&mut self) -> bool {
        self.window_handle.should_close()
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.poll_events();
        self.window_handle.swap_buffers();
    }
}