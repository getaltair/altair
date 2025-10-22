/// Neo-brutalist tag chip widget.
library;

import 'package:flutter/material.dart';

import '../tokens/borders.dart';
import '../tokens/colors.dart';
import '../tokens/spacing.dart';
import '../tokens/typography.dart';

/// Neo-brutalist tag chip widget following Altair design system.
class AltairTagChip extends StatelessWidget {
  /// Creates an Altair tag chip.
  const AltairTagChip({
    required this.label,
    super.key,
    this.color,
    this.onDelete,
    this.onTap,
    this.selected = false,
  });

  /// The label text for the tag.
  final String label;

  /// Optional custom color for the tag.
  final Color? color;

  /// Callback when delete button is pressed.
  final VoidCallback? onDelete;

  /// Callback when chip is tapped.
  final VoidCallback? onTap;

  /// Whether the tag is selected.
  final bool selected;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final effectiveColor = color ?? AltairColors.accentOrange;

    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.zero,
        child: Container(
          padding: const EdgeInsets.symmetric(
            horizontal: AltairSpacing.xs,
            vertical: AltairSpacing.xxs,
          ),
          decoration: BoxDecoration(
            color: selected ? effectiveColor : effectiveColor.withValues(alpha: 0.2),
            border: Border.all(
              color: isDark ? AltairColors.borderDark : AltairColors.borderLight,
              width: selected ? AltairBorders.thick : AltairBorders.medium,
            ),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                label,
                style: AltairTypography.bodySmall.copyWith(
                  fontWeight: selected ? FontWeight.bold : FontWeight.normal,
                  color: selected
                      ? AltairColors.textDark
                      : (isDark ? AltairColors.textLight : AltairColors.textDark),
                ),
              ),
              if (onDelete != null) ...[
                const SizedBox(width: AltairSpacing.xxs),
                GestureDetector(
                  onTap: onDelete,
                  child: Icon(
                    Icons.close,
                    size: 14,
                    color: selected
                        ? AltairColors.textDark
                        : (isDark ? AltairColors.textLight : AltairColors.textDark),
                  ),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
