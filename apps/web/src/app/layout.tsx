import type { Metadata } from 'next';
import { SystemProviders } from '@/components/system-providers';

import './globals.css';

export const metadata: Metadata = {
  title: 'Browser Markdown Editor',
  description: 'A markdown editor that runs in the browser'
};

export default function RootLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <SystemProviders>{children}</SystemProviders>
      </body>
    </html>
  );
}
