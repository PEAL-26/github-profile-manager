import { useEffect, useState } from "react"
import { useProfileStore } from "@/stores/useProfileStore"
import { sshService } from "@/services"
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
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { createProfileSchema, type CreateProfileFormData } from "@/lib/validations"

interface CreateProfileDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function CreateProfileDialog({ open, onOpenChange }: CreateProfileDialogProps) {
  const { createProfile, isLoading } = useProfileStore()
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
      name: "",
      email: "",
      githubUsername: "",
      hostAlias: "github.com",
      defaultBranch: "main",
      sshKeyId: undefined,
    },
  })

  const defaultBranch = watch("defaultBranch")
  const sshKeyId = watch("sshKeyId")

  useEffect(() => {
    if (open) {
      sshService.listKeys().then(setSshKeys).catch(() => setSshKeys([]))
    }
  }, [open])

  const onSubmit = async (data: CreateProfileFormData) => {
    await createProfile(data)
    reset()
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Novo Perfil</DialogTitle>
          <DialogDescription>
            Crie um novo perfil para gerenciar suas identidades no GitHub.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="name">Nome</Label>
            <Input
              id="name"
              placeholder="Ex: Trabalho, Pessoal"
              {...register("name")}
            />
            {errors.name && (
              <p className="text-xs text-destructive">{errors.name.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="email"
              placeholder="seu@email.com"
              {...register("email")}
            />
            {errors.email && (
              <p className="text-xs text-destructive">{errors.email.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="githubUsername">Username GitHub</Label>
            <Input
              id="githubUsername"
              placeholder="seu-username"
              {...register("githubUsername")}
            />
            {errors.githubUsername && (
              <p className="text-xs text-destructive">{errors.githubUsername.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="hostAlias">Host Alias</Label>
            <Input
              id="hostAlias"
              placeholder="github.com"
              {...register("hostAlias")}
            />
            {errors.hostAlias && (
              <p className="text-xs text-destructive">{errors.hostAlias.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="defaultBranch">Branch Padrão</Label>
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
              {isLoading ? "Criando..." : "Criar Perfil"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
