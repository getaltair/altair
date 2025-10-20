/// Project BLoC for managing project state.
library;

import 'package:altair_core/altair_core.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';

import 'project_event.dart';
import 'project_state.dart';

/// BLoC for managing project state and operations.
class ProjectBloc extends Bloc<ProjectEvent, ProjectState> {
  /// Creates a project bloc.
  ProjectBloc({
    required ProjectRepository projectRepository,
    Logger? logger,
  })  : _projectRepository = projectRepository,
        _logger = logger ?? Logger(),
        super(const ProjectInitial()) {
    on<ProjectLoadRequested>(_onLoadRequested);
    on<ProjectCreateRequested>(_onCreateRequested);
    on<ProjectUpdateRequested>(_onUpdateRequested);
    on<ProjectDeleteRequested>(_onDeleteRequested);
    on<ProjectSearchRequested>(_onSearchRequested);
    on<ProjectFilterByStatusRequested>(_onFilterByStatusRequested);
    on<ProjectFilterByTagsRequested>(_onFilterByTagsRequested);
    on<ProjectClearFiltersRequested>(_onClearFiltersRequested);
  }

  final ProjectRepository _projectRepository;
  final Logger _logger;

  /// Handles loading all projects.
  Future<void> _onLoadRequested(
    ProjectLoadRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      final projects = await _projectRepository.findAll();
      emit(ProjectLoaded(projects: projects));
      _logger.i('Loaded ${projects.length} projects');
    } catch (e, stackTrace) {
      _logger.e('Failed to load projects', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }

  /// Handles creating a new project.
  Future<void> _onCreateRequested(
    ProjectCreateRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      final createdProject = await _projectRepository.create(event.project);
      _logger.i('Created project: ${createdProject.id}');

      // Emit success state briefly
      emit(ProjectCreated(project: createdProject));

      // Then reload all projects
      final projects = await _projectRepository.findAll();
      emit(ProjectLoaded(projects: projects));
    } catch (e, stackTrace) {
      _logger.e('Failed to create project', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }

  /// Handles updating an existing project.
  Future<void> _onUpdateRequested(
    ProjectUpdateRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      await _projectRepository.update(event.project);
      _logger.i('Updated project: ${event.project.id}');

      final projects = await _projectRepository.findAll();
      emit(ProjectLoaded(projects: projects));
    } catch (e, stackTrace) {
      _logger.e('Failed to update project', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }

  /// Handles deleting a project.
  Future<void> _onDeleteRequested(
    ProjectDeleteRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      await _projectRepository.delete(event.projectId);
      _logger.i('Deleted project: ${event.projectId}');

      final projects = await _projectRepository.findAll();
      emit(ProjectLoaded(projects: projects));
    } catch (e, stackTrace) {
      _logger.e('Failed to delete project', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }

  /// Handles searching projects.
  Future<void> _onSearchRequested(
    ProjectSearchRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      final projects = await _projectRepository.search(event.query);
      emit(ProjectLoaded(projects: projects));
      _logger.i('Search returned ${projects.length} projects');
    } catch (e, stackTrace) {
      _logger.e('Failed to search projects', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }

  /// Handles filtering projects by status.
  Future<void> _onFilterByStatusRequested(
    ProjectFilterByStatusRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      final projects = await _projectRepository.findAll(status: event.status);
      emit(ProjectLoaded(projects: projects, filter: event.status));
      _logger.i('Filtered ${projects.length} projects by status: ${event.status}');
    } catch (e, stackTrace) {
      _logger.e('Failed to filter projects', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }

  /// Handles filtering projects by tags.
  Future<void> _onFilterByTagsRequested(
    ProjectFilterByTagsRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      final projects = await _projectRepository.findAll(tags: event.tags);
      emit(ProjectLoaded(projects: projects, tagFilter: event.tags));
      _logger.i('Filtered ${projects.length} projects by tags: ${event.tags}');
    } catch (e, stackTrace) {
      _logger.e('Failed to filter projects by tags', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }

  /// Handles clearing all filters.
  Future<void> _onClearFiltersRequested(
    ProjectClearFiltersRequested event,
    Emitter<ProjectState> emit,
  ) async {
    emit(const ProjectLoading());

    try {
      final projects = await _projectRepository.findAll();
      emit(ProjectLoaded(projects: projects));
      _logger.i('Cleared filters, showing all ${projects.length} projects');
    } catch (e, stackTrace) {
      _logger.e('Failed to clear filters', error: e, stackTrace: stackTrace);
      emit(ProjectFailure(message: e.toString()));
    }
  }
}
