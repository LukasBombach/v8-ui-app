type OpSync = ((opName: "op_create_window") => number) &
  ((opName: "op_remove_window") => void);

interface Deno {
  core: {
    opSync: OpSync;
  };
}

declare var Deno: Deno;

type Size = { width: number; height: number };
type Position = { x: number; y: number };

interface Events {
  resize: (size: Size) => void;
  move: (position: Position) => void;
}

type EventHandler = <N extends keyof Events>(
  eventName: N,
  listener: Events[N]
) => void;

interface AppWindow {
  readonly id: number;
  getSize: () => Size;
  getPosition: () => Position;
  on: EventHandler;
  off: EventHandler;
}

export function createWindow(): AppWindow {
  const id = Deno.core.opSync("op_create_window");
  const getSize = () => getSizeById(id);
  const getPosition = () => getPositionById(id);
  const on: EventHandler = (n, l) => addEventListener(id, n, l);
  const off: EventHandler = (n, l) => removeEventListener(id, n, l);
  return { id, getSize, getPosition, on, off };
}

export function removeWindow(id: number): void {
  Deno.core.opSync("op_remove_window");
}

function getSizeById(id: number): Size {
  return { width: 0, height: 0 };
}

function getPositionById(id: number): Position {
  return { x: 0, y: 0 };
}

function addEventListener<N extends keyof Events>(
  id: number,
  eventName: N,
  listener: Events[N]
) {}

function removeEventListener<N extends keyof Events>(
  id: number,
  eventName: N,
  listener: Events[N]
) {}
