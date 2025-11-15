import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'features/quest_board/presentation/screens/board_screen.dart';

void main() {
  runApp(const ProviderScope(child: AltairApp()));
}

class AltairApp extends StatelessWidget {
  const AltairApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Altair',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
      ),
      home: const BoardScreen(),
      debugShowCheckedModeBanner: false,
    );
  }
}
