/// Task breakdown dialog with AI-powered subtask suggestions.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../bloc/ai/ai_bloc.dart';
import '../../bloc/ai/ai_event.dart';
import '../../bloc/ai/ai_state.dart';
import '../../bloc/task/task_bloc.dart';
import '../../bloc/task/task_event.dart';
import '../../services/ai/models.dart';

/// Shows task breakdown dialog.
void showTaskBreakdownDialog(
  BuildContext context, {
  required String taskTitle,
  String? taskDescription,
  String? projectContext,
  String? parentTaskId,
  int maxSubtasks = 5,
}) {
  showDialog<void>(
    context: context,
    barrierDismissible: false,
    builder: (context) => TaskBreakdownDialog(
      taskTitle: taskTitle,
      taskDescription: taskDescription,
      projectContext: projectContext,
      parentTaskId: parentTaskId,
      maxSubtasks: maxSubtasks,
    ),
  );
}

/// Dialog for displaying AI-powered task breakdown.
class TaskBreakdownDialog extends StatefulWidget {
  /// Creates a task breakdown dialog.
  const TaskBreakdownDialog({
    required this.taskTitle,
    this.taskDescription,
    this.projectContext,
    this.parentTaskId,
    this.maxSubtasks = 5,
    super.key,
  });

  /// Task title to break down.
  final String taskTitle;

  /// Optional task description.
  final String? taskDescription;

  /// Optional project context.
  final String? projectContext;

  /// Optional parent task ID for subtasks.
  final String? parentTaskId;

  /// Maximum number of subtasks.
  final int maxSubtasks;

  @override
  State<TaskBreakdownDialog> createState() => _TaskBreakdownDialogState();
}

class _TaskBreakdownDialogState extends State<TaskBreakdownDialog> {
  @override
  void initState() {
    super.initState();
    // Trigger breakdown request when dialog opens
    context.read<AIBloc>().add(
          AITaskBreakdownRequested(
            request: TaskBreakdownRequest(
              taskTitle: widget.taskTitle,
              taskDescription: widget.taskDescription,
              context: widget.projectContext,
              maxSubtasks: widget.maxSubtasks,
            ),
          ),
        );
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 600, maxHeight: 700),
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
                    const Icon(Icons.auto_awesome, size: 32),
                    const SizedBox(width: AltairSpacing.md),
                    Expanded(
                      child: Text(
                        'AI Task Breakdown',
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
                          state.operationType == AIOperationType.breakdown) {
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
                          state.operationType == AIOperationType.breakdown) {
                        return _buildLoadingState();
                      }

                      if (state is AITaskBreakdownSuccess) {
                        return _buildSuccessState(state.response);
                      }

                      if (state is AIFailure &&
                          state.operationType == AIOperationType.breakdown) {
                        return _buildErrorState(state.message);
                      }

                      return _buildLoadingState();
                    },
                  ),
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
          Text('AI is analyzing your task...'),
          SizedBox(height: AltairSpacing.sm),
          Text(
            'This may take a few seconds',
            style: TextStyle(color: AltairColors.textSecondary),
          ),
        ],
      ),
    );
  }

  Widget _buildSuccessState(TaskBreakdownResponse response) {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Original task
          Text(
            'Breaking down: "${response.originalTask}"',
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
          ),
          const SizedBox(height: AltairSpacing.md),

          // Estimated time (if available)
          if (response.totalEstimatedMinutes != null) ...[
            Container(
              padding: const EdgeInsets.all(AltairSpacing.sm),
              decoration: BoxDecoration(
                color: AltairColors.accentOrange.withValues(alpha: 0.1),
                border: Border.all(
                  color: AltairColors.accentOrange,
                  width: AltairBorders.medium,
                ),
              ),
              child: Row(
                children: [
                  const Icon(Icons.schedule, size: 20),
                  const SizedBox(width: AltairSpacing.sm),
                  Text(
                    'Total estimated time: ${response.totalEstimatedMinutes} minutes',
                    style: const TextStyle(fontWeight: FontWeight.bold),
                  ),
                ],
              ),
            ),
            const SizedBox(height: AltairSpacing.md),
          ],

          // Reasoning (if available)
          if (response.reasoning != null) ...[
            Text(
              'AI Reasoning:',
              style: Theme.of(context).textTheme.titleSmall?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: AltairSpacing.sm),
            Text(
              response.reasoning!,
              style: const TextStyle(
                fontStyle: FontStyle.italic,
                color: AltairColors.textSecondary,
              ),
            ),
            const SizedBox(height: AltairSpacing.md),
          ],

          // Subtasks
          Text(
            'Suggested Subtasks:',
            style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
          ),
          const SizedBox(height: AltairSpacing.sm),

          ...response.subtasks.map((subtask) => _buildSubtaskItem(subtask)),

          const SizedBox(height: AltairSpacing.md),

          // Action buttons
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              AltairButton(
                onPressed: () => Navigator.of(context).pop(),
                variant: AltairButtonVariant.outlined,
                accentColor: AltairColors.textSecondary,
                child: const Text('Close'),
              ),
              const SizedBox(width: AltairSpacing.sm),
              AltairButton(
                onPressed: () => _createSubtasks(context, response.subtasks),
                variant: AltairButtonVariant.filled,
                accentColor: AltairColors.accentGreen,
                child: const Text('Create Subtasks'),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildSubtaskItem(SubtaskSuggestion subtask) {
    return Padding(
      padding: const EdgeInsets.only(bottom: AltairSpacing.sm),
      child: Container(
        padding: const EdgeInsets.all(AltairSpacing.md),
        decoration: BoxDecoration(
          border: Border.all(
            color: AltairColors.border,
            width: AltairBorders.medium,
          ),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  width: 24,
                  height: 24,
                  alignment: Alignment.center,
                  decoration: BoxDecoration(
                    color: AltairColors.accentBlue,
                    border: Border.all(
                      color: Colors.black,
                      width: AltairBorders.medium,
                    ),
                  ),
                  child: Text(
                    '${subtask.order}',
                    style: const TextStyle(
                      fontWeight: FontWeight.bold,
                      color: Colors.black,
                    ),
                  ),
                ),
                const SizedBox(width: AltairSpacing.sm),
                Expanded(
                  child: Text(
                    subtask.title,
                    style: const TextStyle(fontWeight: FontWeight.bold),
                  ),
                ),
                if (subtask.estimatedMinutes != null) ...[
                  const Icon(Icons.schedule, size: 16),
                  const SizedBox(width: 4),
                  Text(
                    '${subtask.estimatedMinutes}m',
                    style: const TextStyle(
                      color: AltairColors.textSecondary,
                      fontSize: 12,
                    ),
                  ),
                ],
              ],
            ),
            if (subtask.description != null) ...[
              const SizedBox(height: AltairSpacing.sm),
              Text(
                subtask.description!,
                style: const TextStyle(
                  color: AltairColors.textSecondary,
                  fontSize: 14,
                ),
              ),
            ],
          ],
        ),
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
            'Failed to break down task',
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  color: AltairColors.error,
                ),
          ),
          const SizedBox(height: AltairSpacing.sm),
          Text(
            message,
            textAlign: TextAlign.center,
            style: const TextStyle(color: AltairColors.textSecondary),
          ),
          const SizedBox(height: AltairSpacing.lg),
          AltairButton(
            onPressed: () {
              context.read<AIBloc>().add(
                    AITaskBreakdownRequested(
                      request: TaskBreakdownRequest(
                        taskTitle: widget.taskTitle,
                        taskDescription: widget.taskDescription,
                        context: widget.projectContext,
                        maxSubtasks: widget.maxSubtasks,
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

  /// Creates subtasks from AI suggestions.
  void _createSubtasks(
    BuildContext context,
    List<SubtaskSuggestion> subtasks,
  ) {
    final taskBloc = context.read<TaskBloc>();
    final now = DateTime.now();

    // Create tasks from subtask suggestions
    for (final subtask in subtasks) {
      final task = Task(
        id: '', // Will be generated by repository
        title: subtask.title,
        description: subtask.description,
        createdAt: now,
        updatedAt: now,
        status: TaskStatus.todo,
        priority: 3, // Default priority
        estimatedMinutes: subtask.estimatedMinutes,
        parentTaskId: widget.parentTaskId,
      );

      taskBloc.add(TaskCreateRequested(task: task));
    }

    // Close dialog and show success feedback
    Navigator.of(context).pop();

    // Show success snackbar
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Created ${subtasks.length} subtask(s) successfully'),
        backgroundColor: AltairColors.accentGreen,
        duration: const Duration(seconds: 2),
      ),
    );
  }
}
