'use client'

import { cn } from '@/shared/lib/utils'
import { forwardRef, memo } from 'react'

interface SystemInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  name: string
}

const SystemInput = forwardRef<HTMLInputElement, SystemInputProps>(
  (props, ref) => {
    const { type, className, ...rest } = props
    return (
      <input
        type={type}
        className={cn(
          `bg-input text-input-text h-9 sm:h-11 
        text-sm sm:text-base shadow-sm transition-colors
        border border-input-border
        focus-visible:outline-none ring-primary
        focus-visible:ring-2 focus-visible:ring-ring
        rounded-sm px-3 py-1 disabled:cursor-not-allowed disabled:opacity-50`,
          className,
        )}
        ref={ref}
        {...rest}
      />
    )
  },
)

SystemInput.displayName = 'SystemInput'

export default memo(SystemInput)
