/**
 * Opens a window
 * @returns {{id: number}} Window A Window object
 */
export function openWindow() {
  return Deno.core.opSync("op_open_window");
}

/**
 * Closes a window
 * @param {number} id A window id
 */
export function closeWindow(id) {
  Deno.core.opSync("op_close_window", id);
}
