# ShhhType Website Rebuild — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rebuild shhhtype.com as a mobile-first, component-based Next.js static site with modern typography and scroll-driven animations.

**Architecture:** Next.js 15 App Router with static export. Single page composed of 7 section components + nav. Framer Motion for animations. Tailwind v4 for styling. Self-hosted Satoshi + General Sans fonts via next/font/local.

**Tech Stack:** Next.js 15, React 19, Tailwind CSS v4, Framer Motion, Lucide React, Satoshi + General Sans (Fontshare)

**Spec:** `docs/superpowers/specs/2026-03-18-shhhtype-website-rebuild-design.md`

**Existing site reference:** `/Users/g1tech/_dev/shhhtype/index.html` — copy and SEO data to port from here.

---

## File Structure

```
shhhtype-web-rebuild/
├── next.config.js              # Static export config
├── tailwind.config.ts          # Font family + color extensions
├── tsconfig.json               # TypeScript config (from create-next-app)
├── package.json
├── postcss.config.js           # Tailwind v4 PostCSS plugin
├── public/
│   ├── fonts/
│   │   ├── satoshi/
│   │   │   └── Satoshi-Variable.woff2
│   │   └── general-sans/
│   │       └── GeneralSans-Variable.woff2
│   ├── images/
│   │   ├── shhh_logo_thick.png   # From existing site
│   │   └── og-image.png          # From existing site (or placeholder)
│   ├── favicon.ico
│   ├── robots.txt
│   └── sitemap.xml
├── src/
│   ├── app/
│   │   ├── layout.tsx            # Root layout: fonts, metadata, JSON-LD, skip link
│   │   ├── page.tsx              # Composes all sections
│   │   └── globals.css           # Tailwind imports, custom properties, keyframes
│   ├── components/
│   │   ├── nav.tsx               # Sticky nav + mobile menu
│   │   ├── hero.tsx              # Hero section with product mockup
│   │   ├── hero-mockup.tsx       # Animated hotkey→speak→text demo
│   │   ├── how-it-works.tsx      # 3-step flow with SVG lines
│   │   ├── features.tsx          # Bento grid
│   │   ├── local-vs-cloud.tsx    # Split comparison with tabs
│   │   ├── ai-rewrite.tsx        # Interactive before/after demo
│   │   ├── pricing.tsx           # Pricing card with glow
│   │   ├── footer.tsx            # Minimal footer
│   │   └── ui/
│   │       ├── button.tsx        # Gradient + outline variants
│   │       ├── badge.tsx         # Status badges (Launch Special, etc.)
│   │       ├── section.tsx       # Light/dark section wrapper with padding
│   │       └── scroll-reveal.tsx # Framer Motion scroll-triggered wrapper
│   └── lib/
│       ├── fonts.ts              # next/font/local definitions + CSS vars
│       └── constants.ts          # CTA URLs, copy strings, feature lists
```

---

### Task 1: Project Scaffolding + Configuration

**Files:**
- Create: `package.json`, `next.config.js`, `tailwind.config.ts`, `postcss.config.js`, `tsconfig.json`
- Create: `src/app/globals.css`, `src/app/layout.tsx`, `src/app/page.tsx`

- [ ] **Step 1: Create Next.js project**

```bash
cd /Users/g1tech/_dev/shhhtype-web-rebuild
npx create-next-app@latest . --typescript --tailwind --eslint --app --src-dir --no-import-alias --skip-install
```

If the directory isn't empty, accept overwriting. This gives us the base Next.js 15 + Tailwind v4 scaffold.

- [ ] **Step 2: Install dependencies**

```bash
npm install framer-motion lucide-react
```

- [ ] **Step 3: Configure next.config.js for static export**

Replace `next.config.js` contents with:

```js
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true,
  },
};

module.exports = nextConfig;
```

- [ ] **Step 4: Verify build works**

```bash
npm run build
```

Expected: Build succeeds, `out/` directory created with static files.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "chore: scaffold Next.js 15 project with Tailwind v4 + Framer Motion"
```

---

### Task 2: Font Loading + Tailwind Typography Config

**Files:**
- Create: `public/fonts/satoshi/Satoshi-Variable.woff2`
- Create: `public/fonts/general-sans/GeneralSans-Variable.woff2`
- Create: `src/lib/fonts.ts`
- Modify: `tailwind.config.ts`
- Modify: `src/app/layout.tsx`
- Modify: `src/app/globals.css`

- [ ] **Step 1: Download fonts from Fontshare**

Download Satoshi Variable and General Sans Variable WOFF2 files from https://www.fontshare.com/fonts/satoshi and https://www.fontshare.com/fonts/general-sans. Place them:

```
public/fonts/satoshi/Satoshi-Variable.woff2
public/fonts/general-sans/GeneralSans-Variable.woff2
```

If direct download isn't possible via CLI, create placeholder empty files and note that fonts must be manually downloaded.

- [ ] **Step 2: Create font definitions in `src/lib/fonts.ts`**

```ts
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
```

- [ ] **Step 3: Update `tailwind.config.ts` with font families**

Add to the `theme.extend` section:

```ts
fontFamily: {
  heading: ['var(--font-satoshi)', 'system-ui', 'sans-serif'],
  body: ['var(--font-general-sans)', 'system-ui', 'sans-serif'],
},
```

- [ ] **Step 4: Apply font variables in `src/app/layout.tsx`**

Add font imports and apply CSS variable classes to `<body>`:

```tsx
import { satoshi, generalSans } from "@/lib/fonts";

// In the body tag:
<body className={`${satoshi.variable} ${generalSans.variable} font-body antialiased`}>
```

- [ ] **Step 5: Add base typography to `src/app/globals.css`**

Add after Tailwind imports:

```css
h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-satoshi), system-ui, sans-serif;
}
```

- [ ] **Step 6: Verify fonts load**

```bash
npm run dev
```

Open http://localhost:3000 in browser. Inspect body — confirm `--font-satoshi` and `--font-general-sans` CSS variables are present. Text should render in General Sans, headings in Satoshi.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat: add self-hosted Satoshi + General Sans fonts via next/font/local"
```

---

### Task 3: UI Primitives (Button, Badge, Section, ScrollReveal)

**Files:**
- Create: `src/components/ui/button.tsx`
- Create: `src/components/ui/badge.tsx`
- Create: `src/components/ui/section.tsx`
- Create: `src/components/ui/scroll-reveal.tsx`
- Create: `src/lib/constants.ts`

- [ ] **Step 1: Create `src/lib/constants.ts`**

```ts
export const CHECKOUT_URL =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/1ea919ae-5f44-4ea9-bc4d-95e64cb41a87";

export const SUPPORT_EMAIL = "support@g1tech.cloud";
```

- [ ] **Step 2: Create `src/components/ui/button.tsx`**

Two variants: `gradient` (primary CTA) and `outline` (secondary). Both have minimum 44px touch target. Gradient uses rose-to-orange. Outline has border + hover fill.

```tsx
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
```

- [ ] **Step 3: Create `src/components/ui/badge.tsx`**

```tsx
interface BadgeProps {
  children: React.ReactNode;
  variant?: "default" | "success" | "warning";
  className?: string;
}

export function Badge({ children, variant = "default", className = "" }: BadgeProps) {
  const variants = {
    default: "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300",
    success: "bg-green-50 text-green-700 dark:bg-green-900/30 dark:text-green-400",
    warning: "bg-orange-50 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400",
  };

  return (
    <span
      className={`inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-medium ${variants[variant]} ${className}`}
    >
      {children}
    </span>
  );
}
```

- [ ] **Step 4: Create `src/components/ui/section.tsx`**

Wrapper that handles light/dark backgrounds, responsive padding, and max-width container.

```tsx
interface SectionProps {
  id?: string;
  dark?: boolean;
  children: React.ReactNode;
  className?: string;
}

export function Section({ id, dark = false, children, className = "" }: SectionProps) {
  return (
    <section
      id={id}
      className={`px-6 py-20 md:py-28 lg:py-32 ${
        dark ? "bg-[#0a0a0a] text-gray-100" : "bg-[#FAFAFA] text-gray-900"
      } ${className}`}
    >
      <div className="mx-auto max-w-6xl">{children}</div>
    </section>
  );
}
```

- [ ] **Step 5: Create `src/components/ui/scroll-reveal.tsx`**

Framer Motion scroll-triggered fade-up wrapper. Respects prefers-reduced-motion.

```tsx
"use client";

import { motion, useReducedMotion } from "framer-motion";
import { type ReactNode } from "react";

interface ScrollRevealProps {
  children: ReactNode;
  delay?: number;
  className?: string;
}

export function ScrollReveal({ children, delay = 0, className = "" }: ScrollRevealProps) {
  const prefersReducedMotion = useReducedMotion();

  if (prefersReducedMotion) {
    return <div className={className}>{children}</div>;
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 24 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: "-80px" }}
      transition={{ duration: 0.5, delay, ease: "easeOut" }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
```

- [ ] **Step 6: Verify primitives render**

Add a quick test in `page.tsx`:

```tsx
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Section } from "@/components/ui/section";
import { ScrollReveal } from "@/components/ui/scroll-reveal";

export default function Home() {
  return (
    <main>
      <Section>
        <ScrollReveal>
          <Badge variant="success">Test Badge</Badge>
          <h1 className="font-heading text-4xl font-black">Test Heading</h1>
          <Button variant="gradient">Primary CTA</Button>
          <Button variant="outline">Secondary CTA</Button>
        </ScrollReveal>
      </Section>
      <Section dark>
        <h2 className="font-heading text-3xl font-bold">Dark Section</h2>
      </Section>
    </main>
  );
}
```

Run `npm run dev`, verify in browser: gradient button, outline button, badge, light/dark sections, Satoshi headings, General Sans body.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat: add UI primitives — Button, Badge, Section, ScrollReveal"
```

---

### Task 4: Navigation Component

**Files:**
- Create: `src/components/nav.tsx`
- Modify: `src/app/page.tsx` (add Nav)
- Copy: `shhh_logo_thick.png` from existing site to `public/images/`

- [ ] **Step 1: Copy logo asset**

```bash
cp /Users/g1tech/_dev/shhhtype/assets/images/shhh_logo_thick.png /Users/g1tech/_dev/shhhtype-web-rebuild/public/images/shhh_logo_thick.png
```

- [ ] **Step 2: Create `src/components/nav.tsx`**

Implement sticky nav with:
- Glass effect (backdrop-blur, opacity increases on scroll via `useScroll` from Framer Motion)
- Desktop: Logo + 3 anchor links (How It Works, Features, Pricing) + gradient CTA
- Mobile: Logo + hamburger → slide-down sheet with focus trap, Escape to close
- All anchor links smooth-scroll
- Skip-to-content link as first focusable element

```tsx
"use client";

import { useState, useEffect, useRef, useCallback } from "react";
import { Menu, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { CHECKOUT_URL } from "@/lib/constants";

const NAV_LINKS = [
  { label: "How It Works", href: "#how-it-works" },
  { label: "Features", href: "#features" },
  { label: "Pricing", href: "#pricing" },
];

export function Nav() {
  const [mobileOpen, setMobileOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const hamburgerRef = useRef<HTMLButtonElement>(null);
  const [scrolled, setScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => setScrolled(window.scrollY > 50);
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  const closeMobile = useCallback(() => {
    setMobileOpen(false);
    hamburgerRef.current?.focus();
  }, []);

  useEffect(() => {
    if (!mobileOpen) return;
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") closeMobile();
    };
    document.addEventListener("keydown", handleEscape);
    document.body.style.overflow = "hidden";
    return () => {
      document.removeEventListener("keydown", handleEscape);
      document.body.style.overflow = "";
    };
  }, [mobileOpen, closeMobile]);

  return (
    <>
      {/* Skip to content */}
      <a
        href="#main"
        className="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-[60] focus:rounded-lg focus:bg-white focus:px-4 focus:py-2 focus:text-sm focus:font-medium focus:shadow-lg"
      >
        Skip to content
      </a>

      <nav
        className={`fixed top-0 right-0 left-0 z-50 transition-all duration-300 ${
          scrolled
            ? "border-b border-gray-200/50 bg-white/85 backdrop-blur-xl"
            : "border-b border-transparent bg-transparent"
        }`}
      >
        {/* Implementation: desktop links, mobile hamburger, gradient CTA */}
        {/* See spec Section 0 for full behavior */}
        <div className="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
          <a href="/" className="flex items-center gap-2">
            <img src="/images/shhh_logo_thick.png" alt="ShhhType" className="h-8 w-8" />
            <span className="font-heading text-lg font-bold">ShhhType</span>
          </a>

          {/* Desktop links */}
          <div className="hidden items-center gap-8 md:flex">
            {NAV_LINKS.map((link) => (
              <a
                key={link.href}
                href={link.href}
                className="text-sm font-medium text-gray-600 transition-colors hover:text-gray-900"
              >
                {link.label}
              </a>
            ))}
          </div>

          <div className="hidden md:block">
            <Button href={CHECKOUT_URL} target="_blank" rel="noopener">
              Download for macOS
            </Button>
          </div>

          {/* Mobile hamburger */}
          <button
            ref={hamburgerRef}
            onClick={() => setMobileOpen(!mobileOpen)}
            className="flex h-11 w-11 items-center justify-center rounded-lg md:hidden focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-rose-500"
            aria-label={mobileOpen ? "Close menu" : "Open menu"}
            aria-expanded={mobileOpen}
          >
            {mobileOpen ? <X size={24} /> : <Menu size={24} />}
          </button>
        </div>

        {/* Mobile menu sheet */}
        {mobileOpen && (
          <div
            ref={menuRef}
            className="absolute top-full right-0 left-0 border-t border-gray-200 bg-white/95 backdrop-blur-xl md:hidden"
            role="dialog"
            aria-label="Navigation menu"
          >
            <div className="flex flex-col gap-4 px-6 py-6">
              {NAV_LINKS.map((link) => (
                <a
                  key={link.href}
                  href={link.href}
                  onClick={closeMobile}
                  className="text-lg font-medium text-gray-900"
                >
                  {link.label}
                </a>
              ))}
              <Button href={CHECKOUT_URL} target="_blank" rel="noopener" className="mt-2">
                Download for macOS
              </Button>
            </div>
          </div>
        )}
      </nav>
    </>
  );
}
```

- [ ] **Step 3: Add smooth scroll to globals.css**

Add to `src/app/globals.css`:

```css
html {
  scroll-behavior: smooth;
}
```

This is needed now since Nav and Hero both use anchor links.

- [ ] **Step 4: Add Nav to page.tsx**

```tsx
import { Nav } from "@/components/nav";

export default function Home() {
  return (
    <>
      <Nav />
      <main id="main">
        {/* sections will go here */}
      </main>
    </>
  );
}
```

- [ ] **Step 5: Verify nav works**

Run `npm run dev`. Check:
- Desktop: logo, links, CTA visible
- Mobile (resize to <768px): hamburger appears, clicking opens sheet, Escape closes, links smooth-scroll
- Scroll past 50px: background transitions to white/blur

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat: add sticky Nav with glass effect and mobile menu"
```

---

### Task 5: Hero Section

**Files:**
- Create: `src/components/hero.tsx`
- Create: `src/components/hero-mockup.tsx`
- Modify: `src/app/globals.css` (add dot grid background)
- Modify: `src/app/page.tsx` (add Hero)

- [ ] **Step 1: Add dot grid background to globals.css**

```css
.bg-dot-grid {
  background-image: radial-gradient(circle, #d4d4d4 1px, transparent 1px);
  background-size: 24px 24px;
}
```

- [ ] **Step 2: Create `src/components/hero-mockup.tsx`**

Animated card sequence: hotkey badge → waveform → typing text. Built with Framer Motion.

```tsx
"use client";

import { motion, useReducedMotion } from "framer-motion";
import { Keyboard, Mic, Type } from "lucide-react";

const steps = [
  { icon: Keyboard, label: "⌘ + ⌥ + V", sublabel: "Press hotkey", color: "from-blue-500 to-indigo-500" },
  { icon: Mic, label: "Speaking...", sublabel: "Voice captured", color: "from-rose-500 to-pink-500" },
  { icon: Type, label: "Text appears", sublabel: "In any app", color: "from-orange-500 to-amber-500" },
];

export function HeroMockup() {
  const prefersReducedMotion = useReducedMotion();

  return (
    <div className="relative flex flex-col gap-4">
      {steps.map((step, i) => (
        <motion.div
          key={step.label}
          initial={prefersReducedMotion ? {} : { opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.4 + i * 0.2, duration: 0.5, ease: "easeOut" }}
          className="flex items-center gap-4 rounded-2xl border border-gray-200 bg-white p-4 shadow-sm"
        >
          <div className={`flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br ${step.color} text-white`}>
            <step.icon size={22} />
          </div>
          <div>
            <p className="font-heading text-sm font-bold text-gray-900">{step.label}</p>
            <p className="text-xs text-gray-500">{step.sublabel}</p>
          </div>
        </motion.div>
      ))}
      {/* Floating subtle animation */}
      <motion.div
        animate={prefersReducedMotion ? {} : { y: [0, -6, 0] }}
        transition={{ duration: 4, repeat: Infinity, ease: "easeInOut" }}
        className="absolute -right-4 -bottom-4 h-24 w-24 rounded-full bg-gradient-to-br from-rose-500/10 to-orange-500/10 blur-xl"
      />
    </div>
  );
}
```

- [ ] **Step 3: Create `src/components/hero.tsx`**

Full-viewport hero with headline, subheadline, CTAs, and mockup. Stacks vertically on mobile.

```tsx
"use client";

import { motion, useReducedMotion } from "framer-motion";
import { ArrowRight, ChevronDown } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { HeroMockup } from "@/components/hero-mockup";
import { CHECKOUT_URL } from "@/lib/constants";

export function Hero() {
  const prefersReducedMotion = useReducedMotion();
  const animate = !prefersReducedMotion;

  return (
    <section className="relative flex min-h-screen items-center bg-[#FAFAFA] px-6 pt-24 pb-20">
      <div className="bg-dot-grid pointer-events-none absolute inset-0 opacity-40" />
      <div className="relative mx-auto grid max-w-6xl items-center gap-12 lg:grid-cols-2 lg:gap-16">
        {/* Text column */}
        <div>
          <motion.div
            initial={animate ? { opacity: 0, y: 20 } : {}}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
          >
            <Badge variant="success" className="mb-6">
              <span className="h-2 w-2 rounded-full bg-green-500" />
              v1.0 — Available Now
            </Badge>
          </motion.div>

          <motion.h1
            initial={animate ? { opacity: 0, y: 20 } : {}}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1, duration: 0.5 }}
            className="font-heading text-4xl font-black leading-tight tracking-tight sm:text-5xl lg:text-6xl"
          >
            Your voice,{" "}
            <span className="bg-gradient-to-r from-rose-500 to-orange-500 bg-clip-text text-transparent">
              their text field.
            </span>
          </motion.h1>

          <motion.p
            initial={animate ? { opacity: 0, y: 20 } : {}}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2, duration: 0.5 }}
            className="mt-6 max-w-lg text-lg text-gray-600"
          >
            Press a hotkey, speak naturally, and your words appear in any app.
            Local or cloud transcription. Private, fast, effortless.
          </motion.p>

          <motion.div
            initial={animate ? { opacity: 0, y: 20 } : {}}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3, duration: 0.5 }}
            className="mt-8 flex flex-wrap gap-4"
          >
            <Button href={CHECKOUT_URL} target="_blank" rel="noopener">
              Download for macOS <ArrowRight size={16} />
            </Button>
            <Button variant="outline" href="#how-it-works">
              See how it works <ChevronDown size={16} />
            </Button>
          </motion.div>
        </div>

        {/* Mockup column */}
        <div className="flex justify-center lg:justify-end">
          <HeroMockup />
        </div>
      </div>
    </section>
  );
}
```

- [ ] **Step 4: Add Hero to page.tsx**

```tsx
import { Nav } from "@/components/nav";
import { Hero } from "@/components/hero";

export default function Home() {
  return (
    <>
      <Nav />
      <main id="main">
        <Hero />
      </main>
    </>
  );
}
```

- [ ] **Step 5: Verify hero**

Run `npm run dev`. Check:
- Desktop: two-column layout, gradient headline, staggered fade-in, mockup cards animate in
- Mobile: single column, headline above mockup, CTAs wrap nicely, no horizontal scroll
- Click "Download for macOS" → opens LemonSqueezy in new tab
- Click "See how it works" → smooth scroll (will scroll to nothing yet, that's fine)

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat: add Hero section with animated product mockup"
```

---

### Task 6: How It Works Section

**Files:**
- Create: `src/components/how-it-works.tsx`
- Modify: `src/app/page.tsx` (add HowItWorks)

- [ ] **Step 1: Create `src/components/how-it-works.tsx`**

Dark section with 3-step flow. Desktop: horizontal with SVG connecting lines. Mobile: vertical timeline. Scroll-triggered reveal.

Key implementation notes:
- SVG lines use `stroke-dashoffset` CSS animation, triggered when section scrolls into view
- Steps stagger reveal with 0.15s delay between each
- Waveform animation on Step 2 icon uses CSS `@keyframes` (add to globals.css)
- Mobile: vertical layout with left-side connecting line via CSS `::before` pseudo-element

```tsx
"use client";

import { Keyboard, AudioWaveform, CursorClick } from "lucide-react";
import { Section } from "@/components/ui/section";
import { ScrollReveal } from "@/components/ui/scroll-reveal";

const steps = [
  {
    icon: Keyboard,
    title: "Press Hotkey",
    description: "Hit ⌘+⌥+V from any app. ShhhType starts listening instantly.",
    color: "from-blue-500 to-indigo-500",
  },
  {
    icon: AudioWaveform,
    title: "Speak Naturally",
    description: "Talk at your normal pace. No special commands needed.",
    color: "from-rose-500 to-pink-500",
  },
  {
    icon: CursorClick,
    title: "Text Appears",
    description: "Your words are transcribed and injected right where your cursor is.",
    color: "from-orange-500 to-amber-500",
  },
];

export function HowItWorks() {
  return (
    <Section id="how-it-works" dark>
      <ScrollReveal>
        <h2 className="text-center font-heading text-3xl font-bold sm:text-4xl">
          How it works
        </h2>
        <p className="mx-auto mt-4 max-w-lg text-center text-gray-400">
          Three steps. No setup wizards. No training.
        </p>
      </ScrollReveal>

      <div className="relative mt-16 grid gap-8 md:grid-cols-3 md:gap-6">
        {/* SVG connecting lines — desktop only */}
        <svg
          className="pointer-events-none absolute top-12 right-0 left-0 hidden md:block"
          height="2"
          style={{ width: "calc(100% - 200px)", margin: "0 100px" }}
          aria-hidden="true"
        >
          <line
            x1="0"
            y1="1"
            x2="100%"
            y2="1"
            stroke="#f43f5e"
            strokeWidth="2"
            strokeDasharray="6 4"
            className="animate-draw-line"
          />
        </svg>

        {/* Mobile vertical connector */}
        <div className="absolute top-0 bottom-0 left-6 w-px bg-gradient-to-b from-rose-500/50 to-orange-500/50 md:hidden" aria-hidden="true" />

        {steps.map((step, i) => (
          <ScrollReveal key={step.title} delay={i * 0.15}>
            <div className="relative flex flex-col items-center text-center md:items-center">
              {/* Step number */}
              <div className="mb-4 flex h-8 w-8 items-center justify-center rounded-full bg-gradient-to-br from-rose-500 to-orange-500 text-xs font-bold text-white">
                {i + 1}
              </div>
              {/* Icon */}
              <div className={`mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-gradient-to-br ${step.color} text-white shadow-lg`}>
                <step.icon size={28} />
              </div>
              <h3 className="font-heading text-lg font-bold">{step.title}</h3>
              <p className="mt-2 max-w-xs text-sm text-gray-400">{step.description}</p>
            </div>
          </ScrollReveal>
        ))}
      </div>
    </Section>
  );
}
```

- [ ] **Step 2: Add connecting line animation to globals.css**

```css
@keyframes draw-line {
  from { stroke-dashoffset: 1000; }
  to { stroke-dashoffset: 0; }
}

.animate-draw-line {
  stroke-dashoffset: 1000;
  animation: draw-line 1.5s ease-out forwards;
}
```

- [ ] **Step 3: Add to page.tsx**

```tsx
import { HowItWorks } from "@/components/how-it-works";
// Add after Hero:
<HowItWorks />
```

- [ ] **Step 4: Verify**

Desktop: 3 cards in a row with dashed connecting line, dark background. Mobile: stacks vertically with left-side gradient line. Scroll into view triggers fade-up animation.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: add How It Works section with 3-step flow and SVG connecting lines"
```

---

### Task 7: Features Section (Bento Grid)

**Files:**
- Create: `src/components/features.tsx`
- Modify: `src/app/page.tsx`

- [ ] **Step 1: Create `src/components/features.tsx`**

Light section. Bento grid: 2 large cards (top spanning full width, split 50/50) + 4 small cards in bottom row. Each card has icon, title, description. Hover lift + shadow.

```tsx
"use client";

import {
  Monitor,
  CloudCog,
  Globe,
  PanelTop,
  BookText,
  Keyboard,
} from "lucide-react";
import { Section } from "@/components/ui/section";
import { ScrollReveal } from "@/components/ui/scroll-reveal";

const largeFeatures = [
  {
    icon: Monitor,
    title: "Works in Any App",
    description:
      "Emails, documents, code editors, chat apps, browsers — ShhhType injects text wherever your cursor is.",
    color: "from-violet-500 to-purple-500",
  },
  {
    icon: CloudCog,
    title: "Local or Cloud",
    description:
      "Run Whisper privately on your Mac with Metal GPU, or use Groq cloud for sub-second speed. Switch anytime.",
    color: "from-cyan-500 to-blue-500",
  },
];

const smallFeatures = [
  { icon: Globe, title: "9 Languages", description: "English, Spanish, French, German, and more.", color: "from-green-500 to-emerald-500" },
  { icon: PanelTop, title: "Menu Bar App", description: "Lives quietly in your menu bar. Always one hotkey away.", color: "from-amber-500 to-yellow-500" },
  { icon: BookText, title: "Custom Dictionary", description: "Teach it names, jargon, and domain terms you use.", color: "from-pink-500 to-rose-500" },
  { icon: Keyboard, title: "Global Hotkey", description: "Cmd+Alt+V from anywhere. Customizable in settings.", color: "from-indigo-500 to-blue-500" },
];

function FeatureCard({
  icon: Icon,
  title,
  description,
  color,
  large = false,
}: {
  icon: React.ElementType;
  title: string;
  description: string;
  color: string;
  large?: boolean;
}) {
  return (
    <div
      className={`group rounded-2xl border border-gray-200 bg-white p-6 transition-all duration-200 hover:-translate-y-1 hover:shadow-lg ${
        large ? "sm:p-8" : ""
      }`}
    >
      <div
        className={`mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br ${color} text-white shadow-sm`}
      >
        <Icon size={22} />
      </div>
      <h3 className="font-heading text-lg font-bold">{title}</h3>
      <p className="mt-2 text-sm text-gray-500">{description}</p>
    </div>
  );
}

export function Features() {
  return (
    <Section id="features">
      <ScrollReveal>
        <h2 className="text-center font-heading text-3xl font-bold sm:text-4xl">
          Everything you need
        </h2>
        <p className="mx-auto mt-4 max-w-lg text-center text-gray-500">
          Powerful features, simple interface. No learning curve.
        </p>
      </ScrollReveal>

      <div className="mt-16 grid gap-4 sm:grid-cols-2">
        {largeFeatures.map((f, i) => (
          <ScrollReveal key={f.title} delay={i * 0.1}>
            <FeatureCard {...f} large />
          </ScrollReveal>
        ))}
      </div>

      <div className="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {smallFeatures.map((f, i) => (
          <ScrollReveal key={f.title} delay={i * 0.1}>
            <FeatureCard {...f} />
          </ScrollReveal>
        ))}
      </div>
    </Section>
  );
}
```

- [ ] **Step 2: Add to page.tsx after HowItWorks**

- [ ] **Step 3: Verify**

Desktop: 2 large cards top, 4 small cards bottom. Mobile: all single column. Hover lifts cards. Icons have correct gradient colors.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: add Features section with bento grid layout"
```

---

### Task 8: Local vs Cloud Section

**Files:**
- Create: `src/components/local-vs-cloud.tsx`
- Modify: `src/app/page.tsx`

- [ ] **Step 1: Create `src/components/local-vs-cloud.tsx`**

Dark section. Two-panel comparison. Desktop (md+): side by side. Mobile: tab toggle showing one panel at a time with AnimatePresence transition. ARIA tablist/tab/tabpanel roles. Arrow key navigation.

```tsx
"use client";

import { useState, useCallback } from "react";
import { motion, AnimatePresence, useReducedMotion } from "framer-motion";
import { Shield, Zap, Check } from "lucide-react";
import { Section } from "@/components/ui/section";
import { ScrollReveal } from "@/components/ui/scroll-reveal";
import { Badge } from "@/components/ui/badge";

const modes = [
  {
    id: "local",
    label: "Local Mode",
    icon: Shield,
    color: "from-blue-500 to-indigo-500",
    features: [
      "On-device Whisper AI",
      "Zero data leaves your Mac",
      "Works fully offline",
      "Metal GPU accelerated",
      "Complete privacy",
    ],
  },
  {
    id: "cloud",
    label: "Cloud Mode",
    icon: Zap,
    color: "from-rose-500 to-orange-500",
    recommended: true,
    features: [
      "Groq Whisper Large V3 Turbo",
      "Sub-second transcription",
      "Higher accuracy",
      "Free tier included",
      "9 language support",
    ],
  },
];

export function LocalVsCloud() {
  const [activeTab, setActiveTab] = useState(0);
  const prefersReducedMotion = useReducedMotion();

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === "ArrowRight" || e.key === "ArrowDown") {
        e.preventDefault();
        setActiveTab((prev) => (prev + 1) % modes.length);
      } else if (e.key === "ArrowLeft" || e.key === "ArrowUp") {
        e.preventDefault();
        setActiveTab((prev) => (prev - 1 + modes.length) % modes.length);
      }
    },
    []
  );

  function Panel({ mode, index }: { mode: (typeof modes)[number]; index: number }) {
    return (
      <div
        role="tabpanel"
        id={`panel-${mode.id}`}
        aria-labelledby={`tab-${mode.id}`}
        className="rounded-2xl border border-gray-800 bg-gray-900/50 p-6 sm:p-8"
      >
        <div className="mb-4 flex items-center gap-3">
          <div className={`flex h-10 w-10 items-center justify-center rounded-xl bg-gradient-to-br ${mode.color} text-white`}>
            <mode.icon size={20} />
          </div>
          <h3 className="font-heading text-xl font-bold">{mode.label}</h3>
          {mode.recommended && <Badge variant="warning">Recommended</Badge>}
        </div>
        <ul className="space-y-3">
          {mode.features.map((f) => (
            <li key={f} className="flex items-center gap-3 text-sm text-gray-300">
              <Check size={16} className="shrink-0 text-green-500" />
              {f}
            </li>
          ))}
        </ul>
      </div>
    );
  }

  return (
    <Section id="local-vs-cloud" dark>
      <ScrollReveal>
        <h2 className="text-center font-heading text-3xl font-bold sm:text-4xl">
          Your transcription, your rules
        </h2>
        <p className="mx-auto mt-4 max-w-lg text-center text-gray-400">
          Choose between complete privacy or blazing speed. Switch anytime in Settings.
        </p>
      </ScrollReveal>

      {/* Mobile tab toggle */}
      <div className="mt-12 md:hidden" role="tablist" aria-label="Transcription mode" onKeyDown={handleKeyDown}>
        <div className="flex rounded-full border border-gray-800 bg-gray-900/50 p-1">
          {modes.map((mode, i) => (
            <button
              key={mode.id}
              id={`tab-${mode.id}`}
              role="tab"
              aria-selected={activeTab === i}
              aria-controls={`panel-${mode.id}`}
              tabIndex={activeTab === i ? 0 : -1}
              onClick={() => setActiveTab(i)}
              className={`flex-1 rounded-full px-4 py-2.5 text-sm font-medium transition-all ${
                activeTab === i
                  ? "bg-gradient-to-r from-rose-500 to-orange-500 text-white"
                  : "text-gray-400"
              }`}
            >
              {mode.label}
            </button>
          ))}
        </div>

        <div className="mt-6">
          <AnimatePresence mode="wait">
            <motion.div
              key={activeTab}
              initial={prefersReducedMotion ? {} : { opacity: 0, x: activeTab === 0 ? -20 : 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={prefersReducedMotion ? {} : { opacity: 0, x: activeTab === 0 ? 20 : -20 }}
              transition={{ duration: 0.2 }}
            >
              <Panel mode={modes[activeTab]} index={activeTab} />
            </motion.div>
          </AnimatePresence>
        </div>
      </div>

      {/* Desktop side by side */}
      <div className="mt-12 hidden gap-6 md:grid md:grid-cols-2">
        {modes.map((mode, i) => (
          <ScrollReveal key={mode.id} delay={i * 0.15}>
            <Panel mode={mode} index={i} />
          </ScrollReveal>
        ))}
      </div>
    </Section>
  );
}
```

- [ ] **Step 2: Add to page.tsx after Features**

- [ ] **Step 3: Verify**

Desktop: two panels side by side. Mobile: tab toggle, only one visible at a time, arrow keys switch tabs, AnimatePresence slides. ARIA attributes present in DOM.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: add Local vs Cloud comparison section with accessible tabs"
```

---

### Task 9: AI Rewrite Section

**Files:**
- Create: `src/components/ai-rewrite.tsx`
- Modify: `src/app/page.tsx`

- [ ] **Step 1: Create `src/components/ai-rewrite.tsx`**

Light section with dark terminal inset. Style pills (radiogroup) swap the "after" text. Typewriter animation on text change. Left column: feature highlights. Right column: before/after code block.

Key implementation:
- Style pills: ARIA `role="radiogroup"` / `role="radio"`, arrow key navigation, visible focus ring
- Typewriter effect: Framer Motion `animate` on text change, character-by-character reveal using a `motion.span` per character or a clip-path approach
- Mobile: stacks vertically, pills in 2x2 grid

```tsx
"use client";

import { useState, useCallback } from "react";
import { motion, AnimatePresence, useReducedMotion } from "framer-motion";
import { Sparkles, MessageSquareText, CheckCheck } from "lucide-react";
import { Section } from "@/components/ui/section";
import { ScrollReveal } from "@/components/ui/scroll-reveal";

const rawText =
  "ok so basically what I want to say is that the project timeline needs to be moved up because the client wants it done sooner and we need to figure out how to make that work";

const styles = [
  { label: "Professional", text: "The project timeline requires acceleration to meet revised client expectations. I'd like to discuss resource reallocation to ensure we deliver on the new schedule." },
  { label: "Casual", text: "Hey — heads up, the client wants this done sooner. Let's figure out how to move the timeline up and still keep things solid." },
  { label: "Concise", text: "Client needs earlier delivery. We need to adjust the timeline and plan accordingly." },
  { label: "Friendly", text: "Hi team! Looks like the client is eager to see this sooner. Let's brainstorm how we can make the new timeline work together!" },
];

const highlights = [
  { icon: Sparkles, text: "Improve tone and clarity" },
  { icon: MessageSquareText, text: "Match your writing style" },
  { icon: CheckCheck, text: "Fix grammar and filler words" },
];

export function AiRewrite() {
  const [activeStyle, setActiveStyle] = useState(0);
  const prefersReducedMotion = useReducedMotion();

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      let next = activeStyle;
      if (e.key === "ArrowRight" || e.key === "ArrowDown") {
        e.preventDefault();
        next = (activeStyle + 1) % styles.length;
      } else if (e.key === "ArrowLeft" || e.key === "ArrowUp") {
        e.preventDefault();
        next = (activeStyle - 1 + styles.length) % styles.length;
      }
      setActiveStyle(next);
    },
    [activeStyle]
  );

  return (
    <Section id="ai-rewrite">
      <ScrollReveal>
        <h2 className="text-center font-heading text-3xl font-bold sm:text-4xl">
          AI Rewrite
        </h2>
        <p className="mx-auto mt-4 max-w-lg text-center text-gray-500">
          Speak rough. Ship polished. AI transforms your raw dictation into clear, professional text.
        </p>
      </ScrollReveal>

      <div className="mt-16 grid gap-12 lg:grid-cols-5">
        {/* Left: highlights */}
        <div className="flex flex-col justify-center gap-6 lg:col-span-2">
          {highlights.map((h) => (
            <ScrollReveal key={h.text}>
              <div className="flex items-center gap-4">
                <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-rose-50 text-rose-500">
                  <h.icon size={20} />
                </div>
                <p className="font-medium text-gray-700">{h.text}</p>
              </div>
            </ScrollReveal>
          ))}
        </div>

        {/* Right: terminal block */}
        <div className="lg:col-span-3">
          <ScrollReveal>
            <div className="overflow-hidden rounded-2xl border border-gray-800 bg-[#1a1a1a]">
              {/* Before */}
              <div className="border-b border-gray-800 p-5">
                <p className="mb-2 text-xs font-medium uppercase tracking-wider text-gray-500">Before</p>
                <p className="text-sm leading-relaxed text-gray-400">&ldquo;{rawText}&rdquo;</p>
              </div>

              {/* After */}
              <div className="p-5">
                <p className="mb-2 text-xs font-medium uppercase tracking-wider text-rose-400">
                  After — {styles[activeStyle].label}
                </p>
                <AnimatePresence mode="wait">
                  <motion.p
                    key={activeStyle}
                    initial={prefersReducedMotion ? {} : { opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={prefersReducedMotion ? {} : { opacity: 0 }}
                    transition={{ duration: 0.15 }}
                    className="text-sm leading-relaxed text-gray-200"
                  >
                    &ldquo;
                    {prefersReducedMotion
                      ? styles[activeStyle].text
                      : styles[activeStyle].text.split("").map((char, i) => (
                          <motion.span
                            key={`${activeStyle}-${i}`}
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            transition={{ delay: i * 0.015, duration: 0.01 }}
                          >
                            {char}
                          </motion.span>
                        ))}
                    &rdquo;
                  </motion.p>
                </AnimatePresence>
              </div>

              {/* Style pills */}
              <div className="border-t border-gray-800 p-4">
                <div
                  role="radiogroup"
                  aria-label="Rewrite style"
                  onKeyDown={handleKeyDown}
                  className="flex flex-wrap gap-2"
                >
                  {styles.map((s, i) => (
                    <button
                      key={s.label}
                      role="radio"
                      aria-checked={activeStyle === i}
                      tabIndex={activeStyle === i ? 0 : -1}
                      onClick={() => setActiveStyle(i)}
                      className={`rounded-full px-4 py-2 text-xs font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-rose-500 ${
                        activeStyle === i
                          ? "bg-gradient-to-r from-rose-500 to-orange-500 text-white"
                          : "bg-gray-800 text-gray-400 hover:text-gray-200"
                      }`}
                    >
                      {s.label}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          </ScrollReveal>
        </div>
      </div>
    </Section>
  );
}
```

- [ ] **Step 2: Add to page.tsx after LocalVsCloud**

- [ ] **Step 3: Verify**

Clicking style pills swaps text with fade. Arrow keys navigate pills. ARIA radiogroup/radio in DOM. Mobile: stacks vertically, pills wrap.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: add AI Rewrite section with interactive style pills"
```

---

### Task 10: Pricing Section

**Files:**
- Create: `src/components/pricing.tsx`
- Modify: `src/app/globals.css` (add glow keyframes)
- Modify: `src/app/page.tsx`

- [ ] **Step 1: Add glow animation to globals.css**

```css
@keyframes glow-pulse {
  0%, 100% {
    box-shadow: 0 0 20px rgba(244, 63, 94, 0.15), 0 0 60px rgba(244, 63, 94, 0.05);
  }
  50% {
    box-shadow: 0 0 30px rgba(244, 63, 94, 0.25), 0 0 80px rgba(244, 63, 94, 0.1);
  }
}

.animate-glow-pulse {
  animation: glow-pulse 3s ease-in-out infinite;
}

@media (prefers-reduced-motion: reduce) {
  .animate-glow-pulse {
    animation: none;
    box-shadow: 0 0 20px rgba(244, 63, 94, 0.15);
  }
}
```

- [ ] **Step 2: Create `src/components/pricing.tsx`**

Dark section. Centered card with glow border. Launch Special badge, $29 price, feature checklist, gradient CTA.

```tsx
"use client";

import { Check, ArrowRight } from "lucide-react";
import { Section } from "@/components/ui/section";
import { ScrollReveal } from "@/components/ui/scroll-reveal";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { CHECKOUT_URL } from "@/lib/constants";

const features = [
  "Unlimited transcription",
  "Local + Cloud modes",
  "AI Rewrite",
  "9 languages",
  "Lifetime updates",
  "Free Groq tier included",
];

export function Pricing() {
  return (
    <Section id="pricing" dark>
      <ScrollReveal>
        <h2 className="text-center font-heading text-3xl font-bold sm:text-4xl">
          Simple pricing
        </h2>
        <p className="mx-auto mt-4 max-w-lg text-center text-gray-400">
          One price. Everything included. No subscriptions.
        </p>
      </ScrollReveal>

      <ScrollReveal delay={0.15}>
        <div className="mx-auto mt-16 max-w-md">
          <div className="animate-glow-pulse rounded-3xl border border-gray-800 bg-gray-900/80 p-8 sm:p-10">
            <Badge variant="warning" className="mb-6">Launch Special</Badge>

            <div className="mb-6">
              <span className="font-heading text-5xl font-black text-white">$29</span>
              <span className="ml-2 text-gray-400">one-time</span>
            </div>

            <ul className="mb-8 space-y-3">
              {features.map((f) => (
                <li key={f} className="flex items-center gap-3 text-sm text-gray-300">
                  <Check size={16} className="shrink-0 text-green-500" />
                  {f}
                </li>
              ))}
            </ul>

            <Button
              href={CHECKOUT_URL}
              target="_blank"
              rel="noopener"
              className="w-full justify-center text-base"
            >
              Download for macOS <ArrowRight size={16} />
            </Button>

            <p className="mt-4 text-center text-xs text-gray-500">
              Lock in the one-time price before it moves to $9.99/mo.
            </p>
          </div>
        </div>
      </ScrollReveal>
    </Section>
  );
}
```

- [ ] **Step 3: Add to page.tsx after AiRewrite**

- [ ] **Step 4: Verify**

Centered card with subtle glow animation. Badge, price, checklist, CTA all present. Mobile: card fills width with padding. CTA links to LemonSqueezy.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: add Pricing section with glow card and feature checklist"
```

---

### Task 11: Footer

**Files:**
- Create: `src/components/footer.tsx`
- Modify: `src/app/page.tsx`

- [ ] **Step 1: Create `src/components/footer.tsx`**

```tsx
import { SUPPORT_EMAIL } from "@/lib/constants";

export function Footer() {
  return (
    <footer className="border-t border-gray-800 bg-[#0a0a0a] px-6 py-12">
      <div className="mx-auto flex max-w-6xl flex-col items-center gap-6 sm:flex-row sm:justify-between">
        <div className="flex items-center gap-3">
          <img src="/images/shhh_logo_thick.png" alt="ShhhType" className="h-8 w-8" />
          <div>
            <p className="font-heading text-sm font-bold text-white">ShhhType</p>
            <p className="text-xs text-gray-500">Stop typing. Start talking.</p>
          </div>
        </div>

        <div className="flex items-center gap-6 text-sm text-gray-400">
          <a href={`mailto:${SUPPORT_EMAIL}`} className="transition-colors hover:text-white">
            Support
          </a>
          <span className="text-gray-700">|</span>
          <p>&copy; {new Date().getFullYear()} Genesis1 Tech</p>
        </div>
      </div>
    </footer>
  );
}
```

- [ ] **Step 2: Add to page.tsx after Pricing (outside `<main>`)**

- [ ] **Step 3: Verify**

Logo left, support + copyright right on desktop. Centered stack on mobile.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: add minimal Footer"
```

---

### Task 12: SEO Metadata + JSON-LD + Static Assets

**Files:**
- Modify: `src/app/layout.tsx` (add full metadata export + JSON-LD scripts)
- Create: `public/robots.txt`
- Create: `public/sitemap.xml`
- Copy: `public/images/og-image.png` (from existing site or placeholder)
- Copy: `public/favicon.ico`

- [ ] **Step 1: Copy static assets from existing site**

```bash
# Logo already copied in Task 4 — skip if exists
cp -n /Users/g1tech/_dev/shhhtype/assets/images/shhh_logo_thick.png /Users/g1tech/_dev/shhhtype-web-rebuild/public/images/
cp /Users/g1tech/_dev/shhhtype/robots.txt /Users/g1tech/_dev/shhhtype-web-rebuild/public/
cp /Users/g1tech/_dev/shhhtype/sitemap.xml /Users/g1tech/_dev/shhhtype-web-rebuild/public/
```

If `og-image.png` exists in the existing site, copy it too:
```bash
cp /Users/g1tech/_dev/shhhtype/assets/images/og-image.png /Users/g1tech/_dev/shhhtype-web-rebuild/public/images/ 2>/dev/null || echo "og-image.png not found — create a 1200x630 placeholder or use the logo as fallback"
```

- [ ] **Step 2: Add full metadata export to layout.tsx**

Port all metadata from existing site's `<head>` into Next.js `metadata` export:

```tsx
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "ShhhType - Voice to Text for macOS | Speak, Transcribe, Type Instantly",
  description:
    "ShhhType turns your voice into text in any macOS app. Press a hotkey, speak naturally, and words appear instantly. Private and fast with local or cloud transcription.",
  keywords:
    "voice to text, speech to text, dictation, macOS, voice typing, transcription, whisper, AI dictation, voice to text mac, offline voice to text",
  metadataBase: new URL("https://shhhtype.com"),
  alternates: { canonical: "/" },
  openGraph: {
    type: "website",
    url: "https://shhhtype.com/",
    title: "ShhhType - Voice to Text for macOS",
    description:
      "Press a hotkey, speak naturally, and your words appear in any app. Local Whisper or cloud transcription. Private, fast, effortless.",
    siteName: "ShhhType",
    images: [
      {
        url: "/images/og-image.png",
        width: 1200,
        height: 630,
        alt: "ShhhType - Voice to text for macOS menu bar",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "ShhhType - Voice to Text for macOS",
    description:
      "Press a hotkey, speak naturally, and your words appear in any app. Local or cloud transcription.",
    images: ["/images/og-image.png"],
  },
  icons: {
    icon: "/images/shhh_logo_thick.png",
    apple: "/images/shhh_logo_thick.png",
  },
  other: {
    "theme-color": "#f43f5e",
    "apple-mobile-web-app-title": "ShhhType",
    "application-name": "ShhhType",
  },
};
```

- [ ] **Step 3: Add JSON-LD scripts to layout.tsx body**

Add inside the `<body>`, before `{children}`:

```tsx
<script
  type="application/ld+json"
  dangerouslySetInnerHTML={{
    __html: JSON.stringify({
      "@context": "https://schema.org",
      "@type": "SoftwareApplication",
      name: "ShhhType",
      description: "macOS menu bar voice-to-text app. Press a hotkey, speak, and your words are transcribed and injected into any focused application.",
      url: "https://shhhtype.com",
      applicationCategory: "UtilitiesApplication",
      operatingSystem: "macOS",
      offers: {
        "@type": "Offer",
        price: "29.00",
        priceCurrency: "USD",
        availability: "https://schema.org/InStock",
      },
      featureList: "Voice to text, Global hotkey, Local Whisper transcription, Cloud Groq transcription, AI rewrite, 9 language support, Menu bar app, Works in any application",
      softwareVersion: "1.0",
      downloadUrl: "https://shhhtype.lemonsqueezy.com/checkout/buy/1ea919ae-5f44-4ea9-bc4d-95e64cb41a87",
      author: {
        "@type": "Organization",
        name: "Genesis1 Tech",
        url: "https://g1tech.cloud",
      },
    }),
  }}
/>
```

Add the FAQPage JSON-LD as a second `<script>`:

```tsx
<script
  type="application/ld+json"
  dangerouslySetInnerHTML={{
    __html: JSON.stringify({
      "@context": "https://schema.org",
      "@type": "FAQPage",
      mainEntity: [
        {
          "@type": "Question",
          name: "How does ShhhType work?",
          acceptedAnswer: {
            "@type": "Answer",
            text: "Press a global hotkey (Cmd+Alt+V), speak naturally, and ShhhType transcribes your speech and injects the text directly into whatever app you're using. It works in any macOS application.",
          },
        },
        {
          "@type": "Question",
          name: "Does ShhhType work offline?",
          acceptedAnswer: {
            "@type": "Answer",
            text: "Yes. ShhhType supports local transcription using Whisper running directly on your Mac with Metal GPU acceleration. No internet connection required.",
          },
        },
        {
          "@type": "Question",
          name: "What is the difference between Cloud and Local mode?",
          acceptedAnswer: {
            "@type": "Answer",
            text: "Cloud mode uses Groq's API with Whisper Large V3 Turbo for fast, accurate transcription. Local mode runs Whisper on your Mac for complete privacy with no data leaving your device. You can switch anytime in Settings.",
          },
        },
        {
          "@type": "Question",
          name: "How much does ShhhType cost?",
          acceptedAnswer: {
            "@type": "Answer",
            text: "ShhhType is $29 one-time during the launch period. This includes all features, lifetime updates, and free Groq API tier. It will move to $9.99/month after the launch period.",
          },
        },
        {
          "@type": "Question",
          name: "What languages does ShhhType support?",
          acceptedAnswer: {
            "@type": "Answer",
            text: "ShhhType supports 9 languages including English, Spanish, French, German, Italian, Portuguese, Dutch, Japanese, and Chinese.",
          },
        },
      ],
    }),
  }}
/>
```

- [ ] **Step 4: Verify build with SEO**

```bash
npm run build
```

Check `out/index.html` — confirm all meta tags, OG tags, JSON-LD scripts are present in the static output.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: add SEO metadata, JSON-LD schemas, and static assets"
```

---

### Task 13: Final Polish + Build Verification

**Files:**
- Modify: `src/app/globals.css` (any remaining custom CSS)
- Modify: various components (responsive tweaks)

- [ ] **Step 1: Full responsive audit**

Run `npm run dev` and test at these widths:
- 375px (iPhone SE)
- 390px (iPhone 14)
- 768px (iPad)
- 1024px (laptop)
- 1440px (desktop)

Check each section: no horizontal scroll, text readable, touch targets 44px+, cards stack properly.

- [ ] **Step 2: Verify static build**

```bash
npm run build
npx serve out
```

Open the served URL. All sections render. All links work. All animations play. No console errors.

- [ ] **Step 3: Lighthouse audit**

Run Lighthouse in Chrome DevTools on the served static build. Targets:
- Performance: 95+
- Accessibility: 100
- Best Practices: 95+
- SEO: 100

Fix any issues found.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: final polish and responsive audit"
```

- [ ] **Step 5: Verify full build one more time**

```bash
npm run build
```

Expected: Clean build, no warnings, `out/` directory contains all static files.
