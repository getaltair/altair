import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/quest_board/domain/entities/quest.dart';

void main() {
  group('Column Transition Rules', () {
    test('should allow transition from Idea Greenhouse to Quest Log', () {
      final quest = Quest(
        id: '1',
        title: 'Quest',
        energyPoints: 3,
        column: QuestColumn.ideaGreenhouse,
        createdAt: DateTime.now(),
      );
      
      // Valid transition
      final updatedQuest = quest.copyWith(column: QuestColumn.questLog);
      expect(updatedQuest.column, equals(QuestColumn.questLog));
    });

    test('should allow transition through all columns', () {
      final quest = Quest(
        id: '1',
        title: 'Quest',
        energyPoints: 3,
        column: QuestColumn.ideaGreenhouse,
        createdAt: DateTime.now(),
      );
      
      // Flow: Idea Greenhouse -> Quest Log -> This Cycle -> Next Up -> In-Progress -> Harvested
      var currentQuest = quest;
      currentQuest = currentQuest.copyWith(column: QuestColumn.questLog);
      expect(currentQuest.column, equals(QuestColumn.questLog));
      
      currentQuest = currentQuest.copyWith(column: QuestColumn.thisCycle);
      expect(currentQuest.column, equals(QuestColumn.thisCycle));
      
      currentQuest = currentQuest.copyWith(column: QuestColumn.nextUp);
      expect(currentQuest.column, equals(QuestColumn.nextUp));
      
      currentQuest = currentQuest.copyWith(column: QuestColumn.inProgress);
      expect(currentQuest.column, equals(QuestColumn.inProgress));
      
      currentQuest = currentQuest.copyWith(column: QuestColumn.harvested);
      expect(currentQuest.column, equals(QuestColumn.harvested));
    });
  });
}

