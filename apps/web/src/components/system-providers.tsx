'use client'

import { SidebarStoreProvider } from '@/providers/sidebar-provider'
import { ThemeProvider } from '../providers/theme-provider'

interface SystemProvidersProps {
  children: React.ReactNode
}

export function SystemProviders(props: Readonly<SystemProvidersProps>) {
  const { children } = props

  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      disableTransitionOnChange
      storageKey="web::theme"
    >
      <SidebarStoreProvider>{children}</SidebarStoreProvider>
    </ThemeProvider>
  )
}
