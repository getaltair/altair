/// Settings page for Altair Guidance.
library;

import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../bloc/settings/settings_bloc.dart';
import '../bloc/settings/settings_event.dart';
import '../bloc/settings/settings_state.dart';
import '../features/theme/theme_cubit.dart';
import '../models/ai_settings.dart';

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
                        context
                            .read<ThemeCubit>()
                            .setThemeMode(ThemeMode.light);
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

          // AI Features section
          Text(
            'AI FEATURES',
            style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                  letterSpacing: 1.2,
                ),
          ),
          const SizedBox(height: AltairSpacing.md),
          BlocBuilder<SettingsBloc, SettingsState>(
            builder: (context, state) {
              if (state is SettingsLoaded || state is SettingsSaved || state is SettingsSaving) {
                final aiSettings = state is SettingsLoaded
                    ? state.aiSettings
                    : state is SettingsSaved
                        ? state.aiSettings
                        : (state as SettingsSaving).aiSettings;

                return _AISettingsCard(settings: aiSettings);
              }

              return const Center(child: CircularProgressIndicator());
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
        ? (isDark
            ? AltairColors.darkBgSecondary
            : AltairColors.lightBgSecondary)
        : Colors.transparent;

    final borderColor = isSelected
        ? AltairColors.accentBlue
        : (isDark
            ? AltairColors.darkBorderColor
            : AltairColors.lightBorderColor);

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
                      fontWeight:
                          isSelected ? FontWeight.bold : FontWeight.normal,
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

/// AI settings card widget.
class _AISettingsCard extends StatefulWidget {
  const _AISettingsCard({required this.settings});

  final AISettings settings;

  @override
  State<_AISettingsCard> createState() => _AISettingsCardState();
}

class _AISettingsCardState extends State<_AISettingsCard> {
  late AISettings _workingSettings;
  late TextEditingController _openaiKeyController;
  late TextEditingController _anthropicKeyController;
  late TextEditingController _ollamaUrlController;

  @override
  void initState() {
    super.initState();
    _workingSettings = widget.settings;
    _openaiKeyController = TextEditingController(text: widget.settings.openaiApiKey);
    _anthropicKeyController = TextEditingController(text: widget.settings.anthropicApiKey);
    _ollamaUrlController = TextEditingController(
      text: widget.settings.ollamaBaseUrl ?? AIProvider.ollama.defaultBaseUrl,
    );
  }

  @override
  void didUpdateWidget(_AISettingsCard oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.settings != widget.settings) {
      _workingSettings = widget.settings;
      _openaiKeyController.text = widget.settings.openaiApiKey ?? '';
      _anthropicKeyController.text = widget.settings.anthropicApiKey ?? '';
      _ollamaUrlController.text =
          widget.settings.ollamaBaseUrl ?? AIProvider.ollama.defaultBaseUrl;
    }
  }

  @override
  void dispose() {
    _openaiKeyController.dispose();
    _anthropicKeyController.dispose();
    _ollamaUrlController.dispose();
    super.dispose();
  }

  void _updateSettings(AISettings settings) {
    setState(() => _workingSettings = settings);
    context.read<SettingsBloc>().add(SettingsAIUpdated(settings));
  }

  @override
  Widget build(BuildContext context) {
    return AltairCard(
      accentColor: AltairColors.accentOrange,
      showAccentBar: true,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Enable/Disable Toggle
          Row(
            children: [
              const Icon(Icons.auto_awesome, size: 24),
              const SizedBox(width: AltairSpacing.sm),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Enable AI Features',
                      style: Theme.of(context).textTheme.titleMedium?.copyWith(
                            fontWeight: FontWeight.bold,
                          ),
                    ),
                    const SizedBox(height: AltairSpacing.xs),
                    Text(
                      'Task breakdown, time estimates, and suggestions',
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                  ],
                ),
              ),
              Switch(
                value: _workingSettings.enabled,
                onChanged: (value) {
                  _updateSettings(_workingSettings.copyWith(enabled: value));
                },
                activeTrackColor: AltairColors.accentOrange,
                activeThumbColor: Colors.white,
              ),
            ],
          ),

          if (_workingSettings.enabled) ...[
            const SizedBox(height: AltairSpacing.lg),
            const Divider(thickness: 2),
            const SizedBox(height: AltairSpacing.lg),

            // Provider Selection
            Text(
              'AI Provider',
              style: Theme.of(context).textTheme.titleSmall?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: AltairSpacing.sm),

            ...AIProvider.values.map((provider) {
              final isSelected = _workingSettings.provider == provider;
              return Padding(
                padding: const EdgeInsets.only(bottom: AltairSpacing.sm),
                child: _ProviderOption(
                  provider: provider,
                  isSelected: isSelected,
                  onTap: () {
                    _updateSettings(_workingSettings.copyWith(provider: provider));
                  },
                ),
              );
            }),

            const SizedBox(height: AltairSpacing.md),

            // Provider-specific Configuration
            if (_workingSettings.provider == AIProvider.openai)
              _OpenAIConfigSection(
                settings: _workingSettings,
                apiKeyController: _openaiKeyController,
                onSettingsChanged: _updateSettings,
              ),
            if (_workingSettings.provider == AIProvider.anthropic)
              _AnthropicConfigSection(
                settings: _workingSettings,
                apiKeyController: _anthropicKeyController,
                onSettingsChanged: _updateSettings,
              ),
            if (_workingSettings.provider == AIProvider.ollama)
              _OllamaConfigSection(
                settings: _workingSettings,
                urlController: _ollamaUrlController,
                onSettingsChanged: _updateSettings,
              ),

            // Validation Warning
            if (!_workingSettings.isValid) ...[
              const SizedBox(height: AltairSpacing.md),
              Container(
                padding: const EdgeInsets.all(AltairSpacing.sm),
                decoration: BoxDecoration(
                  color: AltairColors.error.withValues(alpha: 0.1),
                  border: Border.all(color: AltairColors.error, width: 2),
                ),
                child: Row(
                  children: [
                    const Icon(Icons.warning, color: AltairColors.error, size: 20),
                    const SizedBox(width: AltairSpacing.sm),
                    Expanded(
                      child: Text(
                        'Please provide an API key to use this provider',
                        style: Theme.of(context).textTheme.bodySmall?.copyWith(
                              color: AltairColors.error,
                              fontWeight: FontWeight.bold,
                            ),
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ],
        ],
      ),
    );
  }
}

/// Provider selection option.
class _ProviderOption extends StatelessWidget {
  const _ProviderOption({
    required this.provider,
    required this.isSelected,
    required this.onTap,
  });

  final AIProvider provider;
  final bool isSelected;
  final VoidCallback onTap;

  String _getProviderDescription(AIProvider provider) {
    switch (provider) {
      case AIProvider.openai:
        return 'GPT models from OpenAI (requires API key)';
      case AIProvider.anthropic:
        return 'Claude models from Anthropic (requires API key)';
      case AIProvider.ollama:
        return 'Run AI models locally (free, no API key needed)';
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    final backgroundColor = isSelected
        ? (isDark ? AltairColors.darkBgSecondary : AltairColors.lightBgSecondary)
        : Colors.transparent;

    final borderColor = isSelected
        ? AltairColors.accentOrange
        : (isDark ? AltairColors.darkBorderColor : AltairColors.lightBorderColor);

    return InkWell(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.all(AltairSpacing.sm),
        decoration: BoxDecoration(
          color: backgroundColor,
          border: Border.all(
            color: borderColor,
            width: isSelected ? 2.0 : 1.0,
          ),
        ),
        child: Row(
          children: [
            Container(
              width: 20,
              height: 20,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                border: Border.all(
                  color: isSelected ? AltairColors.accentOrange : (isDark ? Colors.white70 : Colors.black45),
                  width: 2,
                ),
              ),
              child: isSelected
                  ? Center(
                      child: Container(
                        width: 10,
                        height: 10,
                        decoration: const BoxDecoration(
                          shape: BoxShape.circle,
                          color: AltairColors.accentOrange,
                        ),
                      ),
                    )
                  : null,
            ),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    provider.displayName,
                    style: theme.textTheme.bodyMedium?.copyWith(
                      fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                    ),
                  ),
                  Text(
                    _getProviderDescription(provider),
                    style: theme.textTheme.bodySmall,
                  ),
                ],
              ),
            ),
            if (isSelected)
              const Icon(Icons.check_circle, color: AltairColors.accentOrange, size: 20),
          ],
        ),
      ),
    );
  }
}

/// OpenAI configuration section.
class _OpenAIConfigSection extends StatelessWidget {
  const _OpenAIConfigSection({
    required this.settings,
    required this.apiKeyController,
    required this.onSettingsChanged,
  });

  final AISettings settings;
  final TextEditingController apiKeyController;
  final ValueChanged<AISettings> onSettingsChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'OpenAI Configuration',
          style: Theme.of(context).textTheme.titleSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),
        AltairTextField(
          controller: apiKeyController,
          label: 'API Key',
          hint: 'sk-...',
          obscureText: true,
          onChanged: (value) {
            onSettingsChanged(settings.copyWith(openaiApiKey: value));
          },
        ),
        const SizedBox(height: AltairSpacing.sm),
        Text(
          'Model',
          style: Theme.of(context).textTheme.bodySmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.xs),
        DropdownButtonFormField<String>(
          initialValue: settings.openaiModel,
          decoration: const InputDecoration(
            border: OutlineInputBorder(),
            contentPadding: EdgeInsets.all(AltairSpacing.sm),
            isDense: true,
          ),
          items: const [
            DropdownMenuItem(value: 'gpt-4-turbo-preview', child: Text('GPT-4 Turbo')),
            DropdownMenuItem(value: 'gpt-4', child: Text('GPT-4')),
            DropdownMenuItem(value: 'gpt-3.5-turbo', child: Text('GPT-3.5 Turbo')),
          ],
          onChanged: (value) {
            if (value != null) {
              onSettingsChanged(settings.copyWith(openaiModel: value));
            }
          },
        ),
        const SizedBox(height: AltairSpacing.xs),
        Text(
          'Get your API key from platform.openai.com',
          style: Theme.of(context).textTheme.bodySmall?.copyWith(
                fontStyle: FontStyle.italic,
              ),
        ),
      ],
    );
  }
}

/// Anthropic configuration section.
class _AnthropicConfigSection extends StatelessWidget {
  const _AnthropicConfigSection({
    required this.settings,
    required this.apiKeyController,
    required this.onSettingsChanged,
  });

  final AISettings settings;
  final TextEditingController apiKeyController;
  final ValueChanged<AISettings> onSettingsChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Anthropic Configuration',
          style: Theme.of(context).textTheme.titleSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),
        AltairTextField(
          controller: apiKeyController,
          label: 'API Key',
          hint: 'sk-ant-...',
          obscureText: true,
          onChanged: (value) {
            onSettingsChanged(settings.copyWith(anthropicApiKey: value));
          },
        ),
        const SizedBox(height: AltairSpacing.sm),
        Text(
          'Model',
          style: Theme.of(context).textTheme.bodySmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.xs),
        DropdownButtonFormField<String>(
          initialValue: settings.anthropicModel,
          decoration: const InputDecoration(
            border: OutlineInputBorder(),
            contentPadding: EdgeInsets.all(AltairSpacing.sm),
            isDense: true,
          ),
          items: const [
            DropdownMenuItem(
              value: 'claude-3-5-sonnet-20241022',
              child: Text('Claude 3.5 Sonnet'),
            ),
            DropdownMenuItem(value: 'claude-3-opus-20240229', child: Text('Claude 3 Opus')),
            DropdownMenuItem(value: 'claude-3-sonnet-20240229', child: Text('Claude 3 Sonnet')),
          ],
          onChanged: (value) {
            if (value != null) {
              onSettingsChanged(settings.copyWith(anthropicModel: value));
            }
          },
        ),
        const SizedBox(height: AltairSpacing.xs),
        Text(
          'Get your API key from console.anthropic.com',
          style: Theme.of(context).textTheme.bodySmall?.copyWith(
                fontStyle: FontStyle.italic,
              ),
        ),
      ],
    );
  }
}

/// Ollama configuration section.
class _OllamaConfigSection extends StatelessWidget {
  const _OllamaConfigSection({
    required this.settings,
    required this.urlController,
    required this.onSettingsChanged,
  });

  final AISettings settings;
  final TextEditingController urlController;
  final ValueChanged<AISettings> onSettingsChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Ollama Configuration',
          style: Theme.of(context).textTheme.titleSmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),
        AltairTextField(
          controller: urlController,
          label: 'Ollama Server URL',
          hint: 'http://localhost:11434',
          onChanged: (value) {
            onSettingsChanged(settings.copyWith(ollamaBaseUrl: value));
          },
        ),
        const SizedBox(height: AltairSpacing.sm),
        Text(
          'Model',
          style: Theme.of(context).textTheme.bodySmall?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.xs),
        DropdownButtonFormField<String>(
          initialValue: settings.ollamaModel,
          decoration: const InputDecoration(
            border: OutlineInputBorder(),
            contentPadding: EdgeInsets.all(AltairSpacing.sm),
            isDense: true,
          ),
          items: const [
            DropdownMenuItem(value: 'llama3', child: Text('Llama 3')),
            DropdownMenuItem(value: 'llama3:70b', child: Text('Llama 3 70B')),
            DropdownMenuItem(value: 'mistral', child: Text('Mistral')),
            DropdownMenuItem(value: 'codellama', child: Text('Code Llama')),
          ],
          onChanged: (value) {
            if (value != null) {
              onSettingsChanged(settings.copyWith(ollamaModel: value));
            }
          },
        ),
        const SizedBox(height: AltairSpacing.xs),
        Container(
          padding: const EdgeInsets.all(AltairSpacing.sm),
          decoration: BoxDecoration(
            color: AltairColors.accentBlue.withValues(alpha: 0.1),
            border: Border.all(color: AltairColors.accentBlue, width: 2),
          ),
          child: Row(
            children: [
              const Icon(Icons.info, color: AltairColors.accentBlue, size: 16),
              const SizedBox(width: AltairSpacing.xs),
              Expanded(
                child: Text(
                  'Install Ollama from ollama.com to run AI models locally. Your data never leaves your computer!',
                  style: Theme.of(context).textTheme.bodySmall,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
