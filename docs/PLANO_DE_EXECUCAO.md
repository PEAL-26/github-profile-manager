# Plano de Execução — GitHub Profile Manager MVP v0.1

## Resumo das Decisões Tomadas (Entrevista Grill-Me)

| # | Decisão | Escolha |
|---|---|---|
| 1 | Escopo MVP | Perfis CRUD + SSH Keys + Troca Global + PAT + Cred Manager + UI Básica + Logs |
| 2 | Tooling | pnpm + Vite + Biome + Vitest + cargo test |
| 3 | Database | rusqlite (Rust) + Drizzle (schema TS) + migrações SQL |
| 4 | Autenticação | Apenas PAT (OAuth adiado) |
| 5 | SSH Keys | Subprocess ssh-keygen + parser customizado ~/.ssh/config |
| 6 | Navegação UI | Zustand state-based (sem React Router) |
| 7 | IPC | Commands por domínio + service layer TS |
| 8 | Erros | thiserror enum AppError + toast frontend |
| 9 | Credenciais | crate `keyring` + zeroize |
| 10 | Init projeto | create-tauri-app + shadcn init |
| 11 | Temas | next-themes + Tailwind dark: prefix |
| 12 | Logging | tracing + tracing-appender diário + activity_logs SQLite |

---

## Escopo do MVP v0.1

### Incluir (MVP)
- CRUD de Perfis (RF001)
- Geração e vinculação de chaves SSH (RF002)
- Troca global de perfil (RF003)
- Autenticação PAT do GitHub (RF008)
- Windows Credential Manager (RF009)
- Schema SQLite + Drizzle (Módulo 07)
- UI básica: Dashboard, tela de Chaves SSH
- Logs estruturados com `tracing` (RF012)
- Tema Dark/Light (RF010)

### Adiar (v0.2+)
- Backup/Exportação (Módulo 02.5)
- Sistema de plugins (Módulo 13)
- CLI `ghpm` (Módulo 11)
- Fluxo OAuth 2.0 PKCE
- Integrações com IDEs (VS Code/JetBrains)
- Auto-update / CI/CD (Módulo 12)
- Scanner de repositórios (Módulo 04)
- Motor de regras de automação (Módulo 05)
- Engine de diagnósticos (Módulo 06)
- Estratégias `includeIf`
- Notificações Windows Toast
- Servidor HTTP para plugins

---

## Fases de Desenvolvimento

### Fase 0 — Scaffolding e Infraestrutura

| # | Tarefa | Detalhes |
|---|---|---|
| 0.1 | Criar projeto Tauri 2.x | `pnpm create tauri-app` com React + TypeScript + Vite |
| 0.2 | Configurar Biome | `biome.json` com regras de lint/format |
| 0.3 | Instalar shadcn/ui | `pnpm dlx shadcn@next init` + TailwindCSS v4 |
| 0.4 | Instalar dependências Frontend | `zustand`, `lucide-react`, `framer-motion`, `next-themes`, `@tauri-apps/api` |
| 0.5 | Instalar dependências Rust | `keyring`, `thiserror`, `serde`, `rusqlite`, `tokio`, `tracing`, `tracing-subscriber`, `tracing-appender`, `zeroize` |
| 0.6 | Criar estrutura de pastas | Conforme Módulo 01.2 |
| 0.7 | Configurar Vitest | `vitest.config.ts` + teste smoke |
| 0.8 | Configurar cargo test | Teste smoke no Rust |

### Fase 1 — Database e Modelos

| # | Tarefa | Detalhes |
|---|---|---|
| 1.1 | Criar schema SQL | Tabelas: profiles, ssh_keys, settings, activity_logs |
| 1.2 | Criar migrações SQL | Arquivos .sql em src-tauri/migrations/ |
| 1.3 | Implementar db::init_and_migrate() | Função Rust que executa migrações no startup |
| 1.4 | Criar structs Rust | Profile, SshKey, AppSettings, ActivityLog |
| 1.5 | Criar interfaces TypeScript | Profile, SshKey, AppSettings |
| 1.6 | Implementar Repository Rust | Funções CRUD para cada tabela |

### Fase 2 — Core Backend: SSH + Git Manager

| # | Tarefa | Detalhes |
|---|---|---|
| 2.1 | generate_ssh_key command | Subprocess ssh-keygen, salvar no ~/.ssh/, registrar no SQLite |
| 2.2 | Parser de ~/.ssh/config | Ler/escrever blocos GHPM_BEGIN/GHPM_END |
| 2.3 | test_ssh_connection command | Subprocess ssh -T, parse output |
| 2.4 | list_ssh_keys command | Listar chaves cadastradas no SQLite |
| 2.5 | delete_ssh_key command | Remover chave do ~/.ssh/ e do SQLite |
| 2.6 | set_global_git_identity command | git config --global user.name/email/sshCommand |
| 2.7 | get_global_git_identity command | Ler configurações globais atuais |
| 2.8 | Integração ssh-agent | Verificar serviço Windows, ssh-add |

### Fase 3 — Core Backend: Profile Manager

| # | Tarefa | Detalhes |
|---|---|---|
| 3.1 | create_profile command | Validar dados, gerar host_alias, salvar no SQLite |
| 3.2 | get_all_profiles command | Listar todos os perfis |
| 3.3 | get_profile_by_id command | Buscar perfil por ID |
| 3.4 | update_profile command | Atualizar dados do perfil |
| 3.5 | delete_profile command | Remover perfil e chaves vinculadas |
| 3.6 | set_active_profile command | Alternar perfil ativo com todas as ações |
| 3.7 | Integração GitHub API (PAT) | Validar token, sincronizar avatar |
| 3.8 | Integração keyring | save/get/delete_github_token com zeroize |

### Fase 4 — Frontend: Design System + Layout

| # | Tarefa | Detalhes |
|---|---|---|
| 4.1 | Componentes base shadcn | Button, Card, Badge, Dialog, Toast, Input, Select |
| 4.2 | Layout principal | AppLayout com Sidebar + MainContent |
| 4.3 | Sidebar de navegação | Ícones Lucide + active state |
| 4.4 | Header | Perfil ativo, toggle tema |
| 4.5 | Configuração next-themes | Provider + toggle Dark/Light/System |
| 4.6 | Zustand stores | useProfileStore, useSshKeyStore, useAppStore |
| 4.7 | IPC service layer | profile.service.ts, ssh.service.ts, settings.service.ts |

### Fase 5 — Frontend: Telas de Perfis

| # | Tarefa | Detalhes |
|---|---|---|
| 5.1 | ProfilesView — Lista | Grid/Card de perfis com avatar, nome, email, badge |
| 5.2 | Dialog criar perfil | Formulário React Hook Form + Zod |
| 5.3 | Dialog editar perfil | Mesmo formulário, dados existentes |
| 5.4 | Botão "Ativar Perfil" | Chama setActiveProfile, loading, toast |
| 5.5 | Exclusão de perfil | Dialog de confirmação |
| 5.6 | Header profile status | Perfil ativo atual com avatar |

### Fase 6 — Frontend: Telas de Chaves SSH

| # | Tarefa | Detalhes |
|---|---|---|
| 6.1 | SshKeysView — Tabela | Lista de chaves com nome, tipo, fingerprint |
| 6.2 | Dialog gerar chave | Input nome, select tipo, passphrase opcional |
| 6.3 | Botão "Testar Conexão" | Exibe resultado com latência |
| 6.4 | Botão vincular chave existente | Selecionar chave no ~/.ssh/ |
| 6.5 | Exclusão de chave | Dialog de confirmação |

### Fase 7 — Logging, Auditoria e Configurações

| # | Tarefa | Detalhes |
|---|---|---|
| 7.1 | Setup tracing no Rust | tracing-subscriber + tracing-appender diário |
| 7.2 | Sanitização de logs | Regex ghp_* mascarado |
| 7.3 | Tabela activity_logs | Inserir registros para ações críticas |
| 7.4 | SettingsView | Tela de configurações |
| 7.5 | get_settings / update_settings | CRUD de configurações |

### Fase 8 — Polish, Testes e Hardening

| # | Tarefa | Detalhes |
|---|---|---|
| 8.1 | Testes unitários Rust | Cobrir services e repositories |
| 8.2 | Testes unitários TS | Cobrir stores e services |
| 8.3 | Testes de integração IPC | Testar commands do Tauri |
| 8.4 | UX polish | Loading states, empty states, animações |
| 8.5 | Tratamento de erros global | Error boundary + AppError consistente |
| 8.6 | Validação de inputs | Zod schemas completos |

---

## Dependências entre Fases

```
Fase 0 (Scaffold)
  └──► Fase 1 (DB + Models)
         └──► Fase 2 (SSH + Git Backend)
         │      └──► Fase 3 (Profile Backend)
         │             └──► Fase 5 (UI Perfis)
         └──► Fase 4 (Design System + Layout)
                └──► Fase 5 (UI Perfis)
                └──► Fase 6 (UI SSH Keys)
         └──► Fase 7 (Logging + Config)
                └──► Fase 8 (Polish + Testes)
```

**Caminho crítico:** Fase 0 → 1 → 2 → 3 → 5 → 8

---

## Estimativa Total: ~20-25 dias de desenvolvimento
