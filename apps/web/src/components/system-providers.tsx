'use client'

import { ThemeProvider } from './providers/theme-provider'
import { SessionProvider } from 'next-auth/react'

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
      <SessionProvider>{children}</SessionProvider>
    </ThemeProvider>
  )
}
