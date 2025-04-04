'use client'

import { useSidebarStore } from '@/providers/sidebar-provider'

import { RedirectNewDocument } from '../actions/redirect-new-document'
import { ThemeSwitch } from '../inputs/theme-switch'
import { HeaderSidebar } from './header-sidebar'

export function Sidebar() {
  const isOpen = useSidebarStore((state) => state.isOpen)

  if (!isOpen) {
    return null
  }

  return (
    <aside className='bg-aside-background min-w-64 h-full flex flex-col justify-between px-6 pb-6'>
      <div className='w-full h-full flex flex-col'>
        <HeaderSidebar />
        <RedirectNewDocument />
      </div>

      <div className='flex items-center justify-start'>
        <ThemeSwitch />
      </div>
    </aside>
  )
}
