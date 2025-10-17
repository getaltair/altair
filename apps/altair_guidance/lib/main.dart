/// Altair Guidance - ADHD-friendly task management.
library;

import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const AltairGuidanceApp());
}

/// Main application widget.
class AltairGuidanceApp extends StatelessWidget {
  /// Creates the Altair Guidance app.
  const AltairGuidanceApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Altair Guidance',
      debugShowCheckedModeBanner: false,
      theme: AltairTheme.lightTheme,
      darkTheme: AltairTheme.darkTheme,
      themeMode: ThemeMode.system,
      home: const HomePage(),
    );
  }
}

/// Home page of the application.
class HomePage extends StatelessWidget {
  /// Creates the home page.
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Altair Guidance'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              'Welcome to Altair Guidance',
              style: Theme.of(context).textTheme.displayMedium,
            ),
            const SizedBox(height: AltairSpacing.xl),
            Text(
              'ADHD-friendly task management',
              style: Theme.of(context).textTheme.bodyLarge,
            ),
            const SizedBox(height: AltairSpacing.xl),
            AltairButton(
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('Coming soon!')),
                );
              },
              variant: AltairButtonVariant.filled,
              accentColor: AltairColors.accentYellow,
              child: const Text('Get Started'),
            ),
            const SizedBox(height: AltairSpacing.md),
            AltairCard(
              accentColor: AltairColors.accentBlue,
              showAccentBar: true,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Quick Capture',
                    style: Theme.of(context).textTheme.headlineSmall,
                  ),
                  const SizedBox(height: AltairSpacing.sm),
                  Text(
                    '< 3 second thought-to-save',
                    style: Theme.of(context).textTheme.bodyMedium,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
