import Link from 'next/link'

interface AuthLink {
  href: string
  text: string
}

interface AuthHeaderProps {
  title: string
  subtitle: string
  link: AuthLink
}

export function AuthHeader(props: Readonly<AuthHeaderProps>) {
  const { subtitle, title, link } = props

  return (
    <header className='flex flex-col gap-y-4'>
      <h1 className='text-3xl sm:text-5xl text-auth-title font-roboto-slab font-normal text-start'>
        {title}
      </h1>

      <p className='text-sm text-auth-subtitle font-roboto font-normal'>
        {subtitle}{' '}
        <Link
          className='text-primary underline'
          href={link.href}
          aria-label={link.text}
        >
          {link.text}
        </Link>
      </p>
    </header>
  )
}
