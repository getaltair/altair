import 'package:altair_core/models/project.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('Project', () {
    final now = DateTime.now();
    final testProject = Project(
      id: 'test-id',
      name: 'Test Project',
      description: 'Test Description',
      status: ProjectStatus.active,
      tags: ['important', 'work'],
      color: '#FF5733',
      createdAt: now,
      updatedAt: now,
      targetDate: now.add(const Duration(days: 30)),
    );

    test('creates project with required fields', () {
      expect(testProject.id, 'test-id');
      expect(testProject.name, 'Test Project');
      expect(testProject.description, 'Test Description');
      expect(testProject.status, ProjectStatus.active);
      expect(testProject.tags, ['important', 'work']);
      expect(testProject.color, '#FF5733');
    });

    test('has default values for optional fields', () {
      final minimal = Project(
        id: 'minimal',
        name: 'Minimal Project',
        createdAt: now,
        updatedAt: now,
      );

      expect(minimal.status, ProjectStatus.active);
      expect(minimal.tags, isEmpty);
      expect(minimal.description, isNull);
      expect(minimal.color, isNull);
      expect(minimal.targetDate, isNull);
      expect(minimal.completedAt, isNull);
      expect(minimal.metadata, isNull);
    });

    test('copyWith creates new project with updated fields', () {
      final updated = testProject.copyWith(
        name: 'Updated Name',
        status: ProjectStatus.completed,
        color: '#00FF00',
      );

      expect(updated.id, testProject.id);
      expect(updated.name, 'Updated Name');
      expect(updated.status, ProjectStatus.completed);
      expect(updated.color, '#00FF00');
      expect(updated.description, testProject.description);
      expect(updated.tags, testProject.tags);
    });

    test('copyWith preserves original when no fields provided', () {
      final copied = testProject.copyWith();

      expect(copied.id, testProject.id);
      expect(copied.name, testProject.name);
      expect(copied.status, testProject.status);
      expect(copied.description, testProject.description);
      expect(copied.color, testProject.color);
    });

    test('equality is based on id', () {
      final project1 = Project(
        id: 'same-id',
        name: 'Project 1',
        createdAt: now,
        updatedAt: now,
      );

      final project2 = Project(
        id: 'same-id',
        name: 'Project 2',
        createdAt: now,
        updatedAt: now,
      );

      final project3 = Project(
        id: 'different-id',
        name: 'Project 1',
        createdAt: now,
        updatedAt: now,
      );

      expect(project1, equals(project2));
      expect(project1, isNot(equals(project3)));
      expect(project1.hashCode, equals(project2.hashCode));
    });

    test('toString returns readable format', () {
      final str = testProject.toString();
      expect(str, contains('test-id'));
      expect(str, contains('Test Project'));
      expect(str, contains('ProjectStatus.active'));
    });

    test('handles all project statuses', () {
      expect(ProjectStatus.values.length, 4);
      expect(ProjectStatus.values, contains(ProjectStatus.active));
      expect(ProjectStatus.values, contains(ProjectStatus.onHold));
      expect(ProjectStatus.values, contains(ProjectStatus.completed));
      expect(ProjectStatus.values, contains(ProjectStatus.cancelled));
    });

    test('handles target date', () {
      final targetDate = DateTime(2025, 12, 31);
      final projectWithTarget = testProject.copyWith(
        targetDate: targetDate,
      );

      expect(projectWithTarget.targetDate, targetDate);
    });

    test('handles completion tracking', () {
      final completedTime = DateTime.now();
      final completed = testProject.copyWith(
        status: ProjectStatus.completed,
        completedAt: completedTime,
      );

      expect(completed.status, ProjectStatus.completed);
      expect(completed.completedAt, completedTime);
    });

    test('handles metadata as flexible field', () {
      final metadata = {
        'icon': 'briefcase',
        'priority': 'high',
        'customField': 123,
      };

      final projectWithMeta = testProject.copyWith(metadata: metadata);

      expect(projectWithMeta.metadata, metadata);
      expect(projectWithMeta.metadata!['icon'], 'briefcase');
      expect(projectWithMeta.metadata!['customField'], 123);
    });

    test('handles color field', () {
      final redProject = testProject.copyWith(color: '#FF0000');
      final blueProject = testProject.copyWith(color: '#0000FF');

      expect(redProject.color, '#FF0000');
      expect(blueProject.color, '#0000FF');
    });

    test('handles empty tags list', () {
      final noTagsProject = Project(
        id: 'no-tags',
        name: 'No Tags Project',
        createdAt: now,
        updatedAt: now,
      );

      expect(noTagsProject.tags, isEmpty);
    });

    test('handles multiple tags', () {
      final multiTagProject = Project(
        id: 'multi-tag',
        name: 'Multi Tag Project',
        tags: ['tag1', 'tag2', 'tag3', 'tag4'],
        createdAt: now,
        updatedAt: now,
      );

      expect(multiTagProject.tags.length, 4);
      expect(multiTagProject.tags, contains('tag1'));
      expect(multiTagProject.tags, contains('tag4'));
    });

    test('JSON serialization works correctly', () {
      final json = testProject.toJson();

      expect(json['id'], 'test-id');
      expect(json['name'], 'Test Project');
      expect(json['status'], 'active');
      expect(json['tags'], ['important', 'work']);
      expect(json['color'], '#FF5733');
    });

    test('JSON deserialization works correctly', () {
      final json = {
        'id': 'json-id',
        'name': 'JSON Project',
        'description': 'From JSON',
        'status': 'active',
        'tags': ['json-tag'],
        'color': '#123456',
        'createdAt': now.toIso8601String(),
        'updatedAt': now.toIso8601String(),
      };

      final project = Project.fromJson(json);

      expect(project.id, 'json-id');
      expect(project.name, 'JSON Project');
      expect(project.status, ProjectStatus.active);
      expect(project.tags, ['json-tag']);
      expect(project.color, '#123456');
    });

    test('handles on hold status', () {
      final onHoldProject = testProject.copyWith(
        status: ProjectStatus.onHold,
      );

      expect(onHoldProject.status, ProjectStatus.onHold);
    });

    test('handles cancelled status', () {
      final cancelledProject = testProject.copyWith(
        status: ProjectStatus.cancelled,
      );

      expect(cancelledProject.status, ProjectStatus.cancelled);
    });
  });
}
