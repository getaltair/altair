/// Tag states for the tag bloc.
library;

import 'package:altair_core/altair_core.dart';
import 'package:equatable/equatable.dart';

/// Base class for tag states.
sealed class TagState extends Equatable {
  const TagState();

  @override
  List<Object?> get props => [];
}

/// Initial state when the bloc is first created.
final class TagInitial extends TagState {
  const TagInitial();
}

/// State when tags are being loaded.
final class TagLoading extends TagState {
  const TagLoading();
}

/// State when tags are successfully loaded.
final class TagLoaded extends TagState {
  const TagLoaded({
    required this.tags,
    this.searchQuery,
  });

  final List<Tag> tags;
  final String? searchQuery;

  @override
  List<Object?> get props => [tags, searchQuery];
}

/// State when a tag operation fails.
final class TagFailure extends TagState {
  const TagFailure({required this.message});

  final String message;

  @override
  List<Object?> get props => [message];
}

/// State when most used tags are loaded.
final class TagMostUsedLoaded extends TagState {
  const TagMostUsedLoaded({required this.tags});

  final List<Tag> tags;

  @override
  List<Object?> get props => [tags];
}
