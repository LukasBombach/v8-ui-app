use std::cell::{RefCell, RefMut};
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

fn main() {
    SimpleLogger::new().init().unwrap();

    let mut runtime = JsRuntime::new(Default::default());

    // let event_loop = EventLoop::new();
    let event_loop = Rc::new(EventLoop::new());

    // let mut windows: HashMap<WindowId, Window> = HashMap::new();
    // let windows_rc = Rc::new(RefCell::new(windows));

    let windows: Rc<RefCell<HashMap<WindowId, Window>>> = Rc::new(RefCell::new(HashMap::new()));

    runtime.register_op(
        "openWindow",
        op_sync(|_state, msg: Option<String>, _: ()| {
            let event_loop_c = Rc::clone(&event_loop);
            let mut windows_c: RefMut<_> = windows.borrow_mut();
            let window = Window::new(&event_loop_c).unwrap();
            windows_c.insert(window.id(), window);
            Ok(())
        }),
    );

    runtime.sync_ops_cache();

    let event_loop_c = Rc::clone(&event_loop);

    event_loop_c.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window {:?} has received the signal to close", window_id);
                    // let mut windows: RefMut<_> = windows.borrow_mut();
                    // windows.remove(&window_id);
                }
                _ => (),
            },
            _ => (),
        }
    })
}
