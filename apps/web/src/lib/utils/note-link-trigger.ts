/**
 * Detects a `[[` wiki-link trigger in text at or before the given cursor position.
 *
 * Scans backward from `cursorPos` (inclusive) for `[[` with no intervening whitespace.
 * Returns `{ query, triggerStart }` if found, `null` otherwise.
 *
 * `cursorPos` is the 0-based index of the last typed character (inclusive end of the
 * query region). For example in `'hello [[w'` with cursorPos=8, the query is `'w'`.
 * Callers typically pass `selectionStart - 1` from an input event.
 *
 * @param value     - The full text string
 * @param cursorPos - Index of the last character to consider (inclusive)
 */
export function detectLinkTrigger(
  value: string,
  cursorPos: number,
): { query: string; triggerStart: number } | null {
  // Scan backward from cursorPos looking for [[
  let i = cursorPos;

  while (i >= 1) {
    const ch = value[i];

    // Whitespace in the query text breaks the trigger
    if (ch === ' ' || ch === '\t' || ch === '\n' || ch === '\r') {
      return null;
    }

    // Found the closing bracket of [[
    if (ch === '[' && value[i - 1] === '[') {
      const triggerStart = i - 1;
      const query = value.slice(i + 1, cursorPos + 1);
      return { query, triggerStart };
    }

    i--;
  }

  return null;
}
