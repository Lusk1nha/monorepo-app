import { createStore } from 'zustand/vanilla'

export type SidebarState = {
  isOpen: boolean
}

export type SidebarActions = {
  toggleSidebar: () => void
  openSidebar: () => void
  closeSidebar: () => void
}

export type SidebarStore = SidebarState & SidebarActions

export const defaultInitSidebarState: SidebarState = {
  isOpen: true,
}

export const createSidebarStore = (
  initState: SidebarState = defaultInitSidebarState,
) => {
  return createStore<SidebarStore>()((set) => ({
    ...initState,
    toggleSidebar: () => set((state) => ({ isOpen: !state.isOpen })),
    openSidebar: () => set({ isOpen: true }),
    closeSidebar: () => set({ isOpen: false }),
  }))
}
