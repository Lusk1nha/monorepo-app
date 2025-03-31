import { Button } from '@monorepo/ui'
import { Save } from 'lucide-react'

interface SaveMarkdownActionProps {
  markdownId: string
}

export function SaveMarkdownAction(props: Readonly<SaveMarkdownActionProps>) {
  const { markdownId } = props

  return (
    <Button
      type="submit"
      title={`Save markdown ${markdownId}`}
      aria-label={`Save markdown ${markdownId}`}
    >
      <Save className="size-5" />
      <span className="hidden lg:block">Save Changes</span>
    </Button>
  )
}
