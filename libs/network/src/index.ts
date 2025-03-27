import { NextAuthOptions, getServerSession } from 'next-auth'
import CredentialsProvider from 'next-auth/providers/credentials'

const MAX_AGE = 1 * 24 * 60 * 60

export const authOptions: NextAuthOptions = {
  providers: [
    CredentialsProvider({
      name: 'Credentials',
      credentials: {
        email: { label: 'Email', type: 'email' },
        password: { label: 'Password', type: 'password' },
      },

      async authorize(credentials) {
        if (!credentials) {
          throw new Error('Email and password are required')
        }

        try {
          const res = await fetch(
            'http://localhost:8000/api/auth/login-with-credentials',
            {
              method: 'POST',
              body: JSON.stringify(credentials),
              headers: { 'Content-Type': 'application/json' },
            },
          )

          const user = await res.json()

          console.log('user', user)

          return user
        } catch (error) {
          console.error('error', error)
          if (error instanceof Error) {
            throw new Error(error.message || 'Failed to authenticate')
          }

          throw new Error('Failed to authenticate')
        }
      },
    }),
  ],

  // Enable debug mode for development
  debug: true,

  // Configure session settings
  session: {
    strategy: 'jwt',
    maxAge: MAX_AGE,
  },

  pages: {
    signIn: '/signIn',
  },
}

export const getAuth = () => getServerSession(authOptions)
