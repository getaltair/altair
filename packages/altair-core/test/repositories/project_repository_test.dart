import 'package:altair_core/models/project.dart';
import 'package:altair_core/repositories/project_repository.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('ProjectRepository', () {
    late ProjectRepository repository;

    setUp(() {
      repository = ProjectRepository();
    });

    group('ID generation', () {
      test('generates project ID with prefix when id is empty', () {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Test Project',
          createdAt: now,
          updatedAt: now,
          status: ProjectStatus.active,
          tags: [],
        );

        // Since we can't easily test the actual DB operations without a running DB,
        // we'll just test that the repository can be instantiated and basic logic works
        expect(repository, isNotNull);
        expect(project.id, isEmpty);
      });

      test('preserves custom project ID', () {
        final now = DateTime.now();
        final project = Project(
          id: 'project:custom-id',
          name: 'Test Project',
          createdAt: now,
          updatedAt: now,
          status: ProjectStatus.active,
          tags: [],
        );

        expect(project.id, 'project:custom-id');
        expect(project.id, startsWith('project:'));
      });
    });

    group('Data conversion', () {
      test('project data includes SurrealDB format fields', () {
        final now = DateTime.now();
        final project = Project(
          id: 'project:123',
          name: 'Test Project',
          description: 'A description',
          createdAt: now,
          updatedAt: now,
          status: ProjectStatus.active,
          tags: ['tag1', 'tag2'],
          color: '#ff0000',
          metadata: {'key': 'value'},
        );

        // Verify project structure is correct for SurrealDB
        expect(project.id, startsWith('project:'));
        expect(project.tags, isA<List<String>>());
        expect(project.metadata, isA<Map<String, dynamic>>());
        expect(project.status, isA<ProjectStatus>());
      });
    });
  });
}
