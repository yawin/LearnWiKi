import { invoke } from "@tauri-apps/api/core";

export async function exportBackup(path: string): Promise<string> {
  return invoke("export_backup", { path });
}

export async function importBackup(
  path: string,
  mode: "replace" | "merge"
): Promise<string> {
  return invoke("import_backup", { path, mode });
}

export async function autoBackup(): Promise<string | null> {
  return invoke("auto_backup");
}
