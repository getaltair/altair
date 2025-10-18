import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AltairQuickCapture', () {
    testWidgets('renders with default properties', (tester) async {
      bool capturedCalled = false;
      String? capturedText;

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (text) {
                capturedCalled = true;
                capturedText = text;
              },
            ),
          ),
        ),
      );

      // Should render the quick capture widget
      expect(find.byType(AltairQuickCapture), findsOneWidget);

      // Should have flash icon
      expect(find.byIcon(Icons.flash_on), findsOneWidget);

      // Should have add icon initially
      expect(find.byIcon(Icons.add), findsOneWidget);

      // Should not have captured anything yet
      expect(capturedCalled, false);
    });

    testWidgets('autofocuses input field when autofocus is true', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (_) {},
              autofocus: true,
            ),
          ),
        ),
      );

      // Input should be focused
      final textField = tester.widget<TextField>(find.byType(TextField));
      expect(textField.autofocus, true);
    });

    testWidgets('calls onCapture when text is entered and submitted',
        (tester) async {
      String? capturedText;

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (text) {
                capturedText = text;
              },
            ),
          ),
        ),
      );

      // Enter text
      await tester.enterText(find.byType(TextField), 'Test task');

      // Submit via Enter key
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle();

      // Should have captured the text
      expect(capturedText, 'Test task');
    });

    testWidgets('calls onCapture when add button is tapped', (tester) async {
      String? capturedText;

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (text) {
                capturedText = text;
              },
            ),
          ),
        ),
      );

      // Enter text
      await tester.enterText(find.byType(TextField), 'Test task');

      // Tap the add button
      await tester.tap(find.byIcon(Icons.add));
      await tester.pumpAndSettle();

      // Should have captured the text
      expect(capturedText, 'Test task');
    });

    testWidgets('clears input after capture', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (_) {},
            ),
          ),
        ),
      );

      // Enter text
      await tester.enterText(find.byType(TextField), 'Test task');
      expect(find.text('Test task'), findsOneWidget);

      // Submit
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle();

      // Input should be cleared
      final textField = tester.widget<TextField>(find.byType(TextField));
      expect(textField.controller?.text, '');
    });

    testWidgets('shows checkmark icon briefly after capture', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (_) {},
            ),
          ),
        ),
      );

      // Initially shows add icon
      expect(find.byIcon(Icons.add), findsOneWidget);
      expect(find.byIcon(Icons.check), findsNothing);

      // Enter and submit text
      await tester.enterText(find.byType(TextField), 'Test task');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();

      // Should briefly show checkmark
      expect(find.byIcon(Icons.check), findsOneWidget);
      expect(find.byIcon(Icons.add), findsNothing);

      // After animation completes, should show add icon again
      await tester.pumpAndSettle();
      expect(find.byIcon(Icons.add), findsOneWidget);
      expect(find.byIcon(Icons.check), findsNothing);
    });

    testWidgets('does not capture empty text', (tester) async {
      bool capturedCalled = false;

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (_) {
                capturedCalled = true;
              },
            ),
          ),
        ),
      );

      // Submit without entering text
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle();

      // Should not have captured
      expect(capturedCalled, false);
    });

    testWidgets('does not capture whitespace-only text', (tester) async {
      bool capturedCalled = false;

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (_) {
                capturedCalled = true;
              },
            ),
          ),
        ),
      );

      // Enter only whitespace
      await tester.enterText(find.byType(TextField), '   ');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle();

      // Should not have captured
      expect(capturedCalled, false);
    });

    testWidgets('uses custom hint text when provided', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (_) {},
              hint: 'Custom hint',
            ),
          ),
        ),
      );

      // Should show custom hint
      expect(find.text('Custom hint'), findsOneWidget);
    });

    testWidgets('uses custom accent color', (tester) async {
      const customColor = Color(0xFF123456);

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AltairQuickCapture(
              onCapture: (_) {},
              accentColor: customColor,
            ),
          ),
        ),
      );

      // Flash icon should use accent color
      final flashIcon = tester.widget<Icon>(find.byIcon(Icons.flash_on));
      expect(flashIcon.color, customColor);

      // Add icon should use accent color
      final addIcon = tester.widget<Icon>(find.byIcon(Icons.add));
      expect(addIcon.color, customColor);
    });
  });
}
