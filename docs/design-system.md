# Altair Design System: "Calm Focus"

---

## 1. Philosophy & Principles

The "Calm Focus" design system is built to support neurodiverse minds,
specifically targeting the challenges of ADHD. Traditional productivity
tools often rely on high executive function (planning, prioritizing,
organizing) to operate effectively. Altair reverses this.

Our goal is to **externalize executive function**, **reduce visual and
cognitive noise**, and provide **clear, frictionless paths to action**.
The interface acts as a supportive partner, not a demanding taskmaster.

### Core UX Principles

1. **Single Point of Focus (WIP=1):** The system aggressively filters
   out distractions. At any given moment, the interface should highlight
   only the single most important task, hiding the overwhelming "mountain"
   of future work.
2. **Minimize Cognitive Load:** Information is presented using progressive
   disclosure. Show only what is immediately necessary. Secondary details
   are tucked away until explicitly requested.
3. **Frictionless Capture:** The barrier to entering information must be
   near zero. Getting an idea out of the head and into the system should
   be instantaneous, requiring no decision-making (like categorizing or
   prioritizing) at the moment of capture.
4. **Tangible Time & Energy:** Abstract concepts like time blindness and
   varying energy levels must be made concrete through visual design, such
   as tangible progress bars and energy-based filtering, rather than
   relying solely on numerical dates or times.
5. **Forgiving Aesthetics:** The visual language is soft, airy, and
   low-contrast to reduce sensory overwhelm and anxiety.

---

## 2. Foundation (Design Tokens)

These tokens define the primitive values that construct the "Calm Focus"
aesthetic.

### 2.1 Color Palette

The palette minimizes harsh white backgrounds and pure black text to
reduce eye strain. Semantic colors are soft but clear.

| Semantic Role            | Value (Ref) | Usage Guidelines                 |
| :----------------------- | :---------- | :------------------------------- |
| **Background Primary**   | `#F2F4F7`   | Main canvas background           |
| **Background Secondary** | `#FFFFFF`   | Cards, modals, elevated surfaces |
| **Text Primary**         | `#101828`   | Headings and body content        |
| **Text Secondary**       | `#667085`   | Labels and meta-information      |
| **Action Primary**       | `#007AFF`   | Primary call-to-action buttons   |
| **Focus Accent**         | `#D1E9FF`   | Active task highlighting         |
| **Status Success**       | `#34C759`   | Task completion                  |
| **Status Warning**       | `#FF9F0A`   | Blocked tasks, low energy        |
| **Overlay Dim**          | `40% Black` | Modal background dimming         |

### 2.2 Typography

Select a clean, highly legible sans-serif typeface with open counters and
distinct letterforms (e.g., Inter, Roboto, or Atkinson Hyperlegible).
Font sizes should be generous to improve readability.

- **Hierarchy:**
  - **Heading 1:** Page titles and major section headers.
  - **Heading 2:** Card titles and subsection headers.
  - **Body:** Default text style for content. Line height at least 1.5x.
  - **Label:** Supporting text for form fields and metadata.

### 2.3 Spatial System

- **Grid:** Use a 4px baseline grid for spacing and alignment (e.g., 8px,
  16px, 24px, 32px margins/padding).
- **Radiuses:** Use soft, rounded corners consistently.
  - **Standard Radius (12px):** For cards, buttons, and inputs.
  - **Large Radius (16px+):** For modal surfaces or focus cards.
- **Elevation (Shadows):** Use soft, diffused, large-blur shadows to
  create depth without harsh outlines. Avoid sharp, hard shadows.

---

## 3. Core Components

### 3.1 Cards

Cards are the fundamental container for discrete pieces of information
(a task, a note, an inventory item).

- **Appearance:** White background, rounded corners, soft shadow.
- **Behavior:** Cards are generally actionable (clickable) to reveal
  details or perform primary actions.

### 3.2 Buttons

- **Primary Action:** Large, inviting buttons filled with the Action
  Primary color. Used for the main goal on a screen (e.g., "New Note",
  "Generate List").
- **Success Action:** Used specifically for positive closure, like marking
  a task complete.
- **Buttons should always have rounded corners.**

### 3.3 Modals & Overlays

Modals are used to focus user attention on a single, short task (like
searching or capturing ideas) without losing context.

- **Appearance:** A floating white card centered on the screen.
- **Behavior:** A modal must dim the background content. Modals should
  generally be dismissible by clicking the backdrop or pressing Escape.

---

## 4. UX Patterns (The ADHD Optimizer Layer)

These patterns define the unique behaviors of the Altair system designed
to support executive function.

### 4.1 Focus Mode (Default View)

The default state of the application must not be an overwhelming overview.

- **Principle:** Work In Progress (WIP) = 1.
- **Behavior:** The main dashboard hides full lists and boards. It presents
  only the single, currently active task as a large, central focus card.
  Future tasks are hidden in collapsed "drawers" or tabs, visible only on
  demand through progressive disclosure.

### 4.2 Frictionless Quick Capture

A global mechanism to offload working memory instantly.

- **Principle:** Capture first, organize later.
- **Behavior:** A global trigger (hotkey or prominent button) opens a
  capture modal immediately. The text input _must_ autofocus instantly.
  Submitting the input (e.g., pressing Enter) should save the item to a
  general "inbox" or backlog and immediately close the modal. No
  categorization should be required at the moment of capture.

### 4.3 Tangible Time & Energy

Making abstract concepts concrete to combat time blindness and energy
regulation issues.

- **Visual Timers:** Active tasks should feature prominent visual timers
  (progress bars or countdown circles) rather than just numerical clocks,
  making the passage of time tangible during work sessions.
- **Energy Filtering:** Planning views should include a prominent toggle to
  filter tasks based on the user's currently reported energy level. When
  active, tasks requiring high energy should be hidden from view to
  prevent decision fatigue and unrealistic planning.

### 4.4 Progressive Disclosure

Reduce overwhelm by hiding complexity.

- **Behavior:** Secondary information (backlogs, completed items, advanced
  settings) should be collapsed or hidden by default. Use clear indicators
  (like "Show More" buttons or expanding accordions) to allow the user to
  pull that information only when they are ready to process it.
