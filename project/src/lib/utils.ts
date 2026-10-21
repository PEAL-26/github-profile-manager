import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * Extrai uma mensagem legível de um erro vindo do invoke do Tauri.
 * O Tauri rejeita com objetos `{ code, message }`, não com Error.
 */
export function errorMessage(e: unknown, fallback: string): string {
  if (e instanceof Error && e.message) return e.message
  if (typeof e === "string" && e) return e
  if (
    typeof e === "object" &&
    e !== null &&
    "message" in e &&
    typeof (e as { message: unknown }).message === "string" &&
    (e as { message: string }).message
  ) {
    return (e as { message: string }).message
  }
  return fallback
}

/**
 * Formata data/hora de um log para exibição (dd/mm/aaaa hh:mm:ss).
 * Aceita tanto `createdAt` (camelCase atual) quanto `created_at` (formato
 * legado) e degrada para "—" quando o valor é ausente/inválido, em vez de
 * renderizar "Invalid Date".
 */
export function formatLogDate(value: string | undefined, locale = "pt-BR"): string {
  if (!value) return "—"
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return "—"
  return date.toLocaleString(locale)
}
