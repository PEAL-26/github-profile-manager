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
import { isHostAliasTaken } from "@/lib/validations"
import { type CreateProfileFormData, createProfileSchema } from "@/lib/validations"
import { sshService } from "@/services"
import { useProfileStore } from "@/stores/useProfileStore"
import type { SshKey } from "@/types"
import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect, useRef, useState } from "react"
import { useForm } from "react-hook-form"

interface CreateProfileDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function CreateProfileDialog({ open, onOpenChange }: CreateProfileDialogProps) {
  const { createProfile, isLoading, profiles } = useProfileStore()
  const [sshKeys, setSshKeys] = useState<SshKey[]>([])
  // Controla se o usuário digitou o host alias manualmente;
  // enquanto não digitar, ele é auto-gerado a partir do username.
  const aliasTouched = useRef(false)
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
      hostAlias: "",
      defaultBranch: "main",
      sshKeyId: undefined,
    },
  })

  const defaultBranch = watch("defaultBranch")
  const sshKeyId = watch("sshKeyId")
  const githubUsername = watch("githubUsername")

  // Gera a sugestão "github-<username>" (Fase 3.1 do roadmap: gerar host_alias).
  useEffect(() => {
    if (!aliasTouched.current && githubUsername) {
      setValue("hostAlias", `github-${githubUsername}`)
    }
  }, [githubUsername, setValue])

  useEffect(() => {
    if (open) {
      sshService
        .listKeys()
        .then(setSshKeys)
        .catch(() => setSshKeys([]))
    }
  }, [open])

  const onSubmit = async (data: CreateProfileFormData) => {
    // Validação de unicidade contra perfis existentes (além do backend).
    if (isHostAliasTaken(data.hostAlias, profiles)) {
      appToast.error(
        "Host alias já em uso",
        `Já existe um perfil com o host alias "${data.hostAlias}". Escolha outro alias.`,
      )
      return
    }
    try {
      await createProfile(data)
      reset()
      aliasTouched.current = false
      onOpenChange(false)
    } catch {
      // O erro já foi exibido como toast pelo store;
      // mantém o diálogo aberto para o usuário corrigir.
    }
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
            <Input id="name" placeholder="Ex: Trabalho, Pessoal" {...register("name")} />
            {errors.name && <p className="text-xs text-destructive">{errors.name.message}</p>}
          </div>
          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input id="email" type="email" placeholder="seu@email.com" {...register("email")} />
            {errors.email && <p className="text-xs text-destructive">{errors.email.message}</p>}
          </div>
          <div className="space-y-2">
            <Label htmlFor="githubUsername">Username GitHub</Label>
            <Input id="githubUsername" placeholder="seu-username" {...register("githubUsername")} />
            {errors.githubUsername && (
              <p className="text-xs text-destructive">{errors.githubUsername.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="hostAlias">Host Alias</Label>
            <Input
              id="hostAlias"
              placeholder="github-seu-username"
              {...register("hostAlias", {
                onChange: () => {
                  aliasTouched.current = true
                },
              })}
            />
            <p className="text-xs text-muted-foreground">
              Gerado automaticamente — usado no ~/.ssh/config. Deve ser único.
            </p>
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
