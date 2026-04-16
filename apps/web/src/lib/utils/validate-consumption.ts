/**
 * Validates whether a consumption amount is allowed given current stock.
 * Pure function — no side effects.
 */
export function validateConsumption(
  currentQty: number,
  amount: number,
): { valid: boolean; error: string } {
  if (isNaN(amount) || amount <= 0) {
    return { valid: false, error: 'Enter a positive quantity to consume.' };
  }
  if (currentQty - amount < 0) {
    return {
      valid: false,
      error: `Cannot consume ${amount} — only ${currentQty} in stock.`,
    };
  }
  return { valid: true, error: '' };
}
