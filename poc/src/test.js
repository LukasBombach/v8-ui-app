App.windows.create();

globalThis.__bootstrap.timers.setTimeout(() => {
  App.windows.create();
}, 2000)