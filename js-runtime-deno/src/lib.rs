// use deno_core::error::AnyError;
// use deno_core::FsModuleLoader;
// use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
// use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
pub use deno_runtime::worker::MainWorker;
// use std::rc::Rc;
// use std::sync::Arc;

pub use deno_core::op_sync;
pub use deno_core::ModuleSpecifier;
pub use deno_runtime::worker::WorkerOptions;

// pub fn get_main_worker(js_path: &str, options: &WorkerOptions) -> MainWorker {
pub fn get_main_worker(main_module: ModuleSpecifier, options: &WorkerOptions) -> MainWorker {
    // let main_module = deno_core::resolve_path(js_path).unwrap();
    // let permissions = Permissions::allow_all();
    MainWorker::from_options(main_module.clone(), Permissions::allow_all(), options)
}

pub fn run(mut worker: MainWorker, main_module: ModuleSpecifier, options: &WorkerOptions) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let future = async move {
        worker.bootstrap(options);
        let _ = worker.execute_module(&main_module).await;
        let _ = worker.run_event_loop(false).await;
    };

    runtime.block_on(future);
}
