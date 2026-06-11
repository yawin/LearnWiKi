// Mock for @tauri-apps/plugin-shell
// open() → window.open() in browser prototype

export async function open(url: string): Promise<void> {
  console.debug(`[mock:shell] open("${url}") → window.open`);
  window.open(url, "_blank", "noopener,noreferrer");
}
