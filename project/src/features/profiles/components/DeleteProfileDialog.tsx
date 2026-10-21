import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { useProfileStore } from "@/stores/useProfileStore"
import type { Profile } from "@/types"

interface DeleteProfileDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  profile: Profile
}

export function DeleteProfileDialog({ open, onOpenChange, profile }: DeleteProfileDialogProps) {
  const { deleteProfile, isLoading } = useProfileStore()

  const handleDelete = async () => {
    try {
      await deleteProfile(profile.id)
      onOpenChange(false)
    } catch {
      // O erro já foi exibido como toast pelo store; mantém o diálogo aberto.
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Excluir Perfil</DialogTitle>
          <DialogDescription>
            Tem certeza que deseja excluir o perfil <strong>{profile.name}</strong>? Esta ação não
            pode ser desfeita.
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
