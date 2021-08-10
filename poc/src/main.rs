use gui_runtime;
use js_runtime;
use js_runtime::Event;
use std::thread;

fn main() {
    thread::spawn(|| {
        let js_code = include_str!("test.js");
        js_runtime::run(js_code, |event| match event {
            Event::OpenWindow => {
                println!("open window");
            }
        });
    });

    gui_runtime::run();
}
