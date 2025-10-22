/// Neo-brutalist button widget.
library;

import 'package:flutter/material.dart';

import '../tokens/borders.dart';
import '../tokens/colors.dart';
import '../tokens/spacing.dart';

/// Button variants for different styles.
enum AltairButtonVariant {
  /// Filled button with accent background
  filled,

  /// Outlined button with transparent background
  outlined,

  /// Primary button with dark background
  primary,
}

/// Neo-brutalist button widget following Altair design system.
class AltairButton extends StatefulWidget {
  /// Creates an Altair button.
  const AltairButton({
    required this.onPressed,
    required this.child,
    super.key,
    this.variant = AltairButtonVariant.filled,
    this.accentColor,
    this.fullWidth = false,
    this.isLoading = false,
  });

  /// Callback when button is pressed.
  final VoidCallback? onPressed;

  /// Child widget (usually Text).
  final Widget child;

  /// Button variant (filled, outlined, primary).
  final AltairButtonVariant variant;

  /// Accent color for filled buttons.
  final Color? accentColor;

  /// Whether button should take full width.
  final bool fullWidth;

  /// Whether button is in loading state.
  final bool isLoading;

  @override
  State<AltairButton> createState() => _AltairButtonState();
}

class _AltairButtonState extends State<AltairButton> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    Color backgroundColor;
    Color foregroundColor;
    Color borderColor;

    switch (widget.variant) {
      case AltairButtonVariant.filled:
        backgroundColor = widget.accentColor ?? AltairColors.accentOrange;
        foregroundColor = isDark
            ? AltairColors.darkTextPrimary
            : AltairColors.lightTextPrimary;
        borderColor = isDark
            ? AltairColors.darkBorderColor
            : AltairColors.lightBorderColor;
      case AltairButtonVariant.outlined:
        backgroundColor = Colors.transparent;
        foregroundColor = isDark
            ? AltairColors.darkTextPrimary
            : AltairColors.lightTextPrimary;
        borderColor = isDark
            ? AltairColors.darkBorderColor
            : AltairColors.lightBorderColor;
      case AltairButtonVariant.primary:
        backgroundColor = isDark
            ? AltairColors.darkTextPrimary
            : AltairColors.lightTextPrimary;
        foregroundColor = isDark
            ? AltairColors.darkBgPrimary
            : AltairColors.lightBgSecondary;
        borderColor = isDark
            ? AltairColors.darkBorderColor
            : AltairColors.lightBorderColor;
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 100),
        width: widget.fullWidth ? double.infinity : null,
        transform: _isHovered && widget.onPressed != null
            ? Matrix4.translationValues(0, -2, 0)
            : Matrix4.identity(),
        child: Material(
          color: backgroundColor,
          child: InkWell(
            onTap: widget.isLoading ? null : widget.onPressed,
            child: Container(
              padding: const EdgeInsets.symmetric(
                horizontal: AltairSpacing.md,
                vertical: AltairSpacing.sm,
              ),
              decoration: BoxDecoration(
                border: Border.all(
                  color: borderColor,
                  width: AltairBorders.standard,
                ),
              ),
              child: widget.isLoading
                  ? SizedBox(
                      height: 20,
                      width: 20,
                      child: CircularProgressIndicator(
                        strokeWidth: 2,
                        valueColor: AlwaysStoppedAnimation<Color>(
                          foregroundColor,
                        ),
                      ),
                    )
                  : DefaultTextStyle(
                      style: theme.textTheme.labelLarge!.copyWith(
                        color: foregroundColor,
                      ),
                      textAlign: TextAlign.center,
                      child: widget.child,
                    ),
            ),
          ),
        ),
      ),
    );
  }
}
