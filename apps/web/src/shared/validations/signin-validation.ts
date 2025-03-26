import { z } from 'zod';

export const signInValidation = z.object({
  email: z
    .string({
      required_error: 'Email is required'
    })
    .email('Invalid email address'),
  password: z
    .string({
      required_error: 'Password is required'
    })
    .min(8, 'Password must be at least 8 characters')
});

export type SignInPayload = z.infer<typeof signInValidation>;
