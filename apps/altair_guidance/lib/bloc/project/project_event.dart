/// Project events for the project bloc.
library;

import 'package:altair_core/altair_core.dart';
import 'package:equatable/equatable.dart';

/// Base class for project events.
sealed class ProjectEvent extends Equatable {
  const ProjectEvent();

  @override
  List<Object?> get props => [];
}

/// Event to load all projects.
final class ProjectLoadRequested extends ProjectEvent {
  const ProjectLoadRequested();
}

/// Event to create a new project.
final class ProjectCreateRequested extends ProjectEvent {
  const ProjectCreateRequested({required this.project});

  final Project project;

  @override
  List<Object?> get props => [project];
}

/// Event to update an existing project.
final class ProjectUpdateRequested extends ProjectEvent {
  const ProjectUpdateRequested({required this.project});

  final Project project;

  @override
  List<Object?> get props => [project];
}

/// Event to delete a project.
final class ProjectDeleteRequested extends ProjectEvent {
  const ProjectDeleteRequested({required this.projectId});

  final String projectId;

  @override
  List<Object?> get props => [projectId];
}

/// Event to search projects.
final class ProjectSearchRequested extends ProjectEvent {
  const ProjectSearchRequested({required this.query});

  final String query;

  @override
  List<Object?> get props => [query];
}

/// Event to filter projects by status.
final class ProjectFilterByStatusRequested extends ProjectEvent {
  const ProjectFilterByStatusRequested({required this.status});

  final ProjectStatus status;

  @override
  List<Object?> get props => [status];
}

/// Event to filter projects by tags.
final class ProjectFilterByTagsRequested extends ProjectEvent {
  const ProjectFilterByTagsRequested({required this.tags});

  final List<String> tags;

  @override
  List<Object?> get props => [tags];
}

/// Event to clear all filters.
final class ProjectClearFiltersRequested extends ProjectEvent {
  const ProjectClearFiltersRequested();
}
