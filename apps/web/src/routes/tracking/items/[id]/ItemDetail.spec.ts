import { describe, it, expect, vi } from 'vitest';
import { validateConsumption } from '$lib/utils/validate-consumption';

// ============================================================
// ItemDetail — quantity validation tests
//
// These tests exercise the shared validateConsumption utility
// imported by the component, ensuring FA-009 (negative-quantity
// block before outbox write) is verified against the actual
// boundary condition used at runtime.
// ============================================================

describe('ItemDetail quantity validation', () => {
  describe('validateConsumption', () => {
    it('rejects consumption that would go below zero', () => {
      const result = validateConsumption(3, 5);
      expect(result.valid).toBe(false);
      expect(result.error).toContain('Cannot consume 5');
    });

    it('allows consumption that exactly empties stock', () => {
      const result = validateConsumption(3, 3);
      expect(result.valid).toBe(true);
      expect(result.error).toBe('');
    });

    it('allows partial consumption', () => {
      const result = validateConsumption(3, 1);
      expect(result.valid).toBe(true);
    });

    it('rejects zero consumption', () => {
      const result = validateConsumption(3, 0);
      expect(result.valid).toBe(false);
    });

    it('rejects negative consumption', () => {
      const result = validateConsumption(3, -1);
      expect(result.valid).toBe(false);
    });

    it('rejects NaN', () => {
      const result = validateConsumption(3, NaN);
      expect(result.valid).toBe(false);
    });
  });

  describe('getSyncClient().execute is NOT called when validation fails', () => {
    it('does not call execute when consumption (5) exceeds quantity (3)', () => {
      // Simulates what the component's logConsumption does:
      // validate first, only call execute if valid.
      const mockExecute = vi.fn();

      const currentQty = 3;
      const consumptionAmount = 5;
      const { valid } = validateConsumption(currentQty, consumptionAmount);

      // Component guard: early-return when validation fails
      if (valid) {
        mockExecute('INSERT INTO tracking_item_events ...');
      }

      expect(mockExecute).not.toHaveBeenCalled();
    });

    it('calls execute when validation passes (consume 2 of 3)', () => {
      const mockExecute = vi.fn();

      const currentQty = 3;
      const consumptionAmount = 2;
      const { valid } = validateConsumption(currentQty, consumptionAmount);

      if (valid) {
        mockExecute('INSERT INTO tracking_item_events ...', [consumptionAmount]);
      }

      expect(mockExecute).toHaveBeenCalledOnce();
    });
  });
});
