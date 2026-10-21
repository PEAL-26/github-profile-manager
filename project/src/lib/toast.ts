import { toast } from "@/components/ui/toast"
import { logService } from "@/services"

/**
 * Envia o erro para o ghpm_errors.txt no backend (fire-and-forget).
 * Silencioso quando não há backend (testes, browser puro).
 */
function logErrorToFile(message: string) {
  try {
    void logService.logError(message).catch(() => {})
  } catch {
    // sem backend disponível — ignora
  }
}

/**
 * Feedback visual padronizado da aplicação.
 * Erros são também registados no ficheiro ghpm_errors.txt do backend.
 */
export const appToast = {
  success: (title: string, description?: string) => {
    toast.add({ type: "success", title, description })
  },
  error: (title: string, description?: string) => {
    logErrorToFile(description ? `${title}: ${description}` : title)
    toast.add({ type: "error", title, description, timeout: 8000 })
  },
  info: (title: string, description?: string) => {
    toast.add({ type: "info", title, description })
  },
}
