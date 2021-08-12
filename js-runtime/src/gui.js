globalThis.App = {
  windows: {
    create: () => sendEvent("create_window"),
  }

}

function sendEvent(name) {
  Deno.core.opSync("op_send_gui_event", name);
}
