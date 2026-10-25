import { useAppStore } from "@/stores/useAppStore"
import { Button } from "@/components/ui/button"
import { FolderGit2, KeyRound, User, Settings, ScrollText } from "lucide-react"
import { cn } from "@/lib/utils"

const navItems = [
  { id: "profiles" as const, label: "Perfis", icon: User },
  { id: "repositories" as const, label: "Repositórios", icon: FolderGit2 },
  { id: "ssh-keys" as const, label: "Chaves SSH", icon: KeyRound },
  { id: "logs" as const, label: "Logs", icon: ScrollText },
  { id: "settings" as const, label: "Configurações", icon: Settings },
]

export function Sidebar() {
  const { activeView, setActiveView, sidebarOpen } = useAppStore()

  return (
    <aside
      className={cn(
        "flex flex-col border-r border-border bg-card overflow-hidden transition-all duration-300 ease-in-out",
        sidebarOpen ? "w-64" : "w-0 border-r-0"
      )}
    >
      <nav className="flex flex-1 flex-col gap-1 p-2 pt-2">
        {navItems.map((item) => (
          <Button
            key={item.id}
            variant={activeView === item.id ? "secondary" : "ghost"}
            className="justify-start gap-2"
            onClick={() => setActiveView(item.id)}
          >
            <item.icon className="h-4 w-4" />
            <span className="whitespace-nowrap">{item.label}</span>
          </Button>
        ))}
      </nav>
    </aside>
  )
}
