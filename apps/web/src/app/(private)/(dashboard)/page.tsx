import { HeaderDashboard } from './__components/header-dashboard'

import { NewMarkdownForm } from '@/components/forms/new-markdown-form'
import { MarkdownActions } from './__components/markdown-actions'

export default function DashboardPage() {
  return (
    <NewMarkdownForm>
      <HeaderDashboard className="flex items-center justify-between pr-4">
        <MarkdownActions />
      </HeaderDashboard>
    </NewMarkdownForm>
  )
}
