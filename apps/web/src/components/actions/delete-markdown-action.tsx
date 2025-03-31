import { Button } from '@monorepo/ui'
import { Trash2 } from 'lucide-react'

interface DeleteMarkdownActionProps {
  markdownId: string
  onClick?: () => void
}

export function DeleteMarkdownAction(
  props: Readonly<DeleteMarkdownActionProps>,
) {
  const { markdownId, onClick } = props

  return (
    <Button
      type="button"
      variant="ghost"
      onClick={onClick}
      title={`Delete markdown ${markdownId}`}
      aria-label={`Delete markdown ${markdownId}`}
    >
      <Trash2 className="size-5" />
    </Button>
  )
}
