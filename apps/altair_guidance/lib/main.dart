/// Altair Guidance - ADHD-friendly task management.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import 'bloc/task/task_bloc.dart';
import 'bloc/task/task_event.dart';
import 'bloc/task/task_state.dart';
import 'pages/task_edit_page.dart';

void main() {
  runApp(const AltairGuidanceApp());
}

/// Main application widget.
class AltairGuidanceApp extends StatelessWidget {
  /// Creates the Altair Guidance app.
  const AltairGuidanceApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Altair Guidance',
      debugShowCheckedModeBanner: false,
      theme: AltairTheme.lightTheme,
      darkTheme: AltairTheme.darkTheme,
      themeMode: ThemeMode.system,
      home: BlocProvider(
        create: (_) => TaskBloc(
          taskRepository: TaskRepository(),
        )..add(const TaskLoadRequested()),
        child: const HomePage(),
      ),
    );
  }
}

/// Home page of the application.
class HomePage extends StatelessWidget {
  /// Creates the home page.
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Altair Guidance'),
        actions: [
          // Filter buttons
          IconButton(
            icon: const Icon(Icons.filter_list),
            onPressed: () {
              // TODO: Show filter menu
            },
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Navigator.of(context).push(
            MaterialPageRoute<void>(
              builder: (context) => BlocProvider.value(
                value: context.read<TaskBloc>(),
                child: const TaskEditPage(),
              ),
            ),
          );
        },
        backgroundColor: AltairColors.accentGreen,
        child: const Icon(Icons.add, color: Colors.black),
      ),
      body: Column(
        children: [
          // Quick Capture at the top - always visible
          Container(
            padding: const EdgeInsets.all(AltairSpacing.md),
            decoration: BoxDecoration(
              color: Theme.of(context).scaffoldBackgroundColor,
              border: Border(
                bottom: BorderSide(
                  color: Theme.of(context).dividerColor,
                  width: AltairBorders.medium,
                ),
              ),
            ),
            child: AltairQuickCapture(
              onCapture: (text) {
                context.read<TaskBloc>().add(
                      TaskQuickCaptureRequested(title: text),
                    );
              },
              hint: 'Quick capture (< 3 seconds)...',
              accentColor: AltairColors.accentYellow,
            ),
          ),

          // Task list
          Expanded(
            child: BlocConsumer<TaskBloc, TaskState>(
              listener: (context, state) {
                if (state is TaskCaptured) {
                  // Show brief success feedback
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text('Task captured: "${state.task.title}"'),
                      duration: const Duration(seconds: 2),
                      backgroundColor: AltairColors.accentGreen,
                    ),
                  );
                } else if (state is TaskFailure) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text(state.message),
                      backgroundColor: AltairColors.error,
                    ),
                  );
                }
              },
              builder: (context, state) {
                if (state is TaskLoading) {
                  return const Center(
                    child: CircularProgressIndicator(),
                  );
                }

                if (state is TaskLoaded) {
                  if (state.tasks.isEmpty) {
                    return Center(
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Icon(
                            Icons.checklist,
                            size: 64,
                            color: AltairColors.textSecondary,
                          ),
                          const SizedBox(height: AltairSpacing.md),
                          Text(
                            'No tasks yet',
                            style: Theme.of(context).textTheme.headlineSmall,
                          ),
                          const SizedBox(height: AltairSpacing.sm),
                          Text(
                            'Use quick capture above to add your first task',
                            style: Theme.of(context).textTheme.bodyMedium,
                            textAlign: TextAlign.center,
                          ),
                        ],
                      ),
                    );
                  }

                  return ListView.builder(
                    padding: const EdgeInsets.all(AltairSpacing.md),
                    itemCount: state.tasks.length,
                    itemBuilder: (context, index) {
                      final task = state.tasks[index];
                      return Padding(
                        padding: const EdgeInsets.only(
                          bottom: AltairSpacing.md,
                        ),
                        child: _TaskListItem(task: task),
                      );
                    },
                  );
                }

                // Initial or failure state
                return Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(
                        'Welcome to Altair Guidance',
                        style: Theme.of(context).textTheme.displayMedium,
                      ),
                      const SizedBox(height: AltairSpacing.md),
                      AltairButton(
                        onPressed: () {
                          context
                              .read<TaskBloc>()
                              .add(const TaskLoadRequested());
                        },
                        variant: AltairButtonVariant.filled,
                        accentColor: AltairColors.accentBlue,
                        child: const Text('Load Tasks'),
                      ),
                    ],
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

/// Widget to display a task in the list.
class _TaskListItem extends StatelessWidget {
  const _TaskListItem({required this.task});

  final Task task;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {
        Navigator.of(context).push(
          MaterialPageRoute<void>(
            builder: (context) => BlocProvider.value(
              value: context.read<TaskBloc>(),
              child: TaskEditPage(task: task),
            ),
          ),
        );
      },
      child: AltairCard(
        accentColor: _getAccentColorForStatus(task.status),
        showAccentBar: true,
        child: Row(
        children: [
          // Checkbox
          Checkbox(
            value: task.status == TaskStatus.completed,
            onChanged: (value) {
              final updatedTask = task.copyWith(
                status: value == true
                    ? TaskStatus.completed
                    : TaskStatus.todo,
                completedAt: value == true ? DateTime.now() : null,
              );
              context.read<TaskBloc>().add(
                    TaskUpdateRequested(task: updatedTask),
                  );
            },
            activeColor: AltairColors.accentGreen,
          ),

          // Task content
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  task.title,
                  style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                        decoration: task.status == TaskStatus.completed
                            ? TextDecoration.lineThrough
                            : null,
                        color: task.status == TaskStatus.completed
                            ? AltairColors.textSecondary
                            : null,
                      ),
                ),
                if (task.description != null) ...[
                  const SizedBox(height: AltairSpacing.xs),
                  Text(
                    task.description!,
                    style: Theme.of(context).textTheme.bodySmall,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ],
            ),
          ),

          // Delete button
          IconButton(
            icon: const Icon(Icons.delete_outline),
            onPressed: () {
              context.read<TaskBloc>().add(
                    TaskDeleteRequested(taskId: task.id),
                  );
            },
            color: AltairColors.error,
          ),
        ],
      ),
      ),
    );
  }

  Color _getAccentColorForStatus(TaskStatus status) {
    return switch (status) {
      TaskStatus.todo => AltairColors.accentYellow,
      TaskStatus.inProgress => AltairColors.accentBlue,
      TaskStatus.completed => AltairColors.accentGreen,
      TaskStatus.cancelled => AltairColors.textSecondary,
    };
  }
}
