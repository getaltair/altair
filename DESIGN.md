# Design System: Altair
**Project ID:** 17347578666140077899

---

## 1. Visual Theme & Atmosphere

Altair's design language is called **"The Digital Sanctuary"** — or more precisely, **"The Ethereal Canvas."** The interface is not a tool; it is an atmospheric environment. Think of a high-end editorial magazine printed on heavy-weight paper: airy, intentionally asymmetric, and deeply unhurried.

The defining quality is **tonal depth without structural lines**. There are no 1px borders boxing sections. Hierarchy is established through near-imperceptible shifts in paper-white tones and generous negative space. The result feels less like software and more like a curated personal environment — calm, intelligent, and uncluttered.

Altair supports both a **Light Mode** (default) and a **Dark Mode**. Light mode reads as soft warm-white mist; dark mode reads as deep cool-black marine space. Both share the same spatial logic and type system.

---

## 2. Color Palette & Roles

### Light Mode (Default)

| Descriptive Name | Hex | Role |
|---|---|---|
| Foggy Canvas White | `#f8fafa` | Base page background (`background`, `surface`) |
| Gossamer White | `#ffffff` | Interactive cards and input fields (`surface-container-lowest`) |
| Pale Seafoam Mist | `#f0f4f5` | Zoning panels and secondary content areas (`surface-container-low`) |
| Cool Linen Gray | `#e9eff0` | Divider-free content groupings (`surface-container`) |
| Faded Glacier | `#e1eaeb` | Elevated sidebar and panel backgrounds (`surface-container-high`) |
| Dusty Mineral Blue | `#dae5e6` | Highest-contrast surface layer, tags, chips (`surface-container-highest`, `surface-variant`) |
| Soft Slate Haze | `#cfddde` | Dimmed background state when Focus Mode activates (`surface-dim`) |
| Deep Muted Teal-Navy | `#446273` | Primary accent — action buttons, links, active states (`primary`) |
| Sky-Washed Aqua | `#c7e7fa` | Primary containers, gradient endpoint, highlighted content zones (`primary-container`) |
| Midnight Charcoal | `#2a3435` | All body text, headings, high-legibility content (`on-surface`, `on-background`) |
| Weathered Slate | `#566162` | Secondary labels, metadata, captions — "editorial tag" color (`on-surface-variant`) |
| Steel Blue-Gray | `#4f626b` | Supporting interactive elements, secondary icons (`secondary`) |
| Wintry Powder Blue | `#d1e6f0` | Secondary container fill for chips, badges (`secondary-container`) |
| Dusty Sage Teal | `#546260` | Tertiary accents — decorative dividers, subtle highlights (`tertiary`) |
| Pale Spearmint | `#e8f7f4` | Tertiary container fill for informational callouts (`tertiary-container`) |
| Sophisticated Terracotta | `#9f403d` | Error state — calm and considered, not alarming (`error`) |
| Pewter Teal | `#727d7e` | Low-emphasis borders, separators when absolutely required (`outline`) |
| Ghost Border Ash | `#a9b4b5` | Accessibility fallback borders at 15% opacity only (`outline-variant`) |

### Dark Mode

| Descriptive Name | Hex | Role |
|---|---|---|
| Abyssal Ink | `#080f10` | Base dark background — near-black with a marine undertone |
| Deep Sea Charcoal | `#0c1c1e` | Surface containers in dark mode |
| Midnight Teal Void | `#10292c` | Surface variant layer in dark |
| Moonlit Teal | `#123034` | Bright surface — elevated modals and panels in dark |
| Glacial Sky Blue | `#abcbdd` | Primary accent in dark mode — cool, luminous |
| Abyssal Navy | `#2c4b5a` | Primary container in dark mode |
| Arctic Whisper | `#d0eaee` | Primary text on dark surfaces |
| Faded Teal Smoke | `#617a7d` | Outline and secondary text in dark mode |
| Warm Coral Ember | `#ee7d77` | Error state in dark mode |

### Signature Gradient
All primary CTAs and progress indicators blend **Deep Muted Teal-Navy** (`#446273`) into **Sky-Washed Aqua** (`#c7e7fa`) at a **135-degree angle** in light mode. In dark mode, use **Abyssal Navy** (`#2c4b5a`) to **Glacial Sky Blue** (`#abcbdd`) at the same angle. This "lit-from-within" treatment distinguishes interactive surfaces from passive ones.

---

## 3. Typography Rules

Altair uses a deliberate **dual-font editorial pairing**:

- **Manrope** — all Display and Headline text. Wide apertures and open geometry evoke modern openness. Use for dashboard greetings, section titles, empty-state headers, and any text intended to anchor visual hierarchy.
- **Plus Jakarta Sans** — all Body, Label, and UI text. Approachable and highly legible at small sizes; standardized with generous line-height (1.6+) and letter-spacing of 0.02em for readability.

**Hierarchy rules:**
- **Display** (3.5rem, Manrope): Dashboard greetings, "Zen mode" full-screen headings. Rare by design.
- **Headline Large** (1.75–2rem, Manrope, regular-medium weight): Primary section titles per screen.
- **Title** (1rem–1.25rem, Plus Jakarta Sans, semibold): Card headers, item names, grouped list titles.
- **Body** (0.875rem–1rem, Plus Jakarta Sans, regular): All descriptive text, summaries, explanations.
- **Label / Metadata** (0.75rem, Plus Jakarta Sans, **all-caps**, 0.1em tracking, `#566162`): Timestamps, source tags, category chips. This "editorial tag" look elevates metadata into curated information. Never use pure black for metadata.

---

## 4. Component Stylings

### Buttons
- **Primary CTA:** Pill-shaped (border-radius: `9999px`), Deep Muted Teal-Navy (`#446273`) background, Gossamer White (`#f1f9ff`) text. Optionally rendered as the Signature Gradient for high-emphasis actions. Transition on hover: 300ms `cubic-bezier(0.4, 0, 0.2, 1)`.
- **Secondary / Ghost Button:** Pill-shaped, transparent background with Dusty Mineral Blue (`#dae5e6`) border at very low opacity, Midnight Charcoal (`#2a3435`) text.
- **Icon Buttons:** Circular, Pale Seafoam Mist (`#f0f4f5`) background, no visible border. On hover: background transitions to Cool Linen Gray (`#e9eff0`).

### Cards & Containers
- **Standard Cards:** Generously rounded corners (border-radius: `1rem`, i.e. `rounded-2xl`). Background: Gossamer White (`#ffffff`) placed atop Pale Seafoam Mist (`#f0f4f5`) — the contrast alone creates a "soft architectural lift" with no shadow needed for static cards.
- **Elevated Floating Elements (Modals, Popovers):** Glassmorphism treatment — `surface` at 80% opacity with `backdrop-blur: 20px`. For shadows: `box-shadow: 0 20px 40px rgba(42, 52, 53, 0.06)`. The shadow color is a tint of `on-surface`, never pure black.
- **Dividers:** Strictly forbidden inside cards and between list items. Use vertical `spacing-6` (2rem) gaps instead. Content must breathe.

### Inputs & Forms
- **Input Fields:** No border, no bottom line. Fill: Pale Seafoam Mist (`#f0f4f5`). On focus: background transitions to Gossamer White (`#ffffff`) with a Ghost Border of Deep Muted Teal-Navy (`#446273`) at 20% opacity. Transition: 300ms `cubic-bezier(0.4, 0, 0.2, 1)`.
- **Search Bars:** Same as inputs, typically pill-shaped with an embedded icon. Background subtly darkens toward Cool Linen Gray (`#e9eff0`) when inactive.

### Navigation & Sidebar
- **Left Sidebar:** Faded Glacier (`#e1eaeb`) background with no visible borders. Navigation items use Midnight Charcoal (`#2a3435`) text; active/selected state uses Deep Muted Teal-Navy (`#446273`) with a Pale Seafoam Mist (`#f0f4f5`) pill background.
- **Top Navigation Bar:** Foggy Canvas White (`#f8fafa`) background, flush with the page. Contains branding, notification icon, and user avatar. No bottom border.

### Tags & Badges
- Background: Dusty Mineral Blue (`#dae5e6`) or Sky-Washed Aqua (`#c7e7fa`). Text: Midnight Charcoal (`#2a3435`). Roundness: `rounded-full`. All-caps label treatment with 0.1em letter-spacing.

### Capacity / Progress Indicators ("The Pulse")
- Track: Dusty Mineral Blue (`#dae5e6`). Fill: Deep Muted Teal-Navy (`#446273`) with a 2px outer glow in the same color at 20% opacity. This represents energy without alarm.

### Expandable / Disclosure Patterns
- Trigger: "Soft Chevron" icon. On hover: parent gains a `surface-variant` background tint. Transition: slow 300ms `cubic-bezier(0.4, 0, 0.2, 1)` — never snappy.

### Focus State Containers
- When a writing area or task is selected: surrounding elements dim to Soft Slate Haze (`#cfddde`); the focused container scales to 1.02x and background transitions to Gossamer White (`#ffffff`).

---

## 5. Layout Principles

### The "No-Line" Rule
Explicit borders between sections are **prohibited**. Visual zones are created exclusively through:
1. **Tonal Shifts** — placing a lighter surface atop a slightly darker surface.
2. **Negative Space** — using 8px, 12px, and 16px spacing increments to define zones of focus.

### Spacing Philosophy
When in doubt, **add more padding**. If a section looks like it needs more content, it needs more space. The spacing scale anchors at multiples of 4px, with key values at 8, 12, 16, 24, 32, and 48px. List items are separated by `spacing-6` (2rem) — no dividers.

### Grid & Asymmetry
High-level stats and editorial anchors are placed **off-center** to create a magazine-like, curated feel. Symmetrical three-column grids signal "utility software" — avoid them for key views. Embrace intentional asymmetry: a wide content panel beside a narrow metadata strip reads as sophisticated, not unfinished.

### The "Breathe" Transition
All state changes — hover, focus, active — use a **300ms** duration with `cubic-bezier(0.4, 0, 0.2, 1)` easing. This creates a tactile, fluid sensation. Avoid instant or "snappy" transitions; in Altair, deliberate pacing is a design decision.

### Absolute Rules
- **Never use pure black (`#000000`)** — use Midnight Charcoal (`#2a3435`) for maximum-contrast text.
- **Never use "Alert Red"** — use Sophisticated Terracotta (`#9f403d`) for errors; it communicates urgency without panic.
- **Minimum border-radius of 0.5rem** — the "Soft Corner" rule. Prefer `1rem` (rounded-2xl) for cards. Pure square corners (`rounded-none`) are never used.
- **Ghost Border fallback** — if a border is ever required for accessibility, use Ghost Border Ash (`#a9b4b5`) at **15% opacity maximum**. Any higher is visual clutter.
