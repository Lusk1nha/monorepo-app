interface FormContentProps {
  children: React.ReactNode
  onSubmit: (e: React.FormEvent<HTMLFormElement>) => void
}

export function FormContent(props: Readonly<FormContentProps>) {
  const { children, onSubmit } = props
  return (
    <form onSubmit={onSubmit} className='flex flex-col gap-y-6'>
      {children}
    </form>
  )
}
