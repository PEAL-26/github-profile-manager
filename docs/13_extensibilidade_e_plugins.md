# Módulo 13 — Extensibilidade e Plugins

## 1. Visão Geral do Sistema de Plugins

O **Sistema de Extensibilidade** permite que desenvolvedores e empresas expandam as capacidades do GitHub Profile Manager criando plugins customizados sem modificar o código-fonte principal da aplicação.

```
+-------------------------------------------------------------------+
|                        GHPM PLUGIN SYSTEM                         |
+-------------------------------------------------------------------+
                                  │
  ┌───────────────────────────────┼───────────────────────────────┐
  ▼                               ▼                               ▼
[ Webhooks & Notificações ]     [ Provedores de Git ]           [ Enforcamento de Regras ]
- Envio de alertas ao Slack     - Suporte a GitLab / Bitbucket  - Bloqueio de e-mails
- Integração com Teams          - Servidores Git corporativos   - Políticas de segurança
  └───────────────────────────────┼───────────────────────────────┘
                                  │
                                  ▼
+-------------------------------------------------------------------+
|                      SANDBOXING & PERMISSÕES                      |
|  Isolamento por Permissões Declaradas no `manifest.json`          |
+-------------------------------------------------------------------+
```

---

## 2. Event Hooks (Pontos de Extensão)

Plugins podem se inscrever para ouvir e reagir a eventos do ciclo de vida da aplicação:

| Hook | Momento de Disparo | Payload Fornecido |
| :--- | :--- | :--- |
| `onProfileSwitch` | Imediatamente após a troca de perfil. | `{ oldProfile, newProfile, triggerSource }` |
| `onRepositoryScan` | Quando um novo repositório é detectado. | `{ repositoryPath, remoteUrl }` |
| `onDiagnosticFailure` | Quando o Health Check detecta uma falha. | `{ issueCode, category, message }` |
| `onPreGitCommit` | Antes de efetivar um commit (via Git hook).| `{ repoPath, currentEmail, expectedProfile }` |

---

## 3. Manifesto do Plugin (`plugin.json`)

Todo plugin deve conter um arquivo de manifesto declarando suas metadados e permissões solicitadas:

```json
{
  "id": "com.empresa.slack-notifier",
  "name": "GHPM Slack Notifier",
  "version": "1.0.0",
  "description": "Notifica o canal da equipe no Slack quando um perfil de trabalho for ativado.",
  "author": "Equipe DevOps",
  "entrypoint": "main.js",
  "permissions": [
    "notifications:send",
    "profiles:read",
    "network:fetch"
  ],
  "allowed_hosts": [
    "https://hooks.slack.com/*"
  ]
}
```

---

## 4. Exemplo de Código do Plugin (TypeScript SDK)

```typescript
import { registerPlugin, onProfileSwitch, sendNotification } from '@ghpm/plugin-sdk';

registerPlugin({
  id: 'com.empresa.slack-notifier',
  onInit: () => {
    console.log('Plugin Slack Notifier inicializado.');
    
    onProfileSwitch(async (event) => {
      if (event.newProfile.name === 'Trabalho') {
        await sendNotification({
          title: 'Perfil Alterado',
          body: `Você mudou para o perfil ${event.newProfile.name}`,
        });
      }
    });
  },
});
```

---

## 5. Segurança e Sandboxing

1. **Permissões Granulares:** Plugins não podem acessar a chave privada SSH nem tokens de acesso PAT a menos que uma permissão explícita de alto risco seja concedida pelo usuário.
2. **Execução em Contexto Isolado:** Plugins baseados em JavaScript rodam em um contexto WebView/WebWorker isolado sem acesso direto ao sistema de arquivos do computador, operando estritamente através das APIs mediadas pelo SDK do GHPM.
