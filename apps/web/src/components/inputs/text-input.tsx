'use client';

import { Control, Controller, FieldValues, Path } from 'react-hook-form';
import SystemInput from '../ui/input';

import WrapperErrorMessageInput from './wrapper-error-message-input';

interface TextInputProps<T extends FieldValues>
  extends React.InputHTMLAttributes<HTMLInputElement> {
  name: Path<T>;
  control: Control<T>;
  disabled?: boolean;
}

export function TextInput<T extends FieldValues>(
  props: Readonly<TextInputProps<T>>
) {
  const { name, control, disabled, ...rest } = props;

  return (
    <Controller
      name={name}
      control={control}
      disabled={disabled}
      render={({
        field: { name, onBlur, onChange, value, disabled },
        fieldState: { error }
      }) => (
        <WrapperErrorMessageInput error={error}>
          <SystemInput
            name={name}
            value={value ?? ''}
            onBlur={onBlur}
            onChange={onChange}
            disabled={disabled}
            {...rest}
          />
        </WrapperErrorMessageInput>
      )}
    />
  );
}
