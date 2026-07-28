# Módulo 09 — Segurança e Gestão de Credenciais

## 1. Visão Geral do Modelo de Segurança

O **GitHub Profile Manager** lida com informações de alta sensibilidade (chaves privadas SSH, Personal Access Tokens e credenciais OAuth do GitHub). A arquitetura de segurança é baseada nos princípios de **mínimo privilégio**, **isolamento de dados** e **armazenamento cifrado pelo sistema operacional**.

```
+-------------------------------------------------------------------+
|                        MODELO DE SEGURANÇA                        |
+-------------------------------------------------------------------+
                                  │
  ┌───────────────────────────────┴───────────────────────────────┐
  ▼                                                               ▼
[ Cofre de Credenciais (Keyring) ]              [ Proteção de Chaves SSH ]
- Windows Credential Manager                    - Restrição de ACL (icacls)
- DPAPI (Data Protection API)                   - Passphrase opcional
- Tokens PAT / OAuth Cifrados                   - Isolamento em ~/.ssh/
  └───────────────────────────────┬───────────────────────────────┘
                                  │
                                  ▼
+-------------------------------------------------------------------+
|                   AUDITORIA E SANITIZAÇÃO                         |
|                                                                   |
|  - Mascaramento de Tokens em Logs (`ghp_****`)                    |
|  - Limpeza de Memória (Zeroization em Rust)                       |
+-------------------------------------------------------------------+
```

---

## 2. Classificação de Dados e Níveis de Proteção

| Dado | Nível de Sensibilidade | Mecanismo de Armazenamento |
| :--- | :--- | :--- |
| **PAT / OAuth Tokens** | Extrema | Windows Credential Manager (`keyring` crate em Rust) |
| **Chave SSH Privada** | Extrema | Sistema de Arquivos (`~/.ssh/`) com ACL estrita (`icacls`) |
| **Passphrase SSH** | Alta | Armazenada temporariamente na sessão ou `ssh-agent` |
| **Nome / E-mail / Username** | Pública | Banco de Dados SQLite local (`app_data.sqlite`) |

---

## 3. Armazenamento Seguro de Tokens (Windows Credential Manager)

Tokens da API do GitHub **NUNCA** são armazenados em texto puro no banco de dados SQLite nem em arquivos `.json`.

```rust
use keyring::Entry;

pub fn save_github_token(profile_id: &str, token: &str) -> Result<(), String> {
    let service_name = format!("github-profile-manager:{}", profile_id);
    let entry = Entry::new(&service_name, "github_pat").map_err(|e| e.to_string())?;
    entry.set_password(token).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_github_token(profile_id: &str) -> Result<String, String> {
    let service_name = format!("github-profile-manager:{}", profile_id);
    let entry = Entry::new(&service_name, "github_pat").map_err(|e| e.to_string())?;
    entry.get_password().map_err(|e| e.to_string())
}
```

---

## 4. Proteção de Arquivos de Backup (Criptografia AES-256-GCM)

Quando o usuário opta por exportar um backup contendo credenciais ou chaves privadas:

1. É solicitada uma **Senha de Exportação**.
2. Uma chave de 256 bits é derivada utilizando **PBKDF2 com SHA-256** (100.000 iterações) e Salt aleatório de 16 bytes.
3. O payload JSON é cifrado utilizando **AES-256-GCM** com IV (Initialization Vector) único.
4. O arquivo gerado possui o cabeçalho seguro `GHPM_ENC_V2`.

---

## 5. Regras de Sanitização de Logs e Memória

- **Mascara de Tokens:** O sistema aplica expressões regulares sobre todos os eventos de log para substituir instâncias de tokens conhecidos:
  - Tokens GitHub: `ghp_[a-zA-Z0-9]{36}` ➔ `ghp_************************************`
- **Zeroization de Memória:** Estruturas Rust que manipulam senhas e tokens utilizam a trait `zeroize::Zeroize` para limpar os buffers de RAM imediatamente após o envio da requisição HTTP ou escrita no cofre.
