import { useSshKeyStore } from "@/stores/useSshKeyStore"
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
import { generateKeySchema, type GenerateKeyFormData } from "@/lib/validations"

interface GenerateKeyDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function GenerateKeyDialog({ open, onOpenChange }: GenerateKeyDialogProps) {
  const { generateKey, isLoading } = useSshKeyStore()
  const {
    register,
    handleSubmit,
    setValue,
    watch,
    reset,
    formState: { errors },
  } = useForm<GenerateKeyFormData>({
    resolver: zodResolver(generateKeySchema),
    defaultValues: {
      name: "",
      keyType: "ed25519",
      passphrase: "",
    },
  })

  const keyType = watch("keyType")

  const onSubmit = async (data: GenerateKeyFormData) => {
    await generateKey(data.name, data.keyType, data.passphrase || undefined)
    reset()
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Gerar Chave SSH</DialogTitle>
          <DialogDescription>
            Gere uma nova chave SSH para usar com o GitHub.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="key-name">Nome da Chave</Label>
            <Input
              id="key-name"
              placeholder="Ex: trabalho, pessoal"
              {...register("name")}
            />
            {errors.name && (
              <p className="text-xs text-destructive">{errors.name.message}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="key-type">Tipo da Chave</Label>
            <Select
              value={keyType}
              onValueChange={(v) => v && setValue("keyType", v as "ed25519" | "rsa")}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="ed25519">ED25519 (Recomendado)</SelectItem>
                <SelectItem value="rsa">RSA</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label htmlFor="passphrase">Passphrase (Opcional)</Label>
            <Input
              id="passphrase"
              type="password"
              placeholder="Deixe vazio para sem passphrase"
              {...register("passphrase")}
            />
          </div>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              Cancelar
            </Button>
            <Button type="submit" disabled={isLoading}>
              {isLoading ? "Gerando..." : "Gerar Chave"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
