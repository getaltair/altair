/// Time estimate dialog with AI-powered time predictions.
library;

import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../bloc/ai/ai_bloc.dart';
import '../../bloc/ai/ai_event.dart';
import '../../bloc/ai/ai_state.dart';
import '../../services/ai/models.dart';

/// Shows time estimate dialog.
void showTimeEstimateDialog(
  BuildContext context, {
  required String taskTitle,
  String? taskDescription,
  List<String>? subtasks,
  SkillLevel skillLevel = SkillLevel.intermediate,
}) {
  final aiBloc = context.read<AIBloc>();
  showDialog<void>(
    context: context,
    barrierDismissible: true,
    builder: (context) => BlocProvider<AIBloc>.value(
      value: aiBloc,
      child: TimeEstimateDialog(
        taskTitle: taskTitle,
        taskDescription: taskDescription,
        subtasks: subtasks,
        skillLevel: skillLevel,
      ),
    ),
  );
}

/// Dialog for displaying AI-powered time estimates.
class TimeEstimateDialog extends StatefulWidget {
  /// Creates a time estimate dialog.
  const TimeEstimateDialog({
    required this.taskTitle,
    this.taskDescription,
    this.subtasks,
    this.skillLevel = SkillLevel.intermediate,
    super.key,
  });

  /// Task title to estimate.
  final String taskTitle;

  /// Optional task description.
  final String? taskDescription;

  /// Optional list of subtasks.
  final List<String>? subtasks;

  /// User skill level.
  final SkillLevel skillLevel;

  @override
  State<TimeEstimateDialog> createState() => _TimeEstimateDialogState();
}

class _TimeEstimateDialogState extends State<TimeEstimateDialog> {
  @override
  void initState() {
    super.initState();
    // Trigger estimate request when dialog opens
    try {
      if (widget.taskTitle.trim().isEmpty) {
        return;
      }

      context.read<AIBloc>().add(
            AITimeEstimateRequested(
              request: TimeEstimateRequest(
                taskTitle: widget.taskTitle,
                taskDescription: widget.taskDescription,
                subtasks: widget.subtasks,
                skillLevel: widget.skillLevel,
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
        constraints: const BoxConstraints(maxWidth: 600, maxHeight: 700),
        child: AltairCard(
          accentColor: AltairColors.accentGreen,
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
                    const Icon(Icons.timer, size: 32),
                    const SizedBox(width: AltairSpacing.md),
                    Expanded(
                      child: Text(
                        'AI Time Estimate',
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

                // Content
                Expanded(
                  child: BlocConsumer<AIBloc, AIState>(
                    listener: (context, state) {
                      if (state is AIFailure &&
                          state.operationType == AIOperationType.timeEstimate) {
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
                          state.operationType == AIOperationType.timeEstimate) {
                        return _buildLoadingState();
                      }

                      if (state is AITimeEstimateSuccess) {
                        return _buildSuccessState(state.response);
                      }

                      if (state is AIFailure &&
                          state.operationType == AIOperationType.timeEstimate) {
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
        label: 'Loading time estimate from AI',
        child: const Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            CircularProgressIndicator(),
            SizedBox(height: AltairSpacing.md),
            Text('AI is calculating time estimate...'),
          ],
        ),
      ),
    );
  }

  Widget _buildSuccessState(TimeEstimateResponse response) {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Task title
          Text(
            'Task: ${response.taskTitle}',
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
          ),
          const SizedBox(height: AltairSpacing.lg),

          // Time estimates card
          AltairCard(
            accentColor: AltairColors.accentGreen,
            showAccentBar: true,
            child: Padding(
              padding: const EdgeInsets.all(AltairSpacing.md),
              child: Column(
                children: [
                  _EstimateRow(
                    label: 'Optimistic (Best Case)',
                    minutes: response.estimate.optimisticMinutes,
                    color: AltairColors.accentGreen,
                  ),
                  const SizedBox(height: AltairSpacing.sm),
                  _EstimateRow(
                    label: 'Realistic (Most Likely)',
                    minutes: response.estimate.realisticMinutes,
                    color: AltairColors.accentYellow,
                    isHighlighted: true,
                  ),
                  const SizedBox(height: AltairSpacing.sm),
                  _EstimateRow(
                    label: 'Pessimistic (Worst Case)',
                    minutes: response.estimate.pessimisticMinutes,
                    color: AltairColors.error,
                  ),
                  const SizedBox(height: AltairSpacing.md),
                  const Divider(),
                  const SizedBox(height: AltairSpacing.sm),
                  Row(
                    children: [
                      const Icon(Icons.verified, size: 20),
                      const SizedBox(width: AltairSpacing.xs),
                      Text(
                        'Confidence: ${(response.estimate.confidence * 100).toStringAsFixed(0)}%',
                        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                              fontWeight: FontWeight.bold,
                            ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),

          const SizedBox(height: AltairSpacing.lg),

          // Factors considered
          Text(
            'Factors Considered',
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
          ),
          const SizedBox(height: AltairSpacing.sm),
          ...response.factors.map((factor) => Padding(
                padding: const EdgeInsets.only(bottom: AltairSpacing.xs),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Icon(Icons.check_circle, size: 16),
                    const SizedBox(width: AltairSpacing.xs),
                    Expanded(
                      child: Text(
                        factor,
                        style: Theme.of(context).textTheme.bodyMedium,
                      ),
                    ),
                  ],
                ),
              )),

          const SizedBox(height: AltairSpacing.lg),

          // Assumptions
          Text(
            'Assumptions',
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
          ),
          const SizedBox(height: AltairSpacing.sm),
          ...response.assumptions.map((assumption) => Padding(
                padding: const EdgeInsets.only(bottom: AltairSpacing.xs),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Icon(Icons.info_outline, size: 16),
                    const SizedBox(width: AltairSpacing.xs),
                    Expanded(
                      child: Text(
                        assumption,
                        style: Theme.of(context).textTheme.bodyMedium,
                      ),
                    ),
                  ],
                ),
              )),
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
            'Failed to get time estimate',
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
              // Retry estimation
              context.read<AIBloc>().add(
                    AITimeEstimateRequested(
                      request: TimeEstimateRequest(
                        taskTitle: widget.taskTitle,
                        taskDescription: widget.taskDescription,
                        subtasks: widget.subtasks,
                        skillLevel: widget.skillLevel,
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
}

/// Row displaying a time estimate.
class _EstimateRow extends StatelessWidget {
  const _EstimateRow({
    required this.label,
    required this.minutes,
    required this.color,
    this.isHighlighted = false,
  });

  final String label;
  final int minutes;
  final Color color;
  final bool isHighlighted;

  @override
  Widget build(BuildContext context) {
    final hours = minutes ~/ 60;
    final mins = minutes % 60;
    final timeText = hours > 0 ? '${hours}h ${mins}m' : '${mins}m';

    return Container(
      padding: const EdgeInsets.all(AltairSpacing.sm),
      decoration: isHighlighted
          ? BoxDecoration(
              color: color.withValues(alpha: 0.2),
              borderRadius: BorderRadius.circular(4),
              border: Border.all(
                color: Colors.black,
                width: AltairBorders.medium,
              ),
            )
          : null,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Expanded(
            child: Text(
              label,
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                    fontWeight:
                        isHighlighted ? FontWeight.bold : FontWeight.normal,
                  ),
            ),
          ),
          Container(
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
              timeText,
              style: const TextStyle(
                fontWeight: FontWeight.bold,
                color: Colors.black,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
