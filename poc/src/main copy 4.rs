use std::{collections::HashMap, sync::mpsc, thread, time::Duration};

use simple_logger::SimpleLogger;
use winit::window::Window;
use winit::window::WindowId;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Position, Size},
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{CursorIcon, Fullscreen, WindowBuilder},
};

const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(600, 400);

fn main() {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new();
    let mut window_senders = HashMap::new();
    let mut windows: HashMap<WindowId, Window> = HashMap::new();

    let window = WindowBuilder::new()
        .with_inner_size(WINDOW_SIZE)
        .build(&event_loop)
        .unwrap();

    let (tx, rx) = mpsc::channel();
    window_senders.insert(window.id(), tx);

    thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            match event {}
        }
    });

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = match !window_senders.is_empty() {
            true => ControlFlow::Wait,
            false => ControlFlow::Exit,
        };
        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::Destroyed
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    window_senders.remove(&window_id);
                }
                _ => {
                    if let Some(tx) = window_senders.get(&window_id) {
                        if let Some(event) = event.to_static() {
                            tx.send(event).unwrap();
                        }
                    }
                }
            },
            _ => (),
        }
    })
}
