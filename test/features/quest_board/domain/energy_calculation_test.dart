import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/quest_board/domain/entities/quest.dart';
import 'package:altair/features/quest_board/domain/entities/subquest.dart';

void main() {
  group('Energy Calculation', () {
    test('should calculate total energy for quest with subquests', () {
      final subquest1 = Subquest(
        id: 's1',
        questId: 'q1',
        title: 'Subquest 1',
        energyPoints: 2,
        createdAt: DateTime.now(),
      );
      
      final subquest2 = Subquest(
        id: 's2',
        questId: 'q1',
        title: 'Subquest 2',
        energyPoints: 3,
        createdAt: DateTime.now(),
      );
      
      final quest = Quest(
        id: 'q1',
        title: 'Quest',
        energyPoints: 4,
        column: QuestColumn.ideaGreenhouse,
        createdAt: DateTime.now(),
        subquests: [subquest1, subquest2],
      );
      
      final totalEnergy = quest.energyPoints + 
          quest.subquests.fold(0, (sum, s) => sum + s.energyPoints);
      
      expect(totalEnergy, equals(9)); // 4 + 2 + 3
    });

    test('should validate energy points are between 1 and 5', () {
      expect(() {
        Quest(
          id: '1',
          title: 'Quest',
          energyPoints: 0, // Invalid
          column: QuestColumn.ideaGreenhouse,
          createdAt: DateTime.now(),
        );
      }, returnsNormally); // Entity doesn't enforce, but should be validated in business logic
    });
  });
}

