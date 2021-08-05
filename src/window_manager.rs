use std::collections::HashMap;

pub use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowId},
};

pub struct WindowManager {
  event_loop: EventLoop<()>,
  windows: HashMap<WindowId, Window>,
}

impl WindowManager {
  pub fn new() -> WindowManager {
    WindowManager {
      event_loop: EventLoop::new(),
      windows: HashMap::new(),
    }
  }

  pub fn add(&mut self) {
    let window = Window::new(&self.event_loop).unwrap();
    self.windows.insert(window.id(), window);
  }

  pub fn remove(&mut self, id: WindowId) {
    self.windows.remove(&id);
  }

  pub fn run_loop<F>(&mut self, event_handler: F) -> !
  where
    F: 'static + FnMut(Event<'_, ()>, &mut WindowManager),
  {
    self.event_loop.run(|event, event_loop, control_flow| {
      *control_flow = ControlFlow::Wait;
    })
  }
}
