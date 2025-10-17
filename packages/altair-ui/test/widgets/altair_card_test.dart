import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AltairCard', () {
    testWidgets('renders with child content', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: AltairCard(
              child: Text('Card Content'),
            ),
          ),
        ),
      );

      expect(find.text('Card Content'), findsOneWidget);
    });

    testWidgets('shows accent bar when enabled', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: AltairCard(
              showAccentBar: true,
              accentColor: AltairColors.accentBlue,
              child: Text('Card with Accent'),
            ),
          ),
        ),
      );

      final card = tester.widget<AltairCard>(find.byType(AltairCard));
      expect(card.showAccentBar, isTrue);
      expect(card.accentColor, AltairColors.accentBlue);
    });

    testWidgets('accent color is null when not specified',
        (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: AltairCard(
              child: Text('Default Card'),
            ),
          ),
        ),
      );

      final card = tester.widget<AltairCard>(find.byType(AltairCard));
      expect(card.accentColor, isNull);
    });

    testWidgets('hides accent bar by default', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: AltairCard(
              child: Text('Card without Accent'),
            ),
          ),
        ),
      );

      final card = tester.widget<AltairCard>(find.byType(AltairCard));
      expect(card.showAccentBar, isFalse);
    });
  });
}
