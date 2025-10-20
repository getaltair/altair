/// Quick capture widget for ADHD-friendly task creation.
library;

import 'package:flutter/material.dart';

import '../tokens/borders.dart';
import '../tokens/colors.dart';
import '../tokens/spacing.dart';

/// Quick capture widget optimized for < 3 second task creation.
///
/// This widget provides a minimal, distraction-free interface for
/// capturing thoughts quickly. Features:
/// - Auto-focus on mount
/// - Submit on Enter key
/// - Clear on submit
/// - Visual feedback
/// - Keyboard-first design
class AltairQuickCapture extends StatefulWidget {
  /// Creates a quick capture widget.
  const AltairQuickCapture({
    required this.onCapture,
    this.hint = 'What needs to be done?',
    this.accentColor = AltairColors.accentYellow,
    this.autofocus = true,
    this.focusNode,
    super.key,
  });

  /// Callback when a task is captured.
  /// Called with the captured text when user submits.
  final void Function(String text) onCapture;

  /// Hint text shown in the input field.
  final String hint;

  /// Accent color for the capture button and focus border.
  final Color accentColor;

  /// Whether to autofocus the input field on mount.
  final bool autofocus;

  /// Optional external focus node for programmatic focus control.
  /// If not provided, an internal focus node will be created.
  final FocusNode? focusNode;

  @override
  State<AltairQuickCapture> createState() => _AltairQuickCaptureState();
}

class _AltairQuickCaptureState extends State<AltairQuickCapture> {
  final TextEditingController _controller = TextEditingController();
  late final FocusNode _internalFocusNode;
  late final FocusNode _focusNode;
  bool _isCapturing = false;

  @override
  void initState() {
    super.initState();
    _internalFocusNode = FocusNode();
    _focusNode = widget.focusNode ?? _internalFocusNode;
  }

  @override
  void dispose() {
    _controller.dispose();
    // Only dispose the internal focus node if we created it
    if (widget.focusNode == null) {
      _internalFocusNode.dispose();
    }
    super.dispose();
  }

  void _handleCapture() {
    final text = _controller.text.trim();
    if (text.isEmpty) return;

    setState(() => _isCapturing = true);

    // Call the callback
    widget.onCapture(text);

    // Clear the field and refocus
    _controller.clear();

    // Show brief visual feedback then reset
    Future.delayed(const Duration(milliseconds: 150), () {
      if (mounted) {
        setState(() => _isCapturing = false);
        _focusNode.requestFocus();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      decoration: BoxDecoration(
        color: theme.cardColor,
        border: Border.all(
          color: _focusNode.hasFocus ? widget.accentColor : theme.dividerColor,
          width: AltairBorders.thick,
        ),
      ),
      child: Row(
        children: [
          // Quick capture icon
          Container(
            padding: const EdgeInsets.symmetric(
              horizontal: AltairSpacing.md,
              vertical: AltairSpacing.sm,
            ),
            decoration: BoxDecoration(
              color: widget.accentColor.withValues(alpha: 0.1),
              border: Border(
                right: BorderSide(
                  color: widget.accentColor,
                  width: AltairBorders.medium,
                ),
              ),
            ),
            child: Icon(
              Icons.flash_on,
              color: widget.accentColor,
              size: 24,
            ),
          ),

          // Input field
          Expanded(
            child: Focus(
              onFocusChange: (hasFocus) => setState(() {}),
              child: TextField(
                controller: _controller,
                focusNode: _focusNode,
                autofocus: widget.autofocus,
                style: const TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                ),
                decoration: InputDecoration(
                  hintText: widget.hint,
                  hintStyle: theme.textTheme.bodyMedium?.copyWith(
                    color: theme.hintColor,
                    fontWeight: FontWeight.w400,
                  ),
                  border: InputBorder.none,
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: AltairSpacing.md,
                    vertical: AltairSpacing.md,
                  ),
                ),
                onSubmitted: (_) => _handleCapture(),
                textInputAction: TextInputAction.done,
              ),
            ),
          ),

          // Capture button
          AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            decoration: BoxDecoration(
              color: _isCapturing
                  ? widget.accentColor
                  : widget.accentColor.withValues(alpha: 0.1),
              border: Border(
                left: BorderSide(
                  color: widget.accentColor,
                  width: AltairBorders.medium,
                ),
              ),
            ),
            child: Material(
              color: Colors.transparent,
              child: InkWell(
                onTap: _handleCapture,
                child: Padding(
                  padding: const EdgeInsets.symmetric(
                    horizontal: AltairSpacing.lg,
                    vertical: AltairSpacing.md,
                  ),
                  child: Icon(
                    _isCapturing ? Icons.check : Icons.add,
                    color: _isCapturing
                        ? theme.cardColor
                        : widget.accentColor,
                    size: 24,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
