# ShhhType Landing Page Design Spec

**Date:** 2026-03-17
**Status:** Draft
**Approach:** Developer-tuned template adaptation (Approach B)

## Overview

A single-page marketing landing page for ShhhType, a macOS menu bar voice-to-text app. Built as a static HTML page using Tailwind CSS CDN, adapting the OpenTranslateAI template's design system (glass nav, rounded cards, rose/orange palette, Instrument Serif + Inter + Montserrat fonts, fade-up animations).

**Target audience:** Everyone — developers, business professionals, students, everyday users.
**Primary CTA:** "Buy Now — $29" → LemonSqueezy checkout.
**Color palette:** Rose/orange (template default). Primary gradient: `from-rose-500 to-orange-500`. Accent backgrounds: orange-50, rose-50, indigo-100.

## Tech Stack

- Single `index.html` file
- Tailwind CSS via CDN (`cdn.tailwindcss.com`)
- Lucide icons via CDN
- Google Fonts URL: `https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=Inter:wght@300;400;500;600&family=Montserrat:wght@300;400;500;600;700&display=swap`
- No build step, no framework, no JavaScript framework
- CSS-only animations (fade-up, slide-in, infinite-scroll marquee)

## Custom CSS (in `<style>` tag)

Since this uses Tailwind CDN (no PostCSS), the following custom classes must be defined in a `<style>` block:

```css
/* Glass navbar */
.glass-nav {
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
}

/* Animations */
@keyframes fadeUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}
@keyframes slideInRight {
  from { opacity: 0; transform: translateX(20px); }
  to { opacity: 1; transform: translateX(0); }
}
@keyframes infinite-scroll {
  from { transform: translateX(0); }
  to { transform: translateX(-100%); }
}

.animate-fade-up { animation: fadeUp 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards; opacity: 0; }
.animate-slide-in { animation: slideInRight 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards; opacity: 0; }
.animate-infinite-scroll { animation: infinite-scroll 40s linear infinite; }

/* Stagger delays */
.delay-100 { animation-delay: 0.1s; }
.delay-200 { animation-delay: 0.2s; }
.delay-300 { animation-delay: 0.3s; }
.delay-500 { animation-delay: 0.5s; }
.delay-700 { animation-delay: 0.7s; }

/* Marquee edge fade */
.marquee-mask {
  mask-image: linear-gradient(to right, transparent, black 10%, black 90%, transparent);
  -webkit-mask-image: linear-gradient(to right, transparent, black 10%, black 90%, transparent);
}

/* Pause marquee on hover */
.group:hover .animate-infinite-scroll { animation-play-state: paused; }

/* Selection color */
::selection { background-color: #f43f5e; color: white; }
```

## Placeholder URLs

All "Buy Now" buttons use the same placeholder href for easy find-and-replace:
```
href="https://LEMONSQUEEZY_CHECKOUT_URL"
```
Replace with the actual LemonSqueezy checkout URL before launch.

## Sections

### 1. Navbar

Fixed glass-blur pill nav centered at top (same as template).

- **Brand:** "ShhhType" in Instrument Serif italic, 2xl
- **Links:** Features | How It Works | Pricing (scroll anchors)
- **CTA button:** "Buy Now" dark pill with arrow, hover rose → LemonSqueezy checkout
- **Behavior:** `glass-nav` class with `backdrop-filter: blur(16px)`, border, rounded-full, shadow on hover

### 2. Hero

White rounded card (`rounded-[3rem]`) with subtle background grid. 12-column grid: 7-col left content, 5-col right visual.

**Left content:**
- Badge: `MACOS APP V1.0` with green pulse dot, rounded-full pill
- Headline (Instrument Serif, ~5.5rem):
  ```
  Voice to
  Text
  made Instant &
  Private.
  ```
  - "Voice to" → gray-900
  - "Text" → italic gray-400
  - "Instant" → transparent text with rose-to-orange gradient clip
  - "Private." → gray-900
- Subheadline (Montserrat, lg/xl, gray-500): "Press a hotkey, speak, and your words are transcribed and injected into any app — documents, emails, chat, code editors. Powered by Groq on macOS."
- Buttons:
  - Primary: "Buy Now — $29" dark pill with arrow icon, hover rose with shadow
  - Secondary: "See Features" text link with down-arrow, scrolls to `#features`

**Right visual (4-step flow):**
Decorative offset background (orange-100/50, -rotate-3) behind main container. Main container: gradient from-orange-50 via-white to-rose-50, rounded-2rem, shadow-2xl.

Four stacked cards inside:
1. ⌘ icon (rose-orange gradient square) + "Press Cmd+Alt+V" + "Global hotkey triggers recording"
2. 🎙️ icon + "Speak naturally" + "VAD auto-detects when you stop"
3. ⚡ icon + "Text appears instantly" + "Injected into any focused app"
4. ✨ icon + "AI polishes your text" + "Rewrite in 4 styles with Cmd+Alt+R"

Each card: white bg, rounded-2xl, shadow, flex row with icon + text.

Floating decorative badges (top-right, bottom-left) with microphone and sparkle icons, rotated, hover-to-straighten (same as template pattern).

### 3. How It Works

Centered section with `id="how-it-works"`.

- **Header:** "How It Works" (Montserrat semibold, 4xl/5xl, centered)
- **Layout:** 4 columns desktop, 2 tablet, 1 mobile. Gap-8.

Each step card (white, rounded-2rem, border, shadow, hover-shadow-xl):
- Step number badge: rose-orange gradient circle with white number
- Icon area (colored background, 44px tall)
- Title (Montserrat semibold)
- Description (gray-500)
- Dev touch element (small detail unique to each card)

| Step | Icon | Title | Description | Dev Touch |
|------|------|-------|-------------|-----------|
| 1 | Keyboard/Command | Trigger | Press `Cmd+Alt+V` from any app | Monospace hotkey badge: `⌘ ⌥ V` styled as keyboard keys |
| 2 | Microphone | Speak | Talk naturally, silence detection stops recording automatically | Small CSS waveform bars animation |
| 3 | Zap/Lightning | Transcribe | Groq cloud processes your audio in under a second | Terminal-style output: `→ "Schedule the meeting for Tuesday at 3pm"` |
| 4 | Sparkles | Polish | AI rewrites your text in your chosen style | Before/after pill: casual → professional |

### 4. Feature Cards

Section with `id="features"`.

- **Header:** "Features" (Montserrat semibold, 4xl/5xl, centered)
- **Layout:** 3x2 grid on desktop, 2-col tablet, 1-col mobile. Gap-8.

Each card follows template pattern: white rounded-2rem, colored icon header area (h-44), title, description, subtitle tag, Buy Now button at bottom.

| # | Icon BG | Icon | Title | Description | Tag |
|---|---------|------|-------|-------------|-----|
| 1 | Orange (#FFE4D6) | Keyboard | Global Hotkey | Push-to-talk or toggle mode. Works from any app — documents, emails, chat, code editors, browsers. | Configurable · Universal |
| 2 | Indigo (#E0E7FF) | Lock | Privacy First | Local mode keeps everything on-device. Cloud mode sends only audio to Groq — nothing else leaves your machine. | On-device · Secure |
| 3 | Orange (#FFE4D6) | Lightning | Blazing Fast | Groq cloud transcription returns results in under a second. Metal GPU accelerates local mode on Apple Silicon. | Sub-second · Optimized |
| 4 | Indigo (#E0E7FF) | Sparkles | AI Rewrite | Polish transcriptions with 4 styles: Professional, Casual, Concise, Friendly. One hotkey, instant results. | Llama 3.3 70B · Smart |
| 5 | Orange (#FFE4D6) | Globe | 9 Languages + Auto-detect | English, Spanish, French, German, Italian, Portuguese, Japanese, Korean, Chinese — plus automatic language detection. | Multilingual · Auto-detect |
| 6 | Indigo (#E0E7FF) | Clock/History | Smart History | Searchable transcription history with JSON export. Custom dictionary corrects terms automatically. | Searchable · Exportable |

Each card has a full-width "Buy Now" button at bottom (dark bg, white text, hover gray-800).

### 5. Local vs Cloud Comparison

Adapts the template's Pros & Cons card layout.

- **Header:** "Two Modes, One App" (Instrument Serif, 4xl/5xl) + subtitle: "Cloud is the default for speed and accuracy. Local mode is there when you need full privacy."
- **Card:** White rounded-2.5rem, border, shadow-xl. Split into 2 columns (grid md:grid-cols-2).

**Left — Cloud (Groq):**
- Header: green checkmark icon circle + "Cloud (Groq)" title
- "RECOMMENDED" rose pill badge
- Items:
  - Whisper Large V3 Turbo model
  - Sub-second transcription
  - No model download needed
  - AI Rewrite enabled
  - Requires internet + free Groq API key

**Right — Local (Whisper):**
- Header: info icon circle + "Local (Whisper)" title
- Gray background (bg-gray-50/50)
- Items:
  - On-device via whisper.cpp
  - Metal GPU acceleration on Apple Silicon
  - Models from Tiny (75MB) to Large V3 (3.1GB)
  - Fully offline, zero data leaves your machine
  - Slower on larger models

**Bottom footer bar:** Dark (bg-[#1A2626]), centered text: "We recommend Cloud mode for the best experience. Switch to Local anytime in Settings for full offline privacy."

### 6. AI Rewrite Showcase

Dark section (bg-gray-900, rounded-3rem) with blur orbs for visual contrast.

- **Badge:** `AI REWRITE` rose pill with pulse dot
- **Header:** "Your Voice," (white) + "Polished." (gray-400 italic)
- **Subtitle:** "Rewrite transcriptions with a single hotkey. Four styles. Instant results."

**Left side:**
- Benefit rows (icon box + title + description):
  1. Style Selector — "Professional, Casual, Concise, Friendly — pick your tone"
  2. Composition Buffer — "Accumulates segments across recordings for longer rewrites"
  3. One Hotkey — "Press Cmd+Alt+R to rewrite instantly"

- Buttons: "Buy Now — $29" (white bg) + "See Pricing" (border gray-700, scrolls to `#pricing`)

**Right side (terminal-style card):**
Dark card (bg-gray-800/50, backdrop-blur, border gray-700, rounded-3xl, mono font).
- Window dots (red, yellow, green)
- Content:
  ```
  // Voice input:
  "ok so basically the meeting is moved to
   tuesday and we need to update the client
   on the timeline changes asap"

  // → Professional rewrite:
  "The meeting has been rescheduled to Tuesday.
   We need to promptly update the client
   regarding the revised timeline."
  ```
- Style pills row below: `Professional` (highlighted), `Casual`, `Concise`, `Friendly`
- Decorative bar at bottom: "Powered by Llama 3.3 70B" with green progress bar (illustrative, not a real metric)

### 7. Pro/Premium Section

Dark section (bg-gray-900, rounded-3rem). Upsell for future subscription.

- **Badge:** `PRO VERSION` rose pill with pulse dot
- **Header:** "Unlimited Cloud" (white) + "Transcription & AI Rewrite." (gray-400 italic)
- **Subtitle:** "The Early Adopter deal won't last forever. Lock in unlimited access before we switch to monthly pricing."

**Left side benefits:**
1. Unlimited Transcriptions — No caps on Groq cloud usage
2. AI Rewrite Included — All 4 styles, composition buffer, Llama 3.3 70B
3. All 9 Languages — Plus auto-detect
4. Lifetime v1.x Updates — Early adopters get all updates through v1

Buttons: "Buy Now — $29" (white bg) + "Learn More" (border, scrolls to pricing)

**Right side (terminal-style card):**
```
// ShhhType License
plan: "Early Adopter"
price: "$29 one-time"
cloud_transcription: unlimited
ai_rewrite: enabled
languages: 9 + auto-detect
updates: "all v1.x releases"
```
Progress bar: "Early Adopter Spots → filling up"

### 8. Marquee

Full-width scrolling strip (same as template). Transparent bg, `marquee-mask` gradient edges, `animate-infinite-scroll` at 40s, pause on hover.

**Logos (text labels with simple SVG icons from Lucide where available, otherwise generic icons):**
- Groq (generic `Cpu` icon)
- Whisper (generic `AudioWaveform` icon)
- Apple Silicon (`Apple` — use simple custom SVG apple shape)
- Tauri (`Layers` icon)
- Rust (`Cog` icon)
- Llama 3.3 (`Brain` icon)

Note: These are not brand logos — they are generic Lucide icons paired with text labels. No brand SVGs required.

Duplicate set for seamless infinite scroll. Gray-400 default, colored on hover.

### 9. Pricing

Section with `id="pricing"`. Single centered pricing card.

- **Header:** "Simple Pricing" (Montserrat semibold, centered)
- **Subtitle:** "One price. Everything included. No subscriptions — yet."

**Card** (white, large, rounded-2.5rem, shadow-xl, max-w-lg, centered):
- "LAUNCH SPECIAL" rose pill badge at top
- Price: "$29" (5xl+ bold) + "one-time" (gray-500, sm)
- Tagline: "Lock in lifetime v1.x access before we switch to $9.99/mo"
- Feature checklist (green checkmarks):
  - Unlimited cloud transcription (Groq)
  - AI Rewrite — 4 styles
  - 9 languages + auto-detect
  - Local Whisper mode
  - Custom dictionary
  - Searchable history with export
  - All v1.x updates
- CTA: "Buy Now" full-width button, rose bg, white text, rounded-xl, shadow → LemonSqueezy

**Below card:** muted small text: "Pro subscription ($9.99/mo) coming soon with priority support and all future updates."

### 10. Final CTA

White rounded card (rounded-3rem, shadow, border).

- **Header:** "Stop typing." (Instrument Serif, 4xl+) + "Start talking." (italic gray-400)
- **Subtitle:** "ShhhType lives in your menu bar, ready whenever you are. One hotkey. Any app. Instant text."
- **CTA:** "Buy Now — $29" large rose pill button → LemonSqueezy
- **Below button:** "macOS · Apple Silicon recommended" muted small text

### 11. Footer

Minimal. Light-gray bar (bg-gray-100, rounded top).

- Left: "© 2026 ShhhType. All rights reserved."
- Right: "support@shhhtype.com" (placeholder — replace before launch)

## Animations

All animations from template preserved:
- `fadeUp` — 0.8s cubic-bezier, staggered delays (100ms-700ms)
- `slideInRight` — hero right visual
- `infinite-scroll` — marquee at 40s linear, pause on hover
- `animate-ping` — badge pulse dots
- Hover transitions: shadow, scale, color, transform (300-500ms)

## Responsive Behavior

- **Desktop (lg+):** Full grid layouts, 12-col hero, 3-col features, 4-col how-it-works
- **Tablet (md):** 2-col grids, hero stacks vertically
- **Mobile:** Single column, reduced padding, smaller headings. Floating decorative badges in hero hidden on mobile (`hidden md:block`). Decorative offset background hidden on mobile.

## External Links

- All "Buy Now" buttons → LemonSqueezy checkout URL (placeholder until set up)
- "See Features" → `#features` scroll
- "How It Works" nav link → `#how-it-works`
- "Pricing" nav link → `#pricing`
- No GitHub links anywhere on the page

## File Structure

```
shhhtype/
└── index.html    # Single-file landing page (HTML + Tailwind CDN)
```
