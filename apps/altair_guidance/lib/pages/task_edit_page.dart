/// Task editing page for detailed task management.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../bloc/project/project_bloc.dart';
import '../bloc/project/project_state.dart';
import '../bloc/settings/settings_bloc.dart';
import '../bloc/settings/settings_state.dart';
import '../bloc/task/task_bloc.dart';
import '../bloc/task/task_event.dart';
import '../features/ai/ai_consent_dialog.dart';
import '../features/ai/context_suggestions_dialog.dart';
import '../features/ai/task_breakdown_dialog.dart';
import '../features/ai/time_estimate_dialog.dart';

/// Page for creating or editing a task with full details.
class TaskEditPage extends StatefulWidget {
  /// Creates a task edit page.
  const TaskEditPage({
    super.key,
    this.task,
  });

  /// The task to edit. If null, creates a new task.
  final Task? task;

  @override
  State<TaskEditPage> createState() => _TaskEditPageState();
}

class _TaskEditPageState extends State<TaskEditPage> {
  late final TextEditingController _titleController;
  late final TextEditingController _descriptionController;
  late final TextEditingController _estimatedMinutesController;
  late TaskStatus _selectedStatus;
  late int _selectedPriority;
  late List<String> _tags;
  String? _selectedProjectId;
  bool _isModified = false;

  @override
  void initState() {
    super.initState();
    final task = widget.task;

    _titleController = TextEditingController(text: task?.title ?? '');
    _descriptionController =
        TextEditingController(text: task?.description ?? '');
    _estimatedMinutesController = TextEditingController(
      text: task?.estimatedMinutes?.toString() ?? '',
    );
    _selectedStatus = task?.status ?? TaskStatus.todo;
    _selectedPriority = task?.priority ?? 3;
    _tags = List.from(task?.tags ?? []);
    _selectedProjectId = task?.projectId;

    // Mark as modified when any field changes
    _titleController.addListener(_markModified);
    _descriptionController.addListener(_markModified);
    _estimatedMinutesController.addListener(_markModified);
  }

  void _markModified() {
    if (!_isModified) {
      setState(() => _isModified = true);
    }
  }

  @override
  void dispose() {
    _titleController.dispose();
    _descriptionController.dispose();
    _estimatedMinutesController.dispose();
    super.dispose();
  }

  bool _validateForm() {
    if (_titleController.text.trim().isEmpty) {
      _showError('Task title cannot be empty');
      return false;
    }
    return true;
  }

  void _showError(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: AltairColors.error,
      ),
    );
  }

  void _saveTask() {
    if (!_validateForm()) return;

    final now = DateTime.now();
    final task = widget.task?.copyWith(
          title: _titleController.text.trim(),
          description: _descriptionController.text.trim().isEmpty
              ? null
              : _descriptionController.text.trim(),
          status: _selectedStatus,
          priority: _selectedPriority,
          tags: _tags,
          projectId: _selectedProjectId,
          estimatedMinutes: _estimatedMinutesController.text.isEmpty
              ? null
              : int.tryParse(_estimatedMinutesController.text),
          updatedAt: now,
          completedAt: _selectedStatus == TaskStatus.completed ? now : null,
        ) ??
        Task(
          id: DateTime.now().millisecondsSinceEpoch.toString(),
          title: _titleController.text.trim(),
          description: _descriptionController.text.trim().isEmpty
              ? null
              : _descriptionController.text.trim(),
          status: _selectedStatus,
          priority: _selectedPriority,
          tags: _tags,
          projectId: _selectedProjectId,
          estimatedMinutes: _estimatedMinutesController.text.isEmpty
              ? null
              : int.tryParse(_estimatedMinutesController.text),
          createdAt: now,
          updatedAt: now,
        );

    if (widget.task == null) {
      context.read<TaskBloc>().add(TaskCreateRequested(task: task));
    } else {
      context.read<TaskBloc>().add(TaskUpdateRequested(task: task));
    }

    Navigator.of(context).pop();
  }

  Future<bool> _onWillPop() async {
    if (!_isModified) return true;

    final result = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Discard changes?'),
        content: const Text(
          'You have unsaved changes. Are you sure you want to discard them?',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(false),
            child: const Text('Cancel'),
          ),
          AltairButton(
            onPressed: () => Navigator.of(context).pop(true),
            variant: AltairButtonVariant.filled,
            accentColor: AltairColors.error,
            child: const Text('Discard'),
          ),
        ],
      ),
    );

    return result ?? false;
  }

  @override
  Widget build(BuildContext context) {
    final isNewTask = widget.task == null;

    return PopScope(
      canPop: false,
      onPopInvokedWithResult: (didPop, result) async {
        if (didPop) return;
        final shouldPop = await _onWillPop();
        if (shouldPop && context.mounted) {
          Navigator.of(context).pop();
        }
      },
      child: Scaffold(
        appBar: AppBar(
          title: Text(isNewTask ? 'New Task' : 'Edit Task'),
          actions: [
            TextButton(
              onPressed: _saveTask,
              child: Text(
                'Save',
                style: TextStyle(
                  color: Theme.of(context).colorScheme.primary,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
          ],
        ),
        body: SingleChildScrollView(
          padding: const EdgeInsets.all(AltairSpacing.lg),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Title field
              AltairTextField(
                controller: _titleController,
                hint: 'Title - What needs to be done?',
                autofocus: isNewTask,
                maxLines: 2,
              ),
              const SizedBox(height: AltairSpacing.lg),

              // Description field
              AltairTextField(
                controller: _descriptionController,
                label: 'Description',
                hint: 'Add more details...',
                maxLines: 5,
              ),
              const SizedBox(height: AltairSpacing.lg),

              // Status dropdown
              _buildStatusDropdown(),
              const SizedBox(height: AltairSpacing.lg),

              // Project dropdown
              _buildProjectDropdown(),
              const SizedBox(height: AltairSpacing.lg),

              // Priority slider
              _buildPrioritySlider(),
              const SizedBox(height: AltairSpacing.lg),

              // Estimated time
              AltairTextField(
                controller: _estimatedMinutesController,
                label: 'Estimated time (minutes)',
                hint: 'e.g., 30',
                keyboardType: TextInputType.number,
                inputFormatters: [
                  FilteringTextInputFormatter.digitsOnly,
                ],
              ),
              const SizedBox(height: AltairSpacing.lg),

              // Tags
              _buildTagsSection(),
              const SizedBox(height: AltairSpacing.xl),

              // AI Assistant Section - Rebuild when title or settings change
              BlocBuilder<SettingsBloc, SettingsState>(
                builder: (context, settingsState) => ListenableBuilder(
                  listenable: _titleController,
                  builder: (context, _) =>
                      _buildAIAssistantSection(settingsState),
                ),
              ),
              const SizedBox(height: AltairSpacing.xl),

              // Save button (also in app bar)
              AltairButton(
                onPressed: _saveTask,
                variant: AltairButtonVariant.filled,
                accentColor: AltairColors.accentGreen,
                child: Text(isNewTask ? 'Create Task' : 'Save Changes'),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildStatusDropdown() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Status',
          style: Theme.of(context).textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),
        InputDecorator(
          decoration: InputDecoration(
            border: OutlineInputBorder(
              borderSide: BorderSide(
                color: Theme.of(context).dividerColor,
                width: AltairBorders.medium,
              ),
            ),
            contentPadding: const EdgeInsets.symmetric(
              horizontal: AltairSpacing.md,
              vertical: AltairSpacing.sm,
            ),
          ),
          child: DropdownButtonHideUnderline(
            child: DropdownButton<TaskStatus>(
              value: _selectedStatus,
              isExpanded: true,
              items: TaskStatus.values.map((status) {
                return DropdownMenuItem(
                  value: status,
                  child: Row(
                    children: [
                      Container(
                        width: 16,
                        height: 16,
                        decoration: BoxDecoration(
                          color: _getStatusColor(status),
                          borderRadius: BorderRadius.circular(4),
                          border: Border.all(
                            color: Colors.black,
                            width: 2,
                          ),
                        ),
                      ),
                      const SizedBox(width: AltairSpacing.sm),
                      Text(_getStatusLabel(status)),
                    ],
                  ),
                );
              }).toList(),
              onChanged: (value) {
                if (value != null) {
                  setState(() {
                    _selectedStatus = value;
                    _markModified();
                  });
                }
              },
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildProjectDropdown() {
    return BlocBuilder<ProjectBloc, ProjectState>(
      builder: (context, state) {
        final projects = state is ProjectLoaded ? state.projects : <Project>[];

        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Project',
              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: AltairSpacing.sm),
            InputDecorator(
              decoration: InputDecoration(
                border: OutlineInputBorder(
                  borderSide: BorderSide(
                    color: Theme.of(context).dividerColor,
                    width: AltairBorders.medium,
                  ),
                ),
                contentPadding: const EdgeInsets.symmetric(
                  horizontal: AltairSpacing.md,
                  vertical: AltairSpacing.sm,
                ),
              ),
              child: DropdownButtonHideUnderline(
                child: DropdownButton<String?>(
                  value: _selectedProjectId,
                  isExpanded: true,
                  hint: const Text('No project (personal task)'),
                  items: [
                    // None option
                    const DropdownMenuItem<String?>(
                      value: null,
                      child: Row(
                        children: [
                          Icon(Icons.clear, size: 16),
                          SizedBox(width: AltairSpacing.sm),
                          Text('No project (personal task)'),
                        ],
                      ),
                    ),
                    // Project options
                    ...projects.map((project) {
                      return DropdownMenuItem<String?>(
                        value: project.id,
                        child: Row(
                          children: [
                            Container(
                              width: 16,
                              height: 16,
                              decoration: BoxDecoration(
                                color: project.color != null
                                    ? _hexToColor(project.color!)
                                    : AltairColors.accentBlue,
                                borderRadius: BorderRadius.circular(4),
                                border: Border.all(
                                  color: Colors.black,
                                  width: 2,
                                ),
                              ),
                            ),
                            const SizedBox(width: AltairSpacing.sm),
                            Flexible(
                              child: Text(
                                project.name,
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                          ],
                        ),
                      );
                    }),
                  ],
                  onChanged: (value) {
                    setState(() {
                      _selectedProjectId = value;
                      _markModified();
                    });
                  },
                ),
              ),
            ),
          ],
        );
      },
    );
  }

  Widget _buildPrioritySlider() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              'Priority',
              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            Text(
              _getPriorityLabel(_selectedPriority),
              style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                    color: _getPriorityColor(_selectedPriority),
                    fontWeight: FontWeight.bold,
                  ),
            ),
          ],
        ),
        Slider(
          value: _selectedPriority.toDouble(),
          min: 1,
          max: 5,
          divisions: 4,
          label: _getPriorityLabel(_selectedPriority),
          onChanged: (value) {
            setState(() {
              _selectedPriority = value.round();
              _markModified();
            });
          },
        ),
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text('Highest', style: Theme.of(context).textTheme.bodySmall),
            Text('Lowest', style: Theme.of(context).textTheme.bodySmall),
          ],
        ),
      ],
    );
  }

  Widget _buildTagsSection() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Tags',
          style: Theme.of(context).textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),
        if (_tags.isEmpty)
          Text(
            'No tags added',
            style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  color: AltairColors.textSecondary,
                ),
          )
        else
          Wrap(
            spacing: AltairSpacing.sm,
            runSpacing: AltairSpacing.sm,
            children: _tags.map((tag) {
              return Chip(
                label: Text(tag),
                deleteIcon: const Icon(Icons.close, size: 18),
                onDeleted: () {
                  setState(() {
                    _tags.remove(tag);
                    _markModified();
                  });
                },
                backgroundColor:
                    AltairColors.accentOrange.withValues(alpha: 0.2),
                side: const BorderSide(
                  color: Colors.black,
                  width: AltairBorders.thin,
                ),
              );
            }).toList(),
          ),
        const SizedBox(height: AltairSpacing.sm),
        AltairButton(
          onPressed: _showAddTagDialog,
          variant: AltairButtonVariant.outlined,
          accentColor: AltairColors.accentOrange,
          child: const Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(Icons.add, size: 18),
              SizedBox(width: AltairSpacing.xs),
              Text('Add Tag'),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildAIAssistantSection(SettingsState settingsState) {
    final hasTitle = _titleController.text.trim().isNotEmpty;
    final settingsLoading = settingsState is SettingsLoading;
    final settingsLoaded = settingsState is SettingsLoaded;
    final aiEnabled = settingsLoaded && settingsState.aiSettings.enabled;

    // Determine if AI features should be enabled
    final aiAvailable = hasTitle && settingsLoaded && aiEnabled;

    // Determine status message
    String? statusMessage;
    if (!hasTitle) {
      statusMessage = 'Add a task title to use AI features';
    } else if (settingsLoading) {
      statusMessage = 'Loading AI settings...';
    } else if (!aiEnabled) {
      statusMessage = 'AI features disabled. Enable in Settings to use.';
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            const Icon(Icons.auto_awesome, size: 24),
            const SizedBox(width: AltairSpacing.sm),
            Text(
              'AI Assistant',
              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            if (settingsLoading) ...[
              const SizedBox(width: AltairSpacing.sm),
              const SizedBox(
                width: 16,
                height: 16,
                child: CircularProgressIndicator(strokeWidth: 2),
              ),
            ],
          ],
        ),
        const SizedBox(height: AltairSpacing.sm),
        Text(
          'Get AI-powered suggestions to help with your task',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                color: AltairColors.textSecondary,
              ),
        ),
        const SizedBox(height: AltairSpacing.md),
        Wrap(
          spacing: AltairSpacing.sm,
          runSpacing: AltairSpacing.sm,
          children: [
            // Task Breakdown
            _AIFeatureButton(
              icon: Icons.format_list_bulleted,
              label: 'Break Down Task',
              accentColor: AltairColors.accentBlue,
              enabled: aiAvailable,
              onPressed: () => _showAIFeature(() {
                showTaskBreakdownDialog(
                  context,
                  taskTitle: _titleController.text.trim(),
                  taskDescription: _descriptionController.text.trim().isEmpty
                      ? null
                      : _descriptionController.text.trim(),
                  parentTaskId: widget.task?.id,
                );
              }),
            ),
            // Time Estimate
            _AIFeatureButton(
              icon: Icons.timer,
              label: 'Estimate Time',
              accentColor: AltairColors.accentGreen,
              enabled: aiAvailable,
              onPressed: () => _showAIFeature(() {
                showTimeEstimateDialog(
                  context,
                  taskTitle: _titleController.text.trim(),
                  taskDescription: _descriptionController.text.trim().isEmpty
                      ? null
                      : _descriptionController.text.trim(),
                );
              }),
            ),
            // Context Suggestions
            _AIFeatureButton(
              icon: Icons.lightbulb,
              label: 'Get Suggestions',
              accentColor: AltairColors.accentOrange,
              enabled: aiAvailable,
              onPressed: () => _showAIFeature(() {
                showContextSuggestionsDialog(
                  context,
                  taskTitle: _titleController.text.trim(),
                  taskDescription: _descriptionController.text.trim().isEmpty
                      ? null
                      : _descriptionController.text.trim(),
                );
              }),
            ),
          ],
        ),
        if (statusMessage != null) ...[
          const SizedBox(height: AltairSpacing.sm),
          Text(
            statusMessage,
            style: Theme.of(context).textTheme.bodySmall?.copyWith(
                  color: settingsLoading
                      ? AltairColors.accentBlue
                      : AltairColors.error,
                  fontStyle: FontStyle.italic,
                ),
          ),
        ],
      ],
    );
  }

  Future<void> _showAIFeature(VoidCallback showDialog) async {
    if (!mounted) return;

    try {
      // Check if settings are loaded before allowing AI features
      final settingsState = context.read<SettingsBloc>().state;
      if (settingsState is! SettingsLoaded) {
        _showError(
          'Settings are still loading. Please wait a moment and try again.',
        );
        return;
      }

      // Check if AI is enabled
      if (!settingsState.aiSettings.enabled) {
        _showError(
          'AI features are disabled. Please configure an AI provider in Settings.',
        );
        return;
      }

      final hasConsent = await showAIConsentDialog(context);
      if (!hasConsent || !mounted) return;

      showDialog();
    } catch (e) {
      if (mounted) {
        _showError('Failed to show AI feature: $e');
      }
    }
  }

  void _showAddTagDialog() {
    final controller = TextEditingController();

    showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Add Tag'),
        content: TextField(
          controller: controller,
          autofocus: true,
          decoration: const InputDecoration(
            hintText: 'Tag name',
          ),
          onSubmitted: (value) {
            if (value.trim().isNotEmpty && !_tags.contains(value.trim())) {
              setState(() {
                _tags.add(value.trim());
                _markModified();
              });
            }
            Navigator.of(context).pop();
          },
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          AltairButton(
            onPressed: () {
              final tag = controller.text.trim();
              if (tag.isNotEmpty && !_tags.contains(tag)) {
                setState(() {
                  _tags.add(tag);
                  _markModified();
                });
              }
              Navigator.of(context).pop();
            },
            variant: AltairButtonVariant.filled,
            accentColor: AltairColors.accentOrange,
            child: const Text('Add'),
          ),
        ],
      ),
    );
  }

  Color _getStatusColor(TaskStatus status) {
    return switch (status) {
      TaskStatus.todo => AltairColors.accentOrange,
      TaskStatus.inProgress => AltairColors.accentBlue,
      TaskStatus.completed => AltairColors.accentGreen,
      TaskStatus.cancelled => AltairColors.textSecondary,
    };
  }

  String _getStatusLabel(TaskStatus status) {
    return switch (status) {
      TaskStatus.todo => 'To Do',
      TaskStatus.inProgress => 'In Progress',
      TaskStatus.completed => 'Completed',
      TaskStatus.cancelled => 'Cancelled',
    };
  }

  String _getPriorityLabel(int priority) {
    return switch (priority) {
      1 => 'Critical',
      2 => 'High',
      3 => 'Medium',
      4 => 'Low',
      5 => 'Lowest',
      _ => 'Medium',
    };
  }

  Color _getPriorityColor(int priority) {
    return switch (priority) {
      1 => AltairColors.error,
      2 => const Color(0xFFFF6B6B),
      3 => AltairColors.accentOrange,
      4 => AltairColors.accentBlue,
      5 => AltairColors.textSecondary,
      _ => AltairColors.accentOrange,
    };
  }

  Color _hexToColor(String hex) {
    final hexColor = hex.replaceAll('#', '');
    return Color(int.parse('FF$hexColor', radix: 16));
  }
}

/// Button for AI features.
class _AIFeatureButton extends StatelessWidget {
  const _AIFeatureButton({
    required this.icon,
    required this.label,
    required this.accentColor,
    required this.enabled,
    required this.onPressed,
  });

  final IconData icon;
  final String label;
  final Color accentColor;
  final bool enabled;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return AltairButton(
      onPressed: enabled ? onPressed : null,
      variant: AltairButtonVariant.outlined,
      accentColor: accentColor,
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 18),
          const SizedBox(width: AltairSpacing.xs),
          Text(label),
        ],
      ),
    );
  }
}
