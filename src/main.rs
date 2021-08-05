mod window_manager;

use deno_core::op_sync;
use deno_core::JsRuntime;
use simple_logger::SimpleLogger;
use window_manager::{Event, WindowEvent, WindowManager};

fn main() {
    SimpleLogger::new().init().unwrap();

    let mut runtime = JsRuntime::new(Default::default());
    let mut window_manager = WindowManager::new();

    // runtime.register_op(
    //     "openWindow",
    //     op_sync(|_state, msg: Option<String>, _: ()| {
    //         window_manager.add();
    //         Ok(())
    //     }),
    // );

    runtime.sync_ops_cache();

    window_manager.run_loop(move |event, window_manager| {
        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window {:?} has received the signal to close", window_id);
                    // let mut windows: RefMut<_> = windows.borrow_mut();
                    window_manager.remove(window_id);
                }
                _ => (),
            },
            _ => (),
        }
    })
}
