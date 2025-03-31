import { NewMarkdownPayload } from '../validations/markdown-validation'

export const defaultNewMarkdown = (
  schema?: NewMarkdownPayload,
): NewMarkdownPayload => {
  let defaultValues: NewMarkdownPayload = {
    name: 'welcome',
    content: '# Welcome to Markdown',
  }

  return {
    ...defaultValues,
    ...schema,
  }
}
