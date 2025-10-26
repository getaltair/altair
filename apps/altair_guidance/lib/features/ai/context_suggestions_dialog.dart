/// Context suggestions dialog with AI-powered recommendations.
library;

import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../bloc/ai/ai_bloc.dart';
import '../../bloc/ai/ai_event.dart';
import '../../bloc/ai/ai_state.dart';
import '../../services/ai/models.dart';

/// Shows context suggestions dialog.
void showContextSuggestionsDialog(
  BuildContext context, {
  required String taskTitle,
  String? taskDescription,
  String? projectContext,
  String suggestionType = 'general',
}) {
  final aiBloc = context.read<AIBloc>();
  showDialog<void>(
    context: context,
    barrierDismissible: true,
    builder: (context) => BlocProvider<AIBloc>.value(
      value: aiBloc,
      child: ContextSuggestionsDialog(
        taskTitle: taskTitle,
        taskDescription: taskDescription,
        projectContext: projectContext,
        suggestionType: suggestionType,
      ),
    ),
  );
}

/// Dialog for displaying AI-powered context suggestions.
class ContextSuggestionsDialog extends StatefulWidget {
  /// Creates a context suggestions dialog.
  const ContextSuggestionsDialog({
    required this.taskTitle,
    this.taskDescription,
    this.projectContext,
    this.suggestionType = 'general',
    super.key,
  });

  /// Task title to get suggestions for.
  final String taskTitle;

  /// Optional task description.
  final String? taskDescription;

  /// Optional project context.
  final String? projectContext;

  /// Type of suggestions (general, resources, tips, blockers).
  final String suggestionType;

  @override
  State<ContextSuggestionsDialog> createState() =>
      _ContextSuggestionsDialogState();
}

class _ContextSuggestionsDialogState extends State<ContextSuggestionsDialog> {
  late String _currentSuggestionType;

  @override
  void initState() {
    super.initState();
    _currentSuggestionType = widget.suggestionType;
    _requestSuggestions(_currentSuggestionType);
  }

  void _requestSuggestions(String suggestionType) {
    try {
      if (widget.taskTitle.trim().isEmpty) {
        return;
      }

      context.read<AIBloc>().add(
            AIContextSuggestionsRequested(
              request: ContextSuggestionRequest(
                taskTitle: widget.taskTitle,
                taskDescription: widget.taskDescription,
                projectContext: widget.projectContext,
                suggestionType: suggestionType,
              ),
            ),
          );
    } catch (e) {
      // Error will be handled by BlocConsumer listener
    }
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 700, maxHeight: 700),
        child: AltairCard(
          accentColor: AltairColors.accentBlue,
          showAccentBar: true,
          child: Padding(
            padding: const EdgeInsets.all(AltairSpacing.lg),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Header
                Row(
                  children: [
                    const Icon(Icons.lightbulb, size: 32),
                    const SizedBox(width: AltairSpacing.md),
                    Expanded(
                      child: Text(
                        'AI Context Suggestions',
                        style: Theme.of(context).textTheme.headlineSmall,
                      ),
                    ),
                    IconButton(
                      icon: const Icon(Icons.close),
                      tooltip: 'Close dialog',
                      onPressed: () => Navigator.of(context).pop(),
                    ),
                  ],
                ),
                const SizedBox(height: AltairSpacing.md),
                const Divider(),
                const SizedBox(height: AltairSpacing.md),

                // Suggestion type selector
                _SuggestionTypeSelector(
                  currentType: _currentSuggestionType,
                  onTypeChanged: (type) {
                    setState(() {
                      _currentSuggestionType = type;
                    });
                    _requestSuggestions(type);
                  },
                ),
                const SizedBox(height: AltairSpacing.md),

                // Content
                Expanded(
                  child: BlocConsumer<AIBloc, AIState>(
                    listener: (context, state) {
                      if (state is AIFailure &&
                          state.operationType ==
                              AIOperationType.contextSuggestions) {
                        ScaffoldMessenger.of(context).showSnackBar(
                          SnackBar(
                            content: Text('Error: ${state.message}'),
                            backgroundColor: AltairColors.error,
                          ),
                        );
                      }
                    },
                    builder: (context, state) {
                      if (state is AILoading &&
                          state.operationType ==
                              AIOperationType.contextSuggestions) {
                        return _buildLoadingState();
                      }

                      if (state is AIContextSuggestionsSuccess) {
                        return _buildSuccessState(state.response);
                      }

                      if (state is AIFailure &&
                          state.operationType ==
                              AIOperationType.contextSuggestions) {
                        return _buildErrorState(state.message);
                      }

                      return _buildLoadingState();
                    },
                  ),
                ),

                // Footer
                const SizedBox(height: AltairSpacing.md),
                const Divider(),
                const SizedBox(height: AltairSpacing.md),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    AltairButton(
                      onPressed: () => Navigator.of(context).pop(),
                      variant: AltairButtonVariant.outlined,
                      child: const Text('Close'),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildLoadingState() {
    return Center(
      child: Semantics(
        label: 'Loading context suggestions from AI',
        child: const Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            CircularProgressIndicator(),
            SizedBox(height: AltairSpacing.md),
            Text('AI is generating suggestions...'),
          ],
        ),
      ),
    );
  }

  Widget _buildSuccessState(ContextSuggestionResponse response) {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Summary (if available)
          if (response.summary != null) ...[
            AltairCard(
              accentColor: AltairColors.accentOrange,
              showAccentBar: true,
              child: Padding(
                padding: const EdgeInsets.all(AltairSpacing.md),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        const Icon(Icons.summarize, size: 20),
                        const SizedBox(width: AltairSpacing.xs),
                        Text(
                          'Summary',
                          style:
                              Theme.of(context).textTheme.titleSmall?.copyWith(
                                    fontWeight: FontWeight.bold,
                                  ),
                        ),
                      ],
                    ),
                    const SizedBox(height: AltairSpacing.sm),
                    Text(
                      response.summary!,
                      style: Theme.of(context).textTheme.bodyMedium,
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: AltairSpacing.lg),
          ],

          // Suggestions
          if (response.suggestions.isEmpty)
            Center(
              child: Padding(
                padding: const EdgeInsets.all(AltairSpacing.xl),
                child: Column(
                  children: [
                    const Icon(
                      Icons.check_circle_outline,
                      size: 64,
                      color: AltairColors.accentGreen,
                    ),
                    const SizedBox(height: AltairSpacing.md),
                    Text(
                      'No suggestions needed',
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                    const SizedBox(height: AltairSpacing.sm),
                    Text(
                      'This task looks good to go!',
                      style: Theme.of(context).textTheme.bodyMedium,
                    ),
                  ],
                ),
              ),
            )
          else
            ...response.suggestions.map((suggestion) {
              return Padding(
                padding: const EdgeInsets.only(bottom: AltairSpacing.md),
                child: AltairCard(
                  accentColor: _getCategoryColor(suggestion.category),
                  showAccentBar: true,
                  child: Padding(
                    padding: const EdgeInsets.all(AltairSpacing.md),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            _CategoryIcon(category: suggestion.category),
                            const SizedBox(width: AltairSpacing.sm),
                            Expanded(
                              child: Text(
                                suggestion.title,
                                style: Theme.of(context)
                                    .textTheme
                                    .bodyLarge
                                    ?.copyWith(
                                      fontWeight: FontWeight.bold,
                                    ),
                              ),
                            ),
                            if (suggestion.priority != null)
                              _PriorityIndicator(
                                priority: suggestion.priority!,
                              ),
                          ],
                        ),
                        const SizedBox(height: AltairSpacing.sm),
                        Text(
                          suggestion.description,
                          style: Theme.of(context).textTheme.bodyMedium,
                        ),
                        const SizedBox(height: AltairSpacing.sm),
                        _CategoryBadge(category: suggestion.category),
                      ],
                    ),
                  ),
                ),
              );
            }),
        ],
      ),
    );
  }

  Widget _buildErrorState(String message) {
    return SingleChildScrollView(
      child: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(
              Icons.error_outline,
              size: 64,
              color: AltairColors.error,
            ),
            const SizedBox(height: AltairSpacing.md),
            Text(
              'Failed to get suggestions',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: AltairSpacing.sm),
            Text(
              message,
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
            const SizedBox(height: AltairSpacing.lg),
            AltairButton(
              onPressed: () => _requestSuggestions(_currentSuggestionType),
              variant: AltairButtonVariant.filled,
              accentColor: AltairColors.accentBlue,
              child: const Text('Retry'),
            ),
          ],
        ),
      ),
    );
  }

  Color _getCategoryColor(String category) {
    return switch (category.toLowerCase()) {
      'resource' => AltairColors.accentBlue,
      'tip' => AltairColors.accentGreen,
      'blocker' => AltairColors.error,
      'warning' => Colors.orange,
      _ => AltairColors.accentOrange,
    };
  }
}

/// Suggestion type selector.
class _SuggestionTypeSelector extends StatelessWidget {
  const _SuggestionTypeSelector({
    required this.currentType,
    required this.onTypeChanged,
  });

  final String currentType;
  final ValueChanged<String> onTypeChanged;

  @override
  Widget build(BuildContext context) {
    final types = ['general', 'resources', 'tips', 'blockers'];

    return Wrap(
      spacing: AltairSpacing.sm,
      children: types.map((type) {
        final isSelected = type == currentType;
        return FilterChip(
          label: Text(
            type.toUpperCase(),
            style: TextStyle(
              color: Colors.black,
              fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
            ),
          ),
          selected: isSelected,
          onSelected: (selected) {
            if (selected) {
              onTypeChanged(type);
            }
          },
          backgroundColor: Colors.white,
          selectedColor: AltairColors.accentOrange,
          side: BorderSide(
            color: Colors.black,
            width: AltairBorders.medium,
          ),
        );
      }).toList(),
    );
  }
}

/// Category icon widget.
class _CategoryIcon extends StatelessWidget {
  const _CategoryIcon({required this.category});

  final String category;

  @override
  Widget build(BuildContext context) {
    final icon = _getIcon();
    return Icon(icon, size: 24);
  }

  IconData _getIcon() {
    return switch (category.toLowerCase()) {
      'resource' => Icons.book,
      'tip' => Icons.tips_and_updates,
      'blocker' => Icons.block,
      'warning' => Icons.warning_amber,
      _ => Icons.info,
    };
  }
}

/// Category badge widget.
class _CategoryBadge extends StatelessWidget {
  const _CategoryBadge({required this.category});

  final String category;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(
        horizontal: AltairSpacing.sm,
        vertical: AltairSpacing.xs,
      ),
      decoration: BoxDecoration(
        color: _getColor(),
        borderRadius: BorderRadius.circular(4),
        border: Border.all(
          color: Colors.black,
          width: AltairBorders.medium,
        ),
      ),
      child: Text(
        category.toUpperCase(),
        style: const TextStyle(
          fontSize: 10,
          fontWeight: FontWeight.bold,
          color: Colors.black,
        ),
      ),
    );
  }

  Color _getColor() {
    return switch (category.toLowerCase()) {
      'resource' => AltairColors.accentBlue,
      'tip' => AltairColors.accentGreen,
      'blocker' => AltairColors.error,
      'warning' => Colors.orange,
      _ => AltairColors.accentOrange,
    };
  }
}

/// Priority indicator widget.
class _PriorityIndicator extends StatelessWidget {
  const _PriorityIndicator({required this.priority});

  final PriorityLevel priority;

  @override
  Widget build(BuildContext context) {
    final color = _getColor();
    return Container(
      width: 12,
      height: 12,
      decoration: BoxDecoration(
        color: color,
        shape: BoxShape.circle,
        border: Border.all(
          color: Colors.black,
          width: AltairBorders.thin,
        ),
      ),
    );
  }

  Color _getColor() {
    return switch (priority) {
      PriorityLevel.critical => AltairColors.error,
      PriorityLevel.high => Colors.orange,
      PriorityLevel.medium => AltairColors.accentOrange,
      PriorityLevel.low => AltairColors.accentGreen,
    };
  }
}
