import { BaseLayoutProps } from '@/shared/base-types/next-types';

export default function AuthLayout({ children }: Readonly<BaseLayoutProps>) {
  return (
    <div className="w-full h-full flex items-center justify-center">
      {children}
    </div>
  );
}
