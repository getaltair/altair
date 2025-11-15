import 'package:flutter/material.dart';

/// Widget to display energy level (1-5) with lightning bolt icons
class EnergyIndicator extends StatelessWidget {
  final int energyPoints;
  final double size;

  const EnergyIndicator({
    super.key,
    required this.energyPoints,
    this.size = 16.0,
  }) : assert(energyPoints >= 1 && energyPoints <= 5,
            'Energy points must be between 1 and 5');

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: List.generate(
        5,
        (index) => Icon(
          Icons.bolt,
          size: size,
          color: index < energyPoints ? Colors.amber : Colors.grey.shade300,
        ),
      ),
    );
  }
}
