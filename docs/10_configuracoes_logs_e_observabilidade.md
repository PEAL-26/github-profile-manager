# Módulo 10 — Configurações, Logs e Observabilidade

## 1. Sistema de Configurações da Aplicação

O **ConfigService** gerencia todas as preferências de comportamento da aplicação, persistidas no banco de dados SQLite (`settings`) em pares chave-valor tipados.

```
+-------------------------------------------------------------------+
|                     CATEGORIAS DE CONFIGURAÇÃO                    |
+-------------------------------------------------------------------+
  │
  ├──► [ Geral & UI ]          - Tema (System / Dark / Light), Idioma
  ├──► [ Automação ]           - Auto-Switch ativo, Debounce (ms), Notificações
  ├──► [ SSH & Git ]           - Algoritmo SSH padrão, Auto-start ssh-agent
  └──► [ Logs & Diagnóstico ]  - Nível de Log (INFO/DEBUG), Retenção (dias)
```

---

## 2. Modelo de Dados de Configurações

### 2.1 Struct Rust (`AppSettings`)
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,                   // "system", "dark", "light"
    pub auto_switch_enabled: bool,       // true / false
    pub notify_on_switch: bool,          // Notificação Windows Toast
    pub default_ssh_algorithm: String,   // "ed25519" / "rsa4096"
    pub auto_start_ssh_agent: bool,      // Tenta iniciar serviço ssh-agent
    pub scan_depth: i32,                 // Profundidade da busca de repos (default: 4)
    pub log_level: String,               // "INFO", "DEBUG", "WARN", "ERROR"
    pub log_retention_days: i32,         // Dias de retenção dos logs (default: 30)
}
```

---

## 3. Arquitetura de Logging e Observabilidade

O backend em Rust utiliza o ecossistema `tracing` para captura estruturada de telemetria, eventos e erros com contextualização assíncrona.

```
[ Rust Code (Services / Commands) ]
              │
              ▼
    [ tracing::info! / warn! / error! ]
              │
    ┌─────────┴─────────┐
    ▼                   ▼
[ Console Output ]   [ File Appender (Daily Rotation) ]
(Dev Debug)          %APPDATA%/github-profile-manager/logs/app-YYYY-MM-DD.log
```

### 3.1 Exemplo de Log Estruturado
```json
{
  "timestamp": "2026-07-27T21:45:00.123Z",
  "level": "INFO",
  "target": "ghpm::services::profile_service",
  "message": "Perfil alterado com sucesso",
  "fields": {
    "profile_id": "prof_123",
    "host_alias": "github.com-trabalho",
    "execution_time_ms": 42
  }
}
```

---

## 4. Auditoria de Operações (Audit Logging)

Ações críticas que modificam o ambiente do sistema operacional são gravadas na tabela SQL `activity_logs` para fins de histórico e auditoria:

```sql
INSERT INTO activity_logs (id, level, category, message, details) 
VALUES (
    'log_98765', 
    'INFO', 
    'PROFILE_SWITCH', 
    'Perfil alterado para Trabalho (dev@empresa.com)',
    '{"old_profile": "Pessoal", "git_scope": "global"}'
);
```

### 4.1 Categorias de Auditoria
- `PROFILE_CREATE` / `PROFILE_DELETE` / `PROFILE_SWITCH`
- `SSH_KEY_GENERATE` / `SSH_KEY_DELETE`
- `REPO_BIND` / `AUTO_FIX_EXECUTE`
- `BACKUP_EXPORT` / `BACKUP_RESTORE`
