import localFont from "next/font/local";

export const satoshi = localFont({
  src: "../../public/fonts/satoshi/Satoshi-Variable.woff2",
  variable: "--font-satoshi",
  display: "swap",
  weight: "700 900",
});

export const generalSans = localFont({
  src: "../../public/fonts/general-sans/GeneralSans-Variable.woff2",
  variable: "--font-general-sans",
  display: "swap",
  weight: "400 500",
});

export const specialElite = localFont({
  src: "../../public/fonts/special-elite/SpecialElite-Regular.woff2",
  variable: "--font-special-elite",
  display: "swap",
  weight: "400",
});
