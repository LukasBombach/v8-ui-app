use js_runtime;
use std::thread;

use std::collections::HashMap;

use winit::event::Event;
use winit::event::StartCause;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    CreateWindow,
}

fn main() {
    let event_loop = EventLoop::<CustomEvent>::with_user_event();
    let mut windows: HashMap<WindowId, Window> = HashMap::new();

    let event_loop_proxy = event_loop.create_proxy();

    let js_thread = thread::spawn(move || {
        thread::park();

        let js_code = include_str!("test.js");

        js_runtime::run(js_code, move |event| match event {
            js_runtime::Event::OpenWindow => {
                event_loop_proxy.send_event(CustomEvent::CreateWindow).ok();
            }
        });
    });

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                js_thread.thread().unpark();
            }
            Event::UserEvent(CustomEvent::CreateWindow) => {
                let window = Window::new(&event_loop).unwrap();
                windows.insert(window.id(), window);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
