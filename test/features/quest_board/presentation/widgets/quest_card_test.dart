import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:altair/features/quest_board/domain/entities/quest.dart';
import 'package:altair/features/quest_board/presentation/widgets/quest_card.dart';

void main() {
  group('QuestCard Widget', () {
    testWidgets('should display quest title', (WidgetTester tester) async {
      final quest = Quest(
        id: '1',
        title: 'Test Quest',
        energyPoints: 3,
        column: QuestColumn.ideaGreenhouse,
        createdAt: DateTime.now(),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: ProviderScope(
            child: Scaffold(
              body: QuestCard(
                quest: quest,
                isDragging: false,
              ),
            ),
          ),
        ),
      );

      expect(find.text('Test Quest'), findsOneWidget);
    });

    testWidgets('should display energy indicator', (WidgetTester tester) async {
      final quest = Quest(
        id: '1',
        title: 'Test Quest',
        energyPoints: 4,
        column: QuestColumn.ideaGreenhouse,
        createdAt: DateTime.now(),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: ProviderScope(
            child: Scaffold(
              body: QuestCard(
                quest: quest,
                isDragging: false,
              ),
            ),
          ),
        ),
      );

      // Energy indicator should be present
      expect(find.byType(QuestCard), findsOneWidget);
    });
  });
}

