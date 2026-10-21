import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import { useProfileStore } from "@/stores/useProfileStore"
import { LogicalSize } from "@tauri-apps/api/dpi"
import { listen } from "@tauri-apps/api/event"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { ChevronDown, X } from "lucide-react"
import { useEffect, useRef, useState } from "react"

export function TrayPopup() {
  const { profiles, activeProfile, fetchProfiles, setActiveProfile } = useProfileStore()
  const [dropdownOpen, setDropdownOpen] = useState(false)
  const appWindow = getCurrentWindow()
  const dropdownRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    fetchProfiles()
    const unlisten = listen("profile-changed", () => {
      fetchProfiles()
    })
    return () => {
      unlisten.then((fn) => fn())
    }
  }, [fetchProfiles])

  useEffect(() => {
    const resize = async () => {
      if (dropdownOpen) {
        const itemCount = profiles.length || 1
        const height = 40 + itemCount * 32 + 8
        await appWindow.setSize(new LogicalSize(250, height))
      } else {
        await appWindow.setSize(new LogicalSize(250, 40))
      }
    }
    resize()
  }, [dropdownOpen, profiles.length, appWindow])

  const handleSelectProfile = async (id: string) => {
    try {
      await setActiveProfile(id)
    } catch {
      // Erro já exibido como toast pelo store.
    }
    setDropdownOpen(false)
    await fetchProfiles()
  }

  const handleClose = async () => {
    await appWindow.hide()
  }

  const handleDropdownBlur = (e: React.FocusEvent) => {
    if (!dropdownRef.current?.contains(e.relatedTarget as Node)) {
      setDropdownOpen(false)
    }
  }

  return (
    <div data-tauri-drag-region className="h-full bg-background text-foreground flex flex-col">
      <div data-tauri-drag-region className="flex items-center justify-between px-2 py-1.5">
        <div className="flex items-center gap-1 min-w-0 flex-1" data-tauri-drag-region>
          <Button
            variant="ghost"
            size="sm"
            className="flex items-center gap-1 h-6 px-2 font-medium text-xs justify-start min-w-0"
            onClick={() => setDropdownOpen(!dropdownOpen)}
          >
            <span className="truncate max-w-[180px]" data-tauri-drag-region>
              {activeProfile?.name || "Nenhum perfil"}
            </span>
            <ChevronDown
              className={cn("h-3 w-3 shrink-0 transition-transform", dropdownOpen && "rotate-180")}
            />
          </Button>
        </div>
        <Button variant="ghost" size="icon" className="h-5 w-5 shrink-0" onClick={handleClose}>
          <X className="h-3 w-3" />
        </Button>
      </div>

      {dropdownOpen && (
        <div
          ref={dropdownRef}
          className="flex flex-col overflow-y-auto border-t border-border"
          onBlur={handleDropdownBlur}
          tabIndex={-1}
        >
          {profiles.map((profile) => (
            <Button
              key={profile.id}
              variant="ghost"
              className={cn(
                "justify-start h-8 px-3 rounded-none text-xs",
                profile.isActive && "bg-accent text-accent-foreground font-medium",
              )}
              onClick={() => handleSelectProfile(profile.id)}
            >
              <span className="truncate">{profile.name}</span>
            </Button>
          ))}
          {profiles.length === 0 && (
            <p className="text-xs text-muted-foreground text-center py-2 px-2">
              Nenhum perfil cadastrado
            </p>
          )}
        </div>
      )}
    </div>
  )
}
