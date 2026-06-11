// Mock for @tauri-apps/api/window
// Provides getCurrentWindow() with full method set for prototype browsing

export function getCurrentWindow() {
  return {
    // Core window operations
    close: async () => { console.debug("[mock:window] close"); },
    minimize: async () => { console.debug("[mock:window] minimize"); },
    maximize: async () => { console.debug("[mock:window] maximize"); },
    unmaximize: async () => { console.debug("[mock:window] unmaximize"); },
    isMaximized: async () => false,
    show: async () => {},
    hide: async () => {},

    // Position & size
    outerPosition: async () => ({ x: 0, y: 0 }),
    innerPosition: async () => ({ x: 0, y: 0 }),
    outerSize: async () => ({ width: 1200, height: 800 }),
    innerSize: async () => ({ width: 1184, height: 750 }),
    setPosition: async (_pos: { x: number; y: number }) => {},
    setSize: async (_size: { width: number; height: number }) => {},
    scaleFactor: async () => 1.0,

    // Listeners
    onResized: (_cb: (...args: unknown[]) => void) => {
      return () => {};
    },
    onMoved: (_cb: (...args: unknown[]) => void) => {
      return () => {};
    },
    onCloseRequested: (_cb: (...args: unknown[]) => void) => {
      return () => {};
    },
    onFocusChanged: (_cb: (...args: unknown[]) => void) => {
      return () => {};
    },
  };
}
