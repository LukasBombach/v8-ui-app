use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::tokio_util::create_basic_runtime;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

fn main() {
    thread::spawn(|| {
        let main_module = deno_core::resolve_path("src/test.js").unwrap();
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
        let runtime = create_basic_runtime();
        let mut main_worker =
            MainWorker::from_options(main_module.clone(), Permissions::allow_all(), &options);

        runtime.block_on(async {
            main_worker.bootstrap(&options);
            (main_worker.execute_module(&main_module).await).unwrap();
            (main_worker.run_event_loop(false).await).unwrap();
        });
    });

    let event_loop = EventLoop::new();
    let mut windows = HashMap::new();
    let window = Window::new(&event_loop).unwrap();
    windows.insert(window.id(), window);

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => {
                    windows.remove(&window_id);
                    if windows.is_empty() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    let window = Window::new(&event_loop).unwrap();
                    windows.insert(window.id(), window);
                }
                _ => (),
            },
            _ => (),
        }
    })
}
