# ShhhType Website Rebuild — Design Spec

## Overview

Rebuild the ShhhType marketing website (shhhtype.com) from a single static HTML file into a modern, mobile-first Next.js application. The site is a single-page marketing site whose sole purpose is driving macOS app downloads/purchases via LemonSqueezy.

**Current state:** 992-line static HTML file with Tailwind CDN. Visually polished but not truly responsive — grid layouts break on mobile, text doesn't scale, hero section is cramped on small screens. No build system, no component reuse.

**Target state:** Component-based Next.js static export with mobile-first responsive design, modern geometric typography, scroll-driven animations, and a streamlined 7-section layout focused on conversion.

## Tech Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Framework | Next.js 15 (App Router) | Component architecture, static export, built-in SEO via next/metadata |
| Output | Static export (`output: 'export'`) | Zero server cost, deploy anywhere, perfect Lighthouse scores |
| Styling | Tailwind CSS v4 | Utility-first, design tokens, container queries |
| Animations | Framer Motion | Scroll-triggered reveals, layout animations, gesture support, respects prefers-reduced-motion |
| Typography | Satoshi (700/900) + General Sans (400/500) via Fontshare | Geometric, modern, lightweight variable fonts. Self-hosted via `next/font/local` (see Font Loading section) |
| Icons | Lucide React | Tree-shakeable, consistent with current site |
| Deployment | Vercel (free tier) | One-click deploy, CDN, analytics |
| SEO | next/metadata API + JSON-LD schemas | SoftwareApplication + FAQPage structured data (carry over from current site) |

## Font Loading Strategy

Fontshare fonts are not available via `next/font/google`. Fonts must be self-hosted:

1. Download Satoshi (Variable, 700, 900) and General Sans (Variable, 400, 500) WOFF2 files from Fontshare
2. Place in `public/fonts/satoshi/` and `public/fonts/general-sans/`
3. Load via `next/font/local` in `lib/fonts.ts`, exporting CSS variables (`--font-satoshi`, `--font-general-sans`)
4. Apply variables in `layout.tsx` on the `<body>` element
5. Reference in Tailwind config via `fontFamily` extension
6. Subset fonts to Latin characters to keep total font weight under 150KB

## Next.js Configuration

`next.config.js` requirements:
- `output: 'export'` for static generation
- `trailingSlash: true` for clean URLs on static hosts
- `images: { unoptimized: true }` — use native `<img>` tags (via Tailwind) since `next/image` optimization is unavailable in static export. Manually optimize all images (WebP format, responsive srcset where needed)
- No `basePath` needed (deployed at root)

## CTA Destinations

All "Download for macOS" CTA buttons link to the LemonSqueezy checkout URL:
`https://shhhtype.lemonsqueezy.com/checkout/buy/1ea919ae-5f44-4ea9-bc4d-95e64cb41a87`

This is the same URL used in the current site. Opens in a new tab (`target="_blank" rel="noopener"`).

"See how it works" secondary CTA smooth-scrolls to the How It Works section (`#how-it-works`).

## Design Direction

**Style:** Hybrid editorial — alternating light/dark sections, rose-to-orange gradient accents, generous whitespace, oversized geometric headings. Evolved from the current site's DNA, not a complete departure.

**Color palette:**
- Primary gradient: Rose-500 (#f43f5e) → Orange-500 (#f97316)
- Light backgrounds: #FAFAFA, white
- Dark backgrounds: #0a0a0a, #111111, #1a1a1a
- Text: #171717 (on light), #e5e5e5 (on dark)
- Accent: Green-500 for success/checkmarks

**Typography:**
- Headings: Satoshi 900 (hero), Satoshi 700 (section headings)
- Body: General Sans 400
- UI elements: General Sans 500
- Fluid sizing via `clamp()` — no fixed breakpoint jumps

**Mobile-first approach:**
- All CSS written mobile-first, enhanced with Tailwind breakpoints (sm/md/lg/xl)
- Touch targets minimum 44px
- Fluid typography via clamp()
- No horizontal scroll on any viewport
- Framer Motion animations respect `prefers-reduced-motion`

## Page Structure

### Section 0: Navigation
- Sticky top bar with backdrop-blur glass effect (opacity increases on scroll)
- Desktop: Logo (left) → anchor links center (How It Works, Features, Pricing) → gradient CTA button (right)
- Mobile: Logo (left) → hamburger (right) → slide-down sheet with backdrop blur overlay. Focus trapped inside menu when open. Close on Escape key.
- Smooth scroll to anchored sections
- Nav link order matches page section order: How It Works → Features → Pricing

### Section 1: Hero (light)
- Full viewport height
- Left column: oversized Satoshi 900 headline with gradient text, subheadline in General Sans 400, two CTAs (primary gradient "Download for macOS" + secondary outline "See how it works")
- Right column: CSS/Framer Motion animated product mockup — a stylized card sequence showing (1) hotkey badge appearing, (2) waveform animation, (3) text cursor typing out words. Built as React components with Framer Motion, not pre-rendered images or video. Lightweight and responsive.
- Mobile: stacks vertically, headline + CTAs above mockup
- Animations: staggered fade-in on text, floating/subtle motion on mockup
- Background: subtle dot grid or noise texture

### Section 2: How It Works (dark)
- Three-step horizontal flow connected by animated dashed lines
- Step 1: Press Hotkey (keyboard icon) → Step 2: Speak Naturally (waveform icon) → Step 3: Text Appears (cursor-typing icon)
- Each step is a card with animated icon + title + one-line description
- Mobile: vertical timeline with connecting line on left side
- Animations: scroll-triggered staggered reveal, connecting lines drawn via SVG `stroke-dashoffset` animation (CSS `@keyframes`, not JS — cheaper for performance)

### Section 3: Features (light)
- Bento-style grid layout: 2 large cards (top) + 4 smaller cards (bottom row)
- Large cards: "Works in Any App" + "Local or Cloud Transcription"
- Small cards: 9 Languages, Menu Bar App, Custom Dictionary, Global Hotkey
- Each card: icon + title + one-liner description
- Hover: subtle lift + shadow deepening
- Mobile: single column stack, all cards equal width

### Section 4: Local vs Cloud (dark)
- Two-panel split comparison
- Left panel: Local Mode (shield icon) — on-device Whisper, zero data leaves Mac, works offline, Metal GPU accelerated
- Right panel: Cloud Mode (zap icon) — Groq Whisper Large V3, sub-second latency, higher accuracy, free tier included
- "Recommended" badge on Cloud panel
- Tablet (md breakpoint, 768px+): both panels visible side by side
- Mobile (<768px): stacked with tab toggle at top — only one panel visible at a time, animated slide transition between views
- Tab toggle is keyboard navigable (arrow keys to switch, Enter/Space to select)
- Animation: Framer Motion `AnimatePresence` for panel swap on mobile; CSS transition for tab indicator

### Section 5: AI Rewrite (light with dark inset)
- Showcase of the AI Rewrite premium feature
- Dark terminal-style code block showing before/after transformation
- Style selector pills below: Professional, Casual, Concise, Friendly
- Clicking a style pill swaps the "after" text with typewriter animation
- Left side: feature highlights with icons (improve tone, fix grammar, match context)
- Mobile: stacks vertically, pills wrap to 2x2 grid
- This is the strongest upsell moment — copy should emphasize the value

### Section 6: Pricing (dark)
- Centered single pricing card with animated glow border effect (CSS `box-shadow` animation via `@keyframes` — no JS needed for the pulse)
- "Launch Special" badge at top
- $29 one-time price, large and prominent
- Feature checklist (6-7 items with checkmarks): Unlimited transcription, Local + Cloud modes, AI Rewrite, 9 languages, Lifetime updates, Free Groq tier included
- Large gradient CTA button: "Download for macOS"
- Subtle note below card about future pricing changes
- This is the final conversion point — no separate CTA section needed
- Mobile: card takes full width with padding

### Section 7: Footer (dark)
- Minimal two-column layout
- Left: logo + tagline ("Stop typing. Start talking.")
- Right: support email link + social links
- Copyright line below
- Mobile: stacks to centered single column

## Component Architecture

```
src/
├── app/
│   ├── layout.tsx          # Root layout, fonts, metadata
│   ├── page.tsx            # Composes all sections
│   └── globals.css         # Tailwind imports, custom properties
├── components/
│   ├── nav.tsx             # Sticky nav with mobile menu
│   ├── hero.tsx            # Hero section
│   ├── how-it-works.tsx    # 3-step flow
│   ├── features.tsx        # Bento grid
│   ├── local-vs-cloud.tsx  # Split comparison
│   ├── ai-rewrite.tsx      # Interactive demo
│   ├── pricing.tsx         # Pricing card
│   ├── footer.tsx          # Footer
│   └── ui/
│       ├── button.tsx      # Gradient + outline variants
│       ├── badge.tsx       # Status badges
│       ├── card.tsx        # Reusable card wrapper
│       ├── section.tsx     # Section wrapper (handles light/dark + padding)
│       └── scroll-reveal.tsx  # Framer Motion scroll-triggered wrapper
├── lib/
│   └── fonts.ts            # Satoshi + General Sans via next/font
└── public/
    ├── images/             # Logo, OG image
    └── favicon.ico
```

## SEO Preservation

All existing SEO assets must be carried over:
- Title, description, keywords meta tags
- Open Graph + Twitter Card meta tags
- SoftwareApplication JSON-LD schema
- FAQPage JSON-LD schema
- Canonical URL
- robots.txt and sitemap.xml
- Favicon and apple-touch-icon (reuse existing `shhh_logo_thick.png`)
- OG image: reuse existing `og-image.png` from current site for launch. Can be redesigned later to match the new visual style — not blocking.

## Accessibility

Beyond Lighthouse 100, the following must be implemented:
- **Skip-to-content link** as first focusable element, visually hidden until focused
- **Mobile menu:** focus trapped inside when open, close on Escape, return focus to hamburger on close
- **Local vs Cloud tabs:** ARIA `role="tablist"` / `role="tab"` / `role="tabpanel"`, arrow key navigation
- **AI Rewrite style pills:** ARIA `role="radiogroup"` / `role="radio"`, arrow key navigation, visible focus ring
- **All interactive elements:** visible focus rings (Tailwind `focus-visible:ring-2`)
- **Color contrast:** all text meets WCAG AA (4.5:1 body, 3:1 large text)
- **Semantic HTML:** proper heading hierarchy (single h1, h2 per section, h3 for subsections)

## Animations Inventory

| Animation | Trigger | Section |
|-----------|---------|---------|
| Staggered fade-in | Page load | Hero text + CTAs |
| Floating motion | Continuous | Hero product mockup |
| Nav background opacity | Scroll position | Nav |
| Scroll reveal (fade-up) | Scroll into view | All section headings + content |
| Connecting line draw | Scroll into view | How It Works |
| Card hover lift + shadow | Mouse hover | Features, Pricing |
| Tab toggle transition | Click | Local vs Cloud |
| Typewriter text swap | Click style pill | AI Rewrite |
| Glow border pulse | Continuous (subtle) | Pricing card |

All animations respect `prefers-reduced-motion: reduce` — disable or simplify when the user has requested reduced motion.

## Copy Direction

- Inclusive language — ShhhType is for everyone (developers, business professionals, students, creatives), not just developers
- Concise and confident — short sentences, active voice
- Benefit-led — what it does for you, not how it works technically
- Carry over the strong existing copy where it fits the new structure

## Performance Targets

- Lighthouse Performance: 95+
- Lighthouse Accessibility: 100
- First Contentful Paint: <1s
- Total Blocking Time: <50ms
- Cumulative Layout Shift: <0.05
- Total page weight: <500KB (excluding fonts)

## Out of Scope

- Blog, docs, changelog pages
- User authentication or dashboard
- Email capture / waitlist functionality
- CMS integration
- Analytics beyond Vercel Analytics (free)
- Tech stack marquee section (cut)
- Separate Pro Version section (merged into pricing)
- Separate final CTA section (pricing is the final CTA)
