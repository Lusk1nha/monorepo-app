'use client'

import { memo } from 'react'
import { FieldError } from 'react-hook-form'
import { ErrorMessageInput } from './error-message-input'

interface WrapperErrorMessageInputProps {
  error: FieldError | undefined
  children: React.ReactNode
}

function WrapperErrorMessageInput(
  props: Readonly<WrapperErrorMessageInputProps>,
) {
  const { error, children } = props

  return (
    <div className='flex flex-col gap-y-1'>
      {children}
      {error && <ErrorMessageInput error={error} />}
    </div>
  )
}

export default memo(WrapperErrorMessageInput)
