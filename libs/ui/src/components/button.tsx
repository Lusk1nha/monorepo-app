'use client'

import { forwardRef } from 'react'

import Spinner from './spinner'
import { cn } from '../lib/utils'
import { cva, VariantProps } from 'class-variance-authority'

const buttonVariants = cva(
  'inline-flex gap-x-2 text-primary items-center justify-center rounded-sm px-3 py-1 cursor-pointer relative transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-background',
  {
    variants: {
      variant: {
        default: 'bg-button text-white hover:bg-button-hover',
        menu: 'bg-button-menu text-button-text hover:bg-primary rounded-none',
        ghost: 'bg-transparent text-button-ghost-text hover:text-primary',
      },
      size: {
        default: 'h-9 sm:h-11',
        sm: 'h-8',
        lg: 'h-12',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
)

export interface SystemButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  disabled?: boolean
  isSubmitting?: boolean
}

const Button = forwardRef<HTMLButtonElement, SystemButtonProps>(
  (props, ref) => {
    const {
      type,
      disabled = false,
      isSubmitting = false,
      className,
      children,
      variant,
      size,
      ...rest
    } = props

    return (
      <button
        type={type}
        className={cn(
          buttonVariants({ variant, size, className }),
          (disabled || isSubmitting) &&
            'cursor-not-allowed opacity-50 pointer-events-none',
        )}
        ref={ref}
        {...rest}
      >
        {children ?? 'Button'}
        {isSubmitting && <Spinner className="size-5 text-white" />}
      </button>
    )
  },
)

Button.displayName = 'Button'

export default Button
