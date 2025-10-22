/// Projects page showing list of all projects.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../bloc/project/project_bloc.dart';
import '../bloc/project/project_event.dart';
import '../bloc/project/project_state.dart';
import 'project_edit_page.dart';

/// Page displaying all projects with create/edit functionality.
class ProjectsPage extends StatelessWidget {
  /// Creates a projects page.
  const ProjectsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Projects'),
        actions: [
          IconButton(
            icon: const Icon(Icons.filter_list),
            onPressed: () {
              _showFilterMenu(context);
            },
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // Capture bloc from current context before navigation
          final projectBloc = context.read<ProjectBloc>();

          Navigator.of(context).push(
            MaterialPageRoute<void>(
              builder: (context) => BlocProvider.value(
                value: projectBloc,
                child: const ProjectEditPage(),
              ),
            ),
          );
        },
        backgroundColor: AltairColors.accentGreen,
        child: const Icon(Icons.add, color: Colors.black),
      ),
      body: BlocConsumer<ProjectBloc, ProjectState>(
        listener: (context, state) {
          if (state is ProjectCreated) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text('Project created: "${state.project.name}"'),
                duration: const Duration(seconds: 2),
                backgroundColor: AltairColors.accentGreen,
              ),
            );
          } else if (state is ProjectFailure) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(state.message),
                backgroundColor: AltairColors.error,
              ),
            );
          }
        },
        builder: (context, state) {
          if (state is ProjectLoading) {
            return const Center(
              child: CircularProgressIndicator(),
            );
          }

          if (state is ProjectLoaded) {
            if (state.projects.isEmpty) {
              return Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.folder_open,
                      size: 64,
                      color: AltairColors.textSecondary,
                    ),
                    const SizedBox(height: AltairSpacing.md),
                    Text(
                      'No projects yet',
                      style: Theme.of(context).textTheme.headlineSmall,
                    ),
                    const SizedBox(height: AltairSpacing.sm),
                    Text(
                      'Tap + to create your first project',
                      style: Theme.of(context).textTheme.bodyMedium,
                      textAlign: TextAlign.center,
                    ),
                  ],
                ),
              );
            }

            return ListView.builder(
              padding: const EdgeInsets.all(AltairSpacing.md),
              itemCount: state.projects.length,
              itemBuilder: (context, index) {
                final project = state.projects[index];
                return Padding(
                  padding: const EdgeInsets.only(
                    bottom: AltairSpacing.md,
                  ),
                  child: _ProjectListItem(project: project),
                );
              },
            );
          }

          // Initial state
          return Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  'Projects',
                  style: Theme.of(context).textTheme.displayMedium,
                ),
                const SizedBox(height: AltairSpacing.md),
                AltairButton(
                  onPressed: () {
                    context
                        .read<ProjectBloc>()
                        .add(const ProjectLoadRequested());
                  },
                  variant: AltairButtonVariant.filled,
                  accentColor: AltairColors.accentBlue,
                  child: const Text('Load Projects'),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  void _showFilterMenu(BuildContext context) {
    showModalBottomSheet<void>(
      context: context,
      builder: (context) => Container(
        padding: const EdgeInsets.all(AltairSpacing.lg),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'Filter Projects',
              style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: AltairSpacing.md),
            ListTile(
              leading: const Icon(Icons.all_inclusive),
              title: const Text('All Projects'),
              onTap: () {
                context
                    .read<ProjectBloc>()
                    .add(const ProjectClearFiltersRequested());
                Navigator.pop(context);
              },
            ),
            ListTile(
              leading: Container(
                width: 24,
                height: 24,
                decoration: BoxDecoration(
                  color: AltairColors.accentGreen,
                  border: Border.all(color: Colors.black, width: 2),
                  borderRadius: BorderRadius.circular(4),
                ),
              ),
              title: const Text('Active'),
              onTap: () {
                context.read<ProjectBloc>().add(
                      const ProjectFilterByStatusRequested(
                        status: ProjectStatus.active,
                      ),
                    );
                Navigator.pop(context);
              },
            ),
            ListTile(
              leading: Container(
                width: 24,
                height: 24,
                decoration: BoxDecoration(
                  color: AltairColors.accentYellow,
                  border: Border.all(color: Colors.black, width: 2),
                  borderRadius: BorderRadius.circular(4),
                ),
              ),
              title: const Text('On Hold'),
              onTap: () {
                context.read<ProjectBloc>().add(
                      const ProjectFilterByStatusRequested(
                        status: ProjectStatus.onHold,
                      ),
                    );
                Navigator.pop(context);
              },
            ),
            ListTile(
              leading: Container(
                width: 24,
                height: 24,
                decoration: BoxDecoration(
                  color: AltairColors.accentBlue,
                  border: Border.all(color: Colors.black, width: 2),
                  borderRadius: BorderRadius.circular(4),
                ),
              ),
              title: const Text('Completed'),
              onTap: () {
                context.read<ProjectBloc>().add(
                      const ProjectFilterByStatusRequested(
                        status: ProjectStatus.completed,
                      ),
                    );
                Navigator.pop(context);
              },
            ),
            ListTile(
              leading: Container(
                width: 24,
                height: 24,
                decoration: BoxDecoration(
                  color: AltairColors.textSecondary,
                  border: Border.all(color: Colors.black, width: 2),
                  borderRadius: BorderRadius.circular(4),
                ),
              ),
              title: const Text('Cancelled'),
              onTap: () {
                context.read<ProjectBloc>().add(
                      const ProjectFilterByStatusRequested(
                        status: ProjectStatus.cancelled,
                      ),
                    );
                Navigator.pop(context);
              },
            ),
          ],
        ),
      ),
    );
  }
}

/// Widget to display a project in the list.
class _ProjectListItem extends StatelessWidget {
  const _ProjectListItem({required this.project});

  final Project project;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {
        Navigator.of(context).push(
          MaterialPageRoute<void>(
            builder: (context) => BlocProvider.value(
              value: context.read<ProjectBloc>(),
              child: ProjectEditPage(project: project),
            ),
          ),
        );
      },
      child: AltairCard(
        accentColor: _getProjectColor(),
        showAccentBar: true,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    project.name,
                    style: Theme.of(context).textTheme.titleLarge?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                  ),
                ),
                // Status badge
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: AltairSpacing.sm,
                    vertical: AltairSpacing.xs,
                  ),
                  decoration: BoxDecoration(
                    color: _getStatusColor(project.status),
                    border: Border.all(
                      color: Colors.black,
                      width: AltairBorders.thin,
                    ),
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    _getStatusLabel(project.status),
                    style: Theme.of(context).textTheme.bodySmall?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                  ),
                ),
                const SizedBox(width: AltairSpacing.sm),
                // Delete button
                IconButton(
                  icon: const Icon(Icons.delete_outline),
                  onPressed: () {
                    _showDeleteConfirmation(context);
                  },
                  color: AltairColors.error,
                ),
              ],
            ),
            if (project.description != null) ...[
              const SizedBox(height: AltairSpacing.sm),
              Text(
                project.description!,
                style: Theme.of(context).textTheme.bodyMedium,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
            ],
            if (project.targetDate != null) ...[
              const SizedBox(height: AltairSpacing.sm),
              Row(
                children: [
                  const Icon(Icons.calendar_today, size: 16),
                  const SizedBox(width: AltairSpacing.xs),
                  Text(
                    'Target: ${project.targetDate!.year}-${project.targetDate!.month.toString().padLeft(2, '0')}-${project.targetDate!.day.toString().padLeft(2, '0')}',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ],
              ),
            ],
            if (project.tags.isNotEmpty) ...[
              const SizedBox(height: AltairSpacing.sm),
              Wrap(
                spacing: AltairSpacing.xs,
                runSpacing: AltairSpacing.xs,
                children: project.tags.map((tag) {
                  return Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: AltairSpacing.sm,
                      vertical: AltairSpacing.xs,
                    ),
                    decoration: BoxDecoration(
                      color: AltairColors.accentBlue.withValues(alpha: 0.2),
                      border: Border.all(
                        color: Colors.black,
                        width: AltairBorders.thin,
                      ),
                      borderRadius: BorderRadius.circular(4),
                    ),
                    child: Text(
                      tag,
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                  );
                }).toList(),
              ),
            ],
          ],
        ),
      ),
    );
  }

  void _showDeleteConfirmation(BuildContext context) {
    showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Project?'),
        content: Text(
          'Are you sure you want to delete "${project.name}"? This will also delete all tasks in this project.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          AltairButton(
            onPressed: () {
              context.read<ProjectBloc>().add(
                    ProjectDeleteRequested(projectId: project.id),
                  );
              Navigator.of(context).pop();
            },
            variant: AltairButtonVariant.filled,
            accentColor: AltairColors.error,
            child: const Text('Delete'),
          ),
        ],
      ),
    );
  }

  Color _getProjectColor() {
    if (project.color != null) {
      final hexColor = project.color!.replaceAll('#', '');
      return Color(int.parse('FF$hexColor', radix: 16));
    }
    return AltairColors.accentBlue;
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
}
