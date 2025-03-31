import { BaseLayoutProps } from '@/shared/base-types/next-types'

export default function AuthLayout({ children }: Readonly<BaseLayoutProps>) {
  return (
    <div className="w-full h-full flex items-center justify-center">
      <div className="w-full max-w-[600px] flex px-8">{children}</div>
    </div>
  )
}
