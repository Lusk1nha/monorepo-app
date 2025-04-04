'use client'

import { cn } from '@/lib/utils'
import { useSidebarStore } from '@/providers/sidebar-provider'
import { Button } from '@monorepo/ui'
import { Menu, X } from 'lucide-react'

export function SidebarAction() {
  const isOpen = useSidebarStore((state) => state.isOpen)
  const toggleSidebar = useSidebarStore((state) => state.toggleSidebar)

  const actionLabel = isOpen ? 'Close sidebar' : 'Open sidebar'
  const iconSize = 'size-6 sm:size-8'
  const buttonSize = 'h-14 w-14 sm:h-header-md sm:w-header-md'

  return (
    <Button
      type='button'
      className={cn(buttonSize)}
      variant='menu'
      onClick={toggleSidebar}
      title={actionLabel}
      aria-label={actionLabel}
      aria-expanded={isOpen}
    >
      {isOpen ? <X className={iconSize} aria-hidden='true' /> : <Menu className={iconSize} aria-hidden='true' />}
    </Button>
  )
}
