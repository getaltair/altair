/// Altair Guidance - ADHD-friendly task management.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import 'bloc/ai/ai_bloc.dart';
import 'bloc/project/project_bloc.dart';
import 'bloc/project/project_event.dart';
import 'bloc/task/task_bloc.dart';
import 'bloc/task/task_event.dart';
import 'bloc/task/task_state.dart';
import 'features/focus_mode/focus_mode_cubit.dart';
import 'pages/projects_page.dart';
import 'pages/task_edit_page.dart';
import 'services/ai/ai_config.dart';
import 'services/ai/ai_service.dart';
import 'shortcuts/intents.dart';
import 'shortcuts/shortcuts_config.dart';
import 'shortcuts/shortcuts_help_dialog.dart';

void main() {
  runApp(const AltairGuidanceApp());
}

/// Main application widget.
class AltairGuidanceApp extends StatelessWidget {
  /// Creates the Altair Guidance app.
  const AltairGuidanceApp({super.key});

  @override
  Widget build(BuildContext context) {
    // Initialize AI service with environment configuration
    final aiService = AIService(
      config: AIConfig.fromEnvironment(),
    );

    return MaterialApp(
      title: 'Altair Guidance',
      debugShowCheckedModeBanner: false,
      theme: AltairTheme.lightTheme,
      darkTheme: AltairTheme.darkTheme,
      themeMode: ThemeMode.system,
      home: MultiBlocProvider(
        providers: [
          BlocProvider(
            create: (_) => TaskBloc(
              taskRepository: TaskRepository(),
            )..add(const TaskLoadRequested()),
          ),
          BlocProvider(
            create: (_) => ProjectBloc(
              projectRepository: ProjectRepository(),
            )..add(const ProjectLoadRequested()),
          ),
          BlocProvider(
            create: (_) => FocusModeCubit(),
          ),
          BlocProvider(
            create: (_) => AIBloc(aiService: aiService),
          ),
        ],
        child: const HomePage(),
      ),
    );
  }
}

/// Home page of the application.
class HomePage extends StatefulWidget {
  /// Creates the home page.
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final FocusNode _quickCaptureFocusNode = FocusNode();

  @override
  void dispose() {
    _quickCaptureFocusNode.dispose();
    super.dispose();
  }

  void _handleNewTask() {
    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (context) => BlocProvider.value(
          value: context.read<TaskBloc>(),
          child: const TaskEditPage(),
        ),
      ),
    );
  }

  void _handleFocusQuickCapture() {
    _quickCaptureFocusNode.requestFocus();
  }

  void _handleNavigateToProjects() {
    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (context) => MultiBlocProvider(
          providers: [
            BlocProvider.value(
              value: context.read<TaskBloc>(),
            ),
            BlocProvider.value(
              value: context.read<ProjectBloc>(),
            ),
          ],
          child: const ProjectsPage(),
        ),
      ),
    );
  }

  void _handleRefresh() {
    context.read<TaskBloc>().add(const TaskLoadRequested());
  }

  void _handleToggleFocusMode() {
    context.read<FocusModeCubit>().toggle();
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<FocusModeCubit, FocusModeState>(
      builder: (context, focusModeState) {
        return Shortcuts(
          shortcuts: ShortcutsConfig.defaultShortcuts,
          child: Actions(
            actions: {
              NewTaskIntent: CallbackAction<NewTaskIntent>(
                onInvoke: (_) {
                  _handleNewTask();
                  return null;
                },
              ),
              FocusQuickCaptureIntent: CallbackAction<FocusQuickCaptureIntent>(
                onInvoke: (_) {
                  _handleFocusQuickCapture();
                  return null;
                },
              ),
              ShowShortcutsHelpIntent: CallbackAction<ShowShortcutsHelpIntent>(
                onInvoke: (_) {
                  showShortcutsHelp(context);
                  return null;
                },
              ),
              NavigateToProjectsIntent:
                  CallbackAction<NavigateToProjectsIntent>(
                onInvoke: (_) {
                  _handleNavigateToProjects();
                  return null;
                },
              ),
              NavigateToTasksIntent: CallbackAction<NavigateToTasksIntent>(
                onInvoke: (_) {
                  // Already on tasks page, just pop to root
                  Navigator.of(context).popUntil((route) => route.isFirst);
                  return null;
                },
              ),
              RefreshIntent: CallbackAction<RefreshIntent>(
                onInvoke: (_) {
                  _handleRefresh();
                  return null;
                },
              ),
              ToggleFocusModeIntent: CallbackAction<ToggleFocusModeIntent>(
                onInvoke: (_) {
                  _handleToggleFocusMode();
                  return null;
                },
              ),
            },
            child: Focus(
              autofocus: true,
              child: Scaffold(
                appBar: AppBar(
                  title: const Text('Tasks'),
                  leading: focusModeState.isEnabled
                      ? null
                      : null, // Keep default drawer icon when not in focus mode
                  automaticallyImplyLeading: !focusModeState.isEnabled,
                  actions: [
                    // Focus mode toggle
                    IconButton(
                      icon: Icon(
                        focusModeState.isEnabled
                            ? Icons.visibility_off
                            : Icons.visibility,
                      ),
                      tooltip: focusModeState.isEnabled
                          ? 'Exit focus mode (Ctrl/Cmd + D)'
                          : 'Enter focus mode (Ctrl/Cmd + D)',
                      onPressed: _handleToggleFocusMode,
                      color: focusModeState.isEnabled
                          ? AltairColors.accentYellow
                          : null,
                    ),
                    if (!focusModeState.isEnabled) ...[
                      // Filter buttons
                      IconButton(
                        icon: const Icon(Icons.filter_list),
                        onPressed: () {
                          // TODO: Show filter menu
                        },
                      ),
                      // Keyboard shortcuts help
                      IconButton(
                        icon: const Icon(Icons.keyboard),
                        tooltip: 'Keyboard shortcuts (Shift + ?)',
                        onPressed: () => showShortcutsHelp(context),
                      ),
                    ],
                  ],
                ),
                drawer: focusModeState.isEnabled
                    ? null
                    : Drawer(
                        child: ListView(
                          padding: EdgeInsets.zero,
                          children: [
                            DrawerHeader(
                              decoration: BoxDecoration(
                                color: AltairColors.accentYellow,
                              ),
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                mainAxisAlignment: MainAxisAlignment.end,
                                children: [
                                  Text(
                                    'Altair Guidance',
                                    style: Theme.of(context)
                                        .textTheme
                                        .headlineMedium
                                        ?.copyWith(
                                          fontWeight: FontWeight.bold,
                                          color: Colors.black,
                                        ),
                                  ),
                                  const SizedBox(height: AltairSpacing.xs),
                                  Text(
                                    'ADHD-friendly task management',
                                    style: Theme.of(context)
                                        .textTheme
                                        .bodyMedium
                                        ?.copyWith(
                                          color: Colors.black,
                                        ),
                                  ),
                                ],
                              ),
                            ),
                            ListTile(
                              leading: const Icon(Icons.checklist),
                              title: const Text('Tasks'),
                              selected: true,
                              onTap: () {
                                Navigator.pop(context);
                              },
                            ),
                            ListTile(
                              leading: const Icon(Icons.folder),
                              title: const Text('Projects'),
                              onTap: () {
                                Navigator.pop(context);
                                Navigator.of(context).push(
                                  MaterialPageRoute<void>(
                                    builder: (context) => MultiBlocProvider(
                                      providers: [
                                        BlocProvider.value(
                                          value: context.read<TaskBloc>(),
                                        ),
                                        BlocProvider.value(
                                          value: context.read<ProjectBloc>(),
                                        ),
                                      ],
                                      child: const ProjectsPage(),
                                    ),
                                  ),
                                );
                              },
                            ),
                            const Divider(),
                            ListTile(
                              leading: const Icon(Icons.settings),
                              title: const Text('Settings'),
                              onTap: () {
                                Navigator.pop(context);
                                // TODO: Navigate to settings
                              },
                            ),
                          ],
                        ),
                      ),
                floatingActionButton: focusModeState.isEnabled
                    ? null
                    : FloatingActionButton(
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
                        focusNode: _quickCaptureFocusNode,
                        onCapture: (text) {
                          context.read<TaskBloc>().add(
                                TaskQuickCaptureRequested(title: text),
                              );
                        },
                        hint: 'Quick capture (Ctrl/Cmd + K)...',
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
                                content: Text(
                                    'Task captured: "${state.task.title}"'),
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
                                      style: Theme.of(context)
                                          .textTheme
                                          .headlineSmall,
                                    ),
                                    const SizedBox(height: AltairSpacing.sm),
                                    Text(
                                      'Use quick capture above to add your first task',
                                      style: Theme.of(context)
                                          .textTheme
                                          .bodyMedium,
                                      textAlign: TextAlign.center,
                                    ),
                                  ],
                                ),
                              );
                            }

                            return ReorderableListView.builder(
                              padding: const EdgeInsets.all(AltairSpacing.md),
                              itemCount: state.tasks.length,
                              onReorder: (oldIndex, newIndex) {
                                context.read<TaskBloc>().add(
                                      TaskReorderRequested(
                                        oldIndex: oldIndex,
                                        newIndex: newIndex,
                                      ),
                                    );
                              },
                              itemBuilder: (context, index) {
                                final task = state.tasks[index];
                                return Padding(
                                  key: ValueKey(task
                                      .id), // Required for ReorderableListView
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
                                  style:
                                      Theme.of(context).textTheme.displayMedium,
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
              ), // Scaffold
            ), // Focus
          ), // Actions
        ); // Shortcuts
      },
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
                  status:
                      value == true ? TaskStatus.completed : TaskStatus.todo,
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
