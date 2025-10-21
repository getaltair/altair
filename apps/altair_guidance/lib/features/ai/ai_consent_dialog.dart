/// AI consent dialog for privacy disclosure.
library;

import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Shows AI consent dialog and returns whether user consented.
Future<bool> showAIConsentDialog(BuildContext context) async {
  final prefs = await SharedPreferences.getInstance();
  final hasConsented = prefs.getBool('ai_features_consent') ?? false;

  if (hasConsented) {
    return true;
  }

  if (!context.mounted) return false;

  final result = await showDialog<bool>(
    context: context,
    barrierDismissible: false, // Require explicit choice for privacy
    builder: (context) => const _AIConsentDialog(),
  );

  if (result == true) {
    await prefs.setBool('ai_features_consent', true);
  }

  return result ?? false;
}

/// Dialog for AI feature consent.
class _AIConsentDialog extends StatelessWidget {
  const _AIConsentDialog();

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Row(
        children: [
          Icon(Icons.privacy_tip, color: AltairColors.accentYellow),
          const SizedBox(width: AltairSpacing.sm),
          const Text('AI Features Privacy Notice'),
        ],
      ),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'AI-powered features use external services to analyze your tasks and provide suggestions.',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: AltairSpacing.md),
            const Text('What data is sent:'),
            const SizedBox(height: AltairSpacing.xs),
            const Text('• Task titles and descriptions'),
            const Text('• Project context (if provided)'),
            const SizedBox(height: AltairSpacing.md),
            const Text('What is NOT sent:'),
            const SizedBox(height: AltairSpacing.xs),
            const Text('• Your personal information'),
            const Text('• Task completion status'),
            const Text('• Any other app data'),
            const SizedBox(height: AltairSpacing.md),
            Container(
              padding: const EdgeInsets.all(AltairSpacing.sm),
              decoration: BoxDecoration(
                color: AltairColors.accentYellow.withValues(alpha: 0.2),
                borderRadius: BorderRadius.circular(4),
                border: Border.all(
                  color: Colors.black,
                  width: AltairBorders.thin,
                ),
              ),
              child: const Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Important:',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
                  SizedBox(height: AltairSpacing.xs),
                  Text(
                    'Do not include sensitive information (passwords, personal data, confidential details) in your tasks if using AI features.',
                    style: TextStyle(fontSize: 12),
                  ),
                ],
              ),
            ),
            const SizedBox(height: AltairSpacing.md),
            const Text(
              'You can disable AI features anytime in Settings.',
              style: TextStyle(fontSize: 12, fontStyle: FontStyle.italic),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(false),
          child: const Text('Decline'),
        ),
        AltairButton(
          onPressed: () => Navigator.of(context).pop(true),
          variant: AltairButtonVariant.filled,
          accentColor: AltairColors.accentGreen,
          child: const Text('Accept & Continue'),
        ),
      ],
    );
  }
}

/// Resets AI consent (for testing or settings).
Future<void> resetAIConsent() async {
  final prefs = await SharedPreferences.getInstance();
  await prefs.remove('ai_features_consent');
}
