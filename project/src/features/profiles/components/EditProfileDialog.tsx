import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { appToast } from "@/lib/toast"
import {
  type CreateProfileFormData,
  createProfileSchema,
  isHostAliasTaken,
} from "@/lib/validations"
import { sshService } from "@/services"
import { useProfileStore } from "@/stores/useProfileStore"
import type { Profile, SshKey } from "@/types"
import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"

interface EditProfileDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  profile: Profile
}

export function EditProfileDialog({ open, onOpenChange, profile }: EditProfileDialogProps) {
  const { updateProfile, isLoading, profiles } = useProfileStore()
  const [sshKeys, setSshKeys] = useState<SshKey[]>([])
  const {
    register,
    handleSubmit,
    setValue,
    watch,
    reset,
    formState: { errors },
  } = useForm<CreateProfileFormData>({
    resolver: zodResolver(createProfileSchema),
    defaultValues: {
      name: profile.name,
      email: profile.email,
      githubUsername: profile.githubUsername,
      hostAlias: profile.hostAlias,
      defaultBranch: profile.defaultBranch as "main" | "master" | "develop",
      sshKeyId: profile.sshKeyId,
    },
  })

  const defaultBranch = watch("defaultBranch")
  const sshKeyId = watch("sshKeyId")

  useEffect(() => {
    if (open) {
      sshService
        .listKeys()
        .then(setSshKeys)
        .catch(() => setSshKeys([]))
      reset({
        name: profile.name,
        email: profile.email,
        githubUsername: profile.githubUsername,
        hostAlias: profile.hostAlias,
        defaultBranch: profile.defaultBranch as "main" | "master" | "develop",
        sshKeyId: profile.sshKeyId,
      })
    }
  }, [open, profile, reset])

  const onSubmit = async (data: CreateProfileFormData) => {
    // Unicidade: exclui o próprio perfil da comparação.
    if (isHostAliasTaken(data.hostAlias, profiles, profile.id)) {
      appToast.error(
        "Host alias já em uso",
        `Já existe um perfil com o host alias "${data.hostAlias}". Escolha outro alias.`,
      )
      return
    }
    try {
      await updateProfile(profile.id, data)
      onOpenChange(false)
    } catch {
      // O erro já foi exibido como toast pelo store; mantém o diálogo aberto.
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Editar Perfil</DialogTitle>
          <DialogDescription>Atualize as informações do perfil.</DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="edit-name">Nome</Label>
            <Input id="edit-name" {...register("name")} />
            {errors.name && <p className="text-xs text-destructive">{errors.name.message}</p>}
          </div>
          <div className="space-y-2">
            <Label htmlFor="edit-email">Email</Label>
            <Input id="edit-email" type="email" {...register("email")} />
            {errors.email && <p className="text-xs text-destructive">{errors.email.message}</p>}
          </div>
          <div className="space-y-2">
            <Label htmlFor="edit-githubUsername">Username GitHub</Label>
            <Input id="edit-githubUsername" {...register("githubUsername")} />
            {errors.githubUsername && (
              <p className="text-xs text-destructive">{errors.githubUsername.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="edit-hostAlias">Host Alias</Label>
            <Input id="edit-hostAlias" {...register("hostAlias")} />
            {errors.hostAlias && (
              <p className="text-xs text-destructive">{errors.hostAlias.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="edit-defaultBranch">Branch Padrão</Label>
            <Select
              value={defaultBranch}
              onValueChange={(v) => {
                if (v === "main" || v === "master" || v === "develop") {
                  setValue("defaultBranch", v)
                }
              }}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="main">main</SelectItem>
                <SelectItem value="master">master</SelectItem>
                <SelectItem value="develop">develop</SelectItem>
              </SelectContent>
            </Select>
            {errors.defaultBranch && (
              <p className="text-xs text-destructive">{errors.defaultBranch.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label>Chave SSH (opcional)</Label>
            <Select
              value={sshKeyId || "__none__"}
              onValueChange={(v) => {
                if (v !== null) {
                  setValue("sshKeyId", v === "__none__" ? undefined : v)
                }
              }}
            >
              <SelectTrigger>
                <SelectValue placeholder="Nenhuma chave selecionada" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">Nenhuma</SelectItem>
                {sshKeys.map((key) => (
                  <SelectItem key={key.id} value={key.id}>
                    {key.name} ({key.keyType.toUpperCase()})
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              Cancelar
            </Button>
            <Button type="submit" disabled={isLoading}>
              {isLoading ? "Salvando..." : "Salvar"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
