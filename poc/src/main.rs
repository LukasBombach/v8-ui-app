use deno_core::error::AnyError;
use js_runtime_deno;
use js_runtime_deno::op_sync;
use js_runtime_deno::MainWorker;
// use js_runtime_deno::ModuleSpecifier;
use js_runtime_deno::WorkerOptions;

// use std::path::Path;
// use std::thread;

use std::collections::HashMap;

use winit::event::Event;
// use winit::event::StartCause;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;
// use winit::window::WindowBuilder;
use winit::event::StartCause;
use winit::window::WindowId;

use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use std::rc::Rc;
use std::sync::Arc;

struct WindowManager<'a, T: 'static> {
    event_loop: &'a EventLoopWindowTarget<T>,
    windows: HashMap<WindowId, Window>,
}

impl<T> WindowManager<'_, T> {
    fn new(event_loop: &EventLoopWindowTarget<T>) -> WindowManager<'_, T> {
        WindowManager {
            event_loop,
            windows: HashMap::new(),
        }
    }

    fn add(&mut self) {
        let window = Window::new(self.event_loop).unwrap();
        self.windows.insert(window.id(), window);
    }
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

fn main() {
    let module_loader = Rc::new(FsModuleLoader);

    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });

    let options = WorkerOptions {
        apply_source_maps: false,
        args: vec![],
        debug_flag: false,
        unstable: false,
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        user_agent: "hello_runtime".to_string(),
        seed: None,
        js_error_create_fn: None,
        create_web_worker_cb,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        module_loader,
        runtime_version: "x".to_string(),
        ts_version: "x".to_string(),
        no_color: false,
        get_error_class_fn: Some(&get_error_class_name),
        location: None,
        origin_storage_dir: None,
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        cpu_count: 1,
    };

    let main_module = deno_core::resolve_path("test.js").unwrap();

    let el = EventLoop::new();
    let mut wm = WindowManager::new(&el);
    let mut js_worker = js_runtime_deno::get_main_worker(main_module, &options);

    js_worker.js_runtime.register_op(
        "op_open_window",
        op_sync(|_state, _: (), _: ()| {
            wm.add();
            Ok(())
        }),
    );

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        println!("{:?}", event);

        match event {
            Event::NewEvents(StartCause::Init) => {
                js_runtime_deno::run(js_worker, main_module, &options);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: _,
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
