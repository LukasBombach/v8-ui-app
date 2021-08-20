use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use winit::event::Event;
use winit::event::StartCause;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use deno_core::error::AnyError;
use deno_core::op_async;
use deno_core::op_sync;
use deno_core::FsModuleLoader;
use deno_core::OpState;
// use deno_core::ResourceId;
// use deno_core::ZeroCopyBuf;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::tokio_util::create_basic_runtime;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;

use serde::Serialize;

#[derive(Debug)]
enum CustomEvent {
    RequestCreateWindow,
    WindowCreated, /* (Window) */
}

async fn op_open_window(
    op_state: Rc<RefCell<OpState>>,
    _: (),
    _: (),
) -> Result<() /* Window */, AnyError> {
    let mut op_state = op_state.borrow_mut();
    let event_loop_proxy = op_state.take::<EventLoopProxy<CustomEvent>>();
    event_loop_proxy
        .send_event(CustomEvent::RequestCreateWindow)
        .ok();
    Ok(())
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
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

        let main_module = deno_core::resolve_path("src/test.js").unwrap();
        let permissions = Permissions::allow_all();

        let mut worker = MainWorker::from_options(main_module.clone(), permissions, &options);
        let tokio_runtime = create_basic_runtime();

        worker
            .js_runtime
            .op_state()
            .borrow_mut()
            .put::<EventLoopProxy<CustomEvent>>(event_loop_proxy);

        worker
            .js_runtime
            .register_op("op_open_window", op_async(op_open_window));

        worker.js_runtime.sync_ops_cache();

        thread::park();

        tokio_runtime.block_on(async {
            worker.bootstrap(&options);
            (worker.execute_module(&main_module).await).unwrap();
            (worker.run_event_loop(false).await).unwrap();
        });

        /*
        while let Ok(event) = rx.recv() {
            match event {
                Event::UserEvent(CustomEvent::CreateWindow) => {
                    println!("windows {:?}", windows_js.read().unwrap());
                }
                _ => {}
            }
        } */
    });

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                js_thread.thread().unpark();
            }
            Event::UserEvent(CustomEvent::RequestCreateWindow) => {
                let window = Window::new(&event_loop).unwrap();
                windows_el.write().unwrap().insert(window.id(), window);
                tx.send(Event::UserEvent(
                    CustomEvent::WindowCreated, /* (window) */
                ))
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
