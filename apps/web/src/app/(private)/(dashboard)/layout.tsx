import { Sidebar } from '@/components/sidebar'
import { BaseLayoutProps } from '@/shared/base-types/next-types'

export default function DashboardLayout({
  children,
}: Readonly<BaseLayoutProps>) {
  return (
    <div className="w-full h-full flex overflow-x-auto">
      <Sidebar />
      <div className="w-full h-full flex">{children}</div>
    </div>
  )
}
