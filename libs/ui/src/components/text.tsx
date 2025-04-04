import { cva, VariantProps } from 'class-variance-authority'
import { forwardRef } from 'react'
import { cn } from '../lib/utils'

const textVariants = cva('font-regular', {
  variants: {
    variant: {
      default: 'text-header-logo font-semibold',
      title: 'text-title-text font-semibold tracking-[0.313rem]',
      subtitle: 'text-subtitle-text font-medium tracking-[0.125rem]',
    },
    size: {
      default: 'text-md',
      sm: 'text-sm',
      lg: 'text-lg',
    },
  },
  defaultVariants: {
    variant: 'default',
    size: 'default',
  },
})

export interface TitleProps extends React.HTMLAttributes<HTMLHeadingElement>, VariantProps<typeof textVariants> {
  className?: string
  children: React.ReactNode
}

const Text = forwardRef<HTMLHeadingElement, TitleProps>((props, ref) => {
  const { className, variant, size, children, ...rest } = props

  return (
    <h1
      className={cn(textVariants({ variant, size, className }), className)}
      ref={ref}
      {...rest}
    >
      {children}
    </h1>
  )
})

Text.displayName = 'Text'

export default Text
