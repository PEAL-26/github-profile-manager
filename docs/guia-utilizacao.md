# Guia de Utilização — GHPM

## Instalação

1. Descarregue o instalador `.msi` ou `.exe` da página de [Releases](https://github.com/PEAL-26/github-profile-manager/releases)
2. Execute o ficheiro descarregado
4. Siga as instruções do assistente de instalação
4. A app inicia automaticamente e aparece na bandeja do sistema (ícone junto ao relógio)

## Primeiros passos

### Criar o primeiro perfil

1. Clique com o botão esquerdo no ícone da GHPM na bandeja
2. Clique em **Abrir GHPM** no menu de contexto
3. Na janela principal, clique em **Novo Perfil**
4. Preencha:
   - **Nome** — nome descritivo (ex: "Trabalho", "Pessoal")
   - **Email** — email associado à conta GitHub
   - **Nome de utilizador GitHub** — o seu username no GitHub
   - **Token PAT** (opcional) — Personal Access Token para acesso à API
5. Clique em **Criar**

### Ativar um perfil

1. Na lista de perfis, clique no botão **Ativar** no perfil desejado
2. A app configura automaticamente:
   - `git config --global user.name`
   - `git config --global user.email`
   - Chave SSH associada (se vinculada)

### Gerir chaves SSH

#### Gerar uma nova chave

1. Navegue até **Chaves SSH** no menu lateral
2. Clique em **Gerar Nova Chave**
3. Escolha o nome, tipo (Ed25519, RSA, ECDSA) e opcionalmente uma passphrase
4. A chave é criada em `~/.ssh/` e registada na app

#### Vincular uma chave existente

1. Em **Chaves SSH**, clique em **Vincular Chave Existente**
2. Selecione o ficheiro de chave no seu sistema
3. A chave é adicionada à app sem ser recriada

#### Vincular chave a um perfil

1. No card da chave SSH, clique em **Vincular a Perfil**
2. Selecione o perfil pretendido
3. Ao ativar esse perfil, a chave SSH é automaticamente associada

#### Testar conexão

1. No card da chave, clique em **Testar Conexão**
2. A app tenta ligar ao GitHub via SSH e mostra o resultado

### System Tray

A app funciona em segundo plano na bandeja do sistema:

- **Clique esquerdo** → abre popup rápido com perfil ativo e dropdown
- **Clique direito** → menu de contexto:
  - **Abrir GHPM** — abre a janela principal
  - **Iniciar com o Windows** — ativa/desativa inicialização automática
  - **Sair** — fecha completamente a app

### Fechar a app

Ao fechar a janela principal (botão X), a app **continua a correr na bandeja**. Para sair completamente, use o menu de contexto da bandeja e clique em **Sair**.

## Segurança

- Chaves privadas **não** são copiadas nem armazenadas pela app — permanecem em `~/.ssh/`
- Tokens PAT são armazenados de forma encriptada no Windows Credential Manager
- Logs são sanitizados — dados sensíveis (tokens, chaves) são mascarados

## Localização dos dados

- Base de dados: `~/.ghpm/app_data.sqlite`
- Logs: `~/.ghpm/logs/`
- Chaves SSH: `~/.ssh/` (localização padrão)
