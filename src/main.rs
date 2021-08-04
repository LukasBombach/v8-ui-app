use std::collections::HashMap;
use deno_core::op_sync;
use deno_core::JsRuntime;

fn main() {
  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(Default::default());

  let (_options, file) = parse_args();
  if file.is_empty() {
    panic!("no script was specified");
  }
  let source = std::fs::read_to_string(&file).expect("cannot read file");



  // Register an op for summing a number array.
  runtime.register_op(
    "op_sum",
    // The op-layer automatically deserializes inputs
    // and serializes the returned Result & value
    op_sync(|_state, nums: Vec<f64>, _: ()| {
      // Sum inputs
      let sum = nums.iter().fold(0.0, |a, v| a + v);
      // return as a Result<f64, AnyError>
      Ok(sum)
    }),
  );
  runtime.sync_ops_cache();

  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.
  runtime
    .execute_script("<usage>", source.as_str())
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