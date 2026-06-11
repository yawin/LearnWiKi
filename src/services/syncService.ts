import { invoke } from "@tauri-apps/api/core";
import type { SyncFolder, SyncResult } from "../types/sync";

export async function addSyncFolder(path: string): Promise<SyncFolder> {
  return invoke("add_sync_folder", { path });
}

export async function removeSyncFolder(id: string): Promise<void> {
  return invoke("remove_sync_folder", { id });
}

export async function getSyncFolders(): Promise<SyncFolder[]> {
  return invoke("get_sync_folders");
}

export async function updateSyncFolder(id: string, enabled: boolean): Promise<void> {
  return invoke("update_sync_folder", { id, enabled });
}

export async function startSync(folderId?: string): Promise<SyncResult> {
  return invoke("start_sync", { folderId: folderId ?? null });
}
