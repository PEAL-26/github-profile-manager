import { useEffect, useState } from "react"
import { useProfileStore } from "@/stores/useProfileStore"
import { profileService } from "@/services"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Label } from "@/components/ui/label"

interface LinkProfileDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  keyId: string
  keyName: string
}

export function LinkProfileDialog({ open, onOpenChange, keyId, keyName }: LinkProfileDialogProps) {
  const { profiles } = useProfileStore()
  const [selectedProfileId, setSelectedProfileId] = useState<string>("")
  const [isSaving, setIsSaving] = useState(false)

  useEffect(() => {
    if (open) {
      const linked = profiles.find((p) => p.sshKeyId === keyId)
      setSelectedProfileId(linked?.id || "")
    }
  }, [open, profiles, keyId])

  const handleSave = async () => {
    if (!selectedProfileId) return
    setIsSaving(true)
    try {
      const profile = profiles.find((p) => p.id === selectedProfileId)
      if (profile) {
        await profileService.update(profile.id, { sshKeyId: keyId })
      }
      onOpenChange(false)
    } catch {
    } finally {
      setIsSaving(false)
    }
  }

  const handleUnlink = async () => {
    setIsSaving(true)
    try {
      const linked = profiles.find((p) => p.sshKeyId === keyId)
      if (linked) {
        await profileService.update(linked.id, { sshKeyId: undefined })
      }
      setSelectedProfileId("")
      onOpenChange(false)
    } catch {
    } finally {
      setIsSaving(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[400px]">
        <DialogHeader>
          <DialogTitle>Vincular a Perfil</DialogTitle>
          <DialogDescription>
            Vincule a chave <strong>{keyName}</strong> a um perfil existente.
          </DialogDescription>
        </DialogHeader>
        <div className="space-y-4">
          <div className="space-y-2">
            <Label>Perfil</Label>
            <Select value={selectedProfileId} onValueChange={(v) => {
              if (v !== null) setSelectedProfileId(v)
            }}>
              <SelectTrigger>
                <SelectValue placeholder="Selecione um perfil" />
              </SelectTrigger>
              <SelectContent>
                {profiles.map((profile) => (
                  <SelectItem key={profile.id} value={profile.id}>
                    {profile.name} ({profile.email})
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </div>
        <DialogFooter className="gap-2">
          {selectedProfileId && (
            <Button variant="destructive" onClick={handleUnlink} disabled={isSaving}>
              Desvincular
            </Button>
          )}
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancelar
          </Button>
          <Button onClick={handleSave} disabled={!selectedProfileId || isSaving}>
            {isSaving ? "Salvando..." : "Vincular"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
