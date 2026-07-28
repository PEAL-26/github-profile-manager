# GitHub Profile Manager (GHPM)

Gerir múltiplas identidades GitHub num só lugar. Troque entre perfis, chaves SSH e configurações Git com um clique.

## O que é?

O **GHPM** é uma aplicação desktop para Windows que resolve um problema comum entre developers que trabalham com múltiplas contas GitHub: alternar entre perfis, chaves SSH e configurações Git manualmente é tedioso e propenso a erros.

Com o GHPM, pode:

- Criar e gerir múltiplos perfis GitHub
- Gerar e vincular chaves SSH a perfis específicos
- Trocar a identidade Git global com um clique
- Validar conexões SSH diretamente pela app
- Manter tokens de acesso pessoal (PAT) seguros no Windows Credential Manager
- Iniciar automaticamente com o Windows

## Como instalar

1. Descarregue o ficheiro `.msi` ou `.exe` na secção [Releases](https://github.com/PEAL-26/github-profile-manager/releases)
2. Execute o instalador
3. A app inicia automaticamente na bandeja do sistema

## Como utilizar

### Perfil ativo

1. Abra a app pela bandeja do sistema (ícone GHPM)
2. Clique em **Novo Perfil** para adicionar uma conta GitHub
3. Preencha o nome, email e opcionalmente um token PAT
4. Clique em **Ativar** no perfil desejado — a app aplica as configurações globalmente

### Chaves SSH

1. Navegue até **Chaves SSH** no menu lateral
2. Clique em **Gerar Nova Chave** ou **Vincular Chave Existente**
3. Use **Testar Conexão** para verificar se a chave está a funcionar
4. Vincule uma chave a um perfil específico

### Bandeja do sistema

- **Clique esquerdo** no ícone → popup rápido com perfil ativo
- **Clique direito** → menu com Abrir GHPM, Iniciar com o Windows, Sair
- Fechar a janela principal → a app continua na bandeja

## Features

- Gestão de perfis GitHub (criar, editar, eliminar, ativar)
- Geração de chaves SSH (Ed25519, RSA, ECDSA)
- Vinculação de chaves SSH a perfis
- Teste de conexão SSH
- Configuração Git global (user.name, user.email)
- Armazenamento seguro de PAT via Windows Credential Manager
- System tray com popup rápido
- Inicialização automática com o Windows
- Tema claro/escuro
- Registo de atividade (logs)

## Roadmap

### v0.1.0 (atual)

- [x] Gestão de perfis GitHub
- [x] Geração e vinculação de chaves SSH
- [x] Troca global de perfil
- [x] Autenticação PAT do GitHub
- [x] Windows Credential Manager
- [x] UI desktop com tema claro/escuro
- [x] System tray com popup rápido
- [x] Logs estruturados
- [x] Inicialização com o Windows

### v0.2.0

- [ ] Backup e exportação de perfis
- [ ] Git config visual por perfil
- [ ] Notificações Windows
- [ ] SSH agent integration

### v0.3.0

- [ ] Suporte macOS e Linux
- [ ] Auto-update via GitHub Releases
- [ ] Integração com VS Code e JetBrains
- [ ] CLI dedicada (`ghpm`)

### Futuro

- [ ] OAuth 2.0 PKCE
- [ ] Motor de regras de automação
- [ ] Scanner de repositórios
- [ ] Engine de diagnósticos
- [ ] Sistema de plugins

## Desenvolvimento

```bash
# Instalar dependências
pnpm install

# Executar em modo desenvolvimento
pnpm tauri dev

# Compilar para produção
pnpm tauri build

# Executar testes
pnpm test
```

## Licença

[MIT](LICENSE)
