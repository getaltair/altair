/// Task prioritization dialog with AI-powered priority suggestions.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../bloc/ai/ai_bloc.dart';
import '../../bloc/ai/ai_event.dart';
import '../../bloc/ai/ai_state.dart';
import '../../services/ai/models.dart';

/// Shows task prioritization dialog.
void showTaskPrioritizationDialog(
  BuildContext context, {
  required List<Task> tasks,
  String? projectContext,
}) {
  showDialog<void>(
    context: context,
    barrierDismissible: false,
    builder: (context) => TaskPrioritizationDialog(
      tasks: tasks,
      projectContext: projectContext,
    ),
  );
}

/// Dialog for displaying AI-powered task prioritization.
class TaskPrioritizationDialog extends StatefulWidget {
  /// Creates a task prioritization dialog.
  const TaskPrioritizationDialog({
    required this.tasks,
    this.projectContext,
    super.key,
  });

  /// Tasks to prioritize.
  final List<Task> tasks;

  /// Optional project context.
  final String? projectContext;

  @override
  State<TaskPrioritizationDialog> createState() =>
      _TaskPrioritizationDialogState();
}

class _TaskPrioritizationDialogState extends State<TaskPrioritizationDialog> {
  @override
  void initState() {
    super.initState();
    // Trigger prioritization request when dialog opens
    final tasksData = widget.tasks
        .map((task) => {
              'title': task.title,
              if (task.description != null) 'description': task.description!,
            })
        .toList();

    context.read<AIBloc>().add(
          AITaskPrioritizationRequested(
            request: TaskPrioritizationRequest(
              tasks: tasksData,
              context: widget.projectContext,
            ),
          ),
        );
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 700, maxHeight: 700),
        child: AltairCard(
          accentColor: Colors.purple,
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
                    const Icon(Icons.priority_high, size: 32),
                    const SizedBox(width: AltairSpacing.md),
                    Expanded(
                      child: Text(
                        'AI Task Prioritization',
                        style: Theme.of(context).textTheme.headlineSmall,
                      ),
                    ),
                    IconButton(
                      icon: const Icon(Icons.close),
                      onPressed: () => Navigator.of(context).pop(),
                    ),
                  ],
                ),
                const SizedBox(height: AltairSpacing.md),
                const Divider(),
                const SizedBox(height: AltairSpacing.md),

                // Content
                Expanded(
                  child: BlocConsumer<AIBloc, AIState>(
                    listener: (context, state) {
                      if (state is AIFailure &&
                          state.operationType ==
                              AIOperationType.prioritization) {
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
                              AIOperationType.prioritization) {
                        return _buildLoadingState();
                      }

                      if (state is AITaskPrioritizationSuccess) {
                        return _buildSuccessState(state.response);
                      }

                      if (state is AIFailure &&
                          state.operationType ==
                              AIOperationType.prioritization) {
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
    return const Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          CircularProgressIndicator(),
          SizedBox(height: AltairSpacing.md),
          Text('AI is analyzing task priorities...'),
        ],
      ),
    );
  }

  Widget _buildSuccessState(TaskPrioritizationResponse response) {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Recommended Order Section
          Text(
            'Recommended Execution Order',
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
          ),
          const SizedBox(height: AltairSpacing.sm),
          ...List.generate(
            response.recommendedOrder.length,
            (index) => Padding(
              padding: const EdgeInsets.only(bottom: AltairSpacing.xs),
              child: Row(
                children: [
                  Container(
                    width: 32,
                    height: 32,
                    decoration: BoxDecoration(
                      color: Colors.purple,
                      borderRadius: BorderRadius.circular(4),
                      border: Border.all(
                        color: Colors.black,
                        width: AltairBorders.medium,
                      ),
                    ),
                    child: Center(
                      child: Text(
                        '${index + 1}',
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          color: Colors.black,
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(width: AltairSpacing.sm),
                  Expanded(
                    child: Text(
                      response.recommendedOrder[index],
                      style: Theme.of(context).textTheme.bodyMedium,
                    ),
                  ),
                ],
              ),
            ),
          ),

          const SizedBox(height: AltairSpacing.lg),
          const Divider(),
          const SizedBox(height: AltairSpacing.lg),

          // Priority Suggestions Section
          Text(
            'Priority Details',
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
          ),
          const SizedBox(height: AltairSpacing.sm),

          ...response.suggestions.map((suggestion) {
            return Padding(
              padding: const EdgeInsets.only(bottom: AltairSpacing.md),
              child: AltairCard(
                accentColor: _getPriorityColor(suggestion.priority),
                showAccentBar: true,
                child: Padding(
                  padding: const EdgeInsets.all(AltairSpacing.md),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          Expanded(
                            child: Text(
                              suggestion.taskTitle,
                              style: Theme.of(context)
                                  .textTheme
                                  .bodyLarge
                                  ?.copyWith(
                                    fontWeight: FontWeight.bold,
                                  ),
                            ),
                          ),
                          _PriorityBadge(priority: suggestion.priority),
                        ],
                      ),
                      const SizedBox(height: AltairSpacing.sm),
                      Text(
                        suggestion.reasoning,
                        style: Theme.of(context).textTheme.bodyMedium,
                      ),
                      const SizedBox(height: AltairSpacing.sm),
                      Row(
                        children: [
                          _ScoreIndicator(
                            label: 'Urgency',
                            score: suggestion.urgencyScore,
                            color: AltairColors.error,
                          ),
                          const SizedBox(width: AltairSpacing.md),
                          _ScoreIndicator(
                            label: 'Impact',
                            score: suggestion.impactScore,
                            color: AltairColors.accentGreen,
                          ),
                        ],
                      ),
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
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(
            Icons.error_outline,
            size: 64,
            color: AltairColors.error,
          ),
          const SizedBox(height: AltairSpacing.md),
          Text(
            'Failed to get prioritization',
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
            onPressed: () {
              // Retry prioritization
              final tasksData = widget.tasks
                  .map((task) => {
                        'title': task.title,
                        if (task.description != null)
                          'description': task.description!,
                      })
                  .toList();

              context.read<AIBloc>().add(
                    AITaskPrioritizationRequested(
                      request: TaskPrioritizationRequest(
                        tasks: tasksData,
                        context: widget.projectContext,
                      ),
                    ),
                  );
            },
            variant: AltairButtonVariant.filled,
            accentColor: AltairColors.accentBlue,
            child: const Text('Retry'),
          ),
        ],
      ),
    );
  }

  Color _getPriorityColor(PriorityLevel priority) {
    return switch (priority) {
      PriorityLevel.critical => AltairColors.error,
      PriorityLevel.high => Colors.orange,
      PriorityLevel.medium => AltairColors.accentYellow,
      PriorityLevel.low => AltairColors.accentGreen,
    };
  }
}

/// Badge showing priority level.
class _PriorityBadge extends StatelessWidget {
  const _PriorityBadge({required this.priority});

  final PriorityLevel priority;

  @override
  Widget build(BuildContext context) {
    final color = _getColor();
    final text = _getText();

    return Container(
      padding: const EdgeInsets.symmetric(
        horizontal: AltairSpacing.sm,
        vertical: AltairSpacing.xs,
      ),
      decoration: BoxDecoration(
        color: color,
        borderRadius: BorderRadius.circular(4),
        border: Border.all(
          color: Colors.black,
          width: AltairBorders.medium,
        ),
      ),
      child: Text(
        text,
        style: const TextStyle(
          fontSize: 12,
          fontWeight: FontWeight.bold,
          color: Colors.black,
        ),
      ),
    );
  }

  Color _getColor() {
    return switch (priority) {
      PriorityLevel.critical => AltairColors.error,
      PriorityLevel.high => Colors.orange,
      PriorityLevel.medium => AltairColors.accentYellow,
      PriorityLevel.low => AltairColors.accentGreen,
    };
  }

  String _getText() {
    return switch (priority) {
      PriorityLevel.critical => 'CRITICAL',
      PriorityLevel.high => 'HIGH',
      PriorityLevel.medium => 'MEDIUM',
      PriorityLevel.low => 'LOW',
    };
  }
}

/// Score indicator with progress bar.
class _ScoreIndicator extends StatelessWidget {
  const _ScoreIndicator({
    required this.label,
    required this.score,
    required this.color,
  });

  final String label;
  final double score;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '$label: ${(score * 100).toStringAsFixed(0)}%',
            style: Theme.of(context).textTheme.bodySmall,
          ),
          const SizedBox(height: AltairSpacing.xs),
          ClipRRect(
            borderRadius: BorderRadius.circular(4),
            child: LinearProgressIndicator(
              value: score,
              backgroundColor: AltairColors.textSecondary.withValues(alpha: 0.2),
              valueColor: AlwaysStoppedAnimation(color),
              minHeight: 8,
            ),
          ),
        ],
      ),
    );
  }
}
