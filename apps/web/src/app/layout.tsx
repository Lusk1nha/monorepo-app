import type { Metadata } from 'next';
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
      <body>{children}</body>
    </html>
  );
}
