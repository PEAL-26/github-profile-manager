# Módulo 03 — SSH e Git Manager

## 1. Módulo SSH Manager

O **SSH Manager** lida com a criação, armazenamento, vinculação e teste de chaves OpenSSH no Windows, além da manutenção do arquivo `~/.ssh/config`.

```
+-------------------------------------------------------------------+
|                        SSH MANAGER                                |
|                                                                   |
|   ~/.ssh/                                                         |
|    ├── id_ed25519_pessoal       (Chave Privada Pessoal)           |
|    ├── id_ed25519_pessoal.pub   (Chave Pública Pessoal)           |
|    ├── id_ed25519_trabalho      (Chave Privada Trabalho)          |
|    ├── id_ed25519_trabalho.pub  (Chave Pública Trabalho)          |
|    └── config                   (Arquivo de Aliases SSH)          |
+-------------------------------------------------------------------+
```

### 1.1 Algoritmos de Chave Suportados
1. **ED25519 (Recomendado):** Alta segurança, performance superior e chaves compactas (256-bit).
2. **RSA 4096-bit (Fallback):** Para integração com servidores legados que não suportam ED25519.

### 1.2 Estrutura do `~/.ssh/config`
O sistema gerencia blocos de aliases exclusivos no arquivo `~/.ssh/config` de forma limpa e determinística:

```sshconfig
# GHPM_BEGIN: perfil-pessoal
Host github.com-pessoal
    HostName github.com
    User git
    IdentityFile C:/Users/Usuario/.ssh/id_ed25519_pessoal
    IdentitiesOnly yes
# GHPM_END: perfil-pessoal

# GHPM_BEGIN: perfil-trabalho
Host github.com-trabalho
    HostName github.com
    User git
    IdentityFile C:/Users/Usuario/.ssh/id_ed25519_trabalho
    IdentitiesOnly yes
# GHPM_END: perfil-trabalho
```

### 1.3 Gerenciamento do `ssh-agent` no Windows
- O sistema interage diretamente com o serviço nativo do Windows `ssh-agent` (`ssh-agent.exe`).
- **Verificação de Status:** Confirma se o serviço `ssh-agent` está em execução e configurado com tipo de inicialização "Automático".
- **Carregamento de Chaves:** Invoca `ssh-add.exe` para registrar as chaves privadas ativas sem solicitar senhas a cada operação de push/pull.

---

## 2. Módulo Git Manager

O **Git Manager** é responsável por inspecionar, alterar e garantir a consistência das configurações do Git nos níveis Global, Local e por Include.

```
Níveis de Configuração do Git:
┌─────────────────────────────────────────────────────────┐
│ System Config (C:/Program Files/Git/etc/gitconfig)      │
├─────────────────────────────────────────────────────────┤
│ Global Config (C:/Users/Usuario/.gitconfig)            │
├─────────────────────────────────────────────────────────┤
│ IncludeIf Config (Diretórios específicos por Perfil)    │
├─────────────────────────────────────────────────────────┤
│ Local Config (C:/Projetos/meu-repo/.git/config)        │  ◄── Sobrescreve todas
└─────────────────────────────────────────────────────────┘
```

### 2.1 Alteração de Identidade Global vs Local
- **Troca Global (`git config --global`):**
  ```bash
  git config --global user.name "João Silva"
  git config --global user.email "joao@email.com"
  git config --global core.sshCommand "ssh -i C:/Users/Usuario/.ssh/id_ed25519_pessoal"
  ```
- **Troca Local (`git config --local` dentro de um repositório):**
  ```bash
  git config --local user.name "João Silva (Corp)"
  git config --local user.email "joao.silva@empresa.com"
  git config --local core.sshCommand "ssh -i C:/Users/Usuario/.ssh/id_ed25519_trabalho"
  ```

### 2.2 Assinatura de Commits (SSH Commit Signing)
O Git moderno suporta assinatura de commits diretamente via chave SSH, sem necessidade de instalar GPG.

**Configuração Automática aplicável ao Perfil:**
```bash
git config --global gpg.format ssh
git config --global user.signingkey "C:/Users/Usuario/.ssh/id_ed25519_pessoal.pub"
git config --global commit.gpgsign true
```

---

## 3. Interfaces Rust para Git e SSH

### 3.1 Commands Rust
```rust
#[tauri::command]
pub async fn generate_ssh_key(
    name: String,
    key_type: String,
    passphrase: Option<String>,
) -> Result<SshKeyDto, String> {
    // Executa ssh-keygen com os parâmetros corretos e salva em ~/.ssh/
}

#[tauri::command]
pub async fn test_ssh_connection(host_alias: String) -> Result<SshTestResultDto, String> {
    // Executa `ssh -T -i key_path git@github.com` e analisa o status de retorno
}

#[tauri::command]
pub async fn set_global_git_identity(
    name: String,
    email: String,
    ssh_key_path: Option<String>,
) -> Result<(), String> {
    // Aplica as alterações no .gitconfig global do usuário
}
```

### 3.2 Structs de Resposta (DTOs)
```rust
#[derive(Serialize, Deserialize)]
pub struct SshTestResultDto {
    pub success: bool,
    pub username: Option<String>,
    pub raw_output: String,
    pub latency_ms: u64,
}
```
