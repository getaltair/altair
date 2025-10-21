import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/project/project_bloc.dart';
import 'package:altair_guidance/bloc/project/project_event.dart';
import 'package:altair_guidance/bloc/project/project_state.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:logger/logger.dart';
import 'package:mocktail/mocktail.dart';

class MockProjectRepository extends Mock implements ProjectRepository {}

class MockLogger extends Mock implements Logger {}

class FakeProject extends Fake implements Project {}

void main() {
  late MockProjectRepository mockProjectRepository;
  late MockLogger mockLogger;

  setUpAll(() {
    registerFallbackValue(FakeProject());
  });

  // Sample test data
  final now = DateTime.now();
  final project1 = Project(
    id: '1',
    name: 'Test Project 1',
    description: 'First test project',
    createdAt: now,
    updatedAt: now,
    status: ProjectStatus.active,
    tags: ['work'],
  );

  final project2 = Project(
    id: '2',
    name: 'Test Project 2',
    description: 'Second test project',
    createdAt: now,
    updatedAt: now,
    status: ProjectStatus.completed,
    completedAt: now,
    tags: ['personal'],
  );

  final project3 = Project(
    id: '3',
    name: 'On Hold Project',
    createdAt: now,
    updatedAt: now,
    status: ProjectStatus.onHold,
  );

  setUp(() {
    mockProjectRepository = MockProjectRepository();
    mockLogger = MockLogger();
  });

  group('ProjectBloc', () {
    test('initial state is ProjectInitial', () {
      final bloc = ProjectBloc(
        projectRepository: mockProjectRepository,
        logger: mockLogger,
      );

      expect(bloc.state, const ProjectInitial());
    });

    group('ProjectLoadRequested', () {
      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectLoaded] when projects are loaded successfully',
        build: () {
          when(() => mockProjectRepository.findAll())
              .thenAnswer((_) async => [project1, project2, project3]);
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const ProjectLoadRequested()),
        expect: () => [
          const ProjectLoading(),
          ProjectLoaded(projects: [project1, project2, project3]),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.findAll()).called(1);
        },
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectFailure] when loading fails',
        build: () {
          when(() => mockProjectRepository.findAll())
              .thenThrow(Exception('Failed to load projects'));
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const ProjectLoadRequested()),
        expect: () => [
          const ProjectLoading(),
          const ProjectFailure(message: 'Exception: Failed to load projects'),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.findAll()).called(1);
        },
      );
    });

    group('ProjectCreateRequested', () {
      final newProject = Project(
        id: '4',
        name: 'New Project',
        createdAt: now,
        updatedAt: now,
        status: ProjectStatus.active,
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectCreated, ProjectLoaded] when project is created successfully',
        build: () {
          when(() => mockProjectRepository.create(any()))
              .thenAnswer((_) async => newProject);
          when(() => mockProjectRepository.findAll())
              .thenAnswer((_) async => [newProject, project1, project2]);
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(ProjectCreateRequested(project: newProject)),
        expect: () => [
          const ProjectLoading(),
          ProjectCreated(project: newProject),
          ProjectLoaded(projects: [newProject, project1, project2]),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.create(any())).called(1);
          verify(() => mockProjectRepository.findAll()).called(1);
        },
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectFailure] when creation fails',
        build: () {
          when(() => mockProjectRepository.create(any()))
              .thenThrow(Exception('Failed to create'));
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(ProjectCreateRequested(project: newProject)),
        expect: () => [
          const ProjectLoading(),
          const ProjectFailure(message: 'Exception: Failed to create'),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.create(any())).called(1);
        },
      );
    });

    group('ProjectUpdateRequested', () {
      final updatedProject = project1.copyWith(
        name: 'Updated Project Name',
        status: ProjectStatus.completed,
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectLoaded] when project is updated successfully',
        build: () {
          when(() => mockProjectRepository.update(updatedProject))
              .thenAnswer((_) async => updatedProject);
          when(() => mockProjectRepository.findAll())
              .thenAnswer((_) async => [updatedProject, project2]);
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) =>
            bloc.add(ProjectUpdateRequested(project: updatedProject)),
        expect: () => [
          const ProjectLoading(),
          ProjectLoaded(projects: [updatedProject, project2]),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.update(updatedProject)).called(1);
          verify(() => mockProjectRepository.findAll()).called(1);
        },
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectFailure] when update fails',
        build: () {
          when(() => mockProjectRepository.update(updatedProject))
              .thenThrow(Exception('Failed to update'));
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) =>
            bloc.add(ProjectUpdateRequested(project: updatedProject)),
        expect: () => [
          const ProjectLoading(),
          const ProjectFailure(message: 'Exception: Failed to update'),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.update(updatedProject)).called(1);
        },
      );
    });

    group('ProjectDeleteRequested', () {
      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectLoaded] when project is deleted successfully',
        build: () {
          when(() => mockProjectRepository.delete('1'))
              .thenAnswer((_) async => {});
          when(() => mockProjectRepository.findAll())
              .thenAnswer((_) async => [project2, project3]);
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const ProjectDeleteRequested(projectId: '1')),
        expect: () => [
          const ProjectLoading(),
          ProjectLoaded(projects: [project2, project3]),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.delete('1')).called(1);
          verify(() => mockProjectRepository.findAll()).called(1);
        },
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectFailure] when deletion fails',
        build: () {
          when(() => mockProjectRepository.delete('1'))
              .thenThrow(Exception('Failed to delete'));
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const ProjectDeleteRequested(projectId: '1')),
        expect: () => [
          const ProjectLoading(),
          const ProjectFailure(message: 'Exception: Failed to delete'),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.delete('1')).called(1);
        },
      );
    });

    group('ProjectSearchRequested', () {
      const searchQuery = 'test';

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectLoaded] when search succeeds',
        build: () {
          when(() => mockProjectRepository.search(searchQuery))
              .thenAnswer((_) async => [project1]);
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) =>
            bloc.add(const ProjectSearchRequested(query: searchQuery)),
        expect: () => [
          const ProjectLoading(),
          ProjectLoaded(projects: [project1]),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.search(searchQuery)).called(1);
        },
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectFailure] when search fails',
        build: () {
          when(() => mockProjectRepository.search(searchQuery))
              .thenThrow(Exception('Search failed'));
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) =>
            bloc.add(const ProjectSearchRequested(query: searchQuery)),
        expect: () => [
          const ProjectLoading(),
          const ProjectFailure(message: 'Exception: Search failed'),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.search(searchQuery)).called(1);
        },
      );
    });

    group('ProjectFilterByStatusRequested', () {
      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectLoaded] with filter when filtering succeeds',
        build: () {
          when(() => mockProjectRepository.findAll(
                  status: ProjectStatus.completed))
              .thenAnswer((_) async => [project2]);
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(
          const ProjectFilterByStatusRequested(status: ProjectStatus.completed),
        ),
        expect: () => [
          const ProjectLoading(),
          ProjectLoaded(projects: [project2], filter: ProjectStatus.completed),
        ],
        verify: (_) {
          verify(
            () =>
                mockProjectRepository.findAll(status: ProjectStatus.completed),
          ).called(1);
        },
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectFailure] when filtering fails',
        build: () {
          when(() =>
                  mockProjectRepository.findAll(status: ProjectStatus.active))
              .thenThrow(Exception('Filter failed'));
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(
          const ProjectFilterByStatusRequested(status: ProjectStatus.active),
        ),
        expect: () => [
          const ProjectLoading(),
          const ProjectFailure(message: 'Exception: Filter failed'),
        ],
        verify: (_) {
          verify(() =>
                  mockProjectRepository.findAll(status: ProjectStatus.active))
              .called(1);
        },
      );
    });

    group('ProjectClearFiltersRequested', () {
      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectLoaded] without filter when clearing succeeds',
        build: () {
          when(() => mockProjectRepository.findAll())
              .thenAnswer((_) async => [project1, project2, project3]);
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const ProjectClearFiltersRequested()),
        expect: () => [
          const ProjectLoading(),
          ProjectLoaded(projects: [project1, project2, project3]),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.findAll()).called(1);
        },
      );

      blocTest<ProjectBloc, ProjectState>(
        'emits [ProjectLoading, ProjectFailure] when clearing filters fails',
        build: () {
          when(() => mockProjectRepository.findAll())
              .thenThrow(Exception('Failed to clear filters'));
          return ProjectBloc(
            projectRepository: mockProjectRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const ProjectClearFiltersRequested()),
        expect: () => [
          const ProjectLoading(),
          const ProjectFailure(message: 'Exception: Failed to clear filters'),
        ],
        verify: (_) {
          verify(() => mockProjectRepository.findAll()).called(1);
        },
      );
    });
  });
}
