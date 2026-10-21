import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useProfileStore } from "@/stores/useProfileStore"
import type { Profile } from "@/types"
import { Check, Edit, Trash2 } from "lucide-react"
import { useState } from "react"
import { DeleteProfileDialog } from "./DeleteProfileDialog"
import { EditProfileDialog } from "./EditProfileDialog"

interface ProfileCardProps {
  profile: Profile
}

export function ProfileCard({ profile }: ProfileCardProps) {
  const { setActiveProfile, isLoading } = useProfileStore()
  const [editOpen, setEditOpen] = useState(false)
  const [deleteOpen, setDeleteOpen] = useState(false)

  return (
    <>
      <Card className={profile.isActive ? "ring-2 ring-primary" : ""}>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">{profile.name}</CardTitle>
          {profile.isActive && <Badge>Ativo</Badge>}
        </CardHeader>
        <CardContent className="space-y-3">
          <div className="space-y-1">
            <p className="text-sm text-muted-foreground">{profile.email}</p>
            <p className="text-xs text-muted-foreground">@{profile.githubUsername}</p>
          </div>
          <div className="flex gap-2">
            {profile.isActive ? (
              <Button variant="secondary" size="sm" className="gap-1" disabled>
                <Check className="h-3 w-3" />
                Ativo
              </Button>
            ) : (
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  // Erros viram toast via store; catch evita rejeição não tratada.
                  setActiveProfile(profile.id).catch(() => {})
                }}
                disabled={isLoading}
              >
                Ativar
              </Button>
            )}
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8"
              onClick={() => setEditOpen(true)}
            >
              <Edit className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8 text-destructive"
              onClick={() => setDeleteOpen(true)}
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        </CardContent>
      </Card>

      <EditProfileDialog open={editOpen} onOpenChange={setEditOpen} profile={profile} />
      <DeleteProfileDialog open={deleteOpen} onOpenChange={setDeleteOpen} profile={profile} />
    </>
  )
}
