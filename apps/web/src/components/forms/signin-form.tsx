'use client';

import { useForm } from 'react-hook-form';
import { TextInput } from '../inputs/text-input';
import { zodResolver } from '@hookform/resolvers/zod';
import {
  SignInPayload,
  signInValidation
} from '@/shared/validations/signin-validation';

import SystemButton from '../ui/button';
import { delay } from '@/shared/helpers/mock-helper';

export function SignInForm() {
  const form = useForm<SignInPayload>({
    defaultValues: {
      email: 'lucaspedro517@gmail.com',
      password: '123456789'
    },
    resolver: zodResolver(signInValidation)
  });

  const { handleSubmit, control, formState } = form;
  const { isSubmitting, isValid } = formState;

  async function onSubmit(data: SignInPayload) {
    await delay(5000);
    console.log(data);
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-y-4">
      <fieldset className="flex flex-col gap-y-4">
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

      <SystemButton
        type="submit"
        className="w-full"
        disabled={!isValid}
        isSubmitting={isSubmitting}
      >
        Sign In
      </SystemButton>
    </form>
  );
}
