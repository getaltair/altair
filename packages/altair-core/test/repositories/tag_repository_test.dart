import 'package:altair_core/database/database.dart';
import 'package:altair_core/models/tag.dart';
import 'package:altair_core/repositories/tag_repository.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  // Initialize database for testing
  setUpAll(() {
    AltairDatabase.enableTestMode();
  });

  group('TagRepository', () {
    late TagRepository tagRepository;

    setUp(() {
      tagRepository = TagRepository();
    });

    tearDown(() async {
      // Reset database between tests for isolation
      await AltairDatabase().close();
      AltairDatabase.reset();
      AltairDatabase.enableTestMode();
    });

    group('CRUD Operations', () {
      test('create() should create a new tag', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'urgent',
          description: 'Urgent tasks',
          color: '#FF0000',
          createdAt: now,
          usageCount: 0,
        );

        final createdTag = await tagRepository.create(tag);

        expect(createdTag.id, isNotEmpty);
        expect(createdTag.name, 'urgent');
        expect(createdTag.description, 'Urgent tasks');
        expect(createdTag.color, '#FF0000');
        expect(createdTag.usageCount, 0);
      });

      test('create() generates UUID if id is empty', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'auto-id',
          createdAt: now,
        );

        final createdTag = await tagRepository.create(tag);

        expect(createdTag.id, isNotEmpty);
        expect(createdTag.id.length, 36); // UUID length
      });

      test('create() uses provided id if not empty', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: 'custom-id-123',
          name: 'custom',
          createdAt: now,
        );

        final createdTag = await tagRepository.create(tag);

        expect(createdTag.id, 'custom-id-123');
      });

      test('findById() should return tag when exists', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'test-tag',
          createdAt: now,
        );

        final created = await tagRepository.create(tag);
        final found = await tagRepository.findById(created.id);

        expect(found, isNotNull);
        expect(found!.id, created.id);
        expect(found.name, 'test-tag');
      });

      test('findById() should return null when tag does not exist', () async {
        final found = await tagRepository.findById('non-existent-id');
        expect(found, isNull);
      });

      test('findByName() should return tag when exists', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'unique-name',
          createdAt: now,
        );

        await tagRepository.create(tag);
        final found = await tagRepository.findByName('unique-name');

        expect(found, isNotNull);
        expect(found!.name, 'unique-name');
      });

      test('findByName() should return null when tag does not exist', () async {
        final found = await tagRepository.findByName('non-existent');
        expect(found, isNull);
      });

      test('update() should update tag fields', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'original',
          description: 'Original description',
          createdAt: now,
        );

        final created = await tagRepository.create(tag);
        final updated = created.copyWith(
          name: 'updated',
          description: 'Updated description',
          color: '#00FF00',
        );

        final result = await tagRepository.update(updated);

        expect(result.name, 'updated');
        expect(result.description, 'Updated description');
        expect(result.color, '#00FF00');

        // Verify persistence
        final found = await tagRepository.findById(created.id);
        expect(found!.name, 'updated');
        expect(found.description, 'Updated description');
      });

      test('delete() should remove tag', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'to-delete',
          createdAt: now,
        );

        final created = await tagRepository.create(tag);
        await tagRepository.delete(created.id);

        final found = await tagRepository.findById(created.id);
        expect(found, isNull);
      });
    });

    group('Query Operations', () {
      test('findAll() should return all tags', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'tag1', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'tag2', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'tag3', createdAt: now));

        final tags = await tagRepository.findAll();

        expect(tags.length, 3);
      });

      test('findAll() should support limit', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'tag1', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'tag2', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'tag3', createdAt: now));

        final tags = await tagRepository.findAll(limit: 2);

        expect(tags.length, 2);
      });

      test('findAll() should support offset', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'tag1', createdAt: now, usageCount: 3));
        await tagRepository.create(Tag(id: '', name: 'tag2', createdAt: now, usageCount: 2));
        await tagRepository.create(Tag(id: '', name: 'tag3', createdAt: now, usageCount: 1));

        final tags = await tagRepository.findAll(offset: 1, limit: 2);

        expect(tags.length, 2);
      });

      test('findAll() orders by usage count DESC then name ASC by default', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'zebra', createdAt: now, usageCount: 1));
        await tagRepository.create(Tag(id: '', name: 'apple', createdAt: now, usageCount: 5));
        await tagRepository.create(Tag(id: '', name: 'banana', createdAt: now, usageCount: 5));

        final tags = await tagRepository.findAll();

        expect(tags[0].name, 'apple'); // usage_count 5, alphabetically first
        expect(tags[1].name, 'banana'); // usage_count 5, alphabetically second
        expect(tags[2].name, 'zebra'); // usage_count 1
      });

      test('search() should find tags by name', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'urgent', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'important', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'work', createdAt: now));

        final results = await tagRepository.search('urg');

        expect(results.length, 1);
        expect(results.first.name, 'urgent');
      });

      test('search() should find tags by description', () async {
        final now = DateTime.now();

        await tagRepository.create(
          Tag(id: '', name: 'tag1', description: 'High priority items', createdAt: now),
        );
        await tagRepository.create(
          Tag(id: '', name: 'tag2', description: 'Low priority items', createdAt: now),
        );

        final results = await tagRepository.search('High');

        expect(results.length, 1);
        expect(results.first.name, 'tag1');
      });

      test('search() should be case-insensitive', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'URGENT', createdAt: now));

        final results = await tagRepository.search('urg');

        expect(results.length, 1);
        expect(results.first.name, 'URGENT');
      });

      test('search() should return empty list when no matches', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'test', createdAt: now));

        final results = await tagRepository.search('nonexistent');

        expect(results, isEmpty);
      });

      test('findMostUsed() should return tags with usage > 0 ordered by count', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'tag1', createdAt: now, usageCount: 10));
        await tagRepository.create(Tag(id: '', name: 'tag2', createdAt: now, usageCount: 5));
        await tagRepository.create(Tag(id: '', name: 'tag3', createdAt: now, usageCount: 0));
        await tagRepository.create(Tag(id: '', name: 'tag4', createdAt: now, usageCount: 15));

        final mostUsed = await tagRepository.findMostUsed(limit: 2);

        expect(mostUsed.length, 2);
        expect(mostUsed[0].name, 'tag4'); // 15 uses
        expect(mostUsed[1].name, 'tag1'); // 10 uses
      });

      test('findMostUsed() should respect limit parameter', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'tag1', createdAt: now, usageCount: 3));
        await tagRepository.create(Tag(id: '', name: 'tag2', createdAt: now, usageCount: 2));
        await tagRepository.create(Tag(id: '', name: 'tag3', createdAt: now, usageCount: 1));

        final mostUsed = await tagRepository.findMostUsed(limit: 2);

        expect(mostUsed.length, 2);
      });

      test('findByIds() should return tags with matching IDs', () async {
        final now = DateTime.now();

        final tag1 = await tagRepository.create(Tag(id: '', name: 'tag1', createdAt: now));
        final tag2 = await tagRepository.create(Tag(id: '', name: 'tag2', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'tag3', createdAt: now));

        final tags = await tagRepository.findByIds([tag1.id, tag2.id]);

        expect(tags.length, 2);
        expect(tags.any((t) => t.name == 'tag1'), isTrue);
        expect(tags.any((t) => t.name == 'tag2'), isTrue);
        expect(tags.any((t) => t.name == 'tag3'), isFalse);
      });

      test('findByIds() should return empty list for empty input', () async {
        final tags = await tagRepository.findByIds([]);
        expect(tags, isEmpty);
      });

      test('findByIds() should return empty list for non-existent IDs', () async {
        final tags = await tagRepository.findByIds(['non-existent-1', 'non-existent-2']);
        expect(tags, isEmpty);
      });

      test('count() should return total number of tags', () async {
        final now = DateTime.now();

        expect(await tagRepository.count(), 0);

        await tagRepository.create(Tag(id: '', name: 'tag1', createdAt: now));
        expect(await tagRepository.count(), 1);

        await tagRepository.create(Tag(id: '', name: 'tag2', createdAt: now));
        await tagRepository.create(Tag(id: '', name: 'tag3', createdAt: now));
        expect(await tagRepository.count(), 3);
      });
    });

    group('Usage Count Operations', () {
      test('incrementUsageCount() should increase usage count by 1', () async {
        final now = DateTime.now();
        final tag = await tagRepository.create(
          Tag(id: '', name: 'test', createdAt: now, usageCount: 5),
        );

        await tagRepository.incrementUsageCount(tag.id);

        final updated = await tagRepository.findById(tag.id);
        expect(updated!.usageCount, 6);
      });

      test('incrementUsageCount() should work multiple times', () async {
        final now = DateTime.now();
        final tag = await tagRepository.create(
          Tag(id: '', name: 'test', createdAt: now, usageCount: 0),
        );

        await tagRepository.incrementUsageCount(tag.id);
        await tagRepository.incrementUsageCount(tag.id);
        await tagRepository.incrementUsageCount(tag.id);

        final updated = await tagRepository.findById(tag.id);
        expect(updated!.usageCount, 3);
      });

      test('decrementUsageCount() should decrease usage count by 1', () async {
        final now = DateTime.now();
        final tag = await tagRepository.create(
          Tag(id: '', name: 'test', createdAt: now, usageCount: 5),
        );

        await tagRepository.decrementUsageCount(tag.id);

        final updated = await tagRepository.findById(tag.id);
        expect(updated!.usageCount, 4);
      });

      test('decrementUsageCount() should not go below 0', () async {
        final now = DateTime.now();
        final tag = await tagRepository.create(
          Tag(id: '', name: 'test', createdAt: now, usageCount: 1),
        );

        await tagRepository.decrementUsageCount(tag.id);
        await tagRepository.decrementUsageCount(tag.id);
        await tagRepository.decrementUsageCount(tag.id);

        final updated = await tagRepository.findById(tag.id);
        expect(updated!.usageCount, 0);
      });
    });

    group('Edge Cases', () {
      test('create() should handle tags with no description', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'simple',
          createdAt: now,
        );

        final created = await tagRepository.create(tag);

        expect(created.description, isNull);
      });

      test('create() should handle tags with no color', () async {
        final now = DateTime.now();
        final tag = Tag(
          id: '',
          name: 'colorless',
          createdAt: now,
        );

        final created = await tagRepository.create(tag);

        expect(created.color, isNull);
      });

      test('findAll() should handle empty database', () async {
        final tags = await tagRepository.findAll();
        expect(tags, isEmpty);
      });

      test('search() should handle special characters', () async {
        final now = DateTime.now();

        await tagRepository.create(Tag(id: '', name: 'c++', createdAt: now));

        final results = await tagRepository.search('c++');

        expect(results.length, 1);
        expect(results.first.name, 'c++');
      });
    });
  });
}
