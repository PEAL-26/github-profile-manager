# Módulo 01 — Arquitetura e Visão Geral

## 1. Visão Geral e Problema

### 1.1 Introdução
O **GitHub Profile Manager** é uma aplicação desktop nativa para Windows (10/11) concebida para simplificar o gerenciamento de múltiplas identidades e contas do GitHub no mesmo ambiente de desenvolvimento.

### 1.2 O Problema
Desenvolvedores que atuam em múltiplos contextos (pessoal, trabalho corporativo, projetos freelance ou open-source) enfrentam atrito constante ao alternar contas:
- Necessidade de alterar manualmente variáveis de configuração globais (`user.name`, `user.email`).
- Conflitos de chaves SSH no `~/.ssh/config` ou no `ssh-agent`.
- Erros de autenticação no Git Credential Manager (GCM).
- Risco de realizar commits com a identidade errada em repositórios da empresa ou pessoais.

### 1.3 Solução
Uma aplicação desktop ultra-rápida (Tauri 2.x + Rust) que gerencia perfis de forma isolada, automatizando a configuração SSH, variáveis Git locais/globais e a seleção automática de perfil com base no diretório ativo.

---

## 2. Requisitos da Aplicação

### 2.1 Casos de Uso Principais (UC)
- **UC-001 — Criar/Editar Perfil:** Configurar nome, email, usuário GitHub, avatar e chave SSH vinculada.
- **UC-002 — Alternância Global de Perfil:** Trocar a identidade padrão do ambiente do usuário em 1 clique.
- **UC-003 — Vinculação por Repositório:** Associar um perfil específico a um diretório de repositório Git local via `.git/config` ou `includeIf`.
- **UC-004 — Diagnóstico e Health Check:** Detectar chaves SSH inválidas, `ssh-agent` inativo ou divergências de e-mail e aplicar reparo automático.
- **UC-005 — Automação por Diretório:** Alternar contexto automaticamente quando o terminal ou IDE navega para uma pasta de projeto.
- **UC-006 — Backup e Restauração:** Exportar e importar dados de perfis de forma criptografada.

### 2.2 Requisitos Funcionais (RF)
| ID | Descrição | Prioridade |
| :--- | :--- | :--- |
| **RF001** | Permite cadastrar, listar, editar e excluir perfis de usuário do GitHub. | Alta (MVP) |
| **RF002** | Gerenciar chaves SSH (gerar novas ED25519/RSA ou vincular existentes). | Alta (MVP) |
| **RF003** | Alternar o perfil Git global (`user.name`, `user.email`, `core.sshCommand`). | Alta (MVP) |
| **RF004** | Associar perfis a repositórios locais específicos via `.git/config`. | Alta (MVP) |
| **RF005** | Suportar regreamento avançado com `includeIf` no `.gitconfig`. | Média |
| **RF006** | Executar diagnósticos no ambiente Windows (SSH, Git, Permissões). | Alta (MVP) |
| **RF007** | Auto-reparo de problemas comuns de SSH e Git. | Média |
| **RF008** | Autenticação via GitHub Personal Access Token (PAT) ou OAuth. | Alta (MVP) |
| **RF009** | Armazenar segredos com segurança no Windows Credential Manager. | Alta (MVP) |
| **RF010** | Interface em modo escuro/claro baseada em shadcn/ui. | Média |
| **RF011** | Execução via linha de comando (CLI dedicada `ghpm`). | Média |
| **RF012** | Sistema de logs estruturados e auditoria de ações. | Alta (MVP) |
| **RF013** | Notificações nativas no Windows para trocas de contexto. | Baixa |

### 2.3 Requisitos Não Funcionais (RNF)
| ID | Descrição | Meta / Métrica |
| :--- | :--- | :--- |
| **RNF001** | **Performance:** Tempo de resposta para troca de perfil. | `< 100ms` |
| **RNF002** | **Consumo de Memória:** Footprint de RAM em execução. | `< 80MB RAM` |
| **RNF003** | **Segurança:** Chaves privadas nunca expostas em texto puro em logs. | Criptografia DPAPI / AES-GCM |
| **RNF004** | **Offline First:** Operação completa sem dependência de internet. | 100% funcional offline (exceto sync API) |
| **RNF005** | **Compatibilidade OS:** Suporte nativo. | Windows 10 e Windows 11 (x64 / ARM64) |
| **RNF006** | **Tamanho do Instalador:** Binário leve. | `< 15MB` |

---

## 3. Stack Tecnológica

```
+-------------------------------------------------------------------+
|                        FRONTEND (UI)                              |
|   React 19 + TypeScript + Vite + TailwindCSS + shadcn/ui          |
|   Estado: Zustand | Form: React Hook Form + Zod                  |
+-------------------------------------------------------------------+
                                  |  IPC (Tauri Commands / Events)
+-------------------------------------------------------------------+
|                        BACKEND (CORE)                             |
|   Tauri 2.x (Rust)                                                |
|   - Async Runtime: Tokio                                          |
|   - DB ORM: SQLite + Drizzle ORM (via Tauri plugin/driver)        |
|   - Security: keyring (Windows Credential Manager) / DPAPI        |
|   - System Integration: std::process, tokio::fs, winapi           |
+-------------------------------------------------------------------+
```

### 3.1 Justificativa das Escolhas
- **Tauri 2.x + Rust:** Consumo mínimo de memória e binário compacto comparado ao Electron, aliado à segurança de memória e alta performance do Rust.
- **React 19 + TypeScript:** Ecossistema maduro para criação de interfaces ricas, tipadas e declarativas.
- **SQLite + Drizzle:** Persistência local rápida, relacional e confiável, sem necessidade de servidores externos.
- **shadcn/ui + TailwindCSS:** Design moderno, acessível e totalmente customizável.

---

## 4. Arquitetura do Sistema e Estrutura do Projeto

### 4.1 Arquitetura em Camadas (Clean Architecture)

```
[ Frontend React UI ]
        │
        ▼  (IPC Invocation / JSON DTOs)
[ Tauri Commands Layer ] (src-tauri/src/commands/)
        │
        ▼
[ Application Services Layer ] (src-tauri/src/services/)
        │
        ├──► [ Repository Layer ] (SQLite Drizzle)
        ├──► [ System / OS Layer ] (OpenSSH / Git / Windows Registry)
        └──► [ Credentials Vault ] (Windows Credential Manager)
```

### 4.2 Estrutura de Diretórios do Projeto

```
github-profile-manager/
├── docs/                      # Documentação técnica v2
├── project/                   # Código fonte da aplicação
│   ├── src/                   # Frontend React + TypeScript
│   │   ├── assets/            # Imagens e ícones
│   │   ├── components/        # Componentes visuais UI (shadcn)
│   │   ├── features/          # Módulos da aplicação (Profiles, SSH, Repos)
│   │   ├── hooks/             # Custom React Hooks
│   │   ├── layouts/           # Layouts principais da aplicação
│   │   ├── services/          # Cliente IPC Tauri
│   │   ├── stores/            # Estados globais Zustand
│   │   ├── types/             # Definições de tipos TypeScript
│   │   ├── utils/             # Formatadores e helpers
│   │   ├── App.tsx            # Componente raiz
│   │   └── main.tsx           # Ponto de entrada
│   │
│   ├── src-tauri/             # Backend Tauri + Rust
│   │   ├── src/
│   │   │   ├── commands/      # Tauri Commands #[tauri::command]
│   │   │   ├── services/      # Lógica de negócio em Rust
│   │   │   ├── db/            # Schema SQLite e migrações
│   │   │   ├── utils/         # Helpers de OS, Git e SSH
│   │   │   ├── models/        # Structs Rust e DTOs
│   │   │   ├── error.rs       # Trata erros globais em Rust
│   │   │   └── main.rs        # Ponto de entrada Rust
│   │   ├── Cargo.toml         # Dependências Rust
│   │   └── tauri.conf.json    # Configurações do Tauri
│   ├── package.json           # Dependências Node.js
│   └── vite.config.ts         # Configuração do Bundler Vite
```

---

## 5. Comunicação IPC (Frontend ↔ Backend)

### 5.1 Padrão de Invocação (Commands)
O Frontend solicita ações ao Rust através de `invoke('command_name', { payload })`.
- As mensagens utilizam a serialização **serde (JSON)**.
- Todos os comandos Rust retornam `Result<T, AppError>`, garantindo que falhas sejam tratadas no Frontend através de blocos `try/catch`.

### 5.2 Padrão de Eventos (Events)
Ações assíncronas do backend (como detecção de alteração de pasta ou diagnósticos em segundo plano) emitem eventos via `app_handle.emit("event_name", payload)` que são escutados reativamente no React.
