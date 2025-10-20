/// Project states for the project bloc.
library;

import 'package:altair_core/altair_core.dart';
import 'package:equatable/equatable.dart';

/// Base class for project states.
sealed class ProjectState extends Equatable {
  const ProjectState();

  @override
  List<Object?> get props => [];
}

/// Initial state before any projects are loaded.
final class ProjectInitial extends ProjectState {
  const ProjectInitial();
}

/// State when projects are being loaded.
final class ProjectLoading extends ProjectState {
  const ProjectLoading();
}

/// State when projects are successfully loaded.
final class ProjectLoaded extends ProjectState {
  const ProjectLoaded({
    required this.projects,
    this.filter,
  });

  final List<Project> projects;
  final ProjectStatus? filter;

  @override
  List<Object?> get props => [projects, filter];
}

/// State when a project operation fails.
final class ProjectFailure extends ProjectState {
  const ProjectFailure({required this.message});

  final String message;

  @override
  List<Object?> get props => [message];
}

/// State when a project is successfully created.
final class ProjectCreated extends ProjectState {
  const ProjectCreated({required this.project});

  final Project project;

  @override
  List<Object?> get props => [project];
}
