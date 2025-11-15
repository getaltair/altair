import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/filter_provider.dart';

/// Filter bar component for the quest board
class FilterBar extends ConsumerWidget {
  const FilterBar({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final filters = ref.watch(activeFiltersProvider);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      decoration: BoxDecoration(
        color: Colors.grey.shade100,
        border: Border(
          bottom: BorderSide(color: Colors.grey.shade300),
        ),
      ),
      child: Row(
        children: [
          const Icon(Icons.filter_list, size: 20),
          const SizedBox(width: 8),
          Expanded(
            child: SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                children: [
                  // Energy filter
                  _EnergyFilterChip(),
                  const SizedBox(width: 8),
                  // Tags filter (placeholder)
                  _TagsFilterChip(),
                  const SizedBox(width: 8),
                  // Epic filter (placeholder)
                  _EpicFilterChip(),
                  const SizedBox(width: 8),
                  // Clear filters button
                  if (filters.hasActiveFilters)
                    ActionChip(
                      label: const Text('Clear'),
                      onPressed: () {
                        ref.read(activeFiltersProvider.notifier).clearFilters();
                      },
                      avatar: const Icon(Icons.clear, size: 16),
                    ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _EnergyFilterChip extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final filters = ref.watch(activeFiltersProvider);
    final hasEnergyFilter = filters.energyLevels != null;

    return FilterChip(
      label: const Text('Energy'),
      selected: hasEnergyFilter,
      onSelected: (selected) {
        // Always open the dialog to allow modification of selections
        _showEnergyFilterDialog(context, ref);
      },
    );
  }

  void _showEnergyFilterDialog(BuildContext context, WidgetRef ref) {
    final currentFilters = ref.read(activeFiltersProvider);
    final selectedLevels = currentFilters.energyLevels?.toSet() ?? <int>{};

    showDialog(
      context: context,
      builder: (context) {
        return StatefulBuilder(
          builder: (context, setState) {
            return AlertDialog(
              title: const Text('Filter by Energy Level'),
              content: SizedBox(
                width: double.maxFinite,
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: List.generate(5, (index) {
                    final level = index + 1;
                    return CheckboxListTile(
                      title: Row(
                        children: [
                          Text('Level $level'),
                          const SizedBox(width: 8),
                          ...List.generate(5, (i) {
                            return Icon(
                              Icons.bolt,
                              size: 16,
                              color: i < level ? Colors.amber : Colors.grey.shade300,
                            );
                          }),
                        ],
                      ),
                      value: selectedLevels.contains(level),
                      onChanged: (checked) {
                        if (checked != null) {
                          setState(() {
                            if (checked) {
                              selectedLevels.add(level);
                            } else {
                              selectedLevels.remove(level);
                            }
                          });
                        }
                      },
                    );
                  }),
                ),
              ),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(context),
                  child: const Text('Cancel'),
                ),
                TextButton(
                  onPressed: () {
                    ref.read(activeFiltersProvider.notifier).updateFilters(
                          currentFilters.copyWith(
                            energyLevels: selectedLevels.isEmpty ? null : selectedLevels,
                          ),
                        );
                    Navigator.pop(context);
                  },
                  child: const Text('Apply'),
                ),
              ],
            );
          },
        );
      },
    );
  }
}

class _TagsFilterChip extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final filters = ref.watch(activeFiltersProvider);
    final hasTagFilter = filters.tags != null;

    return FilterChip(
      label: const Text('Tags'),
      selected: hasTagFilter,
      onSelected: (selected) {
        if (!selected) {
          ref.read(activeFiltersProvider.notifier).updateFilters(
              ref.read(activeFiltersProvider).copyWith(tags: null));
        }
      },
    );
  }
}

class _EpicFilterChip extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final filters = ref.watch(activeFiltersProvider);
    final hasEpicFilter = filters.epicId != null;

    return FilterChip(
      label: const Text('Epic'),
      selected: hasEpicFilter,
      onSelected: (selected) {
        if (!selected) {
          ref.read(activeFiltersProvider.notifier).updateFilters(
              ref.read(activeFiltersProvider).copyWith(epicId: null));
        }
      },
    );
  }
}
