export interface Profile {
  id: string
  name: string
  githubUsername: string
  email: string
  hostAlias: string
  sshKeyId?: string
  defaultDirectory?: string
  defaultBranch: string
  description?: string
  avatarUrl?: string
  isActive: boolean
  createdAt: string
  updatedAt: string
}

export interface CreateProfilePayload {
  name: string
  githubUsername: string
  email: string
  hostAlias: string
  sshKeyId?: string
  defaultDirectory?: string
  defaultBranch: string
  description?: string
  patToken?: string
}

export interface SshKey {
  id: string
  name: string
  keyType: string
  publicKeyPath: string
  privateKeyPath: string
  fingerprint: string
  hasPassphrase: boolean
  createdAt: string
}

export interface SshTestResult {
  success: boolean
  username?: string
  rawOutput: string
  latencyMs: number
}

export interface Settings {
  id: string
  theme: string
  autoSwitchEnabled: boolean
  notifyOnSwitch: boolean
  defaultSshAlgorithm: string
  autoStartSshAgent: boolean
  scanDepth: number
  logLevel: string
  logRetentionDays: number
}

export interface ActivityLog {
  id: string
  level: string
  category: string
  message: string
  details?: string
  createdAt: string
}

export interface AppError {
  code: string
  message: string
}

export interface BackupInfo {
  encrypted: boolean
  version: string
  exportedAt?: string
  profileCount?: number
}

export interface ImportReport {
  profilesCreated: number
  profilesUpdated: number
  keysRestored: number
  settingsApplied: boolean
  patsRestored: number
}
