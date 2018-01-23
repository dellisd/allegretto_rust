extern crate glutin;
pub mod engine {

}

pub struct Engine {
    events_loop: glutin::EventLoop,
    gl_window: glutin::GLWindow,
}

impl Engine {
   pub fn new() -> Engine {
       // Initialize the window
       let window = glutin::WindowBuilder::new()
           .with_title("Hello World!")
           .with_dimensions(800, 600);

       let context = glutin::ContextBuilder::new()
           .with_vsync(true);
       let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

       Engine {
           events_loop: glutin::EventsLoop::new(),
           gl_window
       }
   }

   pub fn do_loop(&self) {
        self.events_loop.poll_events(|event| {
            self.handle_input(event)
       });
   }

    fn handle_input(&self, input: &glutin::Event) {

    }
}