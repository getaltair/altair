import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/tag/tag_bloc.dart';
import 'package:altair_guidance/bloc/tag/tag_event.dart';
import 'package:altair_guidance/bloc/tag/tag_state.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:logger/logger.dart';
import 'package:mocktail/mocktail.dart';

class MockTagRepository extends Mock implements TagRepository {}

class MockLogger extends Mock implements Logger {}

class FakeTag extends Fake implements Tag {}

void main() {
  late MockTagRepository mockTagRepository;
  late MockLogger mockLogger;

  setUpAll(() {
    registerFallbackValue(FakeTag());
  });

  // Sample test data
  final now = DateTime.now();
  final tag1 = Tag(
    id: '1',
    name: 'urgent',
    description: 'Urgent tasks',
    color: '#FF0000',
    createdAt: now,
    usageCount: 10,
  );

  final tag2 = Tag(
    id: '2',
    name: 'work',
    description: 'Work-related tasks',
    color: '#0000FF',
    createdAt: now,
    usageCount: 5,
  );

  final tag3 = Tag(
    id: '3',
    name: 'personal',
    createdAt: now,
    usageCount: 3,
  );

  setUp(() {
    mockTagRepository = MockTagRepository();
    mockLogger = MockLogger();
  });

  group('TagBloc', () {
    test('initial state is TagInitial', () {
      final bloc = TagBloc(
        tagRepository: mockTagRepository,
        logger: mockLogger,
      );

      expect(bloc.state, const TagInitial());
    });

    group('TagLoadRequested', () {
      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagLoaded] when tags are loaded successfully',
        build: () {
          when(() => mockTagRepository.findAll())
              .thenAnswer((_) async => [tag1, tag2, tag3]);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagLoadRequested()),
        expect: () => [
          const TagLoading(),
          TagLoaded(tags: [tag1, tag2, tag3]),
        ],
        verify: (_) {
          verify(() => mockTagRepository.findAll()).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when loading fails',
        build: () {
          when(() => mockTagRepository.findAll())
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagLoadRequested()),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });

    group('TagCreateRequested', () {
      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagLoaded] when tag is created successfully',
        build: () {
          when(() => mockTagRepository.findByName(tag1.name))
              .thenAnswer((_) async => null);
          when(() => mockTagRepository.create(any()))
              .thenAnswer((_) async => tag1);
          when(() => mockTagRepository.findAll())
              .thenAnswer((_) async => [tag1]);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TagCreateRequested(tag: tag1)),
        expect: () => [
          const TagLoading(),
          TagLoaded(tags: [tag1]),
        ],
        verify: (_) {
          verify(() => mockTagRepository.findByName(tag1.name)).called(1);
          verify(() => mockTagRepository.create(tag1)).called(1);
          verify(() => mockTagRepository.findAll()).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when tag name already exists',
        build: () {
          when(() => mockTagRepository.findByName(tag1.name))
              .thenAnswer((_) async => tag1);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TagCreateRequested(tag: tag1)),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Tag name already exists'),
        ],
        verify: (_) {
          verify(() => mockTagRepository.findByName(tag1.name)).called(1);
          verifyNever(() => mockTagRepository.create(any()));
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when creation fails',
        build: () {
          when(() => mockTagRepository.findByName(tag1.name))
              .thenAnswer((_) async => null);
          when(() => mockTagRepository.create(any()))
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TagCreateRequested(tag: tag1)),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });

    group('TagUpdateRequested', () {
      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagLoaded] when tag is updated successfully',
        build: () {
          when(() => mockTagRepository.findByName(tag1.name))
              .thenAnswer((_) async => tag1); // Same tag
          when(() => mockTagRepository.update(any()))
              .thenAnswer((_) async => tag1);
          when(() => mockTagRepository.findAll())
              .thenAnswer((_) async => [tag1]);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TagUpdateRequested(tag: tag1)),
        expect: () => [
          const TagLoading(),
          TagLoaded(tags: [tag1]),
        ],
        verify: (_) {
          verify(() => mockTagRepository.update(tag1)).called(1);
          verify(() => mockTagRepository.findAll()).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when new name conflicts with another tag',
        build: () {
          when(() => mockTagRepository.findByName(tag1.name))
              .thenAnswer((_) async => tag2); // Different tag with same name
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TagUpdateRequested(tag: tag1)),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Tag name already exists'),
        ],
        verify: (_) {
          verify(() => mockTagRepository.findByName(tag1.name)).called(1);
          verifyNever(() => mockTagRepository.update(any()));
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when update fails',
        build: () {
          when(() => mockTagRepository.findByName(tag1.name))
              .thenAnswer((_) async => tag1);
          when(() => mockTagRepository.update(any()))
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TagUpdateRequested(tag: tag1)),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });

    group('TagDeleteRequested', () {
      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagLoaded] when tag is deleted successfully',
        build: () {
          when(() => mockTagRepository.delete('1')).thenAnswer((_) async => {});
          when(() => mockTagRepository.findAll())
              .thenAnswer((_) async => [tag2, tag3]);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagDeleteRequested(tagId: '1')),
        expect: () => [
          const TagLoading(),
          TagLoaded(tags: [tag2, tag3]),
        ],
        verify: (_) {
          verify(() => mockTagRepository.delete('1')).called(1);
          verify(() => mockTagRepository.findAll()).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when deletion fails',
        build: () {
          when(() => mockTagRepository.delete('1'))
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagDeleteRequested(tagId: '1')),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });

    group('TagSearchRequested', () {
      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagLoaded] with search results',
        build: () {
          when(() => mockTagRepository.search('work'))
              .thenAnswer((_) async => [tag2]);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagSearchRequested(query: 'work')),
        expect: () => [
          const TagLoading(),
          TagLoaded(tags: [tag2], searchQuery: 'work'),
        ],
        verify: (_) {
          verify(() => mockTagRepository.search('work')).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when search fails',
        build: () {
          when(() => mockTagRepository.search('work'))
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagSearchRequested(query: 'work')),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });

    group('TagMostUsedRequested', () {
      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagMostUsedLoaded] with most used tags',
        build: () {
          when(() => mockTagRepository.findMostUsed(limit: 5))
              .thenAnswer((_) async => [tag1, tag2]);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagMostUsedRequested(limit: 5)),
        expect: () => [
          const TagLoading(),
          TagMostUsedLoaded(tags: [tag1, tag2]),
        ],
        verify: (_) {
          verify(() => mockTagRepository.findMostUsed(limit: 5)).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'uses default limit of 10 when not specified',
        build: () {
          when(() => mockTagRepository.findMostUsed(limit: 10))
              .thenAnswer((_) async => [tag1, tag2, tag3]);
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagMostUsedRequested()),
        expect: () => [
          const TagLoading(),
          TagMostUsedLoaded(tags: [tag1, tag2, tag3]),
        ],
        verify: (_) {
          verify(() => mockTagRepository.findMostUsed(limit: 10)).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagLoading, TagFailure] when loading most used fails',
        build: () {
          when(() => mockTagRepository.findMostUsed(limit: any(named: 'limit')))
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagMostUsedRequested()),
        expect: () => [
          const TagLoading(),
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });

    group('TagIncrementUsageRequested', () {
      blocTest<TagBloc, TagState>(
        'increments usage count without changing state',
        build: () {
          when(() => mockTagRepository.incrementUsageCount('1'))
              .thenAnswer((_) async => {});
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagIncrementUsageRequested(tagId: '1')),
        expect: () => [], // No state change
        verify: (_) {
          verify(() => mockTagRepository.incrementUsageCount('1')).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagFailure] when increment fails',
        build: () {
          when(() => mockTagRepository.incrementUsageCount('1'))
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagIncrementUsageRequested(tagId: '1')),
        expect: () => [
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });

    group('TagDecrementUsageRequested', () {
      blocTest<TagBloc, TagState>(
        'decrements usage count without changing state',
        build: () {
          when(() => mockTagRepository.decrementUsageCount('1'))
              .thenAnswer((_) async => {});
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagDecrementUsageRequested(tagId: '1')),
        expect: () => [], // No state change
        verify: (_) {
          verify(() => mockTagRepository.decrementUsageCount('1')).called(1);
        },
      );

      blocTest<TagBloc, TagState>(
        'emits [TagFailure] when decrement fails',
        build: () {
          when(() => mockTagRepository.decrementUsageCount('1'))
              .thenThrow(Exception('Database error'));
          return TagBloc(
            tagRepository: mockTagRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TagDecrementUsageRequested(tagId: '1')),
        expect: () => [
          const TagFailure(message: 'Exception: Database error'),
        ],
      );
    });
  });
}
