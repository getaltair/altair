/// Task states for the task bloc.
library;

import 'package:altair_core/altair_core.dart';
import 'package:equatable/equatable.dart';

/// Base class for task states.
sealed class TaskState extends Equatable {
  const TaskState();

  @override
  List<Object?> get props => [];
}

/// Initial state when the bloc is first created.
final class TaskInitial extends TaskState {
  const TaskInitial();
}

/// State when tasks are being loaded.
final class TaskLoading extends TaskState {
  const TaskLoading();
}

/// State when tasks are successfully loaded.
final class TaskLoaded extends TaskState {
  const TaskLoaded({
    required this.tasks,
    this.filter,
    this.tagFilter,
  });

  final List<Task> tasks;
  final TaskStatus? filter;
  final List<String>? tagFilter;

  @override
  List<Object?> get props => [tasks, filter, tagFilter];
}

/// State when a task operation fails.
final class TaskFailure extends TaskState {
  const TaskFailure({required this.message});

  final String message;

  @override
  List<Object?> get props => [message];
}

/// State when a task is successfully created (quick capture success).
final class TaskCaptured extends TaskState {
  const TaskCaptured({required this.task});

  final Task task;

  @override
  List<Object?> get props => [task];
}
