use deno_core::op_sync;
use deno_core::JsRuntime;
use std::collections::HashMap;

fn main() {
    let mut runtime = JsRuntime::new(Default::default());

    let (_options, file) = parse_args();

    if file.is_empty() {
        panic!("no script was specified");
    }

    let source = std::fs::read_to_string(&file).expect("cannot read file");

    runtime.register_op(
        "op_sum",
        op_sync(|_state, nums: Vec<f64>, _: ()| {
            let sum = nums.iter().fold(0.0, |a, v| a + v);

            Ok(sum)
        }),
    );

    runtime.sync_ops_cache();
    runtime
        .execute_script(file.as_str(), source.as_str())
        .unwrap();
}

fn parse_args() -> (HashMap<String, String>, String) {
    use std::env;
    let args: Vec<String> = env::args().collect();
    let mut options = HashMap::new();
    let mut file = String::new();

    for arg in &args {
        if let Some(pos) = arg.find('=') {
            let (key, value) = arg.split_at(pos);
            let value = &value[1..];
            options.insert(key.into(), value.into());
        } else {
            file = arg.into();
        }
    }

    (options, file)
}
