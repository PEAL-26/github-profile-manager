import { useProfileStore } from "@/stores/useProfileStore"
import type { Profile } from "@/types"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"

interface DeleteProfileDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  profile: Profile
}

export function DeleteProfileDialog({
  open,
  onOpenChange,
  profile,
}: DeleteProfileDialogProps) {
  const { deleteProfile, isLoading } = useProfileStore()

  const handleDelete = async () => {
    await deleteProfile(profile.id)
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Excluir Perfil</DialogTitle>
          <DialogDescription>
            Tem certeza que deseja excluir o perfil <strong>{profile.name}</strong>? Esta ação
            não pode ser desfeita.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancelar
          </Button>
          <Button variant="destructive" onClick={handleDelete} disabled={isLoading}>
            {isLoading ? "Excluindo..." : "Excluir"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
