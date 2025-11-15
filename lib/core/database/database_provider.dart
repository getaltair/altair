import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../features/quest_board/data/datasources/surrealdb_datasource.dart';

/// Provider for SurrealDB datasource
final surrealdbDatasourceProvider = Provider<SurrealDbDatasource>((ref) {
  final datasource = SurrealDbDatasource();
  // Initialize on first access
  datasource.initialize();
  // Cleanup on dispose
  ref.onDispose(() {
    datasource.close();
  });
  return datasource;
});

