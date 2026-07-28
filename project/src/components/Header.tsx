import { useState } from "react"
import { useProfileStore } from "@/stores/useProfileStore"
import { useTheme } from "next-themes"
import { Button } from "@/components/ui/button"
import { Moon, Sun, Monitor, CircleHelp } from "lucide-react"
import { HelpDialog } from "./HelpDialog"

export function Header() {
  const activeProfile = useProfileStore((s) => s.activeProfile)
  const { theme, setTheme } = useTheme()
  const [helpOpen, setHelpOpen] = useState(false)

  const cycleTheme = () => {
    if (theme === "light") setTheme("dark")
    else if (theme === "dark") setTheme("system")
    else setTheme("light")
  }

  const ThemeIcon = theme === "dark" ? Moon : theme === "light" ? Sun : Monitor

  return (
    <>
      <header className="flex h-14 items-center justify-between border-b border-border bg-card px-4">
        <div className="flex items-center gap-3">
          {activeProfile ? (
            <>
              <span className="text-sm font-medium">{activeProfile.name}</span>
              <span className="text-xs text-muted-foreground">{activeProfile.email}</span>
            </>
          ) : (
            <span className="text-sm text-muted-foreground">Nenhum perfil ativo</span>
          )}
        </div>
        <div className="flex items-center gap-1">
          <Button variant="ghost" size="icon" onClick={() => setHelpOpen(true)}>
            <CircleHelp className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" onClick={cycleTheme}>
            <ThemeIcon className="h-4 w-4" />
          </Button>
        </div>
      </header>
      <HelpDialog open={helpOpen} onOpenChange={setHelpOpen} />
    </>
  )
}
