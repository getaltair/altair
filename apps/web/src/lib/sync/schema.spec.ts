import { readFileSync } from 'fs';
import { resolve } from 'path';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// ============================================================
// Schema column parity tests — compare against SQL migrations
// ============================================================

const MIGRATIONS_DIR = resolve(
  __dirname,
  '../../../../../infra/migrations',
);

function readMigration(filename: string): string {
  return readFileSync(resolve(MIGRATIONS_DIR, filename), 'utf-8');
}

describe('PowerSync schema column parity with Postgres migrations', () => {
  it('quests schema contains columns matching guidance_quests migration', () => {
    const sql = readMigration('20260412000014_create_guidance_quests.up.sql');
    const schemaSrc = readFileSync(resolve(__dirname, 'schema.ts'), 'utf-8');

    // Required columns from migration
    const requiredColumns = [
      'title',
      'description',
      'status',
      'priority',
      'due_date',
      'epic_id',
      'initiative_id',
      'routine_id',
      'user_id',
      'created_at',
      'updated_at',
      'deleted_at',
    ];

    for (const col of requiredColumns) {
      expect(schemaSrc, `quests schema should include column: ${col}`).toContain(col);
    }

    // Verify the migration source has these same columns
    for (const col of requiredColumns) {
      expect(sql, `migration should define column: ${col}`).toContain(col);
    }
  });

  it('notes schema contains columns matching knowledge_notes migration', () => {
    const sql = readMigration('20260412000017_create_knowledge_notes.up.sql');
    const schemaSrc = readFileSync(resolve(__dirname, 'schema.ts'), 'utf-8');

    const requiredColumns = ['title', 'content', 'user_id', 'initiative_id', 'created_at', 'updated_at', 'deleted_at'];

    for (const col of requiredColumns) {
      expect(schemaSrc, `notes schema should include column: ${col}`).toContain(col);
      expect(sql, `migration should define column: ${col}`).toContain(col);
    }
  });

  it('tracking_items schema contains columns matching migration', () => {
    const sql = readMigration('20260412000021_create_tracking_items.up.sql');
    const schemaSrc = readFileSync(resolve(__dirname, 'schema.ts'), 'utf-8');

    const requiredColumns = [
      'name',
      'description',
      'quantity',
      'barcode',
      'location_id',
      'category_id',
      'user_id',
      'household_id',
      'initiative_id',
      'expires_at',
      'created_at',
      'updated_at',
      'deleted_at',
    ];

    for (const col of requiredColumns) {
      expect(schemaSrc, `tracking_items schema should include column: ${col}`).toContain(col);
      expect(sql, `migration should define column: ${col}`).toContain(col);
    }
  });
});

// ============================================================
// Connector interface tests
// ============================================================

describe('AltairConnector interface', () => {
  it('exports fetchCredentials function', async () => {
    const mod = await import('./connector.js');
    const connector = new mod.AltairConnector();
    expect(typeof connector.fetchCredentials).toBe('function');
  });

  it('exports uploadData function', async () => {
    const mod = await import('./connector.js');
    const connector = new mod.AltairConnector();
    expect(typeof connector.uploadData).toBe('function');
  });

  it('uploadData calls batch.complete() on success', async () => {
    const mod = await import('./connector.js');
    const connector = new mod.AltairConnector();

    const completeFn = vi.fn().mockResolvedValue(undefined);
    const mockBatch = {
      crud: [],
      haveMore: false,
      complete: completeFn,
    };

    const mockDatabase = {
      getCrudBatch: vi.fn().mockResolvedValue(mockBatch),
    };

    await connector.uploadData(mockDatabase as never);
    expect(completeFn).toHaveBeenCalledOnce();
  });

  it('uploadData does not call complete when batch is null', async () => {
    const mod = await import('./connector.js');
    const connector = new mod.AltairConnector();

    const mockDatabase = {
      getCrudBatch: vi.fn().mockResolvedValue(null),
    };

    // Should resolve without error
    await expect(connector.uploadData(mockDatabase as never)).resolves.toBeUndefined();
  });
});

// ============================================================
// getSyncClient() browser guard test
// ============================================================

describe('getSyncClient browser guard', () => {
  beforeEach(() => {
    vi.resetModules();
  });

  afterEach(() => {
    vi.resetModules();
  });

  it('throws when called outside browser context', async () => {
    vi.doMock('$app/environment', () => ({ browser: false }));
    const { getSyncClient } = await import('./index.js');
    expect(() => getSyncClient()).toThrow('PowerSync requires a browser environment');
  });
});
