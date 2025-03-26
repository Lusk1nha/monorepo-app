'use client';

import { cn } from '@/shared/lib/utils';
import { forwardRef, memo } from 'react';

import SystemSpinner from './spinner';

interface SystemButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  disabled?: boolean;
  isSubmitting?: boolean;
}

const SystemButton = forwardRef<HTMLButtonElement, SystemButtonProps>(
  (props, ref) => {
    const {
      type,
      disabled = false,
      isSubmitting = false,
      className,
      children,
      ...rest
    } = props;

    return (
      <button
        type={type}
        className={cn(
          `h-9 sm:h-11 flex gap-x-2 items-center justify-center bg-button text-white hover:bg-button-hover
          rounded-sm px-3 py-1 cursor-pointer relative`,
          className,
          (disabled || isSubmitting) &&
            'cursor-not-allowed opacity-50 pointer-events-none'
        )}
        ref={ref}
        {...rest}
      >
        {children ?? 'Button'}
        {isSubmitting && <SystemSpinner className="size-5 text-white" />}
      </button>
    );
  }
);

SystemButton.displayName = 'SystemButton';

export default memo(SystemButton);
