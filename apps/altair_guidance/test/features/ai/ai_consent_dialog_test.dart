import 'package:altair_guidance/features/ai/ai_consent_dialog.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  group('showAIConsentDialog', () {
    setUp(() {
      SharedPreferences.setMockInitialValues({});
    });

    testWidgets('returns true immediately if user has already consented',
        (tester) async {
      SharedPreferences.setMockInitialValues({'ai_features_consent': true});

      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) {
              return ElevatedButton(
                onPressed: () async {
                  final result = await showAIConsentDialog(context);
                  expect(result, true);
                },
                child: const Text('Test'),
              );
            },
          ),
        ),
      );

      await tester.tap(find.text('Test'));
      await tester.pumpAndSettle();

      // Dialog should not be shown
      expect(find.text('AI Features Privacy Notice'), findsNothing);
    });

    testWidgets('shows consent dialog if user has not consented',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) {
              return ElevatedButton(
                onPressed: () async {
                  showAIConsentDialog(context);
                },
                child: const Text('Test'),
              );
            },
          ),
        ),
      );

      await tester.tap(find.text('Test'));
      await tester.pumpAndSettle();

      // Dialog should be shown
      expect(find.text('AI Features Privacy Notice'), findsOneWidget);
      expect(find.text('Decline'), findsOneWidget);
      expect(find.text('Accept & Continue'), findsOneWidget);
    });

    testWidgets('returns true and saves consent when user accepts',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) {
              return ElevatedButton(
                onPressed: () async {
                  final result = await showAIConsentDialog(context);
                  expect(result, true);
                },
                child: const Text('Test'),
              );
            },
          ),
        ),
      );

      await tester.tap(find.text('Test'));
      await tester.pumpAndSettle();

      // Tap Accept button
      await tester.tap(find.text('Accept & Continue'));
      await tester.pumpAndSettle();

      // Verify consent was saved
      final prefs = await SharedPreferences.getInstance();
      expect(prefs.getBool('ai_features_consent'), true);
    });

    testWidgets('returns false when user declines', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) {
              return ElevatedButton(
                onPressed: () async {
                  final result = await showAIConsentDialog(context);
                  expect(result, false);
                },
                child: const Text('Test'),
              );
            },
          ),
        ),
      );

      await tester.tap(find.text('Test'));
      await tester.pumpAndSettle();

      // Tap Decline button
      await tester.tap(find.text('Decline'));
      await tester.pumpAndSettle();

      // Verify consent was not saved
      final prefs = await SharedPreferences.getInstance();
      expect(prefs.getBool('ai_features_consent'), isNull);
    });

    testWidgets('dialog is not dismissible by tapping barrier',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) {
              return ElevatedButton(
                onPressed: () async {
                  showAIConsentDialog(context);
                },
                child: const Text('Test'),
              );
            },
          ),
        ),
      );

      await tester.tap(find.text('Test'));
      await tester.pumpAndSettle();

      // Try to dismiss by tapping outside
      await tester.tapAt(const Offset(10, 10));
      await tester.pumpAndSettle();

      // Dialog should still be visible
      expect(find.text('AI Features Privacy Notice'), findsOneWidget);
    });

    testWidgets('displays privacy disclosure content', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) {
              return ElevatedButton(
                onPressed: () async {
                  showAIConsentDialog(context);
                },
                child: const Text('Test'),
              );
            },
          ),
        ),
      );

      await tester.tap(find.text('Test'));
      await tester.pumpAndSettle();

      // Verify key privacy disclosure content
      expect(
        find.text(
          'AI-powered features use external services to analyze your tasks and provide suggestions.',
        ),
        findsOneWidget,
      );
      expect(find.text('What data is sent:'), findsOneWidget);
      expect(find.text('• Task titles and descriptions'), findsOneWidget);
      expect(find.text('What is NOT sent:'), findsOneWidget);
      expect(find.text('• Your personal information'), findsOneWidget);
      expect(
        find.textContaining('Do not include sensitive information'),
        findsOneWidget,
      );
    });

    testWidgets('displays privacy icon', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Builder(
            builder: (context) {
              return ElevatedButton(
                onPressed: () async {
                  showAIConsentDialog(context);
                },
                child: const Text('Test'),
              );
            },
          ),
        ),
      );

      await tester.tap(find.text('Test'));
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.privacy_tip), findsOneWidget);
    });
  });

  group('resetAIConsent', () {
    testWidgets('removes consent from storage', (tester) async {
      SharedPreferences.setMockInitialValues({'ai_features_consent': true});

      await resetAIConsent();

      final prefs = await SharedPreferences.getInstance();
      expect(prefs.getBool('ai_features_consent'), isNull);
    });
  });
}
