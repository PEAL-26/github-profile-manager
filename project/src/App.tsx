import { ThemeProvider } from "next-themes"
import { Toaster } from "@/components/ui/toast"
import { TooltipProvider } from "@/components/ui/tooltip"
import { AppLayout } from "@/layouts/AppLayout"
import { TrayPopup } from "@/components/TrayPopup"
import { ErrorBoundary } from "@/components/ErrorBoundary"
import "./App.css"

const isTrayPopup = window.location.hash === "#tray"

function App() {
  return (
    <ErrorBoundary>
      <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
        <TooltipProvider>
          {isTrayPopup ? <TrayPopup /> : <AppLayout />}
          <Toaster />
        </TooltipProvider>
      </ThemeProvider>
    </ErrorBoundary>
  )
}

export default App
