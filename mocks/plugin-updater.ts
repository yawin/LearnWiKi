// Mock for @tauri-apps/plugin-updater
// Returns null (no updates available) in prototype

export interface Update {
  version: string;
  body?: string;
  date?: string;
}

export async function check(): Promise<Update | null> {
  console.debug("[mock:updater] check() → null (no updates)");
  return null;
}
