'use client'

import { Control, Controller, FieldValues, Path } from 'react-hook-form'

import { Input } from '@monorepo/ui'
import { StickyNote } from 'lucide-react'

interface NameInputProps<T extends FieldValues> extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string

  name: Path<T>
  control: Control<T>
  disabled?: boolean
  required?: boolean
}

export const NameInput = <T extends FieldValues>({
  label,
  name,
  control,
  disabled = false,
  required = false,
  ...rest
}: NameInputProps<T>) => {
  return (
    <Controller
      name={name}
      control={control}
      render={({
        field: { ref, onBlur, onChange, value, disabled: fieldDisabled },
        fieldState: { error },
      }) => (
        <div className='flex items-center gap-x-4'>
          <StickyNote className='text-title-text size-5' />

          <div className='flex flex-col'>
            <label htmlFor={name} className='text-xs text-[#7C8187]'>
              {label}
            </label>

            <Input
              variant='underline'
              size='sm'
              className='px-0 text-white'
              id={name}
              name={name}
              value={value ?? ''}
              onBlur={onBlur}
              onChange={onChange}
              disabled={disabled || fieldDisabled}
              aria-invalid={error ? 'true' : 'false'}
              aria-required={required ? 'true' : 'false'}
              ref={ref}
              {...rest}
            />
          </div>
        </div>
      )}
    />
  )
}
