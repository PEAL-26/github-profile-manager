# Módulo 07 — Banco de Dados e Persistência

## 1. Visão Geral da Arquitetura de Dados

O **GitHub Profile Manager** utiliza uma base de dados relacional **SQLite** incorporada na aplicação desktop. A manipulação de dados no backend Rust é realizada de forma assíncrona com conexões seguras e índices otimizados para alta performance.

```
+-------------------------------------------------------------------+
|                        BANCO DE DADOS LOCAL                       |
|                                                                   |
|   Localização Windows:                                            |
|   %APPDATA%/github-profile-manager/app_data.sqlite                |
|                                                                   |
|   Tecnologias: SQLite 3 + Drizzle ORM Schema / Rusqlite           |
+-------------------------------------------------------------------+
```

---

## 2. Diagrama Entidade-Relacionamento (DER)

```
  +------------------+         1:N         +----------------------+
  |     PROFILES     | ------------------< |     REPOSITORIES     |
  +------------------+                     +----------------------+
  | id (PK)          |                               | id (PK)            |
  | name             |                               | path               |
  | email            |                               | profile_id (FK)    |
  | host_alias       |                               +----------------------+
  | ssh_key_id (FK)  |
  +------------------+
        |        |
    1:1 |        | 1:N
        v        v
  +-----------+  +--------------------------+
  | SSH_KEYS  |  |  AUTOMATION_RULES        |
  +-----------+  +--------------------------+
  | id (PK)   |  | id (PK)                  |
  | name      |  | target_profile_id (FK)   |
  +-----------+  +--------------------------+
```

---

## 3. Schemas DDL Consolidados (SQL)

### 3.1 Tabela `profiles`
```sql
CREATE TABLE profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    github_username TEXT NOT NULL,
    email TEXT NOT NULL,
    host_alias TEXT UNIQUE NOT NULL,
    ssh_key_id TEXT REFERENCES ssh_keys(id) ON DELETE SET NULL,
    default_directory TEXT,
    description TEXT,
    avatar_url TEXT,
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_profiles_active ON profiles(is_active);
CREATE UNIQUE INDEX idx_profiles_host_alias ON profiles(host_alias);
```

### 3.2 Tabela `ssh_keys`
```sql
CREATE TABLE ssh_keys (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    key_type TEXT NOT NULL, -- 'ed25519' ou 'rsa'
    public_key_path TEXT NOT NULL,
    private_key_path TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    has_passphrase INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### 3.3 Tabela `repositories`
```sql
CREATE TABLE repositories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT UNIQUE NOT NULL,
    profile_id TEXT REFERENCES profiles(id) ON DELETE SET NULL,
    remote_url TEXT,
    last_scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_repositories_path ON repositories(path);
```

### 3.4 Tabela `automation_rules`
```sql
CREATE TABLE automation_rules (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rule_type TEXT NOT NULL,
    pattern TEXT NOT NULL,
    target_profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    priority INTEGER DEFAULT 0,
    is_enabled INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 3.5 Tabela `settings`
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 3.6 Tabela `activity_logs`
```sql
CREATE TABLE activity_logs (
    id TEXT PRIMARY KEY NOT NULL,
    level TEXT NOT NULL, -- 'INFO', 'WARN', 'ERROR'
    category TEXT NOT NULL,
    message TEXT NOT NULL,
    details TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_logs_created_at ON activity_logs(created_at);
```

---

## 4. Gestão de Migrações

- As migrações de banco de dados são versionadas no subdiretório `src-tauri/migrations/`.
- Ao iniciar a aplicação, a função `db::init_and_migrate()` é executada no Rust antes de expor a janela do Tauri.
- O controle de versão do schema utiliza a tabela interna `_prisma_migrations` ou `drizzle_migrations`.
