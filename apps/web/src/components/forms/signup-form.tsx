'use client'

import { delay } from '@/shared/helpers/mock-helper'
import { SignUpPayload, signUpValidation } from '@/shared/validations/signup-validation'
import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { TextInput } from '../inputs/text-input'

import SystemButton from '../ui/button'
import { FormContent } from './form-content'

export function SignUpForm() {
  const form = useForm<SignUpPayload>({
    defaultValues: {
      email: 'lucaspedro517@gmail.com',
      password: '123456789',
      confirmPassword: '123456789',
    },
    resolver: zodResolver(signUpValidation),
  })

  const { handleSubmit, control, formState } = form
  const { isSubmitting, isValid } = formState

  async function onSubmit(data: SignUpPayload) {
    await delay(5000)
    console.log(data)
  }

  return (
    <FormContent onSubmit={handleSubmit(onSubmit)}>
      <fieldset className='flex flex-col gap-y-4' disabled={isSubmitting}>
        <TextInput
          name='email'
          control={control}
          type='text'
          placeholder='Email'
          aria-label='Email Address'
          autoComplete='username'
        />

        <TextInput
          name='password'
          control={control}
          type='password'
          placeholder='Password'
          aria-label='Password'
        />

        <TextInput
          name='confirmPassword'
          control={control}
          type='password'
          placeholder='Confirm Password'
          aria-label='Confirm Password'
        />
      </fieldset>

      <SystemButton
        type='submit'
        className='w-full'
        disabled={!isValid}
        isSubmitting={isSubmitting}
      >
        Create Account
      </SystemButton>
    </FormContent>
  )
}
