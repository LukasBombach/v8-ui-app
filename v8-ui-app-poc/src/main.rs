use std::{collections::HashMap, thread, time::Duration};

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::Window,
};

use deno_core::{op_sync, JsRuntime};

// use simple_logger::SimpleLogger;

#[derive(Debug, Clone, Copy)]
enum CustomEvent {
  CreateWindow,
}

fn main() {
  // SimpleLogger::new().init().unwrap();

  let event_loop = EventLoop::<CustomEvent>::with_user_event();
  let event_loop_proxy = event_loop.create_proxy();

  let mut windows = HashMap::new();
  let window = Window::new(&event_loop).unwrap();
  windows.insert(window.id(), window);

  thread::spawn(move || {
    thread::sleep(Duration::from_secs(3));
    let mut runtime = JsRuntime::new(Default::default());
    runtime.register_op(
      "openWindows",
      op_sync(move |_state, num_windows: i32, _: ()| {
        let mut n = 0;
        while n < num_windows {
          event_loop_proxy.send_event(CustomEvent::CreateWindow).ok();
          n += 1;
        }
        Ok(())
      }),
    );
    runtime.sync_ops_cache();

    runtime
      .execute_script(
        "test",
        "setTimeout(function () {
        Deno.core.opSync('openWindows', 1);
      }, 0);",
      )
      .unwrap();
  });

  event_loop.run(move |event, event_loop, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::UserEvent(event) => {
        println!("user event: {:?}", event);
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
