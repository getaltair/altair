/// Neo-brutalist text field widget.
library;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../tokens/borders.dart';
import '../tokens/colors.dart';
import '../tokens/spacing.dart';

/// Neo-brutalist text field widget following Altair design system.
class AltairTextField extends StatefulWidget {
  /// Creates an Altair text field.
  const AltairTextField({
    super.key,
    this.controller,
    this.label,
    this.hint,
    this.errorText,
    this.obscureText = false,
    this.maxLines = 1,
    this.autofocus = false,
    this.onChanged,
    this.onSubmitted,
    this.keyboardType,
    this.inputFormatters,
    this.prefixIcon,
    this.suffixIcon,
  });

  /// Text editing controller.
  final TextEditingController? controller;

  /// Label text displayed above the field.
  final String? label;

  /// Hint text displayed when field is empty.
  final String? hint;

  /// Error text displayed below the field.
  final String? errorText;

  /// Whether to obscure the text (for passwords).
  final bool obscureText;

  /// Maximum number of lines.
  final int maxLines;

  /// Whether to autofocus this field.
  final bool autofocus;

  /// Callback when text changes.
  final ValueChanged<String>? onChanged;

  /// Callback when user submits (presses enter).
  final ValueChanged<String>? onSubmitted;

  /// Keyboard type.
  final TextInputType? keyboardType;

  /// Input formatters to restrict/format input.
  final List<TextInputFormatter>? inputFormatters;

  /// Icon displayed at the start of the field.
  final Widget? prefixIcon;

  /// Icon displayed at the end of the field.
  final Widget? suffixIcon;

  @override
  State<AltairTextField> createState() => _AltairTextFieldState();
}

class _AltairTextFieldState extends State<AltairTextField> {
  bool _isFocused = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isDark = theme.brightness == Brightness.dark;

    final borderColor =
        isDark ? AltairColors.darkBorderColor : AltairColors.lightBorderColor;

    final backgroundColor =
        isDark ? AltairColors.darkBgSecondary : AltairColors.lightBgSecondary;

    final focusBackgroundColor = widget.errorText != null
        ? AltairColors.error.withValues(alpha: 0.1)
        : backgroundColor;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.label != null) ...[
          Text(
            widget.label!,
            style: theme.textTheme.labelLarge,
          ),
          const SizedBox(height: AltairSpacing.xs),
        ],
        Focus(
          onFocusChange: (focused) {
            setState(() => _isFocused = focused);
          },
          child: TextField(
            controller: widget.controller,
            obscureText: widget.obscureText,
            maxLines: widget.maxLines,
            autofocus: widget.autofocus,
            onChanged: widget.onChanged,
            onSubmitted: widget.onSubmitted,
            keyboardType: widget.keyboardType,
            inputFormatters: widget.inputFormatters,
            style: theme.textTheme.bodyMedium,
            decoration: InputDecoration(
              hintText: widget.hint,
              filled: true,
              fillColor: _isFocused ? focusBackgroundColor : backgroundColor,
              prefixIcon: widget.prefixIcon,
              suffixIcon: widget.suffixIcon,
              border: OutlineInputBorder(
                borderRadius: BorderRadius.zero,
                borderSide: BorderSide(
                  color: borderColor,
                  width: AltairBorders.thin,
                ),
              ),
              enabledBorder: OutlineInputBorder(
                borderRadius: BorderRadius.zero,
                borderSide: BorderSide(
                  color: borderColor,
                  width: AltairBorders.thin,
                ),
              ),
              focusedBorder: OutlineInputBorder(
                borderRadius: BorderRadius.zero,
                borderSide: BorderSide(
                  color: borderColor,
                  width: AltairBorders.standard,
                ),
              ),
              errorBorder: const OutlineInputBorder(
                borderRadius: BorderRadius.zero,
                borderSide: BorderSide(
                  color: AltairColors.error,
                  width: AltairBorders.thin,
                ),
              ),
              focusedErrorBorder: const OutlineInputBorder(
                borderRadius: BorderRadius.zero,
                borderSide: BorderSide(
                  color: AltairColors.error,
                  width: AltairBorders.standard,
                ),
              ),
              contentPadding: const EdgeInsets.symmetric(
                horizontal: AltairSpacing.md,
                vertical: AltairSpacing.sm,
              ),
            ),
          ),
        ),
        if (widget.errorText != null) ...[
          const SizedBox(height: AltairSpacing.xs),
          Text(
            widget.errorText!,
            style: theme.textTheme.bodySmall?.copyWith(
              color: AltairColors.error,
            ),
          ),
        ],
      ],
    );
  }
}
