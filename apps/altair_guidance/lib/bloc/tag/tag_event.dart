/// Tag events for the tag bloc.
library;

import 'package:altair_core/altair_core.dart';
import 'package:equatable/equatable.dart';

/// Base class for tag events.
sealed class TagEvent extends Equatable {
  const TagEvent();

  @override
  List<Object?> get props => [];
}

/// Event to load all tags.
final class TagLoadRequested extends TagEvent {
  const TagLoadRequested();
}

/// Event to create a new tag.
final class TagCreateRequested extends TagEvent {
  const TagCreateRequested({required this.tag});

  final Tag tag;

  @override
  List<Object?> get props => [tag];
}

/// Event to update an existing tag.
final class TagUpdateRequested extends TagEvent {
  const TagUpdateRequested({required this.tag});

  final Tag tag;

  @override
  List<Object?> get props => [tag];
}

/// Event to delete a tag.
final class TagDeleteRequested extends TagEvent {
  const TagDeleteRequested({required this.tagId});

  final String tagId;

  @override
  List<Object?> get props => [tagId];
}

/// Event to search tags.
final class TagSearchRequested extends TagEvent {
  const TagSearchRequested({required this.query});

  final String query;

  @override
  List<Object?> get props => [query];
}

/// Event to load most used tags.
final class TagMostUsedRequested extends TagEvent {
  const TagMostUsedRequested({this.limit = 10});

  final int limit;

  @override
  List<Object?> get props => [limit];
}

/// Event to increment tag usage count.
final class TagIncrementUsageRequested extends TagEvent {
  const TagIncrementUsageRequested({required this.tagId});

  final String tagId;

  @override
  List<Object?> get props => [tagId];
}

/// Event to decrement tag usage count.
final class TagDecrementUsageRequested extends TagEvent {
  const TagDecrementUsageRequested({required this.tagId});

  final String tagId;

  @override
  List<Object?> get props => [tagId];
}
