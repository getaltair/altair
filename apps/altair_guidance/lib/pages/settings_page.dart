/// Settings page for Altair Guidance.
library;

import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../features/theme/theme_cubit.dart';

/// Settings page.
class SettingsPage extends StatelessWidget {
  /// Creates the settings page.
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
      ),
      body: ListView(
        padding: const EdgeInsets.all(AltairSpacing.lg),
        children: [
          // Appearance section
          Text(
            'APPEARANCE',
            style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                  letterSpacing: 1.2,
                ),
          ),
          const SizedBox(height: AltairSpacing.md),

          // Theme mode setting
          BlocBuilder<ThemeCubit, ThemeState>(
            builder: (context, state) {
              return AltairCard(
                accentColor: AltairColors.accentBlue,
                showAccentBar: true,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Theme Mode',
                      style: Theme.of(context).textTheme.titleMedium?.copyWith(
                            fontWeight: FontWeight.bold,
                          ),
                    ),
                    const SizedBox(height: AltairSpacing.sm),
                    Text(
                      'Choose your preferred color scheme',
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                    const SizedBox(height: AltairSpacing.md),

                    // Theme options
                    _ThemeOption(
                      title: 'Light',
                      description: 'Always use light theme',
                      icon: Icons.light_mode,
                      isSelected: state.isLight,
                      onTap: () {
                        context.read<ThemeCubit>().setThemeMode(ThemeMode.light);
                      },
                    ),
                    const SizedBox(height: AltairSpacing.sm),
                    _ThemeOption(
                      title: 'Dark',
                      description: 'Always use dark theme',
                      icon: Icons.dark_mode,
                      isSelected: state.isDark,
                      onTap: () {
                        context.read<ThemeCubit>().setThemeMode(ThemeMode.dark);
                      },
                    ),
                    const SizedBox(height: AltairSpacing.sm),
                    _ThemeOption(
                      title: 'Follow System',
                      description: 'Match your device settings',
                      icon: Icons.brightness_auto,
                      isSelected: state.isSystem,
                      onTap: () {
                        context.read<ThemeCubit>().useSystemTheme();
                      },
                    ),
                  ],
                ),
              );
            },
          ),

          const SizedBox(height: AltairSpacing.xl),

          // About section
          Text(
            'ABOUT',
            style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                  letterSpacing: 1.2,
                ),
          ),
          const SizedBox(height: AltairSpacing.md),
          AltairCard(
            accentColor: AltairColors.accentOrange,
            showAccentBar: true,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'Altair Guidance',
                  style: Theme.of(context).textTheme.titleLarge?.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                ),
                const SizedBox(height: AltairSpacing.xs),
                Text(
                  'ADHD-friendly task management',
                  style: Theme.of(context).textTheme.bodyMedium,
                ),
                const SizedBox(height: AltairSpacing.sm),
                Text(
                  'Version 0.1.0',
                  style: Theme.of(context).textTheme.bodySmall,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

/// Widget for a theme option radio button.
///
/// Displays a selectable theme option with an icon, title, and description.
/// Shows visual feedback for the selected state with accent colors and a checkmark.
class _ThemeOption extends StatelessWidget {
  /// Creates a theme option widget.
  const _ThemeOption({
    required this.title,
    required this.description,
    required this.icon,
    required this.isSelected,
    required this.onTap,
  });

  /// The title of the theme option (e.g., "Light", "Dark", "Follow System").
  final String title;

  /// A brief description of what this theme option does.
  final String description;

  /// The icon to display for this theme option.
  final IconData icon;

  /// Whether this theme option is currently selected.
  final bool isSelected;

  /// Callback invoked when the user taps this theme option.
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    final backgroundColor = isSelected
        ? (isDark ? AltairColors.darkBgSecondary : AltairColors.lightBgSecondary)
        : Colors.transparent;

    final borderColor = isSelected
        ? AltairColors.accentBlue
        : (isDark ? AltairColors.darkBorderColor : AltairColors.lightBorderColor);

    return InkWell(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.all(AltairSpacing.md),
        decoration: BoxDecoration(
          color: backgroundColor,
          border: Border.all(
            color: borderColor,
            width: isSelected ? 2.0 : 1.0,
          ),
        ),
        child: Row(
          children: [
            Icon(
              icon,
              color: isSelected ? AltairColors.accentBlue : null,
            ),
            const SizedBox(width: AltairSpacing.md),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: theme.textTheme.bodyLarge?.copyWith(
                      fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                    ),
                  ),
                  Text(
                    description,
                    style: theme.textTheme.bodySmall,
                  ),
                ],
              ),
            ),
            if (isSelected)
              const Icon(
                Icons.check_circle,
                color: AltairColors.accentBlue,
              ),
          ],
        ),
      ),
    );
  }
}
