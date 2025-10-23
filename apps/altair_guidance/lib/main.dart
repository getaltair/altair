/// Altair Guidance - ADHD-friendly task management.
library;

import 'dart:io' show Platform;

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
import 'features/theme/theme_cubit.dart';
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

    return MultiBlocProvider(
      providers: [
        BlocProvider(
          create: (_) => ThemeCubit(),
        ),
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
      child: BlocBuilder<ThemeCubit, ThemeState>(
        builder: (context, themeState) {
          return MaterialApp(
            title: 'Altair Guidance',
            debugShowCheckedModeBanner: false,
            theme: AltairTheme.lightTheme,
            darkTheme: AltairTheme.darkTheme,
            themeMode: themeState.themeMode,
            home: const HomePage(),
          );
        },
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
    // Capture blocs from current context before navigation
    final taskBloc = context.read<TaskBloc>();
    final projectBloc = context.read<ProjectBloc>();

    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (context) => MultiBlocProvider(
          providers: [
            BlocProvider.value(value: taskBloc),
            BlocProvider.value(value: projectBloc),
          ],
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
        return LayoutBuilder(
          builder: (context, constraints) {
            // Determine if we're on desktop (>= 1024px)
            final isDesktop = constraints.maxWidth >= 1024;
            final showDrawer = !isDesktop || focusModeState.isEnabled;

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
                  FocusQuickCaptureIntent:
                      CallbackAction<FocusQuickCaptureIntent>(
                    onInvoke: (_) {
                      _handleFocusQuickCapture();
                      return null;
                    },
                  ),
                  ShowShortcutsHelpIntent:
                      CallbackAction<ShowShortcutsHelpIntent>(
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
                  child: Row(
                    children: [
                      // Fixed sidebar on desktop (when not in focus mode)
                      if (isDesktop && !focusModeState.isEnabled)
                        Container(
                          width: 288,
                          decoration: BoxDecoration(
                            color: Theme.of(context).cardColor,
                            border: Border(
                              right: BorderSide(
                                color: Theme.of(context).dividerColor,
                                width: 2.0,
                              ),
                            ),
                          ),
                          child: Material(
                            color: Colors.transparent,
                            child: Column(
                              children: [
                                // Desktop header
                                Padding(
                                  padding:
                                      const EdgeInsets.all(AltairSpacing.lg),
                                  child: Column(
                                    crossAxisAlignment:
                                        CrossAxisAlignment.start,
                                    children: [
                                      Text(
                                        'Altair Guidance',
                                        style: Theme.of(context)
                                            .textTheme
                                            .headlineMedium
                                            ?.copyWith(
                                              fontWeight: FontWeight.bold,
                                            ),
                                      ),
                                      const SizedBox(height: AltairSpacing.xs),
                                      Text(
                                        'ADHD-friendly task management',
                                        style: Theme.of(context)
                                            .textTheme
                                            .bodySmall
                                            ?.copyWith(
                                              color: Theme.of(context)
                                                  .textTheme
                                                  .bodySmall
                                                  ?.color,
                                            ),
                                      ),
                                      const SizedBox(height: AltairSpacing.md),
                                      const Divider(),
                                    ],
                                  ),
                                ),
                                // Navigation buttons
                                Padding(
                                  padding: const EdgeInsets.symmetric(
                                    horizontal: AltairSpacing.md,
                                    vertical: AltairSpacing.sm,
                                  ),
                                  child: Column(
                                    children: [
                                      _SidebarButton(
                                        icon: Icons.checklist,
                                        label: 'TASKS',
                                        isSelected: true,
                                        onTap: () {},
                                      ),
                                      const SizedBox(height: AltairSpacing.sm),
                                      _SidebarButton(
                                        icon: Icons.folder,
                                        label: 'PROJECTS',
                                        isSelected: false,
                                        onTap: () {
                                          Navigator.of(context).push(
                                            MaterialPageRoute<void>(
                                              builder: (context) =>
                                                  MultiBlocProvider(
                                                providers: [
                                                  BlocProvider.value(
                                                    value: context
                                                        .read<TaskBloc>(),
                                                  ),
                                                  BlocProvider.value(
                                                    value: context
                                                        .read<ProjectBloc>(),
                                                  ),
                                                ],
                                                child: const ProjectsPage(),
                                              ),
                                            ),
                                          );
                                        },
                                      ),
                                    ],
                                  ),
                                ),

                                // Push settings to bottom
                                const Expanded(child: SizedBox.shrink()),

                                // Settings button at bottom
                                const Divider(),
                                Padding(
                                  padding:
                                      const EdgeInsets.all(AltairSpacing.md),
                                  child: _SidebarButton(
                                    icon: Icons.settings,
                                    label: 'SETTINGS',
                                    isSelected: false,
                                    onTap: () {
                                      // TODO: Navigate to settings
                                    },
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ),

                      // Main content area
                      Expanded(
                        child: Scaffold(
                          appBar: AppBar(
                            title: const Text('Tasks'),
                            // Only show hamburger menu on mobile or when not in desktop mode
                            automaticallyImplyLeading:
                                showDrawer && !focusModeState.isEnabled,
                            actions: [
                              // New Task button (desktop-friendly replacement for FAB)
                              if (!focusModeState.isEnabled && isDesktop)
                                Padding(
                                  padding: const EdgeInsets.symmetric(
                                    horizontal: AltairSpacing.sm,
                                    vertical: AltairSpacing.xs,
                                  ),
                                  child: ElevatedButton.icon(
                                    onPressed: _handleNewTask,
                                    icon: const Icon(Icons.add, size: 20),
                                    label: const Text('NEW TASK'),
                                    style: ElevatedButton.styleFrom(
                                      backgroundColor:
                                          AltairColors.accentOrange,
                                      foregroundColor: Colors.white,
                                      elevation: 0,
                                      padding: const EdgeInsets.symmetric(
                                        horizontal: AltairSpacing.md,
                                        vertical: AltairSpacing.sm,
                                      ),
                                      shape: const RoundedRectangleBorder(),
                                      side: const BorderSide(
                                        color: Colors.black,
                                        width: AltairBorders.standard,
                                      ),
                                    ),
                                  ),
                                ),
                              // Theme toggle
                              Builder(
                                builder: (context) {
                                  final isDark = Theme.of(context).brightness ==
                                      Brightness.dark;
                                  return IconButton(
                                    icon: Icon(
                                      isDark
                                          ? Icons.light_mode
                                          : Icons.dark_mode,
                                    ),
                                    tooltip: isDark
                                        ? 'Switch to light mode'
                                        : 'Switch to dark mode',
                                    onPressed: () {
                                      context.read<ThemeCubit>().toggle();
                                    },
                                  );
                                },
                              ),
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
                                    ? AltairColors.accentOrange
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
                          // Show drawer on mobile/tablet (when not desktop or in focus mode)
                          drawer: (showDrawer && !focusModeState.isEnabled)
                              ? Drawer(
                                  child: Column(
                                    children: [
                                      // Mobile header
                                      DrawerHeader(
                                        decoration: BoxDecoration(
                                          color: Theme.of(context).cardColor,
                                          border: Border(
                                            bottom: BorderSide(
                                              color: Theme.of(context)
                                                  .dividerColor,
                                              width: 1.0,
                                            ),
                                          ),
                                        ),
                                        child: Column(
                                          crossAxisAlignment:
                                              CrossAxisAlignment.start,
                                          mainAxisAlignment:
                                              MainAxisAlignment.end,
                                          children: [
                                            Text(
                                              'Altair Guidance',
                                              style: Theme.of(context)
                                                  .textTheme
                                                  .headlineMedium
                                                  ?.copyWith(
                                                    fontWeight: FontWeight.bold,
                                                  ),
                                            ),
                                            const SizedBox(
                                                height: AltairSpacing.xs),
                                            Text(
                                              'ADHD-friendly task management',
                                              style: Theme.of(context)
                                                  .textTheme
                                                  .bodyMedium
                                                  ?.copyWith(
                                                    color: Theme.of(context)
                                                        .textTheme
                                                        .bodySmall
                                                        ?.color,
                                                  ),
                                            ),
                                          ],
                                        ),
                                      ),
                                      // Navigation buttons
                                      Padding(
                                        padding: const EdgeInsets.symmetric(
                                          horizontal: AltairSpacing.md,
                                          vertical: AltairSpacing.sm,
                                        ),
                                        child: Column(
                                          children: [
                                            _SidebarButton(
                                              icon: Icons.checklist,
                                              label: 'TASKS',
                                              isSelected: true,
                                              onTap: () {
                                                Navigator.pop(context);
                                              },
                                            ),
                                            const SizedBox(
                                                height: AltairSpacing.sm),
                                            _SidebarButton(
                                              icon: Icons.folder,
                                              label: 'PROJECTS',
                                              isSelected: false,
                                              onTap: () {
                                                Navigator.pop(context);
                                                Navigator.of(context).push(
                                                  MaterialPageRoute<void>(
                                                    builder: (context) =>
                                                        MultiBlocProvider(
                                                      providers: [
                                                        BlocProvider.value(
                                                          value: context
                                                              .read<TaskBloc>(),
                                                        ),
                                                        BlocProvider.value(
                                                          value: context.read<
                                                              ProjectBloc>(),
                                                        ),
                                                      ],
                                                      child:
                                                          const ProjectsPage(),
                                                    ),
                                                  ),
                                                );
                                              },
                                            ),
                                          ],
                                        ),
                                      ),

                                      // Push settings to bottom
                                      const Expanded(child: SizedBox.shrink()),

                                      // Settings button at bottom
                                      const Divider(),
                                      Padding(
                                        padding: const EdgeInsets.all(
                                            AltairSpacing.md),
                                        child: _SidebarButton(
                                          icon: Icons.settings,
                                          label: 'SETTINGS',
                                          isSelected: false,
                                          onTap: () {
                                            Navigator.pop(context);
                                            // TODO: Navigate to settings
                                          },
                                        ),
                                      ),
                                    ],
                                  ),
                                )
                              : null,
                          // FAB kept for mobile, hidden on desktop (replaced by AppBar button)
                          floatingActionButton: (!focusModeState.isEnabled &&
                                  !isDesktop)
                              ? Padding(
                                  padding: const EdgeInsets.only(
                                    right: AltairSpacing.lg,
                                    bottom: AltairSpacing.xl,
                                  ),
                                  child: FloatingActionButton(
                                    onPressed: _handleNewTask,
                                    backgroundColor: AltairColors.accentOrange,
                                    foregroundColor: Colors.white,
                                    child: const Icon(Icons.add),
                                  ),
                                )
                              : null,
                          body: Column(
                            children: [
                              // Quick Capture at the top - always visible
                              Container(
                                padding: const EdgeInsets.all(AltairSpacing.md),
                                decoration: BoxDecoration(
                                  color:
                                      Theme.of(context).scaffoldBackgroundColor,
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
                                          TaskQuickCaptureRequested(
                                              title: text),
                                        );
                                  },
                                  hint: Platform.isAndroid || Platform.isIOS
                                      ? 'Quick capture...'
                                      : 'Quick capture (Ctrl/Cmd + K)...',
                                  accentColor: AltairColors.accentOrange,
                                ),
                              ),

                              // Task list
                              Expanded(
                                child: BlocConsumer<TaskBloc, TaskState>(
                                  listener: (context, state) {
                                    if (state is TaskCaptured) {
                                      // Show brief success feedback
                                      ScaffoldMessenger.of(context)
                                          .showSnackBar(
                                        SnackBar(
                                          content: Text(
                                              'Task captured: "${state.task.title}"'),
                                          duration: const Duration(seconds: 2),
                                          backgroundColor:
                                              AltairColors.accentGreen,
                                        ),
                                      );
                                    } else if (state is TaskFailure) {
                                      ScaffoldMessenger.of(context)
                                          .showSnackBar(
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
                                            mainAxisAlignment:
                                                MainAxisAlignment.center,
                                            children: [
                                              Icon(
                                                Icons.checklist,
                                                size: 64,
                                                color:
                                                    AltairColors.textSecondary,
                                              ),
                                              const SizedBox(
                                                  height: AltairSpacing.md),
                                              Text(
                                                'No tasks yet',
                                                style: Theme.of(context)
                                                    .textTheme
                                                    .headlineSmall,
                                              ),
                                              const SizedBox(
                                                  height: AltairSpacing.sm),
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
                                        padding: const EdgeInsets.all(
                                            AltairSpacing.md),
                                        buildDefaultDragHandles: false,
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
                                            child: _TaskListItem(
                                                task: task, index: index),
                                          );
                                        },
                                      );
                                    }

                                    // Initial or failure state
                                    return Center(
                                      child: Column(
                                        mainAxisAlignment:
                                            MainAxisAlignment.center,
                                        children: [
                                          Text(
                                            'Welcome to Altair Guidance',
                                            style: Theme.of(context)
                                                .textTheme
                                                .displayMedium,
                                          ),
                                          const SizedBox(
                                              height: AltairSpacing.md),
                                          AltairButton(
                                            onPressed: () {
                                              context.read<TaskBloc>().add(
                                                  const TaskLoadRequested());
                                            },
                                            variant: AltairButtonVariant.filled,
                                            accentColor:
                                                AltairColors.accentBlue,
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
                      ), // Expanded
                    ], // Row children
                  ), // Row
                ), // Focus
              ), // Actions
            ); // Shortcuts
          }, // LayoutBuilder builder
        ); // LayoutBuilder
      }, // BlocBuilder builder
    ); // BlocBuilder
  }
}

/// Widget to display a task in the list.
class _TaskListItem extends StatelessWidget {
  const _TaskListItem({required this.task, required this.index});

  final Task task;
  final int index;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {
        // Capture blocs from current context before navigation
        final taskBloc = context.read<TaskBloc>();
        final projectBloc = context.read<ProjectBloc>();

        Navigator.of(context).push(
          MaterialPageRoute<void>(
            builder: (context) => MultiBlocProvider(
              providers: [
                BlocProvider.value(value: taskBloc),
                BlocProvider.value(value: projectBloc),
              ],
              child: TaskEditPage(task: task),
            ),
          ),
        );
      },
      child: AltairCard(
        accentColor: _getAccentColorForStatus(task.status),
        showAccentBar: true,
        child: IntrinsicHeight(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Checkbox - centered vertically
              Center(
                child: Checkbox(
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
              ),

              const SizedBox(width: AltairSpacing.sm),

              // Task content
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.symmetric(
                    vertical: AltairSpacing.xs,
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisAlignment: MainAxisAlignment.center,
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
              ),

              const SizedBox(width: AltairSpacing.sm),

              // Delete button - centered vertically
              Center(
                child: IconButton(
                  icon: const Icon(Icons.delete_outline, size: 20),
                  onPressed: () {
                    context.read<TaskBloc>().add(
                          TaskDeleteRequested(taskId: task.id),
                        );
                  },
                  color: AltairColors.error,
                  padding: const EdgeInsets.all(AltairSpacing.xs),
                  constraints: const BoxConstraints(
                    minWidth: 36,
                    minHeight: 36,
                  ),
                ),
              ),

              const SizedBox(width: AltairSpacing.xs),

              // Custom drag handle - centered vertically
              ReorderableDragStartListener(
                index: index,
                child: Center(
                  child: Icon(
                    Icons.drag_handle,
                    color: Theme.of(context).dividerColor,
                    size: 20,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Color _getAccentColorForStatus(TaskStatus status) {
    return switch (status) {
      TaskStatus.todo => AltairColors.accentOrange,
      TaskStatus.inProgress => AltairColors.accentBlue,
      TaskStatus.completed => AltairColors.accentGreen,
      TaskStatus.cancelled => AltairColors.textSecondary,
    };
  }
}

/// Neo-brutalist sidebar button widget.
class _SidebarButton extends StatefulWidget {
  const _SidebarButton({
    required this.icon,
    required this.label,
    required this.onTap,
    this.isSelected = false,
  });

  final IconData icon;
  final String label;
  final VoidCallback onTap;
  final bool isSelected;

  @override
  State<_SidebarButton> createState() => _SidebarButtonState();
}

class _SidebarButtonState extends State<_SidebarButton> {
  bool _isHovering = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    final backgroundColor =
        isDark ? AltairColors.darkBgSecondary : AltairColors.lightBgSecondary;

    final textColor =
        isDark ? AltairColors.darkTextPrimary : AltairColors.lightTextPrimary;

    final borderColor = widget.isSelected
        ? AltairColors.accentOrange
        : (isDark
            ? AltairColors.darkBorderColor
            : AltairColors.lightBorderColor);

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        transform: _isHovering
            ? Matrix4.translationValues(-2.0, -2.0, 0.0)
            : Matrix4.identity(),
        child: Material(
          color: backgroundColor,
          child: InkWell(
            onTap: widget.onTap,
            child: Container(
              padding: const EdgeInsets.symmetric(
                horizontal: AltairSpacing.md,
                vertical: AltairSpacing.md,
              ),
              decoration: BoxDecoration(
                color: backgroundColor,
                border: Border.all(
                  color: borderColor,
                  width: AltairBorders.standard,
                ),
                boxShadow: [
                  if (_isHovering)
                    AltairBorders.shadowSmall
                  else
                    AltairBorders.shadow,
                ],
              ),
              child: Row(
                children: [
                  Icon(
                    widget.icon,
                    color: textColor,
                    size: 20,
                  ),
                  const SizedBox(width: AltairSpacing.md),
                  Text(
                    widget.label,
                    style: theme.textTheme.bodyMedium?.copyWith(
                      color: textColor,
                      fontWeight: FontWeight.w600,
                      letterSpacing: 1.2,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
