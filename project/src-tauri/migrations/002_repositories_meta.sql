-- Módulo 04: pastas raiz para o Scanner de Repositórios.
-- As colunas extras de `repositories` (metadados Git, sugestão de perfil)
-- são adicionadas em Rust via PRAGMA table_info (SQLite não suporta
-- ADD COLUMN IF NOT EXISTS) — ver db/mod.rs::ensure_column.
CREATE TABLE IF NOT EXISTS scan_roots (
    id TEXT PRIMARY KEY NOT NULL,
    path TEXT UNIQUE NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
