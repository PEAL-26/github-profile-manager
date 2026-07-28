CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    github_username TEXT NOT NULL,
    email TEXT NOT NULL,
    host_alias TEXT UNIQUE NOT NULL,
    ssh_key_id TEXT,
    default_directory TEXT,
    default_branch TEXT NOT NULL DEFAULT 'main',
    description TEXT,
    avatar_url TEXT,
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_profiles_active ON profiles(is_active);
CREATE UNIQUE INDEX IF NOT EXISTS idx_profiles_host_alias ON profiles(host_alias);

CREATE TABLE IF NOT EXISTS ssh_keys (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    key_type TEXT NOT NULL,
    public_key_path TEXT NOT NULL,
    private_key_path TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    has_passphrase INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS repositories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT UNIQUE NOT NULL,
    profile_id TEXT,
    remote_url TEXT,
    last_scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_repositories_path ON repositories(path);

CREATE TABLE IF NOT EXISTS automation_rules (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rule_type TEXT NOT NULL,
    pattern TEXT NOT NULL,
    target_profile_id TEXT NOT NULL,
    priority INTEGER DEFAULT 0,
    is_enabled INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (target_profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS activity_logs (
    id TEXT PRIMARY KEY NOT NULL,
    level TEXT NOT NULL,
    category TEXT NOT NULL,
    message TEXT NOT NULL,
    details TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_logs_created_at ON activity_logs(created_at);

INSERT OR IGNORE INTO settings (key, value) VALUES
    ('theme', 'system'),
    ('auto_switch_enabled', 'false'),
    ('notify_on_switch', 'true'),
    ('default_ssh_algorithm', 'ed25519'),
    ('auto_start_ssh_agent', 'false'),
    ('scan_depth', '3'),
    ('log_level', 'info'),
    ('log_retention_days', '30');
