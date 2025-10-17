import 'package:altair_guidance/main.dart';
import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
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
      await tester.pumpAndSettle();

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
    testWidgets('displays app bar with title', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      expect(find.widgetWithText(AppBar, 'Altair Guidance'), findsOneWidget);
    });

    testWidgets('displays welcome message', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      expect(find.text('Welcome to Altair Guidance'), findsOneWidget);
      expect(find.text('ADHD-friendly task management'), findsOneWidget);
    });

    testWidgets('displays Get Started button', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      expect(find.widgetWithText(AltairButton, 'Get Started'), findsOneWidget);
    });

    testWidgets('Get Started button shows coming soon snackbar',
        (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      await tester.tap(find.widgetWithText(AltairButton, 'Get Started'));
      await tester.pump();

      expect(find.text('Coming soon!'), findsOneWidget);
      expect(find.byType(SnackBar), findsOneWidget);
    });

    testWidgets('displays Quick Capture card', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      expect(find.text('Quick Capture'), findsOneWidget);
      expect(find.text('< 3 second thought-to-save'), findsOneWidget);
    });

    testWidgets('Quick Capture card has accent bar',
        (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      final cardFinder = find.ancestor(
        of: find.text('Quick Capture'),
        matching: find.byType(AltairCard),
      );

      expect(cardFinder, findsOneWidget);

      final card = tester.widget<AltairCard>(cardFinder);
      expect(card.showAccentBar, isTrue);
      expect(card.accentColor, AltairColors.accentBlue);
    });

    testWidgets('uses centered column layout', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      // Find the Center widget in the Scaffold's body
      final centerFinder = find.descendant(
        of: find.byType(Scaffold),
        matching: find.byType(Center),
      );

      expect(centerFinder, findsOneWidget);

      // Find the main Column (the one with mainAxisAlignment.center)
      final columns = find.byType(Column);
      bool foundCenteredColumn = false;

      for (final element in columns.evaluate()) {
        final column = element.widget as Column;
        if (column.mainAxisAlignment == MainAxisAlignment.center) {
          foundCenteredColumn = true;
          break;
        }
      }

      expect(foundCenteredColumn, isTrue);
    });

    testWidgets('applies proper spacing between elements',
        (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HomePage(),
        ),
      );

      // Find SizedBox widgets that provide spacing
      final sizedBoxes = find.byType(SizedBox);
      expect(sizedBoxes, findsWidgets);
    });
  });
}
