/// Task filtering UI components.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../bloc/task/task_bloc.dart';
import '../bloc/task/task_event.dart';
import '../bloc/task/task_state.dart';

/// Shows a filter menu for tasks.
///
/// On mobile/tablet, displays as a bottom sheet.
/// On desktop, displays as a popup dialog.
void showTaskFilterMenu(BuildContext context, {required bool isMobile}) {
  // Capture the TaskBloc before creating the bottom sheet/dialog
  // so it's available in the new widget tree
  final taskBloc = context.read<TaskBloc>();

  if (isMobile) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => BlocProvider<TaskBloc>.value(
        value: taskBloc,
        child: const _TaskFilterBottomSheet(),
      ),
    );
  } else {
    showDialog(
      context: context,
      builder: (context) => BlocProvider<TaskBloc>.value(
        value: taskBloc,
        child: const _TaskFilterDialog(),
      ),
    );
  }
}

/// Bottom sheet for mobile task filtering.
class _TaskFilterBottomSheet extends StatelessWidget {
  const _TaskFilterBottomSheet();

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      decoration: BoxDecoration(
        color: theme.scaffoldBackgroundColor,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(20)),
        border: Border.all(color: Colors.black, width: 2),
      ),
      padding: EdgeInsets.only(
        bottom: MediaQuery.of(context).viewInsets.bottom,
      ),
      child: SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // Handle bar
            Container(
              margin: const EdgeInsets.symmetric(vertical: 12),
              width: 40,
              height: 4,
              decoration: BoxDecoration(
                color: Colors.grey[400],
                borderRadius: BorderRadius.circular(2),
              ),
            ),

            // Header
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    'Filter Tasks',
                    style: theme.textTheme.headlineSmall?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  TextButton(
                    onPressed: () {
                      context
                          .read<TaskBloc>()
                          .add(const TaskClearFiltersRequested());
                      Navigator.pop(context);
                    },
                    child: const Text('Clear All'),
                  ),
                ],
              ),
            ),

            const Divider(thickness: 2, color: Colors.black),

            // Filter content - wrapped in flexible scrollable area
            Flexible(
              child: SingleChildScrollView(
                child: const Padding(
                  padding: EdgeInsets.all(16),
                  child: _TaskFilterContent(),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Dialog for desktop task filtering.
class _TaskFilterDialog extends StatelessWidget {
  const _TaskFilterDialog();

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Dialog(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: const BorderSide(color: Colors.black, width: 2),
      ),
      child: Container(
        width: 400,
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Header
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Text(
                    'Filter Tasks',
                    style: theme.textTheme.headlineSmall?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    TextButton(
                      onPressed: () {
                        context
                            .read<TaskBloc>()
                            .add(const TaskClearFiltersRequested());
                      },
                      child: const Text('Clear All'),
                    ),
                    IconButton(
                      icon: const Icon(Icons.close),
                      onPressed: () => Navigator.pop(context),
                    ),
                  ],
                ),
              ],
            ),

            const SizedBox(height: 16),
            const Divider(thickness: 2, color: Colors.black),
            const SizedBox(height: 16),

            // Filter content - scrollable to prevent overflow
            const Flexible(
              child: SingleChildScrollView(
                child: _TaskFilterContent(),
              ),
            ),

            const SizedBox(height: 24),

            // Apply button
            SizedBox(
              width: double.infinity,
              child: AltairButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('Apply Filters'),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Shared filter content for both dialog and bottom sheet.
class _TaskFilterContent extends StatelessWidget {
  const _TaskFilterContent();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<TaskBloc, TaskState>(
      builder: (context, state) {
        final currentFilter = state is TaskLoaded ? state.filter : null;
        final currentTagFilter = state is TaskLoaded ? state.tagFilter : null;
        final allTasks = state is TaskLoaded ? state.tasks : <Task>[];

        // Extract unique tags from all tasks
        final allTags = <String>{};
        for (final task in allTasks) {
          allTags.addAll(task.tags);
        }
        final tagList = allTags.toList()..sort();

        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            // Status filter section
            Text(
              'Status',
              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: 12),

            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                _FilterChip(
                  label: 'All',
                  isSelected: currentFilter == null,
                  onTap: () {
                    context
                        .read<TaskBloc>()
                        .add(const TaskClearFiltersRequested());
                  },
                ),
                ...TaskStatus.values.map((status) {
                  return _FilterChip(
                    label: _getStatusLabel(status),
                    isSelected: currentFilter == status,
                    onTap: () {
                      context.read<TaskBloc>().add(
                            TaskFilterByStatusRequested(status: status),
                          );
                    },
                  );
                }),
              ],
            ),

            if (tagList.isNotEmpty) ...[
              const SizedBox(height: 24),

              // Tag filter section
              Text(
                'Tags',
                style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
              ),
              const SizedBox(height: 12),

              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: tagList.map((tag) {
                  final isSelected = currentTagFilter?.contains(tag) ?? false;
                  return _FilterChip(
                    label: tag,
                    isSelected: isSelected,
                    onTap: () {
                      final newTags =
                          Set<String>.from(currentTagFilter ?? <String>{});
                      if (isSelected) {
                        newTags.remove(tag);
                      } else {
                        newTags.add(tag);
                      }

                      if (newTags.isEmpty) {
                        context
                            .read<TaskBloc>()
                            .add(const TaskClearFiltersRequested());
                      } else {
                        context.read<TaskBloc>().add(
                              TaskFilterByTagsRequested(tags: newTags.toList()),
                            );
                      }
                    },
                  );
                }).toList(),
              ),
            ],
          ],
        );
      },
    );
  }

  String _getStatusLabel(TaskStatus status) {
    switch (status) {
      case TaskStatus.todo:
        return 'To Do';
      case TaskStatus.inProgress:
        return 'In Progress';
      case TaskStatus.completed:
        return 'Completed';
      case TaskStatus.cancelled:
        return 'Cancelled';
    }
  }
}

/// Custom filter chip with neo-brutalist styling.
class _FilterChip extends StatelessWidget {
  const _FilterChip({
    required this.label,
    required this.isSelected,
    required this.onTap,
  });

  final String label;
  final bool isSelected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        decoration: BoxDecoration(
          color: isSelected ? AltairColors.accentOrange : Colors.white,
          border: Border.all(color: Colors.black, width: 2),
          borderRadius: BorderRadius.circular(20),
          boxShadow: isSelected
              ? [
                  const BoxShadow(
                    color: Colors.black,
                    offset: Offset(2, 2),
                  ),
                ]
              : null,
        ),
        child: Text(
          label,
          style: TextStyle(
            fontWeight: FontWeight.bold,
            color: isSelected ? Colors.white : Colors.black,
          ),
        ),
      ),
    );
  }
}
