import { z } from 'zod'

export const newMarkdownValidation = z.object({
  name: z.string().min(1, 'Name is required'),
  content: z.string().min(1, 'Content is required'),
})

export type NewMarkdownPayload = z.infer<typeof newMarkdownValidation>
