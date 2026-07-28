import { describe, it, expect } from "vitest"
import { createProfileSchema, generateKeySchema } from "@/lib/validations"

describe("createProfileSchema", () => {
  it("validates correct profile data", () => {
    const result = createProfileSchema.safeParse({
      name: "Trabalho",
      email: "work@email.com",
      githubUsername: "work-user",
      hostAlias: "github.com",
      defaultBranch: "main",
    })
    expect(result.success).toBe(true)
  })

  it("rejects invalid email", () => {
    const result = createProfileSchema.safeParse({
      name: "Test",
      email: "not-an-email",
      githubUsername: "user",
      hostAlias: "github.com",
      defaultBranch: "main",
    })
    expect(result.success).toBe(false)
  })

  it("rejects invalid username", () => {
    const result = createProfileSchema.safeParse({
      name: "Test",
      email: "test@email.com",
      githubUsername: "invalid user!",
      hostAlias: "github.com",
      defaultBranch: "main",
    })
    expect(result.success).toBe(false)
  })
})

describe("generateKeySchema", () => {
  it("validates correct key data", () => {
    const result = generateKeySchema.safeParse({
      name: "my-key",
      keyType: "ed25519",
    })
    expect(result.success).toBe(true)
  })

  it("rejects invalid key name", () => {
    const result = generateKeySchema.safeParse({
      name: "invalid key!",
      keyType: "ed25519",
    })
    expect(result.success).toBe(false)
  })

  it("rejects invalid key type", () => {
    const result = generateKeySchema.safeParse({
      name: "my-key",
      keyType: "dsa",
    })
    expect(result.success).toBe(false)
  })
})
