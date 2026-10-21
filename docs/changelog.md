# Changelog

Todas as alterações notáveis neste projeto serão documentadas neste ficheiro.

O formato baseia-se em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Não publicado]

### Adicionado

- Backup/Exportação (Módulo 02.5): ficheiro `.ghpm-backup` com modo padrão
  (dados públicos) e modo completo criptografado (AES-256-GCM + PBKDF2-SHA256),
  importação/restauração com upsert por host alias — nova seção
  "Backup e Restauração" em Configurações
- Log de erros em `ghpm_errors.txt` na pasta de instalação (fallback:
  `~/.ghpm`), capturando erros de commands, panics e erros do frontend
- Toasts de sucesso/erro nas ações de perfis (criar, editar, excluir, ativar)
- Host alias auto-gerado (`github-<username>`) no cadastro de perfis

### Corrigido

- Janelas de console piscando ao trocar de perfil ou executar ações de git/ssh:
  subprocessos agora rodam ocultos (CREATE_NO_WINDOW)
- Perfis adicionais falhando silenciosamente após os primeiros: colisão de
  host alias agora retorna erro amigável e a UI valida a unicidade antes
  de enviar
- Data dos logs de atividade exibindo "Invalid Date" (serialização camelCase)
- Tela de Configurações não refletindo o estado salvo dos toggles
  (mesmo problema de serialização)

## [0.1.0] - 2026-07-28

### Adicionado

- Gestão de perfis GitHub (criar, editar, eliminar, ativar)
- Geração de chaves SSH (Ed25519, RSA, ECDSA)
- Vinculação de chaves SSH a perfis
- Ligação a chaves SSH existentes em `~/.ssh/`
- Teste de conexão SSH
- Configuração Git global (user.name, user.email)
- Armazenamento seguro de PAT via Windows Credential Manager
- System tray com popup rápido e dropdown de perfis
- Inicialização automática com o Windows (toggle no tray menu)
- Tema claro/escuro
- Custom titlebar com toggle de sidebar
- Registo de atividade (logs)
- Sincronização em tempo real entre janelas principais e popup
- Documentação de utilização
