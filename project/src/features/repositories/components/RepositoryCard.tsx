import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { REPOSITORY_STATUS_LABELS, repositoryStatus } from "@/lib/utils"
import type { Profile, Repository } from "@/types"
import { Check, Mail, Monitor, TriangleAlert, Wrench } from "lucide-react"
import { useState } from "react"

interface RepositoryCardProps {
  repository: Repository
  profiles: Profile[]
  isApplying: boolean
  onApply: (repositoryId: string, profileId: string) => Promise<void>
}

export function RepositoryCard({
  repository,
  profiles,
  isApplying,
  onApply,
}: RepositoryCardProps) {
  const status = repositoryStatus(repository)
  const [selected, setSelected] = useState(
    repository.profileId ?? repository.suggestedProfileId ?? "",
  )

  const handleApply = async () => {
    if (!selected) return
    try {
      await onApply(repository.id, selected)
    } catch {
      // store já exibiu o toast de erro
    }
  }

  const badge =
    status === "synced" ? (
      <Badge
        variant="outline"
        className="border-transparent bg-emerald-500/15 text-emerald-600 dark:text-emerald-400"
      >
        <Check className="h-3 w-3" />
        {REPOSITORY_STATUS_LABELS.synced}
      </Badge>
    ) : status === "divergent" ? (
      <Badge
        variant="outline"
        className="border-transparent bg-amber-500/15 text-amber-600 dark:text-amber-400"
      >
        <Wrench className="h-3 w-3" />
        {REPOSITORY_STATUS_LABELS.divergent}
      </Badge>
    ) : (
      <Badge variant="secondary">{REPOSITORY_STATUS_LABELS.unmapped}</Badge>
    )

  return (
    <Card>
      <CardContent className="space-y-3">
        <div className="flex items-start justify-between gap-3">
          <div className="min-w-0">
            <div className="flex items-center gap-2">
              <h3 className="truncate font-semibold">{repository.name}</h3>
              {badge}
            </div>
            <p
              className="truncate font-mono text-xs text-muted-foreground"
              title={repository.path}
            >
              {repository.path}
            </p>
          </div>
        </div>

        <div className="space-y-1 text-sm text-muted-foreground">
          <p className="truncate" title={repository.remoteUrl ?? ""}>
            Remoto: {repository.remoteUrl ?? "sem remoto configurado"}
          </p>
          <p className="truncate">
            <Mail className="mr-1 inline h-3 w-3" />
            {repository.currentGitEmail
              ? `${repository.currentGitName ?? "?"} <${repository.currentGitEmail}>`
              : "sem identidade local"}
          </p>
        </div>

        {repository.suggestionConflict && status !== "synced" && (
          <p className="flex items-start gap-1.5 rounded-md bg-amber-500/10 px-2 py-1.5 text-xs text-amber-600 dark:text-amber-400">
            <TriangleAlert className="mt-0.5 h-3 w-3 shrink-0" />
            Conflito: o e-mail e o remoto apontam para perfis diferentes —
            confirme a escolha.
          </p>
        )}

        {status === "synced" ? (
          <div className="flex items-center justify-between gap-3 border-t border-border pt-3">
            <span className="text-sm">
              Perfil ativo: <strong>{repository.profileName}</strong>
            </span>
            <Badge
              variant="outline"
              className="border-transparent bg-emerald-500/15 text-emerald-600 dark:text-emerald-400"
            >
              <Monitor className="h-3 w-3" />
              Global + local
            </Badge>
          </div>
        ) : (
          <div className="flex items-center justify-between gap-3 border-t border-border pt-3">
            <Select value={selected} onValueChange={(v) => v && setSelected(v)}>
              <SelectTrigger className="w-56" title="Perfil a aplicar">
                <SelectValue placeholder="Selecionar perfil..." />
              </SelectTrigger>
              <SelectContent>
                {profiles.map((profile) => (
                  <SelectItem key={profile.id} value={profile.id}>
                    {profile.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button
              size="sm"
              className="gap-2"
              disabled={!selected || isApplying}
              onClick={handleApply}
            >
              {isApplying
                ? "Aplicando..."
                : status === "divergent"
                  ? "Corrigir"
                  : "Ativar perfil"}
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
