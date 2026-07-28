# Módulo 05 — Automação e Motor de Regras

## 1. Visão Geral do Motor de Automação

O **Motor de Automação** permite que o GitHub Profile Manager altere automaticamente a identidade ativa e as configurações do Git/SSH sem intervenção manual do desenvolvedor.

```
+-------------------------------------------------------------------+
|                        GATILHOS (TRIGGERS)                        |
|                                                                   |
|  - Mudança de diretório no Terminal (PowerShell / Git Bash)       |
|  - Foco de Janela do Windows (VS Code / JetBrains / Windows Term)  |
|  - Invocação de comando Git (Git Hook / CLI Wrapper)             |
+-------------------------------------------------------------------+
                                  │
                                  ▼
+-------------------------------------------------------------------+
|                      MOTOR DE REGRAS (RULES)                      |
|                                                                   |
|  Avalia regras ordenadas por prioridade:                         |
|  1. Caminho de Diretório (Glob Pattern)                           |
|  2. Padrão do URL Remote Git (Regex)                              |
|  3. Horário / Contexto do Sistema                                 |
+-------------------------------------------------------------------+
                                  │
                                  ▼
+-------------------------------------------------------------------+
|                        AÇÕES AUTOMÁTICAS                          |
|                                                                   |
|  - Alterna o Perfil Ativo                                         |
|  - Reconfigura SSH Agent & IdentityFile                           |
|  - Notifica o usuário no Windows Toast                            |
+-------------------------------------------------------------------+
```

---

## 2. Tipos de Regras de Automação

### 2.1 Regra por Diretório (Path-Based Rule)
Mapeia caminhos de arquivos ou glob patterns para um perfil.
- **Exemplo:** `D:\Projetos\Cliente-X\**` ➔ Perfil `Consultoria`.

### 2.2 Regra por Remote URL (Remote-Based Rule)
Examina a URL remota do repositório Git ao executar operações.
- **Exemplo:** `git@github.com:empresa-corp/*` ➔ Perfil `Corporativo`.

### 2.3 Regra por Organização GitHub (Org-Based Rule)
Aplica o perfil quando o repositório pertence a uma organização específica do GitHub.

---

## 3. Modelo de Dados e Structs

### 3.1 Struct Rust (`AutomationRule`)
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    DirectoryPath,
    RemotePattern,
    GithubOrg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub pattern: String,
    pub target_profile_id: String,
    pub priority: i32,
    pub is_enabled: bool,
    pub created_at: String,
}
```

### 3.2 Tabela SQL (`automation_rules`)
```sql
CREATE TABLE automation_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rule_type TEXT NOT NULL, -- 'DirectoryPath', 'RemotePattern', 'GithubOrg'
    pattern TEXT NOT NULL,
    target_profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    priority INTEGER DEFAULT 0,
    is_enabled INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

## 4. Integração com Shell e Sistema Windows

### 4.1 Shell Hook (PowerShell)
O GHPM pode instalar uma função no `$PROFILE` do PowerShell que notifica o backend via CLI leve (`ghpm hook-cd "$PWD"`) sempre que a localização do terminal muda:

```powershell
function prompt {
    $currentDir = Get-Location
    if ($global:lastDir -ne $currentDir) {
        $global:lastDir = $currentDir
        ghpm hook-cd "$currentDir" --quiet
    }
    "PS $currentDir> "
}
```

### 4.2 Detector de Foco de Janela (Windows Event Hook)
Um worker em segundo plano no Rust (utilizando APIs do Windows Win32 `SetWinEventHook`) detecta a troca de foco para editores suportados (VS Code, Cursor, IntelliJ) e lê a pasta aberta no projeto para disparar a avaliação de regras.

---

## 5. Resolução de Conflitos e Prioridade de Regras

1. As regras são ordenadas pelo campo `priority` (maior número = maior prioridade).
2. A primeira regra cuja condição for satisfeita é executada.
3. Se nenhuma regra bater, o perfil global padrão permanece inalterado.
4. Para evitar "flicker" de trocas rápidas de pasta, o motor aplica um **debounce de 300ms** antes de efetuar a troca de contexto.
