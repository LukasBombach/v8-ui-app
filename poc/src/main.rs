// use gui_runtime;
use js_runtime;
use std::thread;

use std::collections::HashMap;

use winit::event::Event;
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

    // let window = Window::new(&event_loop).unwrap();
    // windows.insert(window.id(), window);

    // `EventLoopProxy` allows you to dispatch custom events to the main Winit event
    // loop from any thread.

    let event_loop_proxy = event_loop.create_proxy();

    thread::spawn(move || {
        let js_code = include_str!("test.js");

        std::thread::sleep(std::time::Duration::from_secs(2));

        js_runtime::run(js_code, move |event| match event {
            js_runtime::Event::OpenWindow => {
                event_loop_proxy.send_event(CustomEvent::CreateWindow).ok();
            }
        });
    });

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(user_event) => match user_event {
                CustomEvent::CreateWindow => {
                    let window = Window::new(&event_loop).unwrap();
                    windows.insert(window.id(), window);
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

    // gui_runtime::run();
}
