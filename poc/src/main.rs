use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::{thread, time};

use winit::event::Event;
use winit::event::StartCause;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

enum CustomEvent {
    CreateWindow,
}

fn main() {
    let event_loop = EventLoop::<CustomEvent>::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    let windows_hash: HashMap<WindowId, Window> = HashMap::new();
    let windows_arc = Arc::new(RwLock::new(windows_hash));
    let windows_el = Arc::clone(&windows_arc);
    let windows_js = Arc::clone(&windows_arc);

    let js_thread = thread::spawn(move || {
        thread::park();
        println!("windows {:?}", windows_js.read().unwrap());
        event_loop_proxy.send_event(CustomEvent::CreateWindow).ok();
        thread::sleep(time::Duration::from_secs(1));
        println!("windows {:?}", windows_js.read().unwrap());
    });

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                js_thread.thread().unpark();
            }
            Event::UserEvent(CustomEvent::CreateWindow) => {
                let window = Window::new(&event_loop).unwrap();
                windows_el.write().unwrap().insert(window.id(), window);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
