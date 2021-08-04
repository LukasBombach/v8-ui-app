use std::collections::HashMap;
use std::rc::Rc;

use deno_core::op_sync;
use deno_core::JsRuntime;

use simple_logger::SimpleLogger;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};

/* struct WindowManager {
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
        F: 'static + FnMut(Event<'_, ()>, &mut ControlFlow, &mut WindowManager),
    {
        self.event_loop
            .run(|event, event_loop, control_flow| event_handler(event, control_flow, self))
    }
} */

fn main() {
    SimpleLogger::new().init().unwrap();

    // let mut window_manager = WindowManager::new();
    let mut runtime = JsRuntime::new(Default::default());
    let event_loop = EventLoop::new();
    let mut windows: HashMap<WindowId, Window> = HashMap::new();

    /* runtime.register_op(
        "openWindow",
        op_sync(|_state, msg: Option<String>, _: ()| {
            window_manager.add();
            Ok(())
        }),
    ); */

    runtime.sync_ops_cache();

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window {:?} has received the signal to close", window_id);
                    windows.remove(&window_id);
                }
                _ => (),
            },
            _ => (),
        }
    })
}
