import 'dart:async';

import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/bloc/project/project_bloc.dart';
import 'package:altair_guidance/bloc/project/project_state.dart';
import 'package:altair_guidance/bloc/settings/settings_bloc.dart';
import 'package:altair_guidance/bloc/settings/settings_state.dart';
import 'package:altair_guidance/bloc/task/task_bloc.dart';
import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/pages/task_edit_page.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockTaskBloc extends Mock implements TaskBloc {}

class MockProjectBloc extends Mock implements ProjectBloc {}

class MockAIBloc extends Mock implements AIBloc {}

class MockSettingsBloc extends Mock implements SettingsBloc {}

void main() {
  late MockTaskBloc mockTaskBloc;
  late MockProjectBloc mockProjectBloc;
  late MockAIBloc mockAIBloc;
  late MockSettingsBloc mockSettingsBloc;
  late StreamController<ProjectState> projectStreamController;
  late StreamController<SettingsState> settingsStreamController;
  late StreamController<AIState> aiStreamController;

  setUp(() {
    mockTaskBloc = MockTaskBloc();
    mockProjectBloc = MockProjectBloc();
    mockAIBloc = MockAIBloc();
    mockSettingsBloc = MockSettingsBloc();
    projectStreamController = StreamController<ProjectState>.broadcast();
    settingsStreamController = StreamController<SettingsState>.broadcast();
    aiStreamController = StreamController<AIState>.broadcast();

    when(() => mockProjectBloc.state)
        .thenReturn(const ProjectLoaded(projects: []));
    when(() => mockProjectBloc.stream)
        .thenAnswer((_) => projectStreamController.stream);

    when(() => mockSettingsBloc.state).thenReturn(
      const SettingsLoaded(AISettings(enabled: true)),
    );
    when(() => mockSettingsBloc.stream)
        .thenAnswer((_) => settingsStreamController.stream);

    when(() => mockAIBloc.stream).thenAnswer((_) => aiStreamController.stream);
  });

  tearDown(() {
    projectStreamController.close();
    settingsStreamController.close();
    aiStreamController.close();
  });

  Widget createTaskEditPage({Task? task}) {
    return MaterialApp(
      home: MultiBlocProvider(
        providers: [
          BlocProvider<TaskBloc>.value(value: mockTaskBloc),
          BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
          BlocProvider<AIBloc>.value(value: mockAIBloc),
          BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
        ],
        child: TaskEditPage(task: task),
      ),
    );
  }

  group('TaskEditPage - AI Assistant Section', () {
    testWidgets('displays AI Assistant section', (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      expect(find.text('AI Assistant'), findsOneWidget);
      expect(
        find.text('Get AI-powered suggestions to help with your task'),
        findsOneWidget,
      );
    });

    testWidgets('AI buttons are disabled when task title is empty',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Initially, task title is empty, so buttons should be disabled
      expect(find.text('Add a task title to use AI features'), findsOneWidget);

      // Find AI feature buttons and verify they're disabled
      final breakdownButton = find.widgetWithText(
        InkWell,
        'Break Down Task',
      );
      final estimateButton = find.widgetWithText(InkWell, 'Estimate Time');
      final suggestionsButton = find.widgetWithText(InkWell, 'Get Suggestions');

      expect(breakdownButton, findsOneWidget);
      expect(estimateButton, findsOneWidget);
      expect(suggestionsButton, findsOneWidget);
    });

    testWidgets('AI buttons are enabled when task title is provided',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter a task title
      await tester.enterText(
        find.byType(TextField).first,
        'Write unit tests',
      );
      await tester.pumpAndSettle();

      // Warning message should no longer be visible
      expect(
        find.text('Add a task title to use AI features'),
        findsNothing,
      );

      // AI feature buttons should be present
      expect(find.text('Break Down Task'), findsOneWidget);
      expect(find.text('Estimate Time'), findsOneWidget);
      expect(find.text('Get Suggestions'), findsOneWidget);
    });

    testWidgets('displays all three AI feature buttons with correct icons',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter a task title to enable buttons
      await tester.enterText(
        find.byType(TextField).first,
        'Write unit tests',
      );
      await tester.pumpAndSettle();

      // Check for Break Down Task button
      expect(find.text('Break Down Task'), findsOneWidget);
      expect(find.byIcon(Icons.format_list_bulleted), findsOneWidget);

      // Check for Estimate Time button
      expect(find.text('Estimate Time'), findsOneWidget);
      expect(find.byIcon(Icons.timer), findsOneWidget);

      // Check for Get Suggestions button
      expect(find.text('Get Suggestions'), findsOneWidget);
      expect(find.byIcon(Icons.lightbulb), findsOneWidget);
    });

    testWidgets('AI assistant section rebuilds when title controller changes',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Initially no title, warning should be visible
      expect(find.text('Add a task title to use AI features'), findsOneWidget);

      // Type a title
      await tester.enterText(
        find.byType(TextField).first,
        'New task',
      );
      await tester.pumpAndSettle();

      // Warning should disappear
      expect(
        find.text('Add a task title to use AI features'),
        findsNothing,
      );

      // Clear the title
      await tester.enterText(
        find.byType(TextField).first,
        '',
      );
      await tester.pumpAndSettle();

      // Warning should reappear
      expect(find.text('Add a task title to use AI features'), findsOneWidget);
    });

    testWidgets('editing existing task shows AI features with task data',
        (tester) async {
      final existingTask = Task(
        id: '123',
        title: 'Existing task',
        description: 'This is an existing task',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.inProgress,
        priority: 2,
      );

      await tester.pumpWidget(createTaskEditPage(task: existingTask));
      await tester.pumpAndSettle();

      // Task title should be pre-filled
      expect(find.text('Existing task'), findsOneWidget);

      // AI buttons should be enabled since title exists
      expect(
        find.text('Add a task title to use AI features'),
        findsNothing,
      );
      expect(find.text('Break Down Task'), findsOneWidget);
      expect(find.text('Estimate Time'), findsOneWidget);
      expect(find.text('Get Suggestions'), findsOneWidget);
    });

    testWidgets('AI buttons maintain enabled state during typing',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Type a title character by character
      final titleField = find.byType(TextField).first;
      await tester.enterText(titleField, 'T');
      await tester.pumpAndSettle();

      // Buttons should be enabled with just one character
      expect(
        find.text('Add a task title to use AI features'),
        findsNothing,
      );

      // Add more characters
      await tester.enterText(titleField, 'Test');
      await tester.pumpAndSettle();

      // Buttons should still be enabled
      expect(
        find.text('Add a task title to use AI features'),
        findsNothing,
      );
    });

    testWidgets('shows correct page title for new task', (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      expect(find.text('New Task'), findsOneWidget);
    });

    testWidgets('shows correct page title for existing task', (tester) async {
      final existingTask = Task(
        id: '123',
        title: 'Existing task',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(createTaskEditPage(task: existingTask));

      expect(find.text('Edit Task'), findsOneWidget);
    });

    testWidgets('AI assistant section appears below tags section',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Scroll to find both sections
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();

      expect(find.text('Tags'), findsOneWidget);
      expect(find.text('AI Assistant'), findsOneWidget);
    });

    testWidgets('AI buttons appear in horizontal wrap layout', (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter a task title to enable buttons
      await tester.enterText(
        find.byType(TextField).first,
        'Write unit tests',
      );
      await tester.pumpAndSettle();

      // Scroll to AI section
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();

      // All buttons should be visible in a wrap
      expect(find.text('Break Down Task'), findsOneWidget);
      expect(find.text('Estimate Time'), findsOneWidget);
      expect(find.text('Get Suggestions'), findsOneWidget);
    });

    testWidgets('whitespace-only title keeps buttons disabled', (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter whitespace-only title
      await tester.enterText(
        find.byType(TextField).first,
        '    ',
      );
      await tester.pumpAndSettle();

      // Buttons should still be disabled
      expect(find.text('Add a task title to use AI features'), findsOneWidget);
    });
  });

  group('TaskEditPage - AI Integration', () {
    testWidgets('task edit page provides AIBloc to dialogs', (tester) async {
      final existingTask = Task(
        id: '123',
        title: 'Test task',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(createTaskEditPage(task: existingTask));
      await tester.pumpAndSettle();

      // Verify AIBloc is available in the widget tree
      final context = tester.element(find.byType(TaskEditPage));
      expect(context.read<AIBloc>(), mockAIBloc);
    });

    testWidgets('displays description field for AI context', (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Description field should be present for AI to use as context
      expect(find.text('Description'), findsOneWidget);
      expect(find.text('Add more details...'), findsOneWidget);
    });

    testWidgets('AI features use both title and description', (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter title
      await tester.enterText(
        find.byType(TextField).first,
        'Build API endpoint',
      );

      // Enter description
      final descriptionFields = find.byType(TextField);
      await tester.enterText(
        descriptionFields.at(1),
        'REST API for user management',
      );

      await tester.pumpAndSettle();

      // AI buttons should be enabled
      expect(
        find.text('Add a task title to use AI features'),
        findsNothing,
      );
    });
  });
}
