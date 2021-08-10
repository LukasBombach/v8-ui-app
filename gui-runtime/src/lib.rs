use std::collections::HashMap;

use winit::window::Window;
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
};

#[derive(Debug, Clone, Copy)]
pub enum CustomEvent {
    CreateWindow,
}

pub struct WindowManager {
    event_loop: EventLoop<CustomEvent>,
    event_loop_proxy: EventLoopProxy<CustomEvent>,
}

impl WindowManager {
    fn new() -> WindowManager {
        let event_loop = EventLoop::<CustomEvent>::with_user_event();
        let event_loop_proxy = event_loop.create_proxy();

        WindowManager {
            event_loop,
            event_loop_proxy,
        }
    }

    fn run(&mut self) -> ! {
        let mut windows = HashMap::new();

        self.event_loop.run(move |event, event_loop, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::UserEvent(event) => match event {
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
        })
    }
}
/*
pub fn run() {
    let event_loop = EventLoop::<CustomEvent>::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    let mut windows = HashMap::new();

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(event) => match event {
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
    })
} */
