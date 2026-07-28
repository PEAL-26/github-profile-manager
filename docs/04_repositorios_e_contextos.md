# Módulo 04 — Repositórios e Contextos de Projeto

## 1. Visão Geral

O **Repository Manager** permite rastrear, mapear e associar repositórios Git locais a perfis específicos do GitHub Profile Manager. Isso garante que cada repositório utilize a identidade (`user.name`, `user.email`) e a chave SSH corretas, independentemente de qual perfil esteja ativo globalmente.

---

## 2. Modelagem de Dados

### 2.1 Struct Rust (`Repository`)
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub path: String,
    pub profile_id: Option<String>,
    pub current_git_name: Option<String>,
    pub current_git_email: Option<String>,
    pub is_valid_profile: bool,
    pub remote_url: Option<String>,
    pub last_scanned_at: String,
}
```

### 2.2 Schema da Tabela SQL (`repositories`)
```sql
CREATE TABLE repositories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT UNIQUE NOT NULL,
    profile_id TEXT REFERENCES profiles(id) ON DELETE SET NULL,
    remote_url TEXT,
    last_scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

## 3. Descoberta Automática de Repositórios

O sistema oferece um motor de varredura recursiva de arquivos para localizar pastas contendo o subdiretório `.git`.

```
[ Diretório Raiz: C:/dev/ ]
       ├── Projeto-A/ (.git/)  ──►  Detectado como Repositório
       ├── Projeto-B/ (.git/)  ──►  Detectado como Repositório
       └── Documentos/         ──►  Ignorado
```

### 3.1 Algoritmo de Varredura (Rust Tokio Walkdir)
- **Filtro de Profundidade:** Limita a profundidade máxima de varredura (ex: até 4 subníveis) para otimizar I/O de disco.
- **Ignores Padrão:** Descarta automaticamente pastas conhecidas de build/dependências (`node_modules`, `target`, `.venv`, `vendor`, `bin`, `obj`).
- **Extração de Metadata:** Lê o remota Git (`git remote get-url origin`) e o e-mail local configurado no repositório.

---

## 4. Estratégias de Vinculação de Perfil

O GitHub Profile Manager oferece duas estratégias complementares para garantir que o perfil correto seja aplicado ao repositório:

### 4.1 Estratégia 1: Injeção Direta no `.git/config` (Local Config)
O sistema escreve diretamente no arquivo `.git/config` do repositório local:

```ini
[user]
	name = João Silva (Corp)
	email = joao.silva@empresa.com
[core]
	sshCommand = ssh -i C:/Users/Usuario/.ssh/id_ed25519_trabalho
```

- **Vantagem:** Garantia 100% à prova de falhas para aquele repositório individual.
- **Uso Recomendado:** Repositórios isolados em pastas genéricas.

### 4.2 Estratégia 2: Configuração Condicional (`includeIf`)
Para desenvolvedores que organizam projetos em diretórios dedicados (ex: `C:/trabalho/*` vs `C:/pessoal/*`), o sistema gera regras `includeIf` no `~/.gitconfig` global:

```ini
# ~/.gitconfig
[includeIf "gitdir:C:/Projetos/Empresa/"]
    path = C:/Users/Usuario/.gitconfig-empresa

[includeIf "gitdir:C:/Projetos/Pessoal/"]
    path = C:/Users/Usuario/.gitconfig-pessoal
```

- **Vantagem:** Qualquer novo repositório clonado em `C:/Projetos/Empresa/` assume automaticamente o perfil da Empresa sem necessidade de configuração manual.

---

## 5. Validação e Detecção de Conflitos

A cada varredura ou evento de alteração de arquivo, o sistema verifica a integridade da vinculação do repositório:

| Status de Integridade | Condição | Ação da UI |
| :--- | :--- | :--- |
| **VÁLIDO** | O e-mail e SSH configurados no repo batem exatamente com o perfil vinculado. | Badge Verde ("Sincronizado") |
| **DIVERGENTE** | O e-mail no repo pertence a outro perfil ou é o e-mail global genérico. | Badge Amarelo ("Divergente") + Botão "Corrigir" |
| **SEM PERFIL** | O repositório não possui perfil vinculado no banco de dados. | Badge Cinza ("Não Mapeado") + Dropdown de seleção |
