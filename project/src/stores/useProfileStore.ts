import { appToast } from "@/lib/toast"
import { errorMessage } from "@/lib/utils"
import { profileService } from "@/services"
import type { Profile } from "@/types"
import { emit } from "@tauri-apps/api/event"
import { create } from "zustand"

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
      const message = errorMessage(e, "Falha ao carregar perfis")
      set({ error: message })
      appToast.error("Erro ao carregar perfis", message)
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
      const active = get().activeProfile
      appToast.success("Perfil ativado", active?.name)
    } catch (e) {
      const message = errorMessage(e, "Falha ao ativar perfil")
      set({ error: message })
      appToast.error("Erro ao ativar perfil", message)
      throw new Error(message)
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
      appToast.success("Perfil criado com sucesso", payload.name)
    } catch (e) {
      const message = errorMessage(e, "Falha ao criar perfil")
      set({ error: message })
      appToast.error("Erro ao criar perfil", message)
      throw new Error(message)
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
      appToast.success("Perfil atualizado com sucesso", payload.name)
    } catch (e) {
      const message = errorMessage(e, "Falha ao atualizar perfil")
      set({ error: message })
      appToast.error("Erro ao atualizar perfil", message)
      throw new Error(message)
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
      appToast.success("Perfil excluído com sucesso")
    } catch (e) {
      const message = errorMessage(e, "Falha ao excluir perfil")
      set({ error: message })
      appToast.error("Erro ao excluir perfil", message)
      throw new Error(message)
    } finally {
      set({ isLoading: false })
    }
  },
}))
