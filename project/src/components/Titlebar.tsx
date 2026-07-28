import { getCurrentWindow } from "@tauri-apps/api/window"
import { useAppStore } from "@/stores/useAppStore"
import { Button } from "@/components/ui/button"
import { Minus, X, PanelLeft } from "lucide-react"

export function Titlebar() {
  const appWindow = getCurrentWindow()
  const { toggleSidebar } = useAppStore()

  const handleMinimize = async () => {
    await appWindow.minimize()
  }

  const handleClose = async () => {
    await appWindow.hide()
  }

  return (
    <div
      data-tauri-drag-region
      className="flex h-8 items-center bg-card px-2 select-none"
    >
      <Button
        variant="ghost"
        size="icon"
        className="h-6 w-6 shrink-0"
        onClick={toggleSidebar}
      >
        <PanelLeft className="h-3.5 w-3.5" />
      </Button>

      <div className="flex-1 flex justify-center" data-tauri-drag-region>
        <span
          className="text-xs font-semibold text-muted-foreground"
          data-tauri-drag-region
        >
          GHPM
        </span>
      </div>

      <div className="flex items-center gap-0.5">
        <Button
          variant="ghost"
          size="icon"
          className="h-6 w-6"
          onClick={handleMinimize}
        >
          <Minus className="h-3 w-3" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className="h-6 w-6"
          onClick={handleClose}
        >
          <X className="h-3 w-3" />
        </Button>
      </div>
    </div>
  )
}
