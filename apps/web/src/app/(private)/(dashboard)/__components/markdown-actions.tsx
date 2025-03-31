'use client'

import { DeleteMarkdownAction } from '@/components/actions/delete-markdown-action'
import { SaveMarkdownAction } from '@/components/actions/save-markdown-action'
import { SidebarAction } from '@/components/actions/sidebar-action'
import { NameInput } from '@/components/inputs/name-input'
import { Text } from '@monorepo/ui'
import { useFormContext } from 'react-hook-form'

export function MarkdownActions() {
  const { control } = useFormContext()

  return (
    <>
      <div className="flex items-center h-full gap-x-6">
        <SidebarAction />

        <div className="h-full hidden lg:flex items-center justify-center gap-x-6">
          <Text>Markdown</Text>

          <div className="h-full flex items-center justify-center py-4">
            <div className="h-full w-[1px] bg-white" />
          </div>
        </div>

        <NameInput
          label="Document Name"
          name="name"
          control={control}
          placeholder="Document Name"
          disabled={false}
          required={true}
        />
      </div>

      <div className="flex items-center justify-end gap-x-2">
        <DeleteMarkdownAction markdownId="<guid>" />
        <SaveMarkdownAction markdownId="<guid>" />
      </div>
    </>
  )
}
