// Mock for @tauri-apps/plugin-process
// relaunch() is a no-op in prototype

export async function relaunch(): Promise<void> {
  console.debug("[mock:process] relaunch() → no-op in browser");
}
