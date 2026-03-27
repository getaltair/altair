/**
 * Formatting utilities for the Knowledge domain.
 *
 * Mirrors the pattern established by `guidance-format.ts`.
 */

export { formatDate } from '$lib/utils/date-format.js';

/**
 * Return a human-readable label for a content_type value.
 */
export function contentTypeLabel(type: string): string {
	switch (type) {
		case 'markdown':
			return 'Markdown';
		case 'plain':
			return 'Plain Text';
		case 'rich_text':
			return 'Rich Text';
		case 'code':
			return 'Code';
		default:
			return type.charAt(0).toUpperCase() + type.slice(1);
	}
}

/**
 * Truncate content for preview display, appending an ellipsis when trimmed.
 */
export function truncateContent(content: string, maxLength = 120): string {
	if (content.length <= maxLength) return content;
	return content.slice(0, maxLength).trimEnd() + '\u2026';
}
