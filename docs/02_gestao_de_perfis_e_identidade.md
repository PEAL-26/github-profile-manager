# Módulo 02 — Gestão de Perfis e Identidade GitHub

## 1. Conceito de Perfil (Profile)

O **Perfil** é a unidade fundamental do sistema. Ele encapsula a identidade do desenvolvedor no GitHub, reunindo as configurações do Git, a chave SSH correspondente e as credenciais de acesso à API.

```
+-------------------------------------------------------------------+
|                            PERFIL                                 |
|                                                                   |
|  - Identidade Git:    Nome, E-mail, GPG/SSH Signing Key           |
|  - Conexão SSH:       SSH Key ID, Public/Private Key, Host Alias   |
|  - Credenciais GitHub: PAT / OAuth Token (Cofre Windows)          |
|  - Preferências:      Diretório Padrão, Cor de Destaque, Avatar    |
+-------------------------------------------------------------------+
```

---

## 2. Estrutura de Dados e Tipos

### 2.1 Struct Rust (`Profile`)
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub github_username: String,
    pub email: String,
    pub host_alias: String,
    pub ssh_key_id: Option<String>,
    pub default_directory: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileDto {
    pub name: String,
    pub github_username: String,
    pub email: String,
    pub host_alias: String,
    pub ssh_key_id: Option<String>,
    pub default_directory: Option<String>,
    pub description: Option<String>,
    pub pat_token: Option<String>,
}
```

### 2.2 Interface TypeScript (`Profile`)
```typescript
export interface Profile {
  id: string;
  name: string;
  githubUsername: string;
  email: string;
  hostAlias: string;
  sshKeyId?: string;
  defaultDirectory?: string;
  description?: string;
  avatarUrl?: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateProfilePayload {
  name: string;
  githubUsername: string;
  email: string;
  hostAlias: string;
  sshKeyId?: string;
  defaultDirectory?: string;
  description?: string;
  patToken?: string;
}
```

---

## 3. Fluxo de Vida do Perfil

### 3.1 Criação de Perfil
1. **Coleta de Dados:** O usuário informa Nome, E-mail e Usuário GitHub.
2. **Chave SSH:** O usuário pode selecionar uma chave existente ou solicitar a geração automática de uma chave ED25519 (ex: `id_ed25519_trabalho`).
3. **Host Alias:** O sistema gera um alias exclusivo para o SSH Config (ex: `github.com-trabalho`).
4. **Token GitHub (Opcional):** Inserção de Personal Access Token (PAT) com escopo `read:user`, `repo`, armazenado com segurança no Windows Credential Manager.
5. **Validação:** Verificação de e-mail válido, unicidade do `host_alias` e conexão SSH/API.
6. **Persistência:** Registro no banco SQLite local.

### 3.2 Alternância Ativa de Perfil (Global Switch)
Quando o usuário clica em **"Ativar Perfil"**:
1. O backend desmarca o perfil ativo anterior (`is_active = false`).
2. Define o novo perfil como ativo (`is_active = true`).
3. Atualiza as configurações globais do Git (`git config --global user.name` e `user.email`).
4. Atualiza o `core.sshCommand` global para apontar para a chave do perfil.
5. Registra o evento na tabela `profile_switch_history`.
6. Emite o evento `profile-switched` para atualizar a UI do React.

---

## 4. Integração com GitHub API

### 4.1 Modos de Autenticação
- **Personal Access Token (PAT):** Método principal. O token é inserido na UI e armazenado no cofre do sistema operacional via `keyring`.
- **OAuth 2.0 (PKCE):** Fluxo web nativo utilizando o navegador local para autorização do app GitHub.

### 4.2 Funcionalidades Integradas
- **Validação de Credencial:** Valida o token contra o endpoint `/user` do GitHub.
- **Sincronização de Avatar:** Baixa e atualiza a foto de perfil do usuário.
- **Obtenção de Organizações:** Lista organizações às quais o usuário tem acesso.
- **Publicação de Chave SSH:** Permite enviar automaticamente a chave pública recém-criada para a conta do GitHub via API (`POST /user/keys`).

---

## 5. Backup, Exportação e Migração

### 5.1 Estratégia de Backup
O sistema oferece backup em formato arquivo seguro `.ghpm-backup`.

### 5.2 Estrutura do Arquivo de Backup
```json
{
  "version": "2.0",
  "exported_at": "2026-07-27T21:00:00Z",
  "profiles": [
    {
      "name": "Trabalho",
      "github_username": "dev-corp",
      "email": "dev@empresa.com",
      "host_alias": "github.com-corp",
      "ssh_public_key": "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5..."
    }
  ],
  "settings": { ... }
}
```

### 5.3 Modos de Criptografia do Backup
- **Sem senha (Padrão):** O arquivo contém apenas os dados públicos dos perfis e caminhos de chaves. As chaves privadas e PATs **NÃO** são incluídas por padrão.
- **Criptografado (Completo):** O arquivo inclui chaves privadas e tokens de acesso, sendo totalmente criptografado com **AES-256-GCM** utilizando uma senha forte (PBKDF2) fornecida pelo usuário.
