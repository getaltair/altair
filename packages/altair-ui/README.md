# altair_ui

Neo-brutalist UI components and theme for the Altair ecosystem.

## Features

- Complete Material 3 theme for light and dark modes
- Neo-brutalist design system with thick borders and high contrast
- Design tokens for consistent spacing, colors, typography, and borders
- Common UI widgets (buttons, cards, text fields, quick capture)
- Quick capture widget optimized for < 3 second task creation
- JetBrains Mono font family
- ADHD-friendly visual design

## Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  altair_ui:
    path: ../altair-ui
```

## Usage

### Apply Theme

```dart
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(
    MaterialApp(
      title: 'Altair',
      theme: AltairTheme.lightTheme,
      darkTheme: AltairTheme.darkTheme,
      themeMode: ThemeMode.system,
      home: MyHomePage(),
    ),
  );
}
```

### Using Widgets

#### Button

```dart
import 'package:altair_ui/altair_ui.dart';

AltairButton(
  onPressed: () => print('Pressed!'),
  variant: AltairButtonVariant.filled,
  accentColor: AltairColors.accentYellow,
  child: Text('Click Me'),
)
```

#### Card

```dart
import 'package:altair_ui/altair_ui.dart';

AltairCard(
  accentColor: AltairColors.accentBlue,
  showAccentBar: true,
  child: Text('Card content'),
)
```

#### Text Field

```dart
import 'package:altair_ui/altair_ui.dart';

AltairTextField(
  label: 'Email',
  hint: 'Enter your email',
  onChanged: (value) => print(value),
)
```

#### Quick Capture

```dart
import 'package:altair_ui/altair_ui.dart';

AltairQuickCapture(
  onCapture: (text) {
    // Handle captured text (e.g., save task)
    print('Captured: $text');
  },
  hint: 'What needs to be done?',
  accentColor: AltairColors.accentYellow,
  autofocus: true,
)
```

### Using Design Tokens

```dart
import 'package:altair_ui/altair_ui.dart';

// Colors
Container(color: AltairColors.accentYellow)

// Spacing
SizedBox(height: AltairSpacing.md)

// Typography
Text('Hello', style: AltairTypography.headlineLarge)

// Borders
Container(
  decoration: BoxDecoration(
    border: Border.all(
      width: AltairBorders.standard,
      color: AltairColors.lightBorderColor,
    ),
  ),
)
```

## Design System

### Colors

#### Light Theme

- Primary Background: `#fafafa`
- Secondary Background: `#ffffff`
- Primary Text: `#000000`
- Secondary Text: `#666666`
- Border: `#000000`

#### Dark Theme

- Primary Background: `#1a1a1a`
- Secondary Background: `#2a2a2a`
- Primary Text: `#ffffff`
- Secondary Text: `#999999`
- Border: `#ffffff`

#### Accents

- Yellow: `#ffd93d`
- Blue: `#60a5fa`
- Green: `#6bcb77`
- Red: `#ff6b6b`

### Typography

Uses **JetBrains Mono** font family with weights:

- 400 (Regular)
- 500 (Medium)
- 600 (Semi-bold)
- 700 (Bold)
- 800 (Extra-bold)

### Spacing Scale

- XS: 4px
- SM: 8px
- MD: 16px
- LG: 24px
- XL: 32px
- XXL: 48px

### Border Widths

- Thin: 2px
- Standard: 3px
- Thick: 4px
- Extra Thick: 6px

## Design Principles

### Neo-Brutalism

- **Thick borders**: All components use bold, visible borders
- **High contrast**: Clear distinction between elements
- **No shadows**: Flat design with no elevation effects
- **Geometric shapes**: Simple, rectangular forms
- **Bright accents**: Bold colors for interactive elements
- **Monospace font**: JetBrains Mono for clarity

### ADHD-Friendly

- **Visual clarity**: High contrast and clear boundaries
- **Consistent spacing**: Predictable layout patterns
- **Bold typography**: Easy-to-read text
- **Color coding**: Semantic colors for different states
- **Simple navigation**: Clear visual hierarchy

## License

AGPL-3.0-or-later
