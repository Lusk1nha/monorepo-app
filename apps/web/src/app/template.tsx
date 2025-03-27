import { Roboto, Roboto_Slab } from 'next/font/google'

const roboto = Roboto({
  subsets: ['latin'],
})
const robotoSlab = Roboto_Slab({
  subsets: ['latin'],
})

export default function Template({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <div className={`${roboto.className} ${robotoSlab.className} antialiased`}>
      <div className='bg-background min-h-screen h-screen w-full'>
        {children}
      </div>
    </div>
  )
}
