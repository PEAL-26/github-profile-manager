import { errorMessage, formatLogDate } from "@/lib/utils"
import { isHostAliasTaken } from "@/lib/validations"
import { describe, expect, it } from "vitest"

describe("isHostAliasTaken", () => {
  const profiles = [
    { id: "a", hostAlias: "github-trabalho" },
    { id: "b", hostAlias: "github-pessoal" },
  ]

  it("detects duplicate alias", () => {
    expect(isHostAliasTaken("github-trabalho", profiles)).toBe(true)
  })

  it("ignores case differences", () => {
    expect(isHostAliasTaken("GitHub-Trabalho", profiles)).toBe(true)
  })

  it("allows new aliases", () => {
    expect(isHostAliasTaken("github-novo", profiles)).toBe(false)
  })

  it("allows empty alias to pass (required field handles it)", () => {
    expect(isHostAliasTaken("", profiles)).toBe(false)
  })

  it("excludes the profile being edited", () => {
    expect(isHostAliasTaken("github-trabalho", profiles, "a")).toBe(false)
    expect(isHostAliasTaken("github-trabalho", profiles, "b")).toBe(true)
  })

  it("trims whitespace before comparing", () => {
    expect(isHostAliasTaken("  github-pessoal  ", profiles)).toBe(true)
  })
})

describe("formatLogDate", () => {
  it("formats a valid ISO date with date and time", () => {
    const result = formatLogDate("2026-10-20T14:30:00Z")
    expect(result).not.toBe("—")
    expect(result).not.toContain("Invalid")
    // deve conter dia, mês e ano (timezone local pode mudar a hora)
    expect(result).toMatch(/\d{2}\/\d{2}\/\d{4}/)
  })

  it("returns placeholder for undefined", () => {
    expect(formatLogDate(undefined)).toBe("—")
  })

  it("returns placeholder for unparseable values instead of Invalid Date", () => {
    expect(formatLogDate("not-a-date")).toBe("—")
  })
})

describe("errorMessage", () => {
  it("extracts message from Error", () => {
    expect(errorMessage(new Error("boom"), "fallback")).toBe("boom")
  })

  it("extracts message from Tauri AppError object", () => {
    const tauriError = { code: "VALIDATION_ERROR", message: "alias duplicado" }
    expect(errorMessage(tauriError, "fallback")).toBe("alias duplicado")
  })

  it("extracts plain string errors", () => {
    expect(errorMessage("algo falhou", "fallback")).toBe("algo falhou")
  })

  it("falls back when shape is unknown", () => {
    expect(errorMessage(undefined, "fallback")).toBe("fallback")
    expect(errorMessage({}, "fallback")).toBe("fallback")
    expect(errorMessage({ message: "" }, "fallback")).toBe("fallback")
  })
})
