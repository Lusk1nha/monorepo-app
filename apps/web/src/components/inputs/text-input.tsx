'use client'

import { Control, Controller, FieldValues, Path } from 'react-hook-form'

import { Input } from '@monorepo/ui'
import WrapperErrorMessageInput from './wrapper-error-message-input'

interface TextInputProps<T extends FieldValues> extends React.InputHTMLAttributes<HTMLInputElement> {
  name: Path<T>
  control: Control<T>
  disabled?: boolean
  required?: boolean
}

export const TextInput = <T extends FieldValues>({
  name,
  control,
  disabled = false,
  required = false,
  ...rest
}: TextInputProps<T>) => {
  return (
    <Controller
      name={name}
      control={control}
      render={({
        field: { ref, onBlur, onChange, value, disabled: fieldDisabled },
        fieldState: { error },
      }) => (
        <WrapperErrorMessageInput error={error}>
          <Input
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
        </WrapperErrorMessageInput>
      )}
    />
  )
}
