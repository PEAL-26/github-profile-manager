import { useEffect, useState } from "react"
import { useSettingsStore } from "@/stores/useSettingsStore"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Save } from "lucide-react"

export function SettingsView() {
  const { settings, isLoading, fetchSettings, updateSettings } = useSettingsStore()
  const [localSettings, setLocalSettings] = useState(settings)

  useEffect(() => {
    fetchSettings()
  }, [fetchSettings])

  useEffect(() => {
    if (settings) setLocalSettings(settings)
  }, [settings])

  const handleSave = async () => {
    if (localSettings) {
      await updateSettings(localSettings)
    }
  }

  if (!localSettings) return <p className="text-muted-foreground">Carregando...</p>

  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">Configurações</h2>

      <Card>
        <CardHeader>
          <CardTitle>Aparência</CardTitle>
          <CardDescription>Personalize a aparência do aplicativo</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Tema</Label>
            <Select
              value={localSettings.theme}
              onValueChange={(v) =>
                v && setLocalSettings({ ...localSettings, theme: v })
              }
            >
              <SelectTrigger className="w-48">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="light">Claro</SelectItem>
                <SelectItem value="dark">Escuro</SelectItem>
                <SelectItem value="system">Sistema</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Perfil</CardTitle>
          <CardDescription>Configurações de troca automática de perfil</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <Label>Troca Automática</Label>
              <p className="text-sm text-muted-foreground">
                Ativar troca automática de perfil por diretório
              </p>
            </div>
            <Button
              variant={localSettings.autoSwitchEnabled ? "default" : "outline"}
              size="sm"
              onClick={() =>
                setLocalSettings({
                  ...localSettings,
                  autoSwitchEnabled: !localSettings.autoSwitchEnabled,
                })
              }
            >
              {localSettings.autoSwitchEnabled ? "Ativado" : "Desativado"}
            </Button>
          </div>
          <div className="flex items-center justify-between">
            <div>
              <Label>Notificar na Troca</Label>
              <p className="text-sm text-muted-foreground">
                Mostrar notificação ao trocar de perfil
              </p>
            </div>
            <Button
              variant={localSettings.notifyOnSwitch ? "default" : "outline"}
              size="sm"
              onClick={() =>
                setLocalSettings({
                  ...localSettings,
                  notifyOnSwitch: !localSettings.notifyOnSwitch,
                })
              }
            >
              {localSettings.notifyOnSwitch ? "Ativado" : "Desativado"}
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>SSH</CardTitle>
          <CardDescription>Configurações de chaves SSH</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Algoritmo Padrão</Label>
            <Select
              value={localSettings.defaultSshAlgorithm}
              onValueChange={(v) =>
                v &&
                setLocalSettings({
                  ...localSettings,
                  defaultSshAlgorithm: v,
                })
              }
            >
              <SelectTrigger className="w-48">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="ed25519">ED25519</SelectItem>
                <SelectItem value="rsa">RSA</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center justify-between">
            <div>
              <Label>Auto-start SSH Agent</Label>
              <p className="text-sm text-muted-foreground">
                Iniciar ssh-agent automaticamente
              </p>
            </div>
            <Button
              variant={localSettings.autoStartSshAgent ? "default" : "outline"}
              size="sm"
              onClick={() =>
                setLocalSettings({
                  ...localSettings,
                  autoStartSshAgent: !localSettings.autoStartSshAgent,
                })
              }
            >
              {localSettings.autoStartSshAgent ? "Ativado" : "Desativado"}
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Sistema</CardTitle>
          <CardDescription>Configurações do sistema</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Nível de Log</Label>
            <Select
              value={localSettings.logLevel}
              onValueChange={(v) =>
                v && setLocalSettings({ ...localSettings, logLevel: v })
              }
            >
              <SelectTrigger className="w-48">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="debug">Debug</SelectItem>
                <SelectItem value="info">Info</SelectItem>
                <SelectItem value="warn">Warning</SelectItem>
                <SelectItem value="error">Error</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label>Retenção de Logs (dias)</Label>
            <Input
              type="number"
              className="w-32"
              value={localSettings.logRetentionDays}
              onChange={(e) =>
                setLocalSettings({
                  ...localSettings,
                  logRetentionDays: Number(e.target.value),
                })
              }
              min={1}
              max={365}
            />
          </div>
        </CardContent>
      </Card>

      <div className="flex justify-end">
        <Button onClick={handleSave} disabled={isLoading} className="gap-2">
          <Save className="h-4 w-4" />
          {isLoading ? "Salvando..." : "Salvar Configurações"}
        </Button>
      </div>
    </div>
  )
}
