'use client'

import { defaultNewMarkdown } from '@/shared/helpers/markdown-helper'
import { NewMarkdownPayload, newMarkdownValidation } from '@/shared/validations/markdown-validation'
import { zodResolver } from '@hookform/resolvers/zod'
import { FormProvider, useForm } from 'react-hook-form'

interface NewMarkdownFormProps {
  children: React.ReactNode
}

export function NewMarkdownForm(props: Readonly<NewMarkdownFormProps>) {
  const { children } = props

  const form = useForm<NewMarkdownPayload>({
    defaultValues: defaultNewMarkdown(),
    resolver: zodResolver(newMarkdownValidation),
  })

  const { handleSubmit } = form

  async function onSubmit(data: NewMarkdownPayload) {
    try {
      console.log(data)
    } catch (error) {
      console.error(error)
    }
  }

  return (
    <FormProvider {...form}>
      <form
        className='w-full h-full flex flex-col'
        onSubmit={handleSubmit(onSubmit)}
      >
        {children}
      </form>
    </FormProvider>
  )
}
