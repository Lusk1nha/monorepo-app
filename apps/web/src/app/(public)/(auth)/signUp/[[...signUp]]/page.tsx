import { SignUpForm } from '@/components/forms/signup-form'
import { AuthContent } from '../../__components/auth-content'
import { AuthHeader } from '../../__components/auth-header'
import { PATHS } from '@/path'

export default function SignUpPage() {
  return (
    <AuthContent>
      <AuthHeader
        link={{
          href: PATHS.SIGNIN,
          text: 'Log in',
        }}
        title="Create an account"
        subtitle="Already have an account?"
      />

      <SignUpForm />
    </AuthContent>
  )
}
