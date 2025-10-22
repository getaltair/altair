/// Border tokens for the neo-brutalist design system.
library;

import 'package:flutter/material.dart';

/// Border width constants.
class AltairBorders {
  const AltairBorders._();

  /// Standard border width: 2px (neo-brutalist)
  static const double standard = 2.0;

  /// Thin border width: 1px
  static const double thin = 1.0;

  /// Medium border width: 2px (same as standard)
  static const double medium = 2.0;

  /// Thick border width: 3px
  static const double thick = 3.0;

  /// Extra thick border width for emphasis: 4px
  static const double extraThick = 4.0;

  // Border radius constants (all set to 0 for sharp, brutalist style)
  /// Small border radius: 0px (no rounding)
  static const double radiusSmall = 0.0;

  /// Medium border radius: 0px (no rounding)
  static const double radiusMedium = 0.0;

  /// Large border radius: 0px (no rounding)
  static const double radiusLarge = 0.0;

  /// Extra large border radius: 0px (no rounding)
  static const double radiusXLarge = 0.0;

  // Neo-brutalist shadow system
  /// Standard hard-edge shadow: 4px offset
  static const BoxShadow shadow = BoxShadow(
    color: Color(0xFF000000),
    offset: Offset(4, 4),
    blurRadius: 0,
  );

  /// Hover hard-edge shadow: 6px offset
  static const BoxShadow shadowHover = BoxShadow(
    color: Color(0xFF000000),
    offset: Offset(6, 6),
    blurRadius: 0,
  );

  /// Small hard-edge shadow: 3px offset
  static const BoxShadow shadowSmall = BoxShadow(
    color: Color(0xFF000000),
    offset: Offset(3, 3),
    blurRadius: 0,
  );
}
