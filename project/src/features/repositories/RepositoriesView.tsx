import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { useProfileStore } from "@/stores/useProfileStore"
import { useRepositoryStore } from "@/stores/useRepositoryStore"
import { FolderTree, ScanLine, SearchX } from "lucide-react"
import { useEffect } from "react"
import { RepositoryCard } from "./components/RepositoryCard"
import { ScanRootsSection } from "./components/ScanRootsSection"

export function RepositoriesView() {
  const {
    repositories,
    roots,
    isLoading,
    isScanning,
    fetchRepositories,
    fetchRoots,
    scan,
    applyProfile,
  } = useRepositoryStore()
  const profiles = useProfileStore((s) => s.profiles)
  const fetchProfiles = useProfileStore((s) => s.fetchProfiles)

  useEffect(() => {
    void fetchRoots()
    void fetchRepositories()
    void fetchProfiles()
  }, [fetchRoots, fetchRepositories, fetchProfiles])

  const handleScan = async () => {
    try {
      await scan()
    } catch {
      // store já exibiu o toast de erro
    }
  }

  const showLoading = isLoading && repositories.length === 0
  const showNoRoots = !isLoading && !isScanning && repositories.length === 0 && roots.length === 0
  const showEmpty =
    !isLoading && !isScanning && repositories.length === 0 && roots.length > 0

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold">Repositórios</h2>
        <Button onClick={handleScan} disabled={isScanning} className="gap-2">
          <ScanLine className="h-4 w-4" />
          {isScanning ? "Escaneando..." : "Escanear agora"}
        </Button>
      </div>

      <ScanRootsSection />

      {showLoading && <p className="text-muted-foreground">Carregando...</p>}

      {showNoRoots && (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <FolderTree className="mb-4 h-12 w-12 text-muted-foreground" />
            <p className="text-muted-foreground">
              Nenhuma pasta de varrimento configurada
            </p>
            <p className="text-sm text-muted-foreground">
              Adicione uma pasta acima (ex.: C:/dev) e clique em Escanear
            </p>
          </CardContent>
        </Card>
      )}

      {showEmpty && (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <SearchX className="mb-4 h-12 w-12 text-muted-foreground" />
            <p className="text-muted-foreground">
              Nenhum repositório Git encontrado
            </p>
            <p className="text-sm text-muted-foreground">
              Verifique as pastas configuradas ou aumente a profundidade de
              varredura nas Configurações
            </p>
          </CardContent>
        </Card>
      )}

      <div className="space-y-4">
        {repositories.map((repository) => (
          <RepositoryCard
            key={repository.id}
            repository={repository}
            profiles={profiles}
            isApplying={isLoading}
            onApply={applyProfile}
          />
        ))}
      </div>
    </div>
  )
}
