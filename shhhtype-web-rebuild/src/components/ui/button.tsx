"use client";

import { type ComponentProps } from "react";

type ButtonVariant = "gradient" | "outline";

interface ButtonProps extends ComponentProps<"a"> {
  variant?: ButtonVariant;
}

export function Button({ variant = "gradient", className = "", children, ...props }: ButtonProps) {
  const base =
    "inline-flex items-center justify-center gap-2 rounded-full px-6 py-3 text-sm font-medium transition-all duration-200 min-h-[44px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-rose-500 focus-visible:ring-offset-2";

  const variants: Record<ButtonVariant, string> = {
    gradient:
      "bg-gradient-to-r from-rose-500 to-orange-500 text-white hover:from-rose-600 hover:to-orange-600 shadow-lg shadow-rose-500/25 hover:shadow-xl hover:shadow-rose-500/30",
    outline:
      "border border-gray-300 text-gray-700 hover:bg-gray-50 dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-800/50",
  };

  return (
    <a className={`${base} ${variants[variant]} ${className}`} {...props}>
      {children}
    </a>
  );
}
