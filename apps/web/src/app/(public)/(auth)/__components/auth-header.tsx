import Link from 'next/link';

interface AuthHeaderProps {
  subtitle: string;
  children: string;

  link: {
    href: string;
    text: string;
  };
}

export function AuthHeader(props: Readonly<AuthHeaderProps>) {
  const { subtitle, children, link } = props;

  return (
    <div className="flex flex-col gap-y-6">
      <h1 className="text-4xl text-auth-title font-roboto-slab font-normal">
        {children}
      </h1>

      <p className="text-sm text-auth-subtitle font-roboto font-normal">
        {subtitle}{' '}
        <Link className="text-primary underline" href={link.href}>
          {link.text}
        </Link>
      </p>
    </div>
  );
}
