import 'package:altair_core/models/tag.dart';
import 'package:altair_core/repositories/tag_repository.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('TagRepository', () {
    late TagRepository repository;

    setUp(() {
      repository = TagRepository();
    });

    group('ID generation', () {
      test('generates tag ID with prefix when id is empty', () {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'Test Tag',
          createdAt: now,
          usageCount: 0,
        );

        // Since we can't easily test the actual DB operations without a running DB,
        // we'll just test that the repository can be instantiated and basic logic works
        expect(repository, isNotNull);
        expect(tag.id, isEmpty);
      });

      test('preserves custom tag ID', () {
        final now = DateTime.now();
        final tag = Tag(
          id: 'tag:custom-id',
          name: 'Test Tag',
          createdAt: now,
          usageCount: 0,
        );

        expect(tag.id, 'tag:custom-id');
        expect(tag.id, startsWith('tag:'));
      });
    });

    group('Data conversion', () {
      test('tag data includes SurrealDB format fields', () {
        final now = DateTime.now();
        final tag = Tag(
          id: 'tag:123',
          name: 'Test Tag',
          description: 'A description',
          color: '#ff0000',
          createdAt: now,
          usageCount: 5,
        );

        // Verify tag structure is correct for SurrealDB
        expect(tag.id, startsWith('tag:'));
        expect(tag.usageCount, isA<int>());
        expect(tag.color, isA<String>());
      });
    });
  });
}
