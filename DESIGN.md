# Design System: Altair

**Project ID:** 17347578666140077899

## 1. Visual Theme & Atmosphere

Altair's design language follows the creative north star of **"The Digital Sanctuary"** -- an interface that recedes rather than competes for attention. The aesthetic philosophy treats each screen not as a toolset but as an atmospheric environment: a high-end editorial space where the user's data is the art and the UI is the gallery wall.

The overall mood is **airy, spacious, and intentionally calm**. Density is low. Whitespace is generous and purposeful. The system favors **intentional asymmetry** and **tonal layering** over structural lines, moving away from "utility software" toward a "personal curator" experience. The goal is to reduce cognitive load through a sense of infinite, quiet depth -- like moving through a softly lit architectural space.

Both light and dark modes exist across all screens. Light mode is the primary expression; dark mode inverts the tonal hierarchy while preserving the same calm, sanctuary-like atmosphere.

## 2. Color Palette & Roles

### Primary Palette

| Descriptive Name      | Hex Code  | Functional Role                                                                          |
| --------------------- | --------- | ---------------------------------------------------------------------------------------- |
| Deep Muted Teal-Navy  | `#446273` | Primary actions, interactive elements, active states. The signature color of the system. |
| Subdued Teal-Navy     | `#385666` | Primary dim variant. Pressed/active states for primary elements.                         |
| Whisper-Soft Ice Blue | `#c7e7fa` | Primary container. Backgrounds for selected/active zones, badges, and soft highlights.   |
| Gentle Dawn Blue      | `#bad9ec` | Primary fixed dim. Subtle emphasis in fixed UI regions.                                  |

### Secondary & Tertiary

| Descriptive Name      | Hex Code  | Functional Role                                                              |
| --------------------- | --------- | ---------------------------------------------------------------------------- |
| Slate Harbor          | `#4f626b` | Secondary actions, supporting interactive elements, sidebar active states.   |
| Morning Frost Blue    | `#d1e6f0` | Secondary container. Tag backgrounds, secondary badges, soft grouping fills. |
| Weathered Stone Green | `#546260` | Tertiary accents, success-adjacent states, nature-inspired highlights.       |
| Misty Sage Wash       | `#e8f7f4` | Tertiary container. Subtle success indicators, completion badges.            |

### Surface Hierarchy (The Tonal Stack)

Surfaces create depth through layered sheets of tone -- like stacked fine-weight paper -- rather than through borders or shadows.

| Descriptive Name    | Hex Code  | Functional Role                                                             |
| ------------------- | --------- | --------------------------------------------------------------------------- |
| Frost-Touched Pearl | `#f8fafa` | Base layer. Background and primary surface color.                           |
| Pale Morning Mist   | `#f0f4f5` | Surface container low. Secondary zones, sidebar backgrounds.                |
| Soft Cloud Grey     | `#e9eff0` | Surface container. Elevated content areas.                                  |
| Cool Pebble Wash    | `#e1eaeb` | Surface container high. Tertiary cards, nested elements.                    |
| Gentle Stone        | `#dae5e6` | Surface container highest. Closest-to-user elements, badges.                |
| Pure White          | `#ffffff` | Surface container lowest. Active/focused cards that "lift" toward the user. |
| Veiled Seafoam      | `#cfddde` | Surface dim. Receded/dimmed elements during focus states.                   |

### Text & Foreground

| Descriptive Name  | Hex Code  | Functional Role                                                                     |
| ----------------- | --------- | ----------------------------------------------------------------------------------- |
| Deep Ink Charcoal | `#2a3435` | Primary text. On-background and on-surface. Never use pure black (#000000).         |
| Muted Slate       | `#566162` | Secondary text. On-surface-variant. Metadata, descriptions, editorial labels.       |
| Worn Graphite     | `#727d7e` | Outline. Subtle structural hints, disabled icon outlines.                           |
| Silver Haze       | `#a9b4b5` | Outline variant. Ghost borders at 15% opacity when accessibility requires a stroke. |

### Semantic & State Colors

| Descriptive Name | Hex Code  | Functional Role                                                  |
| ---------------- | --------- | ---------------------------------------------------------------- |
| Warm Terracotta  | `#9f403d` | Error state. Sophisticated and muted, never an alarming crimson. |
| Soft Coral Blush | `#fe8983` | Error container. Backgrounds for error messages and alerts.      |

### Signature Gradient

For primary CTAs and progress indicators, blend Primary into Primary Container at a 135-degree angle to create a "lit-from-within" soul:

```
linear-gradient(135deg, #446273 0%, #bad9ec 100%)
```

## 3. Typography Rules

The type system pairs two families to balance architectural strength with approachable legibility:

- **Display & Headlines: Manrope** -- Wide apertures convey modern openness. Used for dashboard greetings, section headers, and "Zen mode" titles. `display-lg` at 3.5rem for the single most important element on screen.
- **Body & Labels: Plus Jakarta Sans** -- Clean, highly legible at small sizes. Standardized with generous line height (1.6+) and letter spacing (0.02em) for comfortable reading.

**Hierarchy as Brand:** Use `label-sm` sized text in Deep Ink Charcoal or Muted Slate, set in all-caps with 0.1em letter-spacing, for metadata tags. This "editorial tag" treatment elevates mundane data into curated information.

**The Golden Rule:** A massive `display-lg` metric should be balanced by wide margins of whitespace to emphasize its importance -- never crowd a large heading.

## 4. Component Stylings

- **Buttons:**
  - **Primary:** Deep Muted Teal-Navy (`#446273`) fill with near-white (`#f1f9ff`) text. Generously rounded, pill-shaped edges (xl roundedness, 1.5rem). On press, transitions to Subdued Teal-Navy (`#385666`).
  - **Secondary/Ghost:** No container. Text-only in Slate Harbor or Muted Slate, placed in generous whitespace.
  - **Touch targets:** Minimum 48px height.

- **Cards & Containers:**
  - Gently rounded corners (lg, 0.5rem minimum). No sharp edges anywhere in the system.
  - Background of Pure White (`#ffffff`) when sitting on Frost-Touched Pearl or Pale Morning Mist surfaces.
  - **No dividers.** Content is separated by vertical spacing (2rem) rather than horizontal lines. Content should breathe.
  - Depth is achieved through tonal layering (placing a lighter card on a slightly darker surface), not through box shadows.

- **Inputs & Forms:**
  - No visible border or bottom line at rest. Filled with Pale Morning Mist (`#f0f4f5`).
  - On focus, the background transitions to Pure White (`#ffffff`) with a "Ghost Border" of Primary at 20% opacity.
  - Search bars and command palettes (Cmd+K) follow the same pattern.

- **Navigation Sidebar:**
  - Icon-based vertical sidebar using Material Symbols Outlined (FILL: 0, weight: 400, optical size: 24).
  - Icons include: `calendar_today`, `inbox`, `menu_book`, `track_changes`, `inventory_2`, `visibility`.
  - Active state uses Primary color; inactive uses Muted Slate.
  - Sidebar background is Pale Morning Mist (`#f0f4f5`) to create a borderless tonal distinction from the main content area.

- **Progress & Status Indicators:**
  - "Pulse" style capacity bars: Silver Haze (`#a9b4b5`) track with Deep Muted Teal-Navy (`#446273`) fill. The fill carries a subtle 2px outer glow of its own color.
  - Checkmarks and radio buttons for tracking completion.
  - Trending icons and alignment scores for data-driven status.

- **Scrollbars:**
  - Ultra-thin (4-6px width). Track is transparent. Thumb is Silver Haze (`#a9b4b5`) with fully rounded corners (10px). Unobtrusive.

## 5. Layout Principles

**The "No-Line" Rule:** Borders (1px solid lines) are strictly prohibited for defining sections. All boundaries must be established through tonal shifts (placing a lighter surface against a slightly darker one) or through negative space (using the spacing scale to create distinct zones of focus).

**Whitespace Strategy:** Embrace generous, purposeful whitespace. If a section feels like it needs more content, it probably needs more padding. Wide margins around hero metrics, breathing room between modules.

**Asymmetry as Brand:** Off-center placement of high-level stats and editorial-style layouts create a magazine-like feel rather than a rigid dashboard grid.

**Grid Alignment:** Content lives within a soft, implicit grid rather than visible framework lines. Alignment is inferred through consistent spacing, not drawn structures.

**Responsive Adaptation:** The system supports both desktop (1280px-2560px) and mobile (390px-780px) viewports. Mobile layouts shift to single-column stacks while preserving the same tonal hierarchy, typography scale, and breathing room.

## 6. Depth & Elevation Philosophy

Depth in Altair is a **feeling**, not a feature.

- **Static Cards:** No shadows. Depth is created by placing a Pure White (`#ffffff`) card on a Pale Morning Mist (`#f0f4f5`) background. This "soft lift" feels architectural.
- **Floating Elements:** For active overlays, tooltips, or modals, use an ultra-diffused ambient shadow: `0 20px 40px rgba(42, 52, 53, 0.06)`. The shadow color is tinted from on-surface, never pure black.
- **Ghost Border Fallback:** If accessibility requires a stroke, use Silver Haze (`#a9b4b5`) at 15% opacity. Anything more opaque is visual clutter.
- **Glassmorphism:** Floating elements (modals, popovers) use Frost-Touched Pearl at 80% opacity with `backdrop-blur: 20px`. This keeps underlying content visible, maintaining spatial context.
- **Focus Dimming:** When a user focuses on a specific zone, surrounding elements recede to Veiled Seafoam (`#cfddde`) while the focused container scales slightly (1.02x) and adopts a Pure White background.

## 7. Interaction & Motion

All state changes follow the **"Breathe" Transition** -- a 300ms duration that creates a tactile, fluid sensation. Avoid "snappy" or "instant" changes; time is a luxury afforded to the user.

- **Easing:** `cubic-bezier(0.4, 0, 0.2, 1)` for progressive disclosure and expandable sections.
- **Hover/Focus/Active:** Slow, deliberate transitions. Like moving through water.
- **Progressive Disclosure:** "Soft Chevrons" and hover-triggered surface-variant backgrounds signal expandability without visual noise.

## 8. Strict Rules

### Do:

- Use intentional asymmetry to create an editorial, magazine-like feel
- Embrace generous whitespace -- let content breathe
- Use tone-on-tone (Muted Slate `#566162`) for secondary text to keep contrast soft
- Define boundaries through tonal shifts and spacing, never with lines

### Don't:

- **Never use pure black (#000000).** Always use Deep Ink Charcoal (`#2a3435`).
- **Never use standard 4px corners.** Minimum roundedness is 0.5rem (lg), with preference for 1rem.
- **Never use "Alert Red."** Errors use the sophisticated Warm Terracotta (`#9f403d`), not bright crimson.
- **Never use 1px solid borders** to define sections or separate content.
- **Never use horizontal dividers** (`<hr>`) within cards or lists. Use spacing instead.

## 9. Screen Inventory

The design system is expressed across the following canonical screens, each available in light and dark modes for both desktop and mobile:

| Screen                        | Desktop      | Mobile       | Purpose                                          |
| ----------------------------- | ------------ | ------------ | ------------------------------------------------ |
| Today Sheet                   | Light + Dark | Light + Dark | Daily dashboard, schedule, and task overview     |
| Universal Inbox               | Light + Dark | Light + Dark | Unified notification and action item stream      |
| Knowledge Hub                 | Light + Dark | Light + Dark | Notes, connections, and knowledge graph browsing |
| Inventory Tracker             | Light + Dark | Light + Dark | Physical and digital item management             |
| Focus Mode                    | Light + Dark | Light + Dark | Distraction-free single-task deep work view      |
| Guidance Center               | Light + Dark | Light + Dark | Quest tracking and personal growth pathways      |
| Quick Capture                 | Light        | --           | Rapid note and item entry overlay                |
| Add Physical Item             | Light        | --           | Detailed item intake form                        |
| Cognitive Architecture Detail | Light        | --           | Deep-dive system architecture visualization      |
