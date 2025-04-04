'use client'

import { PATHS } from '@/path'
import { Button } from '@monorepo/ui'
import Link from 'next/link'

export function RedirectNewDocument() {
  return (
    <Link href={PATHS.ROOT}>
      <Button className='w-full' type='button'>+ New Document</Button>
    </Link>
  )
}
