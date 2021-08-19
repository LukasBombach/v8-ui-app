import { openWindow } from "./app";

setTimeout(() => {
  console.log("after 500ms");
  openWindow();
}, 2000);

console.log("immediate");
openWindow();
