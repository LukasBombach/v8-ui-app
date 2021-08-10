use js_runtime;

fn main() {
    let js_code = include_str!("test.js");
    js_runtime::run(js_code);
}
