export function openWindow() {
  Deno.core.opSync("op_open_window");
}
