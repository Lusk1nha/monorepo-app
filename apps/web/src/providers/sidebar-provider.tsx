'use client'

import { createContext, useContext, useRef } from 'react'
import { useStore } from 'zustand'

import { createSidebarStore, SidebarState, type SidebarStore } from '@/stores/sidebar-store'

export type SidebarStoreApi = ReturnType<typeof createSidebarStore>

export const SidebarStoreContext = createContext<SidebarStoreApi | undefined>(
  undefined,
)

export interface SidebarStoreProviderProps {
  children: React.ReactNode
  initialState?: SidebarState
}

export const SidebarStoreProvider = ({
  children,
  initialState,
}: Readonly<SidebarStoreProviderProps>) => {
  const storeRef = useRef<SidebarStoreApi | null>(null)
  if (storeRef.current === null) {
    storeRef.current = createSidebarStore(initialState)
  }

  return (
    <SidebarStoreContext.Provider value={storeRef.current}>
      {children}
    </SidebarStoreContext.Provider>
  )
}

export const useSidebarStore = <T,>(
  selector: (store: SidebarStore) => T,
): T => {
  const sidebarStoreContext = useContext(SidebarStoreContext)

  if (!sidebarStoreContext) {
    throw new Error(`useSidebarStore must be used within SidebarStoreProvider`)
  }

  return useStore(sidebarStoreContext, selector)
}
