/// Neo-brutalist tag selector widget.
library;

import 'package:flutter/material.dart';

import '../tokens/borders.dart';
import '../tokens/colors.dart';
import '../tokens/spacing.dart';
import '../tokens/typography.dart';
import 'altair_tag_chip.dart';

/// Represents a tag option.
class TagOption {
  /// Creates a tag option.
  const TagOption({
    required this.id,
    required this.name,
    this.color,
  });

  /// Tag ID.
  final String id;

  /// Tag name.
  final String name;

  /// Optional tag color.
  final Color? color;
}

/// Neo-brutalist tag selector widget for selecting multiple tags.
class AltairTagSelector extends StatefulWidget {
  /// Creates an Altair tag selector.
  const AltairTagSelector({
    required this.availableTags,
    required this.selectedTagIds,
    required this.onTagsChanged,
    super.key,
    this.label = 'Tags',
    this.onCreateTag,
  });

  /// List of available tags to choose from.
  final List<TagOption> availableTags;

  /// List of currently selected tag IDs.
  final List<String> selectedTagIds;

  /// Callback when selected tags change.
  final ValueChanged<List<String>> onTagsChanged;

  /// Optional label for the selector.
  final String label;

  /// Optional callback for creating new tags.
  final ValueChanged<String>? onCreateTag;

  @override
  State<AltairTagSelector> createState() => _AltairTagSelectorState();
}

class _AltairTagSelectorState extends State<AltairTagSelector> {
  final TextEditingController _searchController = TextEditingController();
  final FocusNode _focusNode = FocusNode();
  bool _showDropdown = false;
  List<TagOption> _filteredTags = [];

  @override
  void initState() {
    super.initState();
    _filteredTags = widget.availableTags;
    _searchController.addListener(_onSearchChanged);
    _focusNode.addListener(_onFocusChanged);
  }

  @override
  void dispose() {
    _searchController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  void _onSearchChanged() {
    final query = _searchController.text.toLowerCase();
    setState(() {
      if (query.isEmpty) {
        _filteredTags = widget.availableTags;
      } else {
        _filteredTags = widget.availableTags
            .where((tag) => tag.name.toLowerCase().contains(query))
            .toList();
      }
    });
  }

  void _onFocusChanged() {
    setState(() {
      _showDropdown = _focusNode.hasFocus;
    });
  }

  void _toggleTag(String tagId) {
    final newSelection = List<String>.from(widget.selectedTagIds);
    if (newSelection.contains(tagId)) {
      newSelection.remove(tagId);
    } else {
      newSelection.add(tagId);
    }
    widget.onTagsChanged(newSelection);
  }

  void _removeTag(String tagId) {
    final newSelection = List<String>.from(widget.selectedTagIds);
    newSelection.remove(tagId);
    widget.onTagsChanged(newSelection);
  }

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final selectedTags = widget.availableTags
        .where((tag) => widget.selectedTagIds.contains(tag.id))
        .toList();

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Label
        Text(
          widget.label,
          style: AltairTypography.bodySmall.copyWith(
            fontWeight: FontWeight.bold,
            color: isDark ? AltairColors.textLight : AltairColors.textDark,
          ),
        ),
        const SizedBox(height: AltairSpacing.xxs),

        // Selected tags
        if (selectedTags.isNotEmpty) ...[
          Wrap(
            spacing: AltairSpacing.xxs,
            runSpacing: AltairSpacing.xxs,
            children: selectedTags.map((tag) {
              return AltairTagChip(
                label: tag.name,
                color: tag.color,
                onDelete: () => _removeTag(tag.id),
              );
            }).toList(),
          ),
          const SizedBox(height: AltairSpacing.xxs),
        ],

        // Search input
        Container(
          padding: const EdgeInsets.symmetric(
            horizontal: AltairSpacing.xs,
            vertical: AltairSpacing.xxs,
          ),
          decoration: BoxDecoration(
            color: isDark ? AltairColors.bgDark : AltairColors.bgLight,
            border: Border.all(
              color: _focusNode.hasFocus
                  ? AltairColors.accentOrange
                  : (isDark ? AltairColors.borderDark : AltairColors.borderLight),
              width: _focusNode.hasFocus
                  ? AltairBorders.extraThick
                  : AltairBorders.medium,
            ),
          ),
          child: TextField(
            controller: _searchController,
            focusNode: _focusNode,
            style: AltairTypography.bodyMedium.copyWith(
              color: isDark ? AltairColors.textLight : AltairColors.textDark,
            ),
            decoration: InputDecoration(
              hintText: 'Search or create tags...',
              hintStyle: AltairTypography.bodyMedium.copyWith(
                color: (isDark ? AltairColors.textLight : AltairColors.textDark)
                    .withValues(alpha: 0.5),
              ),
              border: InputBorder.none,
              isDense: true,
              contentPadding: EdgeInsets.zero,
            ),
          ),
        ),

        // Dropdown with tag options
        if (_showDropdown) ...[
          const SizedBox(height: AltairSpacing.xxs),
          Container(
            constraints: const BoxConstraints(maxHeight: 200),
            decoration: BoxDecoration(
              color: isDark ? AltairColors.bgDark : AltairColors.bgLight,
              border: Border.all(
                color: isDark ? AltairColors.borderDark : AltairColors.borderLight,
                width: AltairBorders.medium,
              ),
            ),
            child: _filteredTags.isEmpty && _searchController.text.isNotEmpty
                ? _buildCreateTagOption()
                : ListView.builder(
                    shrinkWrap: true,
                    itemCount: _filteredTags.length,
                    itemBuilder: (context, index) {
                      final tag = _filteredTags[index];
                      final isSelected = widget.selectedTagIds.contains(tag.id);

                      return InkWell(
                        onTap: () => _toggleTag(tag.id),
                        child: Container(
                          padding: const EdgeInsets.all(AltairSpacing.xs),
                          decoration: BoxDecoration(
                            border: Border(
                              bottom: BorderSide(
                                color: isDark
                                    ? AltairColors.borderDark
                                    : AltairColors.borderLight,
                                width: AltairBorders.thin,
                              ),
                            ),
                          ),
                          child: Row(
                            children: [
                              Icon(
                                isSelected
                                    ? Icons.check_box
                                    : Icons.check_box_outline_blank,
                                size: 20,
                                color: tag.color ?? AltairColors.accentOrange,
                              ),
                              const SizedBox(width: AltairSpacing.xs),
                              Text(
                                tag.name,
                                style: AltairTypography.bodySmall.copyWith(
                                  color: isDark
                                      ? AltairColors.textLight
                                      : AltairColors.textDark,
                                ),
                              ),
                            ],
                          ),
                        ),
                      );
                    },
                  ),
          ),
        ],
      ],
    );
  }

  Widget _buildCreateTagOption() {
    if (widget.onCreateTag == null) {
      return Padding(
        padding: const EdgeInsets.all(AltairSpacing.xs),
        child: Text(
          'No tags found',
          style: AltairTypography.bodySmall.copyWith(
            color: (Theme.of(context).brightness == Brightness.dark
                    ? AltairColors.textLight
                    : AltairColors.textDark)
                .withValues(alpha: 0.5),
          ),
        ),
      );
    }

    return InkWell(
      onTap: () {
        widget.onCreateTag!(_searchController.text);
        _searchController.clear();
      },
      child: Container(
        padding: const EdgeInsets.all(AltairSpacing.xs),
        child: Row(
          children: [
            const Icon(Icons.add, size: 20),
            const SizedBox(width: AltairSpacing.xs),
            Text(
              'Create "${_searchController.text}"',
              style: AltairTypography.bodySmall.copyWith(
                fontWeight: FontWeight.bold,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
