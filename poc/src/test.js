setTimeout(() => {
  console.log("new life");
}, 1000);

console.log("new test");

Deno.core.opSync("op_open_window");
