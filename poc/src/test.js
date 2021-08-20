import { openWindow, closeWindow } from "./app";

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

const id = openWindow();

if (typeof id === "number") {
  console.log("✅ openWindow returns a number");
} else {
  console.log("❌ returned", id, typeof id);
}
