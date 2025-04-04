'use client'

import { cn } from '@/lib/utils'
import { Switch } from '@monorepo/ui'
import { Moon, Sun } from 'lucide-react'
import { useTheme } from 'next-themes'
import { useCallback, useEffect, useState } from 'react'

export function ThemeSwitch() {
  const { theme, setTheme } = useTheme()
  const [mounted, setMounted] = useState(false)

  useEffect(() => {
    setMounted(true)
  }, [])

  const handleThemeChange = useCallback(() => {
    setTheme(theme === 'light' ? 'dark' : 'light')
  }, [theme, setTheme])

  if (!mounted) {
    return (
      <ThemeWrapper isLight={false}>
        <div className='w-12 h-6' />
      </ThemeWrapper>
    )
  }

  return (
    <ThemeWrapper isLight={theme === 'light'}>
      <Switch
        checked={theme === 'dark'}
        name='theme-switch'
        onChange={handleThemeChange}
        aria-label='Toggle dark mode'
      />
    </ThemeWrapper>
  )
}

interface ThemeWrapperProps {
  isLight: boolean
  children: React.ReactNode
}

function ThemeWrapper({ isLight, children }: ThemeWrapperProps) {
  return (
    <div className='flex items-center gap-x-3'>
      <Moon
        size={18}
        className={cn(
          'transition-colors',
          isLight ? 'text-[#7C8187]' : 'text-white',
        )}
      />
      {children}
      <Sun
        size={18}
        className={cn(
          'transition-colors',
          isLight ? 'text-white' : 'text-[#7C8187]',
        )}
      />
    </div>
  )
}
