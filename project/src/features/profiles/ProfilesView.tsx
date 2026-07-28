import { useEffect, useState } from "react"
import { listen } from "@tauri-apps/api/event"
import { useProfileStore } from "@/stores/useProfileStore"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Plus, User } from "lucide-react"
import { ProfileCard } from "./components/ProfileCard"
import { CreateProfileDialog } from "./components/CreateProfileDialog"

export function ProfilesView() {
  const { profiles, isLoading, fetchProfiles } = useProfileStore()
  const [createOpen, setCreateOpen] = useState(false)

  useEffect(() => {
    fetchProfiles()
    const unlisten = listen("profile-changed", () => {
      fetchProfiles()
    })
    return () => {
      unlisten.then((fn) => fn())
    }
  }, [fetchProfiles])

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold">Perfis</h2>
        <Button className="gap-2" onClick={() => setCreateOpen(true)}>
          <Plus className="h-4 w-4" />
          Novo Perfil
        </Button>
      </div>

      {isLoading && <p className="text-muted-foreground">Carregando...</p>}

      {!isLoading && profiles.length === 0 && (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <User className="mb-4 h-12 w-12 text-muted-foreground" />
            <p className="text-muted-foreground">Nenhum perfil cadastrado</p>
            <p className="text-sm text-muted-foreground">
              Crie seu primeiro perfil para começar
            </p>
          </CardContent>
        </Card>
      )}

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {profiles.map((profile) => (
          <ProfileCard key={profile.id} profile={profile} />
        ))}
      </div>

      <CreateProfileDialog open={createOpen} onOpenChange={setCreateOpen} />
    </div>
  )
}
