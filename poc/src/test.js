setTimeout(() => {
  console.log("after 500ms");
  Deno.core.opSync("op_open_window");
}, 2000);

console.log("immediate");
Deno.core.opSync("op_open_window");
