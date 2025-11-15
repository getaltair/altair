import 'package:flutter/material.dart';

/// Badge showing WIP limit status
class WipLimitBadge extends StatelessWidget {
  final int currentCount;
  final int limit;
  final bool isViolation;

  const WipLimitBadge({
    super.key,
    required this.currentCount,
    required this.limit,
    this.isViolation = false,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: isViolation ? Colors.red.shade100 : Colors.grey.shade200,
        borderRadius: BorderRadius.circular(12),
        border: isViolation ? Border.all(color: Colors.red, width: 2) : null,
      ),
      child: Text(
        '$currentCount/$limit',
        style: TextStyle(
          fontSize: 12,
          fontWeight: FontWeight.bold,
          color: isViolation ? Colors.red.shade900 : Colors.grey.shade700,
        ),
      ),
    );
  }
}
