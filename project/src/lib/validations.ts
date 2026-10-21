import { z } from "zod"

export const createProfileSchema = z.object({
  name: z.string().min(1, "Nome é obrigatório").max(50, "Nome muito longo"),
  email: z.string().email("Email inválido"),
  githubUsername: z
    .string()
    .min(1, "Username é obrigatório")
    .max(39, "Username muito longo")
    .regex(/^[a-zA-Z0-9-]+$/, "Username inválido"),
  hostAlias: z.string().min(1, "Host é obrigatório"),
  defaultBranch: z.enum(["main", "master", "develop"]),
  sshKeyId: z.string().optional(),
})

export type CreateProfileFormData = z.infer<typeof createProfileSchema>

/**
 * Verifica se um host alias já está em uso por outro perfil.
 * A comparação ignora maiúsculas/minúsculas (aliases SSH são
 * case-insensitive) e permite ignorar o próprio perfil na edição.
 */
export function isHostAliasTaken(
  hostAlias: string,
  profiles: Array<{ id?: string; hostAlias: string }>,
  excludeProfileId?: string,
): boolean {
  const normalized = hostAlias.trim().toLowerCase()
  if (!normalized) return false
  return profiles.some(
    (p) => p.id !== excludeProfileId && p.hostAlias.trim().toLowerCase() === normalized,
  )
}

export const createProfilePayloadSchema = createProfileSchema.extend({
  sshKeyId: z.string().optional(),
  defaultDirectory: z.string().optional(),
  description: z.string().optional(),
  patToken: z.string().optional(),
})

export type CreateProfilePayloadData = z.infer<typeof createProfilePayloadSchema>

export const generateKeySchema = z.object({
  name: z
    .string()
    .min(1, "Nome é obrigatório")
    .max(50, "Nome muito longo")
    .regex(/^[a-zA-Z0-9_-]+$/, "Nome inválido"),
  keyType: z.enum(["ed25519", "rsa"]),
  passphrase: z.string().optional(),
})

export type GenerateKeyFormData = z.infer<typeof generateKeySchema>
