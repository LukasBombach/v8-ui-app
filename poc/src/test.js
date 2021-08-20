// import { openWindow, closeWindow } from "./app";

// /** @type {number | null} */
// let windowId = null;
//
// setInterval(() => {
//   if (windowId) {
//     closeWindow(windowId);
//   } else {
//     windowId = openWindow();
//   }
// }, 1000);

/**
 * Opens a window
 * @returns {Promise<{id: number}>} Window A Window object
 */
function openWindow() {
  return Deno.core.opAsync("op_open_window");
}

/**
 * Closes a window
 * @param {number} id A window id
 */
function closeWindow(id) {
  Deno.core.opSync("op_close_window", id);
}

async function main() {
  const id = await openWindow();

  if (typeof id === "number") {
    console.log("✅ openWindow returns a number");
  } else {
    console.log("❌ returned", id, typeof id);
  }
}

main();
