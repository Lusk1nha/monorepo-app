'use client'

import { cva, VariantProps } from 'class-variance-authority'
import { cn } from '../lib/utils'
import { forwardRef, memo } from 'react'

const inputVariants = cva(
  `text-sm sm:text-base transition-colors px-3 py-1 caret-primary disabled:cursor-not-allowed disabled:opacity-50`,
  {
    variants: {
      variant: {
        default:
          'bg-input text-input-text rounded-sm shadow-sm border border-input-border ring-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
        underline:
          'bg-transparent ring-none text-input-text focus-visible:outline-none focus-visible:border-b focus-visible:border-white focus-visible:ring-ring',
      },
      size: {
        default: 'h-9 sm:h-11',
        sm: 'h-6',
        lg: 'h-12',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
)

interface InputProps
  extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'>,
    VariantProps<typeof inputVariants> {
  name: string
}

const Input = forwardRef<HTMLInputElement, InputProps>((props, ref) => {
  const { type, size, variant, className, ...rest } = props
  return (
    <input
      type={type}
      className={cn(inputVariants({ variant, size, className }))}
      ref={ref}
      {...rest}
    />
  )
})

Input.displayName = 'Input'

export default memo(Input)
