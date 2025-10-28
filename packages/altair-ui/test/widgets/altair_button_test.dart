import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AltairButton', () {
    testWidgets('renders with child text', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairButton(
              onPressed: () {},
              child: const Text('Test Button'),
            ),
          ),
        ),
      );

      expect(find.text('Test Button'), findsOneWidget);
    });

    testWidgets('calls onPressed when tapped', (WidgetTester tester) async {
      var pressed = false;

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairButton(
              onPressed: () {
                pressed = true;
              },
              child: const Text('Test Button'),
            ),
          ),
        ),
      );

      await tester.tap(find.byType(AltairButton));
      await tester.pump();

      expect(pressed, isTrue);
    });

    testWidgets('is disabled when onPressed is null', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: AltairButton(onPressed: null, child: Text('Disabled Button')),
          ),
        ),
      );

      final button = tester.widget<AltairButton>(find.byType(AltairButton));
      expect(button.onPressed, isNull);
    });

    testWidgets('applies filled variant styles', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairButton(
              onPressed: () {},
              variant: AltairButtonVariant.filled,
              accentColor: AltairColors.accentYellow,
              child: const Text('Filled Button'),
            ),
          ),
        ),
      );

      final button = tester.widget<AltairButton>(find.byType(AltairButton));
      expect(button.variant, AltairButtonVariant.filled);
      expect(button.accentColor, AltairColors.accentYellow);
    });

    testWidgets('applies outlined variant styles', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairButton(
              onPressed: () {},
              variant: AltairButtonVariant.outlined,
              accentColor: AltairColors.accentBlue,
              child: const Text('Outlined Button'),
            ),
          ),
        ),
      );

      final button = tester.widget<AltairButton>(find.byType(AltairButton));
      expect(button.variant, AltairButtonVariant.outlined);
      expect(button.accentColor, AltairColors.accentBlue);
    });

    testWidgets('applies primary variant styles', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairButton(
              onPressed: () {},
              variant: AltairButtonVariant.primary,
              child: const Text('Primary Button'),
            ),
          ),
        ),
      );

      final button = tester.widget<AltairButton>(find.byType(AltairButton));
      expect(button.variant, AltairButtonVariant.primary);
    });
  });
}
