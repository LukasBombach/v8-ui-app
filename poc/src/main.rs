// use js_runtime_deno;

// use std::path::Path;
// use std::thread;

use std::collections::HashMap;

use winit::event::Event;
// use winit::event::StartCause;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;
// use winit::window::WindowBuilder;
use winit::window::WindowId;

struct WindowManager<'a, T: 'static> {
    event_loop: &'a EventLoopWindowTarget<T>,
    windows: HashMap<WindowId, Window>,
}

impl<T> WindowManager<'_, T> {
    fn new(event_loop: &EventLoopWindowTarget<T>) -> WindowManager<'_, T> {
        WindowManager {
            event_loop,
            windows: HashMap::new(),
        }
    }

    fn add(&mut self) {
        let window = Window::new(self.event_loop).unwrap();
        self.windows.insert(window.id(), window);
    }
}

fn main() {
    let el = EventLoop::new();
    let mut wm = WindowManager::new(&el);

    wm.add();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: _,
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
