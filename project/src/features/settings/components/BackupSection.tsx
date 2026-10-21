import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
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
import { appToast } from "@/lib/toast"
import { errorMessage } from "@/lib/utils"
import { backupService } from "@/services"
import { useProfileStore } from "@/stores/useProfileStore"
import { useSettingsStore } from "@/stores/useSettingsStore"
import type { BackupInfo, ImportReport } from "@/types"
import { open, save } from "@tauri-apps/plugin-dialog"
import { Download, Lock, Upload } from "lucide-react"
import { useState } from "react"

function today() {
  const d = new Date()
  const pad = (n: number) => String(n).padStart(2, "0")
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`
}

function describeReport(r: ImportReport): string {
  const parts = [`${r.profilesCreated} perfil(is) criado(s)`, `${r.profilesUpdated} atualizado(s)`]
  if (r.keysRestored > 0) parts.push(`${r.keysRestored} chave(s) SSH`)
  if (r.patsRestored > 0) parts.push(`${r.patsRestored} token(s)`)
  return parts.join(", ")
}

export function BackupSection() {
  const [includeSecrets, setIncludeSecrets] = useState(false)
  const [password, setPassword] = useState("")
  const [isExporting, setIsExporting] = useState(false)
  const [isImporting, setIsImporting] = useState(false)

  // Fluxo de importação protegida por senha
  const [pendingImport, setPendingImport] = useState<{
    path: string
    info: BackupInfo
  } | null>(null)
  const [importPassword, setImportPassword] = useState("")

  const refetchEverything = () => {
    void useProfileStore.getState().fetchProfiles()
    void useSettingsStore.getState().fetchSettings()
  }

  const doImport = async (path: string, pw?: string) => {
    setIsImporting(true)
    try {
      const report = await backupService.import(path, pw)
      appToast.success("Backup restaurado com sucesso", describeReport(report))
      refetchEverything()
    } catch (e) {
      appToast.error("Erro ao importar backup", errorMessage(e, "Falha desconhecida"))
    } finally {
      setIsImporting(false)
      setPendingImport(null)
      setImportPassword("")
    }
  }

  const handleExport = async () => {
    if (includeSecrets && password.length < 8) {
      appToast.error(
        "Senha fraca",
        "Use pelo menos 8 caracteres para proteger chaves privadas e tokens.",
      )
      return
    }
    const path = await save({
      title: "Exportar backup GHPM",
      defaultPath: `ghpm-backup-${today()}.ghpm-backup`,
      filters: [{ name: "Backup GHPM", extensions: ["ghpm-backup"] }],
    })
    if (!path) return // usuário cancelou

    setIsExporting(true)
    try {
      const count = await backupService.export(
        path,
        includeSecrets,
        includeSecrets || password ? password : undefined,
      )
      appToast.success("Backup exportado com sucesso", `${count} perfil(is) gravados em ${path}`)
      setPassword("")
    } catch (e) {
      appToast.error("Erro ao exportar backup", errorMessage(e, "Falha desconhecida"))
    } finally {
      setIsExporting(false)
    }
  }

  const handleImport = async () => {
    const picked = await open({
      title: "Selecionar backup GHPM",
      multiple: false,
      filters: [{ name: "Backup GHPM", extensions: ["ghpm-backup", "json"] }],
    })
    if (!picked || Array.isArray(picked)) return

    try {
      const info = await backupService.getInfo(picked)
      if (info.encrypted) {
        setPendingImport({ path: picked, info })
        return
      }
      await doImport(picked)
    } catch (e) {
      appToast.error("Backup inválido", errorMessage(e, "Não foi possível ler o ficheiro"))
    }
  }

  return (
    <>
      <Card>
        <CardHeader>
          <CardTitle>Backup e Restauração</CardTitle>
          <CardDescription>
            Exporte seus perfis, chaves e configurações para um ficheiro
            <code className="mx-1">.ghpm-backup</code>e restaure em outra máquina.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="space-y-3">
            <div className="flex items-center justify-between gap-4">
              <div>
                <Label>Incluir chaves privadas e tokens (PAT)</Label>
                <p className="text-sm text-muted-foreground">
                  Modo completo — o ficheiro será cifrado com AES-256-GCM e exigirá uma senha.
                </p>
              </div>
              <Button
                variant={includeSecrets ? "default" : "outline"}
                size="sm"
                onClick={() => setIncludeSecrets(!includeSecrets)}
              >
                {includeSecrets ? "Ativado" : "Desativado"}
              </Button>
            </div>

            {includeSecrets && (
              <div className="space-y-2">
                <Label htmlFor="backup-password" className="flex items-center gap-1">
                  <Lock className="h-3 w-3" /> Senha de criptografia
                </Label>
                <Input
                  id="backup-password"
                  type="password"
                  placeholder="Mínimo 8 caracteres"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="w-64"
                />
              </div>
            )}

            <div className="flex justify-end">
              <Button onClick={handleExport} disabled={isExporting} className="gap-2">
                <Download className="h-4 w-4" />
                {isExporting ? "Exportando..." : "Exportar Backup"}
              </Button>
            </div>
          </div>

          <div className="space-y-3 border-t border-border pt-4">
            <div className="flex items-center justify-between gap-4">
              <div>
                <Label>Restaurar de um backup</Label>
                <p className="text-sm text-muted-foreground">
                  Perfis são identificados pelo host alias: existentes são atualizados, novos são
                  criados.
                </p>
              </div>
              <Button
                variant="outline"
                onClick={handleImport}
                disabled={isImporting}
                className="gap-2"
              >
                <Upload className="h-4 w-4" />
                {isImporting ? "Importando..." : "Importar Backup"}
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <Dialog
        open={pendingImport !== null}
        onOpenChange={(open) => {
          if (!open) {
            setPendingImport(null)
            setImportPassword("")
          }
        }}
      >
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>Backup protegido com senha</DialogTitle>
            <DialogDescription>
              Este backup está cifrado (GHPM_ENC_V2). Digite a senha usada na exportação para
              restaurá-lo.
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-2">
            <Label htmlFor="import-password">Senha</Label>
            <Input
              id="import-password"
              type="password"
              value={importPassword}
              onChange={(e) => setImportPassword(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter" && pendingImport && importPassword) {
                  void doImport(pendingImport.path, importPassword)
                }
              }}
            />
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setPendingImport(null)}>
              Cancelar
            </Button>
            <Button
              disabled={!importPassword || isImporting}
              onClick={() => pendingImport && doImport(pendingImport.path, importPassword)}
            >
              {isImporting ? "Restaurando..." : "Restaurar"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  )
}
