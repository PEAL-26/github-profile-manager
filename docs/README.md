# GitHub Profile Manager — Especificação Técnica v2

Bem-vindo à **Especificação Técnica Unificada (v2)** do **GitHub Profile Manager**.

Esta versão substitui e consolida os 43 documentos fragmentados originais em **13 módulos temáticos explícitos, abrangentes e totalmente sem redundâncias**.

---

## 📌 Índice da Documentação (v2)

| Módulo | Documento | Descrição Téinca | Partes Originais Consolidadas |
| :--- | :--- | :--- | :--- |
| **01** | [`01_arquitetura_e_visao_geral.md`](./01_arquitetura_e_visao_geral.md) | Visão geral do sistema, objetivos, escopo, requisitos (RF/RNF), stack tecnológica e arquitetura Tauri 2.x + Rust + React. | Partes 01, 02, 10, 20 |
| **02** | [`02_gestao_de_perfis_e_identidade.md`](./02_gestao_de_perfis_e_identidade.md) | Gestão de perfis GitHub, autenticação (OAuth/PAT), integração com API GitHub, gestão de contas, backup e migração. | Partes 03, 11, 21, 31, 37, 38 |
| **03** | [`03_ssh_e_git_manager.md`](./03_ssh_e_git_manager.md) | Gestão de chaves SSH (geração, ssh-agent, config), identidades Git (`user.name`, `user.email`) e assinatura de commits (GPG/SSH). | Partes 04, 05, 13, 14 |
| **04** | [`04_repositorios_e_contextos.md`](./04_repositorios_e_contextos.md) | Descoberta automática de repositórios, vinculação de perfis por projeto e estratégias de isolamento (`includeIf`). | Partes 06, 15, 22, 39 |
| **05** | [`05_automacao_e_motor_de_regras.md`](./05_automacao_e_motor_de_regras.md) | Motor de automação inteligente, regras de troca de contexto, triggers de diretório/foco e execução remota de hooks. | Partes 12, 23, 30, 40 |
| **06** | [`06_diagnostico_e_autoreparo.md`](./06_diagnostico_e_autoreparo.md) | Engine de diagnóstico (Health Check), verificação de integridade do ambiente Git/SSH e estratégias de reparo automático. | Partes 07, 16, 24, 35 |
| **07** | [`07_banco_de_dados_e_persistencia.md`](./07_banco_de_dados_e_persistencia.md) | Arquitetura de dados SQLite + Drizzle ORM, schemas relacionais consolidados, migrações e estratégias de persistência. | Partes 08, 17, 25 |
| **08** | [`08_interface_desktop_ui_ux.md`](./08_interface_desktop_ui_ux.md) | Interface Desktop React 19, Design System (shadcn/ui + TailwindCSS), gerenciamento de estado (Zustand) e componentes UI. | Partes 09, 19, 27 |
| **09** | [`09_seguranca_e_credenciais.md`](./09_seguranca_e_credenciais.md) | Modelo de segurança, cofre de credenciais sensíveis (Windows Credential Manager / DPAPI) e proteção de chaves privadas. | Parte 29 |
| **10** | [`10_configuracoes_logs_e_observabilidade.md`](./10_configuracoes_logs_e_observabilidade.md) | Sistema de configurações globais/locais, logging estruturado (`tracing`), auditoria de ações e observabilidade. | Partes 18, 26, 43 |
| **11** | [`11_cli_terminais_e_ides.md`](./11_cli_terminais_e_ides.md) | CLI dedicada (`ghpm`), integração com shells Windows (PowerShell, WSL, Git Bash, CMD) e plugins para VS Code e JetBrains. | Partes 32, 34, 41, 42 |
| **12** | [`12_distribuicao_e_atualizacoes.md`](./12_distribuicao_e_atualizacoes.md) | Empacotamento Windows (NSIS/MSI), pipeline de build, canais de release e mecanismo de auto-update seguro. | Partes 28, 36 |
| **13** | [`13_extensibilidade_e_plugins.md`](./13_extensibilidade_e_plugins.md) | Arquitetura extensível, SDK para plugins de terceiros, hooks de ciclo de vida e sandboxing. | Parte 33 |

---

## 🎯 Princípios da Reformulação v2

1. **Zero Redundância:** Eliminação de introduções repetidas, duplicidade de tabelas, schemas divergentes e descrições idênticas da stack tecnológica.
2. **Fonte Única da Verdade (SSOT):** Cada entidade, comando IPC, regra de negócio ou tabela SQL possui uma única especificação oficial e inequívoca.
3. **Expliciteza e Clareza:** Manutenção de todos os requisitos funcionais, casos de uso, interfaces TypeScript, structs Rust e comandos CLI detalhados.
4. **Fácil Navegação:** Documentos organizados por domínio funcional com tabelas de referência e diagramas limpos.
