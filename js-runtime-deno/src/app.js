export function createWindow() {
  const id = Deno.core.opSync("op_create_window");
  const getSize = () => getSizeById(id);
  const getPosition = () => getPositionById(id);
  const on = (n, l) => addEventListener(id, n, l);
  const off = (n, l) => removeEventListener(id, n, l);
  return { id, getSize, getPosition, on, off };
}

export function removeWindow(id) {
  Deno.core.opSync("op_remove_window");
}

function getSizeById(id) {
  return { width: 0, height: 0 };
}

function getPositionById(id) {
  return { x: 0, y: 0 };
}

function addEventListener(id, eventName, listener) {}

function removeEventListener(id, eventName, listener) {}
