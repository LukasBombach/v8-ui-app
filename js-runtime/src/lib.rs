use deno_core::JsRuntime;

use std::env;

pub enum Event {
    OpenWindow,
}

pub fn run<F>(code: &str, event_handler: F) -> ()
where
    F: Fn(Event) + 'static,
{
    deno_core::v8_set_flags(env::args().collect());

    let mut js_runtime = JsRuntime::new(Default::default());
    js_runtime.register_op(
        "openWindow",
        deno_core::op_sync(move |_state, _params: (), _buf: ()| {
            event_handler(Event::OpenWindow);
            Ok(())
        }),
    );

    js_runtime.sync_ops_cache();

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
