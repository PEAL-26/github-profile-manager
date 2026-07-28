# Módulo 12 — Distribuição, Build e Atualizações

## 1. Visão Geral da Distribuição

O **GitHub Profile Manager** é empacotado para o sistema operacional Windows utilizando os recursos nativos do **Tauri Bundler**.

```
+-------------------------------------------------------------------+
|                     PIPELINE DE DISTRIBUIÇÃO                      |
+-------------------------------------------------------------------+
                                  │
  ┌───────────────────────────────┴───────────────────────────────┐
  ▼                                                               ▼
[ Instalador NSIS (.exe) ]                      [ Instalador MSI (.msi) ]
- Instalação para o Usuário Atual (per-user)    - Instalação Corporativa (per-machine)
- Instalação Silenciosa (/S)                    - Suporte a GPO / Active Directory
  └───────────────────────────────┬───────────────────────────────┘
                                  │
                                  ▼
+-------------------------------------------------------------------+
|                     CANAL DE RELEASES & UPDATES                   |
|  GitHub Releases + Tauri Auto-Updater (Assinatura Ed25519)        |
+-------------------------------------------------------------------+
```

---

## 2. Formatos de Empacotamento Windows

### 2.1 Instalador NSIS (`github-profile-manager-setup.exe`)
- **Modo Padrão:** Instalação no diretório `%LOCALAPPDATA%\Programs\github-profile-manager`. Não exige elevação de privilégios de Administrador.
- **Integração de Atalhos:** Cria atalhos no Menu Iniciar e adiciona a CLI `ghpm.exe` ao `PATH` do usuário.

### 2.2 Requisitos de Runtime
- **WebView2:** Utiliza o Evergreen WebView2 Runtime pré-instalado nativamente no Windows 10 (atualizado) e Windows 11. Caso ausente, o instalador NSIS efetua o download automático.

---

## 3. Mecanismo de Auto-Atualização (Tauri Updater)

A aplicação inclui verificação periódica em segundo plano por novas versões:

### 3.1 Verificação de Segurança
1. O backend emite uma requisição HTTPS para o manifest de atualização (`latest.json`).
2. O manifest contém o hash SHA-256 e a **assinatura digital Ed25519** do instalador da nova versão.
3. O Tauri valida a assinatura utilizando a chave pública embutida no binário compilado antes de iniciar a extração.

### 3.2 Estrutura do Manifest (`latest.json`)
```json
{
  "version": "2.1.0",
  "notes": "Correções de estabilidade no motor SSH e novo tema escuro.",
  "pub_date": "2026-08-01T10:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXk...",
      "url": "https://github.com/org/repo/releases/download/v2.1.0/github-profile-manager_2.1.0_x64-setup.nsis.zip"
    }
  }
}
```

---

## 4. Pipeline de CI/CD (GitHub Actions)

O workflow automatizado `.github/workflows/release.yml` executa as seguintes etapas a cada nova Tag criada (`v*`):

1. **Lint & Testes:** Roda `cargo test` no Rust e `pnpm test` no Frontend.
2. **Build do Bundle:** Invoca `pnpm tauri build`.
3. **Assinatura de Binário:** Assina o executável com certificado Authenticode Windows.
4. **Geração da Assinatura do Updater:** Assina o bundle gerado com a chave privada Tauri.
5. **Release Publik:** Faz o upload dos instaladores e do arquivo `latest.json` para o GitHub Releases.
