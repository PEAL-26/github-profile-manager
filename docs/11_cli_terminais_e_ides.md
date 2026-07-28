# Módulo 11 — CLI, Terminais e Integração com IDEs

## 1. Interface de Linha de Comando (`ghpm` CLI)

Além da interface gráfica desktop, o sistema disponibiliza a CLI **`ghpm`**, permitindo que desenvolvedores alternem perfis e executem diagnósticos diretamente pelo terminal.

```
+-------------------------------------------------------------------+
|                           CLI `ghpm`                              |
+-------------------------------------------------------------------+
                                  │
  ┌───────────────────────────────┼───────────────────────────────┐
  ▼                               ▼                               ▼
[ ghpm list ]             [ ghpm switch <id> ]            [ ghpm doctor ]
Lista todos os perfis     Alterna o perfil ativo          Executa diagnósticos
  └───────────────────────────────┼───────────────────────────────┘
                                  │
                                  ▼
+-------------------------------------------------------------------+
|                        COMUNICAÇÃO COM CORE                       |
|  Comunica-se via IPC / Named Pipes com o serviço GUI ou direto DB |
+-------------------------------------------------------------------+
```

### 1.1 Comandos da CLI (`ghpm`)
```bash
# Listar todos os perfis cadastrados
ghpm list

# Exibir perfil atualmente ativo
ghpm current

# Alternar para um perfil específico por nome ou alias
ghpm switch "Trabalho"
ghpm switch --alias "github.com-trabalho"

# Executar diagnóstico do ambiente
ghpm doctor

# Notificar mudança de diretório (usado em hooks de shell)
ghpm hook-cd "C:/Projetos/Cliente-X"
```

---

## 2. Integração com Terminais e Shells Windows

```
+-------------------------------------------------------------------+
|                     INTEGRAÇÃO COM TERMINAIS                      |
+-------------------------------------------------------------------+
  ├──► PowerShell 7+    ($PROFILE script integration)
  ├──► Git Bash (MSYS2) (~/.bashrc alias e prompt function)
  ├──► Command Prompt   (Doskey / Registry AutoRun)
  └──► WSL / WSL2       (Interop Windows -> WSL via ghpm.exe)
```

### 2.1 Integração com PowerShell (`$PROFILE`)
Adiciona uma função decoradora que atualiza a indicação do perfil ativo no prompt:
```powershell
function prompt {
    $activeProfile = ghpm current --short 2>$null
    $prefix = if ($activeProfile) { "[$activeProfile] " } else { "" }
    Write-Host $prefix -NoNewline -ForegroundColor Cyan
    "PS $PWD> "
}
```

### 2.2 Integração com WSL / WSL2
No ambiente WSL (Linux dentro do Windows), o executável do Windows pode ser chamado diretamente via interoperabilidade:
```bash
alias ghpm="/mnt/c/Program\ Files/GitHub\ Profile\ Manager/ghpm.exe"
```

---

## 3. Integração com IDEs (VS Code e JetBrains)

### 3.1 Extensão VS Code / Cursor
- **Item de Status Bar:** Exibe no canto inferior da janela do VS Code o nome e o avatar do perfil ativo para a pasta atual (ex: `👤 GHPM: Trabalho`).
- **Command Palette:** Adiciona comandos rápidos:
  - `GitHub Profile Manager: Switch Active Profile`
  - `GitHub Profile Manager: Run Environment Check`

### 3.2 Servidor HTTP/WebSocket Interno para Plugins
Para comunicação com extensões do VS Code e plugins JetBrains (IntelliJ, WebStorm), a aplicação desktop abre um servidor HTTP seguro apenas na interface local (`127.0.0.1:45731`):

- `GET http://127.0.0.1:45731/v1/current-profile` ➔ Retorna JSON com o perfil ativo.
- `POST http://127.0.0.1:45731/v1/switch` ➔ Altera o perfil a partir da IDE.
