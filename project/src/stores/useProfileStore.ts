import { create } from "zustand"
import { emit } from "@tauri-apps/api/event"
import type { Profile } from "@/types"
import { profileService } from "@/services"

interface ProfileState {
  profiles: Profile[]
  activeProfile: Profile | null
  isLoading: boolean
  error: string | null
  fetchProfiles: () => Promise<void>
  setActiveProfile: (id: string) => Promise<void>
  createProfile: (payload: Parameters<typeof profileService.create>[0]) => Promise<void>
  updateProfile: (id: string, payload: Parameters<typeof profileService.update>[1]) => Promise<void>
  deleteProfile: (id: string) => Promise<void>
}

export const useProfileStore = create<ProfileState>((set, get) => ({
  profiles: [],
  activeProfile: null,
  isLoading: false,
  error: null,

  fetchProfiles: async () => {
    set({ isLoading: true, error: null })
    try {
      const profiles = await profileService.getAll()
      const active = profiles.find((p) => p.isActive) || null
      set({ profiles, activeProfile: active })
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to fetch profiles" })
    } finally {
      set({ isLoading: false })
    }
  },

  setActiveProfile: async (id: string) => {
    set({ isLoading: true, error: null })
    try {
      await profileService.setActive(id)
      await get().fetchProfiles()
      await emit("profile-changed")
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to set active profile" })
    } finally {
      set({ isLoading: false })
    }
  },

  createProfile: async (payload) => {
    set({ isLoading: true, error: null })
    try {
      await profileService.create(payload)
      await get().fetchProfiles()
      await emit("profile-changed")
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to create profile" })
    } finally {
      set({ isLoading: false })
    }
  },

  updateProfile: async (id, payload) => {
    set({ isLoading: true, error: null })
    try {
      await profileService.update(id, payload)
      await get().fetchProfiles()
      await emit("profile-changed")
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to update profile" })
    } finally {
      set({ isLoading: false })
    }
  },

  deleteProfile: async (id: string) => {
    set({ isLoading: true, error: null })
    try {
      await profileService.delete(id)
      await get().fetchProfiles()
      await emit("profile-changed")
    } catch (e) {
      set({ error: e instanceof Error ? e.message : "Failed to delete profile" })
    } finally {
      set({ isLoading: false })
    }
  },
}))
