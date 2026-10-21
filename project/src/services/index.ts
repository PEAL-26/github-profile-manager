import type {
  ActivityLog,
  BackupInfo,
  CreateProfilePayload,
  ImportReport,
  Profile,
  Settings,
  SshKey,
  SshTestResult,
} from "@/types"
import { invoke } from "@tauri-apps/api/core"

export const profileService = {
  getAll: () => invoke<Profile[]>("get_all_profiles"),
  getById: (id: string) => invoke<Profile>("get_profile_by_id", { id }),
  create: (payload: CreateProfilePayload) => invoke<Profile>("create_profile", { payload }),
  update: (id: string, payload: Partial<CreateProfilePayload>) =>
    invoke<Profile>("update_profile", { id, payload }),
  delete: (id: string) => invoke<void>("delete_profile", { id }),
  setActive: (id: string) => invoke<void>("set_active_profile", { id }),
}

export const sshService = {
  listKeys: () => invoke<SshKey[]>("list_ssh_keys"),
  generateKey: (name: string, keyType: string, passphrase?: string) =>
    invoke<SshKey>("generate_ssh_key", { name, keyType, passphrase }),
  deleteKey: (id: string) => invoke<void>("delete_ssh_key", { id }),
  testConnection: (hostAlias: string) =>
    invoke<SshTestResult>("test_ssh_connection", { hostAlias }),
  readKeyContent: (path: string) => invoke<string>("read_ssh_key_content", { path }),
}

export const settingsService = {
  get: () => invoke<Settings>("get_settings"),
  update: (settings: Settings) => invoke<void>("update_settings", { settings }),
}

export const gitService = {
  getGlobalIdentity: () =>
    invoke<{ name: string; email: string; sshCommand?: string }>("get_global_git_identity"),
  setGlobalIdentity: (name: string, email: string, sshKeyPath?: string) =>
    invoke<void>("set_global_git_identity", { name, email, sshKeyPath }),
}

export const backupService = {
  getInfo: (path: string) => invoke<BackupInfo>("get_backup_info", { path }),
  export: (path: string, includeSecrets: boolean, password?: string) =>
    invoke<number>("export_backup", { path, includeSecrets, password }),
  import: (path: string, password?: string) =>
    invoke<ImportReport>("import_backup", { path, password }),
}

export const logService = {
  getAll: () => invoke<ActivityLog[]>("get_activity_logs"),
  logError: (message: string) => invoke<void>("log_frontend_error", { message }),
}
