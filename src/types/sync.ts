export interface SyncFolder {
  id: string;
  path: string;
  enabled: boolean;
  last_synced_at: string | null;
  created_at: string;
}

export interface SyncRecord {
  id: string;
  folder_id: string;
  file_path: string;
  file_name: string;
  file_size: number | null;
  file_mtime: string;
  file_type: string;
  content_id: string | null;
  status: string;
  synced_at: string;
}

export interface SyncResult {
  imported: string[];
  updated: string[];
  skipped: number;
  errors: string[];
}
