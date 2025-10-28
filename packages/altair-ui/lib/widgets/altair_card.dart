/// Neo-brutalist card widget.
library;

import 'package:flutter/material.dart';

import '../tokens/borders.dart';
import '../tokens/colors.dart';
import '../tokens/spacing.dart';

/// Neo-brutalist card widget following Altair design system.
class AltairCard extends StatefulWidget {
  /// Creates an Altair card.
  const AltairCard({
    required this.child,
    super.key,
    this.onTap,
    this.padding = const EdgeInsets.all(AltairSpacing.md),
    this.accentColor,
    this.showAccentBar = false,
  });

  /// Child widget to display inside the card.
  final Widget child;

  /// Callback when card is tapped.
  final VoidCallback? onTap;

  /// Internal padding of the card.
  final EdgeInsets padding;

  /// Accent color for the card (used for accent bar if shown).
  final Color? accentColor;

  /// Whether to show an accent color bar on the left.
  final bool showAccentBar;

  @override
  State<AltairCard> createState() => _AltairCardState();
}

class _AltairCardState extends State<AltairCard> {
  bool _isHovering = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    final borderColor = isDark
        ? AltairColors.darkBorderColor
        : AltairColors.lightBorderColor;

    final backgroundColor = isDark
        ? AltairColors.darkBgSecondary
        : AltairColors.lightBgSecondary;

    Widget cardContent = MouseRegion(
      onEnter: (_) => setState(() => _isHovering = true),
      onExit: (_) => setState(() => _isHovering = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        transform: _isHovering
            ? Matrix4.translationValues(-1.0, -1.0, 0.0)
            : Matrix4.identity(),
        padding: widget.padding,
        decoration: BoxDecoration(
          color: backgroundColor,
          border: Border.all(color: borderColor, width: AltairBorders.standard),
          boxShadow: [
            _isHovering ? AltairBorders.shadowHover : AltairBorders.shadow,
          ],
        ),
        child: widget.child,
      ),
    );

    if (widget.showAccentBar && widget.accentColor != null) {
      cardContent = MouseRegion(
        onEnter: (_) => setState(() => _isHovering = true),
        onExit: (_) => setState(() => _isHovering = false),
        child: Stack(
          children: [
            AnimatedContainer(
              duration: const Duration(milliseconds: 150),
              transform: _isHovering
                  ? Matrix4.translationValues(-1.0, -1.0, 0.0)
                  : Matrix4.identity(),
              padding: widget.padding,
              decoration: BoxDecoration(
                color: backgroundColor,
                border: Border.all(
                  color: borderColor,
                  width: AltairBorders.standard,
                ),
                boxShadow: [
                  _isHovering
                      ? AltairBorders.shadowHover
                      : AltairBorders.shadow,
                ],
              ),
              child: widget.child,
            ),
            Positioned(
              left: 0,
              top: 0,
              bottom: 0,
              child: Container(
                width: AltairBorders.thick,
                color: widget.accentColor,
              ),
            ),
          ],
        ),
      );
    }

    if (widget.onTap != null) {
      return Material(
        color: Colors.transparent,
        child: InkWell(onTap: widget.onTap, child: cardContent),
      );
    }

    return cardContent;
  }
}
