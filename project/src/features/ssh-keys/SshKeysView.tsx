import { useEffect, useState } from "react"
import { useSshKeyStore } from "@/stores/useSshKeyStore"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Plus, KeyRound } from "lucide-react"
import { SshKeyCard } from "./components/SshKeyCard"
import { GenerateKeyDialog } from "./components/GenerateKeyDialog"

export function SshKeysView() {
  const { keys, isLoading, fetchKeys } = useSshKeyStore()
  const [generateOpen, setGenerateOpen] = useState(false)

  useEffect(() => {
    fetchKeys()
  }, [fetchKeys])

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold">Chaves SSH</h2>
        <Button className="gap-2" onClick={() => setGenerateOpen(true)}>
          <Plus className="h-4 w-4" />
          Gerar Chave
        </Button>
      </div>

      {isLoading && <p className="text-muted-foreground">Carregando...</p>}

      {!isLoading && keys.length === 0 && (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <KeyRound className="mb-4 h-12 w-12 text-muted-foreground" />
            <p className="text-muted-foreground">Nenhuma chave SSH cadastrada</p>
            <p className="text-sm text-muted-foreground">
              Gere uma nova chave ou vincule uma existente
            </p>
          </CardContent>
        </Card>
      )}

      <div className="grid gap-4">
        {keys.map((key) => (
          <SshKeyCard key={key.id} keyData={key} />
        ))}
      </div>

      <GenerateKeyDialog open={generateOpen} onOpenChange={setGenerateOpen} />
    </div>
  )
}
