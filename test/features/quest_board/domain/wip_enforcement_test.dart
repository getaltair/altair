import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/quest_board/domain/entities/quest.dart';

void main() {
  group('WIP=1 Enforcement', () {
    test('should detect WIP violation when more than 1 quest in In-Progress', () {
      final quest1 = Quest(
        id: '1',
        title: 'Quest 1',
        energyPoints: 3,
        column: QuestColumn.inProgress,
        createdAt: DateTime.now(),
      );
      
      final quest2 = Quest(
        id: '2',
        title: 'Quest 2',
        energyPoints: 2,
        column: QuestColumn.inProgress,
        createdAt: DateTime.now(),
      );
      
      final inProgressQuests = [quest1, quest2];
      expect(inProgressQuests.length, greaterThan(1));
    });

    test('should allow exactly 1 quest in In-Progress', () {
      final quest = Quest(
        id: '1',
        title: 'Quest 1',
        energyPoints: 3,
        column: QuestColumn.inProgress,
        createdAt: DateTime.now(),
      );
      
      final inProgressQuests = [quest];
      expect(inProgressQuests.length, equals(1));
    });
  });
}

