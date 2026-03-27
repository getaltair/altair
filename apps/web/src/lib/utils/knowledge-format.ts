/**
 * Formatting utilities for the Knowledge domain.
 *
 * Mirrors the pattern established by `guidance-format.ts`.
 */

/**
 * Format an ISO date string to a human-readable short date.
 * e.g. "2026-03-27T12:00:00Z" -> "Mar 27, 2026"
 */
export function formatDate(dateStr: string): string {
	const d = new Date(dateStr);
	return d.toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
}

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
