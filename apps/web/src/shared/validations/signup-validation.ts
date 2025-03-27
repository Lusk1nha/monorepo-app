import { z } from 'zod'

export const signUpValidation = z
  .object({
    email: z
      .string({
        required_error: 'Email is required',
      })
      .email('Invalid email address'),
    password: z
      .string({
        required_error: 'Password is required',
      })
      .min(8, 'Password must be at least 8 characters'),

    confirmPassword: z
      .string({
        required_error: 'Password is required',
      })
      .min(8, 'Password must be at least 8 characters'),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: 'Passwords do not match',
    path: ['confirmPassword'],
  })

export type SignUpPayload = z.infer<typeof signUpValidation>
