import { create } from "zustand"
import type { SshKey } from "@/types"
import { sshService } from "@/services"

interface SshKeyState {
  keys: SshKey[]
  isLoading: boolean
  error: string | null
  fetchKeys: () => Promise<void>
  generateKey: (name: string, keyType: string, passphrase?: string) => Promise<void>
  deleteKey: (id: string) => Promise<void>
}

export const useSshKeyStore = create<SshKeyState>((set, get) => ({
  keys: [],
  isLoading: false,
  error: null,

  fetchKeys: async () => {
    set({ isLoading: true, error: null })
    try {
      const keys = await sshService.listKeys()
      set({ keys })
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to fetch SSH keys" })
    } finally {
      set({ isLoading: false })
    }
  },

  generateKey: async (name: string, keyType: string, passphrase?: string) => {
    set({ isLoading: true, error: null })
    try {
      await sshService.generateKey(name, keyType, passphrase)
      await get().fetchKeys()
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to generate SSH key" })
    } finally {
      set({ isLoading: false })
    }
  },

  deleteKey: async (id: string) => {
    set({ isLoading: true, error: null })
    try {
      await sshService.deleteKey(id)
      await get().fetchKeys()
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to delete SSH key" })
    } finally {
      set({ isLoading: false })
    }
  },
}))
