use deno_core::include_js_files;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

use std::env;
use std::vec::Vec;

pub enum Event {
    CreateWindow,
}

pub fn run<F>(code: &str, event_handler: F) -> ()
where
    F: Fn(Event) + 'static,
{
    deno_core::v8_set_flags(env::args().collect());

    let mut extensions = Vec::new();
    extensions.push(
        Extension::builder()
            .js(include_js_files!(
                prefix "gui",
                "src/gui.js",
            ))
            .build(),
    );

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions,
        ..Default::default()
    });

    js_runtime.register_op(
        "op_send_gui_event",
        deno_core::op_sync(move |_state, name: String, _buf: ()| {
            match &name[..] {
                "create_window" => event_handler(Event::CreateWindow),
                _ => println!("unknown event from op_send_gui_event {}", name),
            }
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
