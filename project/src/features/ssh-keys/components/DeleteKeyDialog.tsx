import { useSshKeyStore } from "@/stores/useSshKeyStore"
import type { SshKey } from "@/types"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"

interface DeleteKeyDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  keyData: SshKey
}

export function DeleteKeyDialog({ open, onOpenChange, keyData }: DeleteKeyDialogProps) {
  const { deleteKey, isLoading } = useSshKeyStore()

  const handleDelete = async () => {
    await deleteKey(keyData.id)
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Excluir Chave SSH</DialogTitle>
          <DialogDescription>
            Tem certeza que deseja excluir a chave <strong>{keyData.name}</strong>? Os arquivos da
            chave serão removidos do sistema. Esta ação não pode ser desfeita.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancelar
          </Button>
          <Button variant="destructive" onClick={handleDelete} disabled={isLoading}>
            {isLoading ? "Excluindo..." : "Excluir Chave"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
