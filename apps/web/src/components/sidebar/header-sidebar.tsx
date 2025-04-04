import { Text } from '@monorepo/ui'
import { Logo } from '../logo'

export function HeaderSidebar() {
  return (
    <section className='justify-center flex flex-col mb-4'>
      <div className='flex h-14 sm:h-header-md lg:hidden items-center justify-start'>
        <Logo />
      </div>

      <div className='lg:h-header-md justify-center flex flex-col gap-y-4'>
        <Text size='sm' variant='subtitle'>
          MY DOCUMENTS
        </Text>
      </div>
    </section>
  )
}
