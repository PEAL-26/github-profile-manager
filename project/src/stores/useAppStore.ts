import { create } from "zustand"

type ActiveView = "profiles" | "ssh-keys" | "settings" | "logs"

interface AppState {
  activeView: ActiveView
  sidebarOpen: boolean
  setActiveView: (view: ActiveView) => void
  toggleSidebar: () => void
}

export const useAppStore = create<AppState>((set) => ({
  activeView: "profiles",
  sidebarOpen: false,
  setActiveView: (view: ActiveView) => set({ activeView: view }),
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
}))
