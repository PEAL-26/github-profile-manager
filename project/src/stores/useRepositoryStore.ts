import { appToast } from "@/lib/toast"
import { errorMessage } from "@/lib/utils"
import { repositoryService } from "@/services"
import type { Repository, ScanRoot } from "@/types"
import { emit } from "@tauri-apps/api/event"
import { create } from "zustand"
import { useProfileStore } from "./useProfileStore"

interface RepositoryState {
  repositories: Repository[]
  roots: ScanRoot[]
  isLoading: boolean
  isScanning: boolean
  error: string | null
  fetchRepositories: () => Promise<void>
  fetchRoots: () => Promise<void>
  scan: () => Promise<void>
  addRoot: (path: string) => Promise<void>
  removeRoot: (id: string) => Promise<void>
  applyProfile: (repositoryId: string, profileId: string) => Promise<void>
}

export const useRepositoryStore = create<RepositoryState>((set, get) => ({
  repositories: [],
  roots: [],
  isLoading: false,
  isScanning: false,
  error: null,

  fetchRepositories: async () => {
    set({ isLoading: true, error: null })
    try {
      const repositories = await repositoryService.list()
      set({ repositories })
    } catch (e) {
      const message = errorMessage(e, "Falha ao carregar repositórios")
      set({ error: message })
      appToast.error("Erro ao carregar repositórios", message)
    } finally {
      set({ isLoading: false })
    }
  },

  fetchRoots: async () => {
    try {
      const roots = await repositoryService.listRoots()
      set({ roots })
    } catch (e) {
      appToast.error("Erro ao carregar pastas de varrimento", errorMessage(e, "Falha desconhecida"))
    }
  },

  scan: async () => {
    set({ isScanning: true, error: null })
    try {
      const repositories = await repositoryService.scan()
      set({ repositories })
      appToast.success(
        "Varredura concluída",
        `${repositories.length} repositório(s) mapeado(s)`,
      )
    } catch (e) {
      const message = errorMessage(e, "Falha ao escanear repositórios")
      set({ error: message })
      appToast.error("Erro na varredura", message)
      throw new Error(message)
    } finally {
      set({ isScanning: false })
    }
  },

  addRoot: async (path: string) => {
    try {
      await repositoryService.addRoot(path)
      await get().fetchRoots()
      appToast.success("Pasta adicionada", path)
    } catch (e) {
      appToast.error("Erro ao adicionar pasta", errorMessage(e, "Falha desconhecida"))
      throw new Error(errorMessage(e, "Falha desconhecida"))
    }
  },

  removeRoot: async (id: string) => {
    try {
      await repositoryService.removeRoot(id)
      await get().fetchRoots()
      appToast.success("Pasta removida da varredura")
    } catch (e) {
      appToast.error("Erro ao remover pasta", errorMessage(e, "Falha desconhecida"))
      throw new Error(errorMessage(e, "Falha desconhecida"))
    }
  },

  applyProfile: async (repositoryId: string, profileId: string) => {
    set({ isLoading: true, error: null })
    try {
      const updated = await repositoryService.applyProfile(repositoryId, profileId)
      set({
        repositories: get().repositories.map((r) => (r.id === updated.id ? updated : r)),
      })
      // A ativação global muda a identidade Git — reflete nos perfis e notifica vistas.
      await useProfileStore.getState().fetchProfiles()
      await emit("profile-changed")
      appToast.success(
        "Perfil aplicado ao repositório",
        `${updated.profileName ?? ""} — ${updated.name}`,
      )
    } catch (e) {
      const message = errorMessage(e, "Falha ao aplicar perfil")
      set({ error: message })
      appToast.error("Erro ao aplicar perfil", message)
      throw new Error(message)
    } finally {
      set({ isLoading: false })
    }
  },
}))
