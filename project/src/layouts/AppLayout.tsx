import { useAppStore } from "@/stores/useAppStore"
import { Sidebar } from "@/components/Sidebar"
import { ProfilesView } from "@/features/profiles/ProfilesView"
import { SshKeysView } from "@/features/ssh-keys/SshKeysView"
import { SettingsView } from "@/features/settings/SettingsView"
import { LogsView } from "@/features/logs/LogsView"
import { Header } from "@/components/Header"
import { Titlebar } from "@/components/Titlebar"

export function AppLayout() {
  const activeView = useAppStore((s) => s.activeView)

  return (
    <div className="flex h-screen bg-background text-foreground">
      <Sidebar />
      <div className="flex flex-1 flex-col overflow-hidden">
        <Titlebar />
        <Header />
        <main className="flex-1 overflow-y-auto p-6">
          {activeView === "profiles" && <ProfilesView />}
          {activeView === "ssh-keys" && <SshKeysView />}
          {activeView === "logs" && <LogsView />}
          {activeView === "settings" && <SettingsView />}
        </main>
      </div>
    </div>
  )
}
