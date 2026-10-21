import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { formatLogDate } from "@/lib/utils"
import { logService } from "@/services"
import type { ActivityLog } from "@/types"
import { ScrollText } from "lucide-react"
import { useEffect, useState } from "react"

const levelColors: Record<string, string> = {
  info: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
  warn: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300",
  error: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
  debug: "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300",
}

export function LogsView() {
  const [logs, setLogs] = useState<ActivityLog[]>([])
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    logService
      .getAll()
      .then(setLogs)
      .catch(console.error)
      .finally(() => setIsLoading(false))
  }, [])

  if (isLoading) {
    return <p className="text-muted-foreground">Carregando logs...</p>
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-2">
        <ScrollText className="h-5 w-5" />
        <h2 className="text-2xl font-bold">Logs de Atividade</h2>
      </div>

      {logs.length === 0 ? (
        <Card>
          <CardContent className="py-8 text-center text-muted-foreground">
            Nenhum log registrado ainda.
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardHeader>
            <CardTitle>Atividade Recente</CardTitle>
            <CardDescription>{logs.length} registro(s) encontrado(s)</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {logs.map((log) => (
                <div key={log.id} className="flex items-start gap-3 rounded-lg border p-3 text-sm">
                  <Badge variant="secondary" className={levelColors[log.level] ?? ""}>
                    {log.level.toUpperCase()}
                  </Badge>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="font-medium">{log.category}</span>
                      <span className="text-xs text-muted-foreground">
                        {formatLogDate(log.createdAt)}
                      </span>
                    </div>
                    <p className="text-muted-foreground mt-1">{log.message}</p>
                    {log.details && (
                      <p className="text-xs text-muted-foreground mt-1 font-mono">{log.details}</p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
