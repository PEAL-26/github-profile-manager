import { useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"

interface HelpDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

const tabs = [
  { id: "how-to-use", label: "Como Usar" },
  { id: "profiles", label: "Perfis" },
  { id: "ssh-keys", label: "Chaves SSH" },
  { id: "about", label: "Sobre" },
] as const

export function HelpDialog({ open, onOpenChange }: HelpDialogProps) {
  const [activeTab, setActiveTab] = useState<string>("how-to-use")

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px] max-h-[80vh] overflow-hidden flex flex-col">
        <DialogHeader>
          <DialogTitle>GHPM — Guia Rápido</DialogTitle>
        </DialogHeader>

        <div className="flex gap-1 border-b border-border">
          {tabs.map((tab) => (
            <Button
              key={tab.id}
              variant="ghost"
              size="sm"
              className={cn(
                "rounded-b-none border-b-2 transition-colors",
                activeTab === tab.id
                  ? "border-primary text-foreground"
                  : "border-transparent text-muted-foreground hover:text-foreground"
              )}
              onClick={() => setActiveTab(tab.id)}
            >
              {tab.label}
            </Button>
          ))}
        </div>

        <div className="flex-1 overflow-y-auto py-4 space-y-4 text-sm">
          {activeTab === "how-to-use" && (
            <div className="space-y-4">
              <p className="text-muted-foreground">
                O GHPM (GitHub Profile Manager) permite gerenciar múltiplas identidades do GitHub
                em um só lugar.
              </p>

              <div className="space-y-2">
                <h3 className="font-medium">1. Crie seus perfis</h3>
                <p className="text-muted-foreground">
                  Vá na aba <strong>Perfis</strong> e clique em <strong>Novo Perfil</strong>.
                  Preencha nome, email, username do GitHub e um host alias (ex: github-trabalho).
                </p>
              </div>

              <div className="space-y-2">
                <h3 className="font-medium">2. Gere chaves SSH</h3>
                <p className="text-muted-foreground">
                  Vá na aba <strong>Chaves SSH</strong> e gere uma chave para cada perfil.
                  Recomendamos usar <strong>ed25519</strong> por ser mais seguro e rápido.
                </p>
              </div>

              <div className="space-y-2">
                <h3 className="font-medium">3. Vincule chave ao perfil</h3>
                <p className="text-muted-foreground">
                  Ao criar ou editar um perfil, selecione a chave SSH correspondente.
                  Você também pode vincular pelo ícone de link na tela de chaves.
                </p>
              </div>

              <div className="space-y-2">
                <h3 className="font-medium">4. Ative o perfil</h3>
                <p className="text-muted-foreground">
                  Clique em <strong>Ativar</strong> no perfil desejado. Isso configura
                  automaticamente o <code>git config --global</code> com o nome, email
                  e chave SSH corretos.
                </p>
              </div>

              <div className="rounded-lg bg-muted p-3">
                <p className="text-xs text-muted-foreground">
                  <strong>Dica:</strong> Ao ativar um perfil, o app configura o
                  <code>core.sshCommand</code> do Git para usar a chave SSH vinculada.
                  Isso garante que commits usem a identidade correta.
                </p>
              </div>
            </div>
          )}

          {activeTab === "profiles" && (
            <div className="space-y-4">
              <h3 className="font-medium">Gerenciando Perfis</h3>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Criar perfil</h4>
                <ul className="list-disc list-inside text-muted-foreground space-y-1">
                  <li><strong>Nome:</strong> Identificação do perfil (ex: "Trabalho", "Pessoal")</li>
                  <li><strong>Email:</strong> Email que aparecerá nos commits</li>
                  <li><strong>Username:</strong> Seu usuário no GitHub</li>
                  <li><strong>Host Alias:</strong> Nome único para identificar o host SSH</li>
                  <li><strong>Branch Padrão:</strong> Branch principal do repositório</li>
                  <li><strong>Chave SSH:</strong> Chave vinculada a este perfil (opcional)</li>
                </ul>
              </div>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Ativar perfil</h4>
                <p className="text-muted-foreground">
                  Clique em <strong>Ativar</strong> no card do perfil. Apenas um perfil
                  pode estar ativo por vez. O app configura automaticamente:
                </p>
                <ul className="list-disc list-inside text-muted-foreground">
                  <li><code>git config --global user.name</code></li>
                  <li><code>git config --global user.email</code></li>
                  <li><code>git config --global core.sshCommand</code></li>
                </ul>
              </div>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Editar/Excluir</h4>
                <p className="text-muted-foreground">
                  Use os ícones no card do perfil para editar informações ou excluir.
                  Ao excluir, a chave SSH vinculada não é removida.
                </p>
              </div>
            </div>
          )}

          {activeTab === "ssh-keys" && (
            <div className="space-y-4">
              <h3 className="font-medium">Gerenciando Chaves SSH</h3>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Gerar nova chave</h4>
                <ul className="list-disc list-inside text-muted-foreground space-y-1">
                  <li><strong>Nome:</strong> Identificador da chave (ex: "meu-trabalho")</li>
                  <li><strong>Tipo:</strong> Recomendamos <strong>ed25519</strong> (mais seguro)</li>
                  <li><strong>Senha (opcional):</strong> Protege a chave com passphrase</li>
                </ul>
              </div>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Vincular a perfil</h4>
                <p className="text-muted-foreground">
                  Clique no ícone de link (🔗) no card da chave para vinculá-la a um perfil.
                  A chave também pode ser selecionada ao criar/editar um perfil.
                </p>
              </div>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Testar conexão</h4>
                <p className="text-muted-foreground">
                  Clique em <strong>Testar</strong> para verificar se a chave funciona
                  com o GitHub. É necessário ter adicionado a chave pública ao GitHub.
                </p>
              </div>

              <div className="rounded-lg bg-muted p-3">
                <p className="text-xs text-muted-foreground">
                  <strong>Importante:</strong> Após gerar a chave, copie o conteúdo
                  da chave pública (o texto ao lado da impressão digital) e adicione
                  no GitHub em <strong>Settings → SSH and GPG keys → New SSH key</strong>.
                </p>
              </div>
            </div>
          )}

          {activeTab === "about" && (
            <div className="space-y-4">
              <div className="space-y-2">
                <h3 className="font-medium">GHPM — GitHub Profile Manager</h3>
                <p className="text-muted-foreground">
                  Versão 0.1.0
                </p>
                <p className="text-muted-foreground">
                  Aplicação desktop para gerenciar múltiplas identidades do GitHub.
                  Criada com Tauri 2 (Rust + React).
                </p>
              </div>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Stack</h4>
                <ul className="list-disc list-inside text-muted-foreground">
                  <li>Tauri 2 (Rust backend)</li>
                  <li>React 19 + TypeScript</li>
                  <li>SQLite (via rusqlite)</li>
                  <li>shadcn/ui + Tailwind CSS</li>
                  <li>Zustand (state management)</li>
                </ul>
              </div>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Banco de dados</h4>
                <p className="text-muted-foreground font-mono text-xs">
                  ~/.ghpm/app_data.sqlite
                </p>
              </div>

              <div className="space-y-2">
                <h4 className="text-sm font-medium">Localização</h4>
                <p className="text-muted-foreground font-mono text-xs">
                  ~/.ghpm/
                </p>
              </div>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
