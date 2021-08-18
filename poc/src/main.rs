use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

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
    let event_loop_proxy = event_loop.create_proxy();

    let windows_hash: HashMap<WindowId, Window> = HashMap::new();
    let windows_arc = Arc::new(RwLock::new(windows_hash));
    let windows_el = Arc::clone(&windows_arc);
    let windows_js = Arc::clone(&windows_arc);

    let (tx, rx): (Sender<Event<CustomEvent>>, Receiver<Event<CustomEvent>>) = mpsc::channel();

    let js_thread = thread::spawn(move || {
        thread::park();

        event_loop_proxy.send_event(CustomEvent::CreateWindow).ok();

        while let Ok(event) = rx.recv() {
            match event {
                Event::UserEvent(CustomEvent::CreateWindow) => {
                    println!("windows {:?}", windows_js.read().unwrap());
                }
                _ => {}
            }
        }
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
                tx.send(Event::UserEvent(CustomEvent::CreateWindow))
                    .unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {
                if let Some(event) = event.to_static() {
                    tx.send(event).unwrap();
                }
            }
        }
    });
}
