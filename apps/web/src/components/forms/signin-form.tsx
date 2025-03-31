'use client'

import {
  SignInPayload,
  signInValidation,
} from '@/shared/validations/signin-validation'
import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { TextInput } from '../inputs/text-input'

import { delay } from '@/shared/helpers/mock-helper'
import { Button } from '@monorepo/ui'
import { FormContent } from './form-content'

import { useRouter } from 'next/navigation'
import { PATHS } from '@/path'

export function SignInForm() {
  const router = useRouter()

  const form = useForm<SignInPayload>({
    defaultValues: {
      email: 'lucaspedro517@gmail.com',
      password: '123456789',
    },
    resolver: zodResolver(signInValidation),
  })

  const { handleSubmit, control, formState } = form
  const { isSubmitting, isValid } = formState

  async function onSubmit(data: SignInPayload) {
    try {
      const response = await delay(500)

      router.push(PATHS.ROOT)
    } catch (error) {
      console.error(error)
    }
  }

  return (
    <FormContent onSubmit={handleSubmit(onSubmit)}>
      <fieldset className="flex flex-col gap-y-4" disabled={isSubmitting}>
        <TextInput
          name="email"
          control={control}
          type="text"
          placeholder="Email"
          aria-label="Email Address"
          autoComplete="username"
        />

        <TextInput
          name="password"
          control={control}
          type="password"
          placeholder="Password"
          aria-label="Password"
          autoComplete="current-password"
        />
      </fieldset>

      <Button
        variant="default"
        type="submit"
        className="w-full"
        disabled={!isValid}
        isSubmitting={isSubmitting}
      >
        Log In
      </Button>
    </FormContent>
  )
}
