import { cva, VariantProps } from 'class-variance-authority'
import { forwardRef, memo } from 'react'
import { cn } from '../lib/utils'

const switchVariants = cva(`relative inline-flex items-center cursor-pointer`, {
  variants: {
    variant: {
      default: 'bg-switch rounded-[14.5px]',
    },
    size: {
      default: 'w-12 h-6',
      sm: 'h-6',
      lg: 'h-12',
    },
  },
  defaultVariants: {
    variant: 'default',
    size: 'default',
  },
})

interface SwitchProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'>, VariantProps<typeof switchVariants> {
  name: string
  checked: boolean
}

const Switch = forwardRef<HTMLInputElement, SwitchProps>((props, ref) => {
  const { size, variant, checked, className, ...rest } = props

  return (
    <label className={cn(switchVariants({ variant, size, className }))}>
      <input
        type='checkbox'
        checked={checked}
        className='sr-only peer'
        ref={ref}
        {...rest}
      />

      <div
        className={cn(
          'bg-switch-dot min-w-3 min-h-3 absolute top-1/2 transform -translate-y-1/2 rounded-full transition-transform duration-200 ease-in-out',
          !checked ? 'right-1.5 translate-x-0' : 'left-1.5 translate-x-0',
        )}
      />
    </label>
  )
})

Switch.displayName = 'Switch'

export default memo(Switch)
