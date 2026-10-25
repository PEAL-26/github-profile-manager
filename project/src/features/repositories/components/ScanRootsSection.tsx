import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { useRepositoryStore } from "@/stores/useRepositoryStore"
import { useSettingsStore } from "@/stores/useSettingsStore"
import { open } from "@tauri-apps/plugin-dialog"
import { FolderPlus, FolderTree, Trash2 } from "lucide-react"

export function ScanRootsSection() {
  const { roots, addRoot, removeRoot } = useRepositoryStore()
  const settings = useSettingsStore((s) => s.settings)

  const handleAdd = async () => {
    const picked = await open({
      title: "Selecionar pasta para varrer",
      directory: true,
      multiple: false,
    })
    if (!picked || Array.isArray(picked)) return
    try {
      await addRoot(picked)
    } catch {
      // store já exibiu o toast de erro
    }
  }

  const handleRemove = async (id: string) => {
    try {
      await removeRoot(id)
    } catch {
      // store já exibiu o toast de erro
    }
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between gap-4">
          <div>
            <CardTitle>Pastas de Varrimento</CardTitle>
            <CardDescription>
              O scanner procura pastas com <code className="mx-1">.git</code>
              até {settings?.scanDepth ?? 3} subníveis, ignorando diretórios de
              dependências (node_modules, target, vendor...).
            </CardDescription>
          </div>
          <Button variant="outline" size="sm" className="gap-2" onClick={handleAdd}>
            <FolderPlus className="h-4 w-4" />
            Adicionar pasta
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-2">
        {roots.length === 0 && (
          <p className="text-sm text-muted-foreground">
            Nenhuma pasta configurada — adicione uma pasta raiz para começar a
            escanear.
          </p>
        )}

        {roots.map((root) => (
          <div
            key={root.id}
            className="flex items-center justify-between gap-3 rounded-lg border border-border px-3 py-2"
          >
            <div className="flex min-w-0 items-center gap-2">
              <FolderTree className="h-4 w-4 shrink-0 text-muted-foreground" />
              <span className="truncate font-mono text-sm" title={root.path}>
                {root.path}
              </span>
            </div>
            <Button
              variant="ghost"
              size="sm"
              className="h-7 w-7 p-0 text-muted-foreground hover:text-destructive"
              title="Remover pasta da varredura"
              onClick={() => void handleRemove(root.id)}
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        ))}
      </CardContent>
    </Card>
  )
}
