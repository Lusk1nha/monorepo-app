'use client'

import { cn } from '@/shared/lib/utils'
import { Loader2 } from 'lucide-react'
import React, { forwardRef, memo } from 'react'

interface SystemSpinnerProps extends React.HTMLAttributes<HTMLSpanElement> {
  children?: React.ReactNode
}

const SystemSpinner = forwardRef<HTMLSpanElement, SystemSpinnerProps>(
  (props, ref) => {
    const { children, className, ...rest } = props

    return (
      <span className='flex' {...rest} ref={ref}>
        <Loader2
          className={cn('animate-spin text-primary size-8', className)}
        />
        {children}
      </span>
    )
  },
)

SystemSpinner.displayName = 'SystemSpinner'

export default memo(SystemSpinner)
