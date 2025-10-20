/// Task events for the task bloc.
library;

import 'package:altair_core/altair_core.dart';
import 'package:equatable/equatable.dart';

/// Base class for task events.
sealed class TaskEvent extends Equatable {
  const TaskEvent();

  @override
  List<Object?> get props => [];
}

/// Event to load all tasks.
final class TaskLoadRequested extends TaskEvent {
  const TaskLoadRequested();
}

/// Event to create a new task quickly.
final class TaskQuickCaptureRequested extends TaskEvent {
  const TaskQuickCaptureRequested({required this.title});

  final String title;

  @override
  List<Object?> get props => [title];
}

/// Event to create a new task with full details.
final class TaskCreateRequested extends TaskEvent {
  const TaskCreateRequested({required this.task});

  final Task task;

  @override
  List<Object?> get props => [task];
}

/// Event to update an existing task.
final class TaskUpdateRequested extends TaskEvent {
  const TaskUpdateRequested({required this.task});

  final Task task;

  @override
  List<Object?> get props => [task];
}

/// Event to delete a task.
final class TaskDeleteRequested extends TaskEvent {
  const TaskDeleteRequested({required this.taskId});

  final String taskId;

  @override
  List<Object?> get props => [taskId];
}

/// Event to search tasks.
final class TaskSearchRequested extends TaskEvent {
  const TaskSearchRequested({required this.query});

  final String query;

  @override
  List<Object?> get props => [query];
}

/// Event to filter tasks by status.
final class TaskFilterByStatusRequested extends TaskEvent {
  const TaskFilterByStatusRequested({required this.status});

  final TaskStatus status;

  @override
  List<Object?> get props => [status];
}

/// Event to filter tasks by tags.
final class TaskFilterByTagsRequested extends TaskEvent {
  const TaskFilterByTagsRequested({required this.tags});

  final List<String> tags;

  @override
  List<Object?> get props => [tags];
}

/// Event to clear all filters.
final class TaskClearFiltersRequested extends TaskEvent {
  const TaskClearFiltersRequested();
}
