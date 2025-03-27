interface AuthContentProps {
  children: React.ReactNode
}

export function AuthContent(props: Readonly<AuthContentProps>) {
  const { children } = props
  return <main className='w-full flex flex-col gap-y-8'>{children}</main>
}
