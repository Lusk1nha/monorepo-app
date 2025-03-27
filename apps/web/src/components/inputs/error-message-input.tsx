import { FieldError } from 'react-hook-form'

interface ErrorMessageInputProps {
  error: FieldError
}

export function ErrorMessageInput(props: Readonly<ErrorMessageInputProps>) {
  const { error } = props
  return <p className='text-sm text-destructive font-medium'>{error.message}</p>
}
