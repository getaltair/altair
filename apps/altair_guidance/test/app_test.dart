import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/task/task_bloc.dart';
import 'package:altair_guidance/bloc/task/task_event.dart';
import 'package:altair_guidance/main.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockTaskRepository extends Mock implements TaskRepository {}

class FakeTask extends Fake implements Task {}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeTask());
  });

  group('AltairGuidanceApp', () {
    testWidgets('renders without crashing', (WidgetTester tester) async {
      await tester.pumpWidget(const AltairGuidanceApp());
      expect(find.byType(MaterialApp), findsOneWidget);
    });

    testWidgets('has correct title', (WidgetTester tester) async {
      await tester.pumpWidget(const AltairGuidanceApp());

      final materialApp = tester.widget<MaterialApp>(
        find.byType(MaterialApp),
      );

      expect(materialApp.title, 'Altair Guidance');
    });

    testWidgets('uses Altair theme', (WidgetTester tester) async {
      await tester.pumpWidget(const AltairGuidanceApp());

      final materialApp = tester.widget<MaterialApp>(
        find.byType(MaterialApp),
      );

      expect(materialApp.theme, isNotNull);
      expect(materialApp.darkTheme, isNotNull);
      expect(materialApp.themeMode, ThemeMode.system);
    });

    testWidgets('shows HomePage as home', (WidgetTester tester) async {
      await tester.pumpWidget(const AltairGuidanceApp());
      await tester.pump();

      expect(find.byType(HomePage), findsOneWidget);
    });

    testWidgets('hides debug banner', (WidgetTester tester) async {
      await tester.pumpWidget(const AltairGuidanceApp());

      final materialApp = tester.widget<MaterialApp>(
        find.byType(MaterialApp),
      );

      expect(materialApp.debugShowCheckedModeBanner, isFalse);
    });
  });

  group('HomePage', () {
    late MockTaskRepository mockTaskRepository;

    setUp(() {
      mockTaskRepository = MockTaskRepository();
    });

    Widget createHomePage() {
      return MaterialApp(
        home: BlocProvider(
          create: (_) => TaskBloc(
            taskRepository: mockTaskRepository,
          )..add(const TaskLoadRequested()),
          child: const HomePage(),
        ),
      );
    }

    testWidgets('displays app bar with title', (WidgetTester tester) async {
      when(() => mockTaskRepository.findAll()).thenAnswer((_) async => []);

      await tester.pumpWidget(createHomePage());
      await tester.pumpAndSettle();

      expect(find.widgetWithText(AppBar, 'Tasks'), findsOneWidget);
    });

    testWidgets('displays Quick Capture widget', (WidgetTester tester) async {
      when(() => mockTaskRepository.findAll()).thenAnswer((_) async => []);

      await tester.pumpWidget(createHomePage());
      await tester.pumpAndSettle();

      expect(find.byType(AltairQuickCapture), findsOneWidget);
      expect(find.text('Quick capture (< 3 seconds)...'), findsOneWidget);
    });

    testWidgets('displays empty state when no tasks',
        (WidgetTester tester) async {
      when(() => mockTaskRepository.findAll()).thenAnswer((_) async => []);

      await tester.pumpWidget(createHomePage());
      await tester.pumpAndSettle();

      expect(find.text('No tasks yet'), findsOneWidget);
      expect(
        find.text('Use quick capture above to add your first task'),
        findsOneWidget,
      );
    });

    testWidgets('displays tasks when loaded', (WidgetTester tester) async {
      final now = DateTime.now();
      final task = Task(
        id: '1',
        title: 'Test Task',
        createdAt: now,
        updatedAt: now,
        status: TaskStatus.todo,
        priority: 3,
      );

      when(() => mockTaskRepository.findAll()).thenAnswer((_) async => [task]);

      await tester.pumpWidget(createHomePage());
      await tester.pumpAndSettle();

      expect(find.text('Test Task'), findsOneWidget);
      expect(find.byType(Checkbox), findsOneWidget);
    });

    testWidgets('has filter button in app bar', (WidgetTester tester) async {
      when(() => mockTaskRepository.findAll()).thenAnswer((_) async => []);

      await tester.pumpWidget(createHomePage());
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.filter_list), findsOneWidget);
    });
  });
}
