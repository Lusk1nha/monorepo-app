import { cn } from '@/lib/utils'

interface HeaderDashboardProps {
  children: React.ReactNode
  className?: string
}

export function HeaderDashboard(props: Readonly<HeaderDashboardProps>) {
  const { children, className } = props

  return (
    <header
      className={cn(
        'bg-header-background flex items-center w-full h-14 sm:h-header-md',
        className,
      )}
    >
      {children}
    </header>
  )
}
