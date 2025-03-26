import { SignInForm } from '@/components/forms/signin-form';
import { AuthHeader } from '../__components/auth-header';

export default function SignInPage() {
  return (
    <main className="flex flex-col gap-y-4">
      <AuthHeader
        link={{
          href: '/sign-up',
          text: 'Sign up'
        }}
        subtitle="Don’t have an account?"
      >
        Login to your account
      </AuthHeader>
      <SignInForm />
    </main>
  );
}
