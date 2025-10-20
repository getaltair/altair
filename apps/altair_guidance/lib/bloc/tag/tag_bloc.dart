/// Tag BLoC for managing tag state.
library;

import 'package:altair_core/altair_core.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';

import 'tag_event.dart';
import 'tag_state.dart';

/// BLoC for managing tag state and operations.
class TagBloc extends Bloc<TagEvent, TagState> {
  /// Creates a tag bloc.
  TagBloc({
    required TagRepository tagRepository,
    Logger? logger,
  })  : _tagRepository = tagRepository,
        _logger = logger ?? Logger(),
        super(const TagInitial()) {
    on<TagLoadRequested>(_onLoadRequested);
    on<TagCreateRequested>(_onCreateRequested);
    on<TagUpdateRequested>(_onUpdateRequested);
    on<TagDeleteRequested>(_onDeleteRequested);
    on<TagSearchRequested>(_onSearchRequested);
    on<TagMostUsedRequested>(_onMostUsedRequested);
    on<TagIncrementUsageRequested>(_onIncrementUsageRequested);
    on<TagDecrementUsageRequested>(_onDecrementUsageRequested);
  }

  final TagRepository _tagRepository;
  final Logger _logger;

  /// Handles loading all tags.
  Future<void> _onLoadRequested(
    TagLoadRequested event,
    Emitter<TagState> emit,
  ) async {
    emit(const TagLoading());

    try {
      final tags = await _tagRepository.findAll();
      emit(TagLoaded(tags: tags));
      _logger.i('Loaded ${tags.length} tags');
    } catch (e, stackTrace) {
      _logger.e('Failed to load tags', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }

  /// Handles creating a new tag.
  Future<void> _onCreateRequested(
    TagCreateRequested event,
    Emitter<TagState> emit,
  ) async {
    emit(const TagLoading());

    try {
      // Check if tag name already exists
      final existing = await _tagRepository.findByName(event.tag.name);
      if (existing != null) {
        emit(const TagFailure(message: 'Tag name already exists'));
        return;
      }

      await _tagRepository.create(event.tag);
      _logger.i('Created tag: ${event.tag.name}');

      final tags = await _tagRepository.findAll();
      emit(TagLoaded(tags: tags));
    } catch (e, stackTrace) {
      _logger.e('Failed to create tag', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }

  /// Handles updating an existing tag.
  Future<void> _onUpdateRequested(
    TagUpdateRequested event,
    Emitter<TagState> emit,
  ) async {
    emit(const TagLoading());

    try {
      // Check if new name conflicts with another tag
      final existing = await _tagRepository.findByName(event.tag.name);
      if (existing != null && existing.id != event.tag.id) {
        emit(const TagFailure(message: 'Tag name already exists'));
        return;
      }

      await _tagRepository.update(event.tag);
      _logger.i('Updated tag: ${event.tag.id}');

      final tags = await _tagRepository.findAll();
      emit(TagLoaded(tags: tags));
    } catch (e, stackTrace) {
      _logger.e('Failed to update tag', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }

  /// Handles deleting a tag.
  Future<void> _onDeleteRequested(
    TagDeleteRequested event,
    Emitter<TagState> emit,
  ) async {
    emit(const TagLoading());

    try {
      await _tagRepository.delete(event.tagId);
      _logger.i('Deleted tag: ${event.tagId}');

      final tags = await _tagRepository.findAll();
      emit(TagLoaded(tags: tags));
    } catch (e, stackTrace) {
      _logger.e('Failed to delete tag', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }

  /// Handles searching tags.
  Future<void> _onSearchRequested(
    TagSearchRequested event,
    Emitter<TagState> emit,
  ) async {
    emit(const TagLoading());

    try {
      final tags = await _tagRepository.search(event.query);
      emit(TagLoaded(tags: tags, searchQuery: event.query));
      _logger.i('Search returned ${tags.length} tags for query: ${event.query}');
    } catch (e, stackTrace) {
      _logger.e('Failed to search tags', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }

  /// Handles loading most used tags.
  Future<void> _onMostUsedRequested(
    TagMostUsedRequested event,
    Emitter<TagState> emit,
  ) async {
    emit(const TagLoading());

    try {
      final tags = await _tagRepository.findMostUsed(limit: event.limit);
      emit(TagMostUsedLoaded(tags: tags));
      _logger.i('Loaded ${tags.length} most used tags');
    } catch (e, stackTrace) {
      _logger.e('Failed to load most used tags', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }

  /// Handles incrementing tag usage count.
  Future<void> _onIncrementUsageRequested(
    TagIncrementUsageRequested event,
    Emitter<TagState> emit,
  ) async {
    try {
      await _tagRepository.incrementUsageCount(event.tagId);
      _logger.i('Incremented usage count for tag: ${event.tagId}');
      // Don't reload all tags, just log the action
    } catch (e, stackTrace) {
      _logger.e('Failed to increment usage count', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }

  /// Handles decrementing tag usage count.
  Future<void> _onDecrementUsageRequested(
    TagDecrementUsageRequested event,
    Emitter<TagState> emit,
  ) async {
    try {
      await _tagRepository.decrementUsageCount(event.tagId);
      _logger.i('Decremented usage count for tag: ${event.tagId}');
      // Don't reload all tags, just log the action
    } catch (e, stackTrace) {
      _logger.e('Failed to decrement usage count', error: e, stackTrace: stackTrace);
      emit(TagFailure(message: e.toString()));
    }
  }
}
