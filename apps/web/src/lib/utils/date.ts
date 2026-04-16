/**
 * Returns true if the given ISO date string (YYYY-MM-DD or full ISO timestamp)
 * falls on today's local date.
 */
export function isToday(dateStr: string): boolean {
  const today = new Date();
  const date = new Date(dateStr);
  return (
    date.getFullYear() === today.getFullYear() &&
    date.getMonth() === today.getMonth() &&
    date.getDate() === today.getDate()
  );
}

/**
 * Returns a human-readable relative date string for the given ISO date string.
 * Examples: "Today", "Yesterday", "3 days ago", "In 2 days", or a locale date string.
 */
export function formatRelative(dateStr: string): string {
  const now = new Date();
  const date = new Date(dateStr);

  // Normalise both dates to midnight local time for day-diff calculation
  const todayMidnight = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const dateMidnight = new Date(date.getFullYear(), date.getMonth(), date.getDate());

  const diffMs = dateMidnight.getTime() - todayMidnight.getTime();
  const diffDays = Math.round(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return 'Today';
  if (diffDays === -1) return 'Yesterday';
  if (diffDays === 1) return 'Tomorrow';
  if (diffDays < 0) return `${Math.abs(diffDays)} days ago`;
  return `In ${diffDays} days`;
}
