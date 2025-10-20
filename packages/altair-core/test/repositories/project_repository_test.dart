import 'package:altair_core/models/project.dart';
import 'package:altair_core/repositories/project_repository.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:sqflite_common_ffi/sqflite_ffi.dart';

void main() {
  // Initialize sqflite for testing
  setUpAll(() {
    // Initialize Flutter bindings
    TestWidgetsFlutterBinding.ensureInitialized();
    // Initialize FFI
    sqfliteFfiInit();
    // Change the default factory for unit testing calls for SQFlite
    databaseFactory = databaseFactoryFfi;
  });

  group('ProjectRepository', () {
    late ProjectRepository projectRepository;

    setUp(() {
      projectRepository = ProjectRepository();
    });

    group('CRUD Operations', () {
      test('create() should create a new project', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Test Project',
          description: 'A test project',
          status: ProjectStatus.active,
          tags: ['test', 'sample'],
          createdAt: now,
          updatedAt: now,
        );

        final createdProject = await projectRepository.create(project);

        expect(createdProject.id, isNotEmpty);
        expect(createdProject.name, 'Test Project');
        expect(createdProject.description, 'A test project');
        expect(createdProject.status, ProjectStatus.active);
        expect(createdProject.tags, ['test', 'sample']);
      });

      test('create() generates UUID if id is empty', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Auto ID Project',
          createdAt: now,
          updatedAt: now,
        );

        final createdProject = await projectRepository.create(project);

        expect(createdProject.id, isNotEmpty);
        expect(createdProject.id.length, 36); // UUID length
      });

      test('findById() should return project when it exists', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Find Me',
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);
        final found = await projectRepository.findById(created.id);

        expect(found, isNotNull);
        expect(found!.id, created.id);
        expect(found.name, 'Find Me');
      });

      test('findById() should return null when project does not exist', () async {
        final found = await projectRepository.findById('non-existent-id');

        expect(found, isNull);
      });

      test('findAll() should return all projects', () async {
        final now = DateTime.now();

        final project1 = Project(
          id: '',
          name: 'Project 1',
          createdAt: now,
          updatedAt: now,
        );

        final project2 = Project(
          id: '',
          name: 'Project 2',
          createdAt: now,
          updatedAt: now,
        );

        await projectRepository.create(project1);
        await projectRepository.create(project2);

        final projects = await projectRepository.findAll();

        expect(projects.length, greaterThanOrEqualTo(2));
        expect(projects.any((p) => p.name == 'Project 1'), isTrue);
        expect(projects.any((p) => p.name == 'Project 2'), isTrue);
      });

      test('findAll() with status filter should return filtered projects', () async {
        final now = DateTime.now();

        final activeProject = Project(
          id: '',
          name: 'Active Project',
          status: ProjectStatus.active,
          createdAt: now,
          updatedAt: now,
        );

        final completedProject = Project(
          id: '',
          name: 'Completed Project',
          status: ProjectStatus.completed,
          createdAt: now,
          updatedAt: now,
        );

        await projectRepository.create(activeProject);
        await projectRepository.create(completedProject);

        final activeProjects = await projectRepository.findAll(
          status: ProjectStatus.active,
        );

        expect(activeProjects.every((p) => p.status == ProjectStatus.active), isTrue);
      });

      test('update() should update existing project', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Original Name',
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);
        final updated = created.copyWith(
          name: 'Updated Name',
          status: ProjectStatus.completed,
        );

        final result = await projectRepository.update(updated);

        expect(result.name, 'Updated Name');
        expect(result.status, ProjectStatus.completed);

        final found = await projectRepository.findById(created.id);
        expect(found!.name, 'Updated Name');
      });

      test('delete() should remove project from database', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'To Delete',
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);
        await projectRepository.delete(created.id);

        final found = await projectRepository.findById(created.id);
        expect(found, isNull);
      });
    });

    group('Search and Query Operations', () {
      test('search() should find projects by name', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Unique Search Name',
          createdAt: now,
          updatedAt: now,
        );

        await projectRepository.create(project);
        final results = await projectRepository.search('Unique Search');

        expect(results.isNotEmpty, isTrue);
        expect(results.any((p) => p.name == 'Unique Search Name'), isTrue);
      });

      test('search() should find projects by description', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Project with Description',
          description: 'Unique description text',
          createdAt: now,
          updatedAt: now,
        );

        await projectRepository.create(project);
        final results = await projectRepository.search('Unique description');

        expect(results.isNotEmpty, isTrue);
        expect(
          results.any((p) => p.description?.contains('Unique description') ?? false),
          isTrue,
        );
      });

      test('search() should return empty list when no matches', () async {
        final results = await projectRepository.search('NonExistentSearchTerm123456');

        expect(results, isEmpty);
      });

      test('getTaskCount() should return correct task count for project', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Project with Tasks',
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);
        final count = await projectRepository.getTaskCount(created.id);

        // Should be 0 initially as we haven't added any tasks
        expect(count, 0);
      });
    });

    group('Edge Cases', () {
      test('handles project with all fields populated', () async {
        final now = DateTime.now();
        final targetDate = now.add(const Duration(days: 30));
        final completedAt = now.add(const Duration(days: 20));

        final project = Project(
          id: '',
          name: 'Full Project',
          description: 'Complete description',
          status: ProjectStatus.completed,
          tags: ['tag1', 'tag2', 'tag3'],
          color: '#FF5733',
          createdAt: now,
          updatedAt: now,
          targetDate: targetDate,
          completedAt: completedAt,
          metadata: {'key1': 'value1', 'key2': 123},
        );

        final created = await projectRepository.create(project);
        final found = await projectRepository.findById(created.id);

        expect(found, isNotNull);
        expect(found!.name, 'Full Project');
        expect(found.description, 'Complete description');
        expect(found.status, ProjectStatus.completed);
        expect(found.tags, ['tag1', 'tag2', 'tag3']);
        expect(found.color, '#FF5733');
        expect(found.metadata, {'key1': 'value1', 'key2': 123});
      });

      test('handles project with minimal fields', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Minimal Project',
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);
        final found = await projectRepository.findById(created.id);

        expect(found, isNotNull);
        expect(found!.name, 'Minimal Project');
        expect(found.description, isNull);
        expect(found.status, ProjectStatus.active); // Default value
        expect(found.tags, isEmpty);
        expect(found.color, isNull);
        expect(found.metadata, isNull);
      });

      test('handles empty tags list correctly', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'No Tags',
          tags: [],
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);
        final found = await projectRepository.findById(created.id);

        expect(found!.tags, isEmpty);
      });

      test('handles special characters in name and description', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Project with "quotes" and \'apostrophes\'',
          description: 'Description with special chars: @#\$%^&*()',
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);
        final found = await projectRepository.findById(created.id);

        expect(found!.name, contains('quotes'));
        expect(found.description, contains('special chars'));
      });
    });

    group('Status Transitions', () {
      test('can transition project through all statuses', () async {
        final now = DateTime.now();
        final project = Project(
          id: '',
          name: 'Status Test',
          status: ProjectStatus.active,
          createdAt: now,
          updatedAt: now,
        );

        final created = await projectRepository.create(project);

        // Active -> On Hold
        var updated = await projectRepository.update(
          created.copyWith(status: ProjectStatus.onHold),
        );
        expect(updated.status, ProjectStatus.onHold);

        // On Hold -> Active
        updated = await projectRepository.update(
          updated.copyWith(status: ProjectStatus.active),
        );
        expect(updated.status, ProjectStatus.active);

        // Active -> Completed
        updated = await projectRepository.update(
          updated.copyWith(status: ProjectStatus.completed, completedAt: now),
        );
        expect(updated.status, ProjectStatus.completed);

        // Completed -> Cancelled
        updated = await projectRepository.update(
          updated.copyWith(status: ProjectStatus.cancelled),
        );
        expect(updated.status, ProjectStatus.cancelled);
      });
    });
  });
}
