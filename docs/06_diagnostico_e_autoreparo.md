# Módulo 06 — Diagnóstico e Auto-Reparo

## 1. Visão Geral do Diagnostics Engine

O **Diagnostics Engine** é o subsistema encarregado de verificar continuadamente a integridade do ambiente de desenvolvimento do usuário (Git, SSH, permissões do Windows e estado dos repositórios) e fornecer estratégias de **Auto-Reparo** em 1 clique.

```
+-------------------------------------------------------------------+
|                        DIAGNOSTICS ENGINE                         |
+-------------------------------------------------------------------+
                                  │
  ┌───────────────────────────────┼───────────────────────────────┐
  ▼                               ▼                               ▼
[ SSH Checks ]            [ Git Environment ]            [ Repository Checks ]
- Service ssh-agent       - Git Binary Version           - Remote URL SSH Alias
- SSH Key Permissions     - Global .gitconfig syntax     - Local Email Config
- OpenSSH Host Alias      - Commit Signing config        - SSH Auth Test (ssh -T)
  └───────────────────────────────┼───────────────────────────────┘
                                  │
                                  ▼
+-------------------------------------------------------------------+
|                   RELATÓRIO DE SAÚDE (HEALTH REPORT)              |
|                                                                   |
|  - Status: PASSED / WARNING / CRITICAL                            |
|  - Ações Recomendadas + Botões de "Auto-Fix"                     |
+-------------------------------------------------------------------+
```

---

## 2. Categorias de Diagnóstico e Verificações

### 2.1 Verificações SSH (SSH Diagnostics)
| ID | Diagnóstico | Sintoma / Causa | Gravidade |
| :--- | :--- | :--- | :--- |
| **CHK-SSH-01** | Status do Serviço `ssh-agent` | Serviço OpenSSH SSH Agent parado ou desabilitado no Windows Services. | Alta |
| **CHK-SSH-02** | Permissões da pasta `~/.ssh` | Arquivos com permissões genéricas de escrita/leitura para outros usuários Windows. | Média |
| **CHK-SSH-03** | Integridade do `~/.ssh/config` | Blocos de aliases corrompidos, malformatados ou apontando para chaves inexistentes. | Alta |
| **CHK-SSH-04** | Autenticação SSH no GitHub | Falha ao executar `ssh -T git@github.com-alias` (Chave não cadastrada no GitHub). | Crítica |

### 2.2 Verificações do Ambiente Git (Git Diagnostics)
| ID | Diagnóstico | Sintoma / Causa | Gravidade |
| :--- | :--- | :--- | :--- |
| **CHK-GIT-01** | Instalação do Git | Git CLI não localizado no `PATH` do sistema. | Crítica |
| **CHK-GIT-02** | Configuração Global de E-mail | E-mail global ausente ou em formato inválido. | Alta |
| **CHK-GIT-03** | Conflito de `core.sshCommand` | Declarações conflitantes de `sshCommand` em múltiplos níveis de `.gitconfig`. | Média |

### 2.3 Verificações de Repositorio (Repo Diagnostics)
| ID | Diagnóstico | Sintoma / Causa | Gravidade |
| :--- | :--- | :--- | :--- |
| **CHK-REP-01** | Alias de Remote | Remote URL utiliza `git@github.com` genérico em vez do host alias gerenciado (`git@github.com-alias`). | Média |
| **CHK-REP-02** | Identidade Divergente | E-mail do repositório não bate com o perfil associado no GHPM. | Média |

---

## 3. Estratégias de Auto-Reparo (Auto-Fix Procedures)

Para cada erro detectado, o motor de diagnóstico registra uma rotina de reparo determinística:

### 3.1 Reparar Serviço `ssh-agent` (Windows)
```rust
// Executa comando via PowerShell Admin / Win32 API
pub fn fix_ssh_agent_service() -> Result<(), String> {
    run_command("powershell", &["-Command", "Set-Service -Name ssh-agent -StartupType Automatic; Start-Service ssh-agent"])
}
```

### 3.2 Corrigir Permissões do Windows (`icacls`)
Garante que apenas o usuário atual tenha acesso de leitura/escrita à chave privada SSH:
```bash
icacls "C:\Users\Usuario\.ssh\id_ed25519_pessoal" /inheritance:r /grant:r "%USERNAME%:F"
```

### 3.3 Reescrever Host Alias do Repositório Git
Atualiza a URL remota do repositório local para utilizar o alias SSH correto:
```bash
git remote set-url origin git@github.com-trabalho:minha-org/meu-repo.git
```

---

## 4. Structs e DTOs de Diagnóstico (Rust)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Ok,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticIssue {
    pub code: String,
    pub category: String, // "SSH", "Git", "Repository", "System"
    pub title: String,
    pub description: String,
    pub status: HealthStatus,
    pub can_autofix: bool,
    pub fix_action_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReportDto {
    pub timestamp: String,
    pub overall_status: HealthStatus,
    pub issues: Vec<DiagnosticIssue>,
}
```
