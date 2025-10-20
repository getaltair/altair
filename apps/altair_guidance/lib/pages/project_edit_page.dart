/// Project editing page for creating and managing projects.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../bloc/project/project_bloc.dart';
import '../bloc/project/project_event.dart';

/// Page for creating or editing a project with full details.
class ProjectEditPage extends StatefulWidget {
  /// Creates a project edit page.
  const ProjectEditPage({
    super.key,
    this.project,
  });

  /// The project to edit. If null, creates a new project.
  final Project? project;

  @override
  State<ProjectEditPage> createState() => _ProjectEditPageState();
}

class _ProjectEditPageState extends State<ProjectEditPage> {
  late final TextEditingController _nameController;
  late final TextEditingController _descriptionController;
  late ProjectStatus _selectedStatus;
  late List<String> _tags;
  String? _selectedColor;
  DateTime? _targetDate;
  bool _isModified = false;

  // Predefined project colors
  static const List<Color> projectColors = [
    AltairColors.accentYellow,
    AltairColors.accentBlue,
    AltairColors.accentGreen,
    Color(0xFFFF6B6B), // Red
    Color(0xFFB565D8), // Purple
    Color(0xFFFF8C42), // Orange
    Color(0xFF4ECDC4), // Teal
    Color(0xFFF7B731), // Gold
  ];

  @override
  void initState() {
    super.initState();
    final project = widget.project;

    _nameController = TextEditingController(text: project?.name ?? '');
    _descriptionController = TextEditingController(text: project?.description ?? '');
    _selectedStatus = project?.status ?? ProjectStatus.active;
    _tags = List.from(project?.tags ?? []);
    _selectedColor = project?.color;
    _targetDate = project?.targetDate;

    // Mark as modified when any field changes
    _nameController.addListener(_markModified);
    _descriptionController.addListener(_markModified);
  }

  void _markModified() {
    if (!_isModified) {
      setState(() => _isModified = true);
    }
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }

  bool _validateForm() {
    if (_nameController.text.trim().isEmpty) {
      _showError('Project name cannot be empty');
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

  void _saveProject() {
    if (!_validateForm()) return;

    final now = DateTime.now();
    final project = widget.project?.copyWith(
      name: _nameController.text.trim(),
      description: _descriptionController.text.trim().isEmpty
          ? null
          : _descriptionController.text.trim(),
      status: _selectedStatus,
      tags: _tags,
      color: _selectedColor,
      targetDate: _targetDate,
      updatedAt: now,
      completedAt: _selectedStatus == ProjectStatus.completed ? now : null,
    ) ?? Project(
      id: '',
      name: _nameController.text.trim(),
      description: _descriptionController.text.trim().isEmpty
          ? null
          : _descriptionController.text.trim(),
      status: _selectedStatus,
      tags: _tags,
      color: _selectedColor,
      targetDate: _targetDate,
      createdAt: now,
      updatedAt: now,
    );

    if (widget.project == null) {
      context.read<ProjectBloc>().add(ProjectCreateRequested(project: project));
    } else {
      context.read<ProjectBloc>().add(ProjectUpdateRequested(project: project));
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
    final isNewProject = widget.project == null;

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
          title: Text(isNewProject ? 'New Project' : 'Edit Project'),
          actions: [
            TextButton(
              onPressed: _saveProject,
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
              // Name field
              AltairTextField(
                controller: _nameController,
                label: 'Project Name',
                hint: 'e.g., Altair Development, Personal Goals',
                autofocus: isNewProject,
                maxLines: 1,
              ),
              const SizedBox(height: AltairSpacing.lg),

              // Description field
              AltairTextField(
                controller: _descriptionController,
                label: 'Description',
                hint: 'What is this project about?',
                maxLines: 4,
              ),
              const SizedBox(height: AltairSpacing.lg),

              // Status dropdown
              _buildStatusDropdown(),
              const SizedBox(height: AltairSpacing.lg),

              // Color picker
              _buildColorPicker(),
              const SizedBox(height: AltairSpacing.lg),

              // Target date
              _buildTargetDatePicker(),
              const SizedBox(height: AltairSpacing.lg),

              // Tags
              _buildTagsSection(),
              const SizedBox(height: AltairSpacing.xl),

              // Save button (also in app bar)
              AltairButton(
                onPressed: _saveProject,
                variant: AltairButtonVariant.filled,
                accentColor: AltairColors.accentGreen,
                child: Text(isNewProject ? 'Create Project' : 'Save Changes'),
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
            child: DropdownButton<ProjectStatus>(
              value: _selectedStatus,
              isExpanded: true,
              items: ProjectStatus.values.map((status) {
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

  Widget _buildColorPicker() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Project Color',
          style: Theme.of(context).textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),
        Wrap(
          spacing: AltairSpacing.sm,
          runSpacing: AltairSpacing.sm,
          children: [
            // No color option
            _buildColorOption(null),
            // Predefined colors
            ...projectColors.map((color) => _buildColorOption(_colorToHex(color))),
          ],
        ),
      ],
    );
  }

  Widget _buildColorOption(String? colorHex) {
    final isSelected = _selectedColor == colorHex;
    final color = colorHex != null ? _hexToColor(colorHex) : Colors.grey;

    return InkWell(
      onTap: () {
        setState(() {
          _selectedColor = colorHex;
          _markModified();
        });
      },
      child: Container(
        width: 48,
        height: 48,
        decoration: BoxDecoration(
          color: colorHex != null ? color : Colors.transparent,
          border: Border.all(
            color: isSelected ? Colors.black : Colors.grey,
            width: isSelected ? 3 : 2,
          ),
          borderRadius: BorderRadius.circular(8),
        ),
        child: colorHex == null
            ? const Icon(Icons.block, color: Colors.grey)
            : isSelected
                ? const Icon(Icons.check, color: Colors.black)
                : null,
      ),
    );
  }

  Widget _buildTargetDatePicker() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Target Completion Date',
          style: Theme.of(context).textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.bold,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),
        InkWell(
          onTap: () async {
            final date = await showDatePicker(
              context: context,
              initialDate: _targetDate ?? DateTime.now(),
              firstDate: DateTime.now(),
              lastDate: DateTime.now().add(const Duration(days: 3650)), // ~10 years
            );
            if (date != null) {
              setState(() {
                _targetDate = date;
                _markModified();
              });
            }
          },
          child: Container(
            padding: const EdgeInsets.all(AltairSpacing.md),
            decoration: BoxDecoration(
              border: Border.all(
                color: Theme.of(context).dividerColor,
                width: AltairBorders.medium,
              ),
            ),
            child: Row(
              children: [
                const Icon(Icons.calendar_today),
                const SizedBox(width: AltairSpacing.sm),
                Text(
                  _targetDate != null
                      ? '${_targetDate!.year}-${_targetDate!.month.toString().padLeft(2, '0')}-${_targetDate!.day.toString().padLeft(2, '0')}'
                      : 'No target date',
                  style: Theme.of(context).textTheme.bodyLarge,
                ),
                const Spacer(),
                if (_targetDate != null)
                  IconButton(
                    icon: const Icon(Icons.clear),
                    onPressed: () {
                      setState(() {
                        _targetDate = null;
                        _markModified();
                      });
                    },
                  ),
              ],
            ),
          ),
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
                backgroundColor: AltairColors.accentBlue.withValues(alpha: 0.2),
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
          accentColor: AltairColors.accentBlue,
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
            accentColor: AltairColors.accentBlue,
            child: const Text('Add'),
          ),
        ],
      ),
    );
  }

  Color _getStatusColor(ProjectStatus status) {
    return switch (status) {
      ProjectStatus.active => AltairColors.accentGreen,
      ProjectStatus.onHold => AltairColors.accentYellow,
      ProjectStatus.completed => AltairColors.accentBlue,
      ProjectStatus.cancelled => AltairColors.textSecondary,
    };
  }

  String _getStatusLabel(ProjectStatus status) {
    return switch (status) {
      ProjectStatus.active => 'Active',
      ProjectStatus.onHold => 'On Hold',
      ProjectStatus.completed => 'Completed',
      ProjectStatus.cancelled => 'Cancelled',
    };
  }

  String _colorToHex(Color color) {
    return '#${color.toARGB32().toRadixString(16).substring(2).toUpperCase()}';
  }

  Color _hexToColor(String hex) {
    final hexColor = hex.replaceAll('#', '');
    return Color(int.parse('FF$hexColor', radix: 16));
  }
}
