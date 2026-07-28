import { create } from "zustand"
import type { Settings } from "@/types"
import { settingsService } from "@/services"

interface SettingsState {
  settings: Settings | null
  isLoading: boolean
  error: string | null
  fetchSettings: () => Promise<void>
  updateSettings: (settings: Settings) => Promise<void>
}

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: null,
  isLoading: false,
  error: null,

  fetchSettings: async () => {
    set({ isLoading: true, error: null })
    try {
      const settings = await settingsService.get()
      set({ settings })
    } catch (e) {
      set({
        error: e instanceof Error ? e.message : "Failed to fetch settings",
      })
    } finally {
      set({ isLoading: false })
    }
  },

  updateSettings: async (settings: Settings) => {
    set({ isLoading: true, error: null })
    try {
      await settingsService.update(settings)
      set({ settings })
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to update settings" })
    } finally {
      set({ isLoading: false })
    }
  },
}))
