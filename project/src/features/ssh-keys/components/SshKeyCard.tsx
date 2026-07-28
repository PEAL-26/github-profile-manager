import { useState } from "react"
import { useProfileStore } from "@/stores/useProfileStore"
import type { SshKey, SshTestResult } from "@/types"
import { sshService } from "@/services"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import {
  KeyRound,
  Trash2,
  Copy,
  Check,
  Wifi,
  WifiOff,
  Link,
  FileKey,
  FileKey2,
} from "lucide-react"
import { DeleteKeyDialog } from "./DeleteKeyDialog"
import { LinkProfileDialog } from "./LinkProfileDialog"

interface SshKeyCardProps {
  keyData: SshKey
}

export function SshKeyCard({ keyData }: SshKeyCardProps) {
  const { profiles } = useProfileStore()
  const [deleteOpen, setDeleteOpen] = useState(false)
  const [linkOpen, setLinkOpen] = useState(false)
  const [copiedFingerprint, setCopiedFingerprint] = useState(false)
  const [copiedPrivate, setCopiedPrivate] = useState(false)
  const [copiedPublic, setCopiedPublic] = useState(false)
  const [testResult, setTestResult] = useState<SshTestResult | null>(null)
  const [isTesting, setIsTesting] = useState(false)

  const linkedProfile = profiles.find((p) => p.sshKeyId === keyData.id)

  const handleCopyFingerprint = () => {
    navigator.clipboard.writeText(keyData.fingerprint)
    setCopiedFingerprint(true)
    setTimeout(() => setCopiedFingerprint(false), 2000)
  }

  const handleCopyPrivateKey = async () => {
    try {
      const content = await sshService.readKeyContent(keyData.privateKeyPath)
      await navigator.clipboard.writeText(content)
      setCopiedPrivate(true)
      setTimeout(() => setCopiedPrivate(false), 2000)
    } catch {
      // fallback: copy path
      await navigator.clipboard.writeText(keyData.privateKeyPath)
      setCopiedPrivate(true)
      setTimeout(() => setCopiedPrivate(false), 2000)
    }
  }

  const handleCopyPublicKey = async () => {
    try {
      const content = await sshService.readKeyContent(keyData.publicKeyPath)
      await navigator.clipboard.writeText(content)
      setCopiedPublic(true)
      setTimeout(() => setCopiedPublic(false), 2000)
    } catch {
      await navigator.clipboard.writeText(keyData.publicKeyPath)
      setCopiedPublic(true)
      setTimeout(() => setCopiedPublic(false), 2000)
    }
  }

  const handleTestConnection = async () => {
    setIsTesting(true)
    setTestResult(null)
    try {
      const result = await sshService.testConnection("github.com")
      setTestResult(result)
    } catch {
      setTestResult({
        success: false,
        rawOutput: "Erro ao testar conexão",
        latencyMs: 0,
      })
    } finally {
      setIsTesting(false)
    }
  }

  return (
    <>
      <Card>
        <CardContent className="flex items-center justify-between py-4">
          <div className="flex items-center gap-4">
            <KeyRound className="h-8 w-8 text-muted-foreground" />
            <div className="space-y-1">
              <p className="font-medium">{keyData.name}</p>
              <div className="flex items-center gap-2">
                <p className="text-xs text-muted-foreground font-mono">
                  {keyData.fingerprint}
                </p>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-5 w-5"
                  onClick={handleCopyFingerprint}
                >
                  {copiedFingerprint ? (
                    <Check className="h-3 w-3 text-green-500" />
                  ) : (
                    <Copy className="h-3 w-3" />
                  )}
                </Button>
              </div>
              <div className="flex items-center gap-2">
                <Button
                  variant="ghost"
                  size="sm"
                  className="h-6 gap-1 text-xs"
                  onClick={handleCopyPrivateKey}
                >
                  {copiedPrivate ? (
                    <Check className="h-3 w-3 text-green-500" />
                  ) : (
                    <FileKey className="h-3 w-3" />
                  )}
                  Privada
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  className="h-6 gap-1 text-xs"
                  onClick={handleCopyPublicKey}
                >
                  {copiedPublic ? (
                    <Check className="h-3 w-3 text-green-500" />
                  ) : (
                    <FileKey2 className="h-3 w-3" />
                  )}
                  Pública
                </Button>
              </div>
              {testResult && (
                <div className="flex items-center gap-2 text-xs">
                  {testResult.success ? (
                    <>
                      <Wifi className="h-3 w-3 text-green-500" />
                      <span className="text-green-500">
                        Conectado{testResult.username ? ` (${testResult.username})` : ""}
                        {testResult.latencyMs > 0 ? ` — ${testResult.latencyMs}ms` : ""}
                      </span>
                    </>
                  ) : (
                    <>
                      <WifiOff className="h-3 w-3 text-destructive" />
                      <span className="text-destructive">Falha na conexão</span>
                    </>
                  )}
                </div>
              )}
              {linkedProfile && (
                <div className="flex items-center gap-1 text-xs text-muted-foreground">
                  <Link className="h-3 w-3" />
                  <span>Vinculado a: {linkedProfile.name}</span>
                </div>
              )}
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Badge variant="outline">{keyData.keyType.toUpperCase()}</Badge>
            <Button
              variant="outline"
              size="sm"
              onClick={handleTestConnection}
              disabled={isTesting}
            >
              {isTesting ? "Testando..." : "Testar"}
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8"
              onClick={() => setLinkOpen(true)}
            >
              <Link className="h-4 w-4" />
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onClick={() => setDeleteOpen(true)}
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        </CardContent>
      </Card>

      <DeleteKeyDialog open={deleteOpen} onOpenChange={setDeleteOpen} keyData={keyData} />
      <LinkProfileDialog open={linkOpen} onOpenChange={setLinkOpen} keyId={keyData.id} keyName={keyData.name} />
    </>
  )
}
