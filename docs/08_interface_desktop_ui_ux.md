# Módulo 08 — Interface Desktop, UI/UX e Design System

## 1. Visão Geral da Interface

A interface do **GitHub Profile Manager** foi projetada para oferecer uma experiência limpa, moderna e responsiva. O objetivo principal é permitir a alternância de perfis e resolução de problemas em poucos cliques, com total clareza do estado atual do ambiente Git.

```
+-------------------------------------------------------------------+
|                           LAYOUT DA UI                            |
+-------------------------------------------------------------------+
| [ Header: Perfil Ativo Padrão  | Status SSH | Auto-Switch ON/OFF ] |
+------------------+------------------------------------------------+
| SIDEBAR          | ÁREA DE CONTEÚDO PRINCIPAL (TABS)              |
|                  |                                                |
| 👤 Perfis        | ┌────────────────────────────────────────────┐ |
| 🔑 Chaves SSH    | │ Lista / Grid de Perfis Cadastrados        │ |
| 📁 Repositórios  | │                                            │ |
| ⚡ Automação    | │ [ + Novo Perfil ]   [ Ativar ]  [ Editar ]   │ |
| 🩺 Diagnóstico   | └────────────────────────────────────────────┘ |
| ⚙️ Configurações |                                                |
+------------------+------------------------------------------------+
```

---

## 2. Design System e Componentes UI

### 2.1 Stack Visual
- **React 19 + TypeScript:** Arquitetura reativa baseada em componentes funcionais e hooks.
- **shadcn/ui:** Biblioteca de componentes construída sobre Radix UI acessível.
- **TailwindCSS:** Utilitários de estilização declarativa.
- **Lucide Icons:** Conjunto de ícones vetoriais leves.

### 2.2 Componentes Base do Design System
- `Button` (Primary, Secondary, Destructive, Ghost, Outline).
- `Card` (Para exibição de perfis, repositórios e status de saúde).
- `Badge` (Indicador de status: `Active` [Verde], `Inactive` [Cinza], `Warning` [Amarelo]).
- `Dialog` / `Sheet` (Modais de criação/edição e painéis laterais).
- `Toast` (Sonner / Toast nativo para confirmação de ações).
- `Tooltip` (Explicação de termos técnicos como `includeIf` e `host_alias`).

---

## 3. Arquitetura de Estado Global (Zustand Stores)

O gerenciamento de estado da interface é dividido em stores Zustand focadas e desacopladas:

```typescript
// src/stores/useProfileStore.ts
import { create } from 'zustand';
import { Profile } from '@/types/profile';
import { invoke } from '@tauri-apps/api/core';

interface ProfileState {
  profiles: Profile[];
  activeProfile: Profile | null;
  isLoading: boolean;
  fetchProfiles: () => Promise<void>;
  setActiveProfile: (id: string) => Promise<void>;
}

export const useProfileStore = create<ProfileState>((set, get) => ({
  profiles: [],
  activeProfile: null,
  isLoading: false,

  fetchProfiles: async () => {
    set({ isLoading: true });
    try {
      const profiles = await invoke<Profile[]>('get_all_profiles');
      const active = profiles.find((p) => p.isActive) || null;
      set({ profiles, activeProfile: active });
    } finally {
      set({ isLoading: false });
    }
  },

  setActiveProfile: async (id: string) => {
    await invoke('set_active_profile', { id });
    await get().fetchProfiles();
  },
}));
```

---

## 4. Telas e Fluxos de Interface

### 4.1 Dashboard / Gestão de Perfis
- Exibe o perfil ativado no topo com destaque visual (Avatar, E-mail, Chave SSH associada).
- Botão primário **"Alternar Perfil"** com indicador de carregamento e atalho de teclado (`Ctrl + Shift + P`).

### 4.2 Gerenciador de Chaves SSH
- Tabela com todas as chaves cadastradas em `~/.ssh/`.
- Permite gerar novas chaves ED25519 em 1 clique ou importar chaves existentes.
- Botão **"Testar Conexão com GitHub"** que exibe o resultado do teste SSH em tempo real.

### 4.3 Scanner e Mapeamento de Repositórios
- Visualização em lista/tabela de repositórios locais detectados.
- Dropdown integrado para associar o repositório a um perfil cadastrado.
- Botão **"Efetuar Auto-Mapeamento"** baseado na pasta pai.
