'use client'

import { useSidebarStore } from '@/providers/sidebar-provider'
import { Text } from '@monorepo/ui'
import { DeleteMarkdownAction } from './actions/delete-markdown-action'

export function Sidebar() {
  const isOpen = useSidebarStore((state) => state.isOpen)

  if (!isOpen) {
    return null
  }

  return (
    <aside className="bg-aside-background min-w-64 h-full p-6">
      <section className="flex flex-col gap-y-7">
        <div className="block md:hidden items-center justify-start">
          <Text>Markdown</Text>
        </div>

        <div>
          <Text size="sm" variant="subtitle">
            MY DOCUMENTS
          </Text>
        </div>
      </section>
    </aside>
  )
}
