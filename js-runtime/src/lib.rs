// use deno_core::error::bad_resource_id;
// use deno_core::error::null_opbuf;
use deno_core::error::AnyError;
// use deno_core::AsyncRefCell;
// use deno_core::CancelHandle;
// use deno_core::CancelTryFuture;
use deno_core::JsRuntime;
use deno_core::OpState;
// use deno_core::RcRef;
// use deno_core::Resource;
// use deno_core::ResourceId;
use deno_core::ZeroCopyBuf;
// use std::cell::RefCell;
// use std::convert::TryFrom;
use std::env;
// use std::io::Error;
// use std::net::SocketAddr;
// use std::rc::Rc;
// use tokio::io::AsyncReadExt;
// use tokio::io::AsyncWriteExt;

pub fn run(code: &str) {
    deno_core::v8_set_flags(env::args().collect());
    let mut js_runtime = create_js_runtime();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let future = async move {
        js_runtime.execute_script("javascript code", code).unwrap();
        js_runtime.run_event_loop(false).await
    };
    runtime.block_on(future).unwrap();
}

fn create_js_runtime() -> JsRuntime {
    let mut runtime = JsRuntime::new(Default::default());
    runtime.register_op("log", deno_core::op_sync(op_log));
    runtime.sync_ops_cache();
    runtime
}

fn op_log(
    _state: &mut OpState,
    message: String,
    _buf: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
    println!("{}", message);
    Ok(())
}
