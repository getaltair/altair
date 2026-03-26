import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { UpdateType } from '@powersync/web';

// ---------------------------------------------------------------------------
// Mock @powersync/web before importing the module under test.
//
// vi.hoisted() ensures these declarations are available inside the vi.mock
// factory, which Vitest hoists to the top of the file.
// ---------------------------------------------------------------------------
const { mockInit, mockConnect, mockClose, PowerSyncDatabaseSpy } = vi.hoisted(() => {
	const mockInit = vi.fn().mockResolvedValue(undefined);
	const mockConnect = vi.fn().mockResolvedValue(undefined);
	const mockClose = vi.fn().mockResolvedValue(undefined);

	const PowerSyncDatabaseSpy = vi.fn().mockImplementation(function () {
		return { init: mockInit, connect: mockConnect, close: mockClose };
	});

	return { mockInit, mockConnect, mockClose, PowerSyncDatabaseSpy };
});

vi.mock('@powersync/web', async (importOriginal) => {
	const actual = (await importOriginal()) as Record<string, unknown>;
	return {
		...actual,
		PowerSyncDatabase: PowerSyncDatabaseSpy
	};
});

// Import module under test AFTER the mock is registered.
import {
	AltairConnector,
	createPowerSyncClient,
	initPowerSync,
	getPowerSyncDb,
	closePowerSync
} from './powersync-client.js';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
const mockFetch = vi.fn();

function jsonResponse(body: unknown, status = 200): Partial<Response> {
	return {
		ok: status >= 200 && status < 300,
		status,
		json: vi.fn().mockResolvedValue(body),
		text: vi.fn().mockResolvedValue(JSON.stringify(body))
	};
}

function textResponse(body: string, status: number): Partial<Response> {
	return {
		ok: status >= 200 && status < 300,
		status,
		text: vi.fn().mockResolvedValue(body)
	};
}

// ---------------------------------------------------------------------------
// Global setup / teardown
// ---------------------------------------------------------------------------
beforeEach(() => {
	vi.stubGlobal('fetch', mockFetch);
	mockFetch.mockReset();
	mockInit.mockReset().mockResolvedValue(undefined);
	mockConnect.mockReset().mockResolvedValue(undefined);
	mockClose.mockReset().mockResolvedValue(undefined);
	PowerSyncDatabaseSpy.mockClear();
});

afterEach(async () => {
	// Reset the singleton between tests so they do not leak state.
	await closePowerSync();
	vi.unstubAllGlobals();
});

// ===========================================================================
// AltairConnector.fetchCredentials
// ===========================================================================
describe('AltairConnector.fetchCredentials', () => {
	const connector = new AltairConnector();

	it('returns endpoint and token on successful response', async () => {
		mockFetch.mockResolvedValueOnce(
			jsonResponse({ token: 'jwt-abc', powersync_url: 'https://ps.example.com' })
		);

		const creds = await connector.fetchCredentials();

		expect(creds).toEqual({
			endpoint: 'https://ps.example.com',
			token: 'jwt-abc'
		});
	});

	it('sends POST to /auth/powersync-token with credentials included', async () => {
		mockFetch.mockResolvedValueOnce(jsonResponse({ token: 't', powersync_url: 'http://ps' }));

		await connector.fetchCredentials();

		expect(mockFetch).toHaveBeenCalledWith(
			expect.stringContaining('/auth/powersync-token'),
			expect.objectContaining({ method: 'POST', credentials: 'include' })
		);
	});

	it('throws with status and body on non-OK response', async () => {
		mockFetch.mockResolvedValueOnce(textResponse('Unauthorized', 401));

		await expect(connector.fetchCredentials()).rejects.toThrow(/401.*Unauthorized/);
	});

	it('throws descriptive error when response body is not valid JSON', async () => {
		mockFetch.mockResolvedValueOnce({
			ok: true,
			status: 200,
			json: vi.fn().mockRejectedValue(new SyntaxError('Unexpected token')),
			text: vi.fn().mockResolvedValue('not json')
		});

		await expect(connector.fetchCredentials()).rejects.toThrow(/invalid JSON/);
	});

	it('throws validation error when token field is missing', async () => {
		mockFetch.mockResolvedValueOnce(jsonResponse({ powersync_url: 'http://ps' }));

		await expect(connector.fetchCredentials()).rejects.toThrow(/missing required fields/);
	});

	it('throws validation error when powersync_url field is missing', async () => {
		mockFetch.mockResolvedValueOnce(jsonResponse({ token: 'tok' }));

		await expect(connector.fetchCredentials()).rejects.toThrow(/missing required fields/);
	});

	it('throws validation error when response is null', async () => {
		mockFetch.mockResolvedValueOnce(jsonResponse(null));

		await expect(connector.fetchCredentials()).rejects.toThrow(/missing required fields/);
	});

	it('throws validation error when response is a non-object primitive', async () => {
		mockFetch.mockResolvedValueOnce(jsonResponse('just a string'));

		await expect(connector.fetchCredentials()).rejects.toThrow(/missing required fields/);
	});
});

// ===========================================================================
// AltairConnector.uploadData
// ===========================================================================
describe('AltairConnector.uploadData', () => {
	const connector = new AltairConnector();

	function makeCrudOp(overrides: Record<string, unknown> = {}) {
		return {
			table: 'initiatives',
			id: 'row-1',
			op: UpdateType.PUT,
			opData: { name: 'Test' },
			...overrides
		};
	}

	function makeMockDatabase(crud: unknown[] = [], hasTransaction = true) {
		const completeFn = vi.fn().mockResolvedValue(undefined);
		return {
			db: {
				getNextCrudTransaction: vi
					.fn()
					.mockResolvedValue(hasTransaction ? { crud, complete: completeFn } : null)
			},
			completeFn
		};
	}

	it('returns early when there is no pending transaction', async () => {
		const { db } = makeMockDatabase([], false);

		// Should not throw and not call fetch
		await connector.uploadData(db as never);

		expect(mockFetch).not.toHaveBeenCalled();
	});

	it('sends PUT operations to /core/{table} with POST method', async () => {
		const op = makeCrudOp({ op: UpdateType.PUT, table: 'initiatives', id: 'r1' });
		const { db } = makeMockDatabase([op]);
		mockFetch.mockResolvedValueOnce(jsonResponse({}, 200));

		await connector.uploadData(db as never);

		expect(mockFetch).toHaveBeenCalledWith(
			expect.stringMatching(/\/core\/initiatives$/),
			expect.objectContaining({
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify({ id: 'r1', name: 'Test' })
			})
		);
	});

	it('sends PATCH operations to /core/{table}/{id} with PATCH method', async () => {
		const op = makeCrudOp({
			op: UpdateType.PATCH,
			table: 'tags',
			id: 'r2',
			opData: { color: 'blue' }
		});
		const { db } = makeMockDatabase([op]);
		mockFetch.mockResolvedValueOnce(jsonResponse({}, 200));

		await connector.uploadData(db as never);

		expect(mockFetch).toHaveBeenCalledWith(
			expect.stringMatching(/\/core\/tags\/r2$/),
			expect.objectContaining({
				method: 'PATCH',
				body: JSON.stringify({ color: 'blue' })
			})
		);
	});

	it('sends DELETE operations to /core/{table}/{id} with DELETE method', async () => {
		const op = makeCrudOp({ op: UpdateType.DELETE, table: 'users', id: 'r3' });
		const { db } = makeMockDatabase([op]);
		mockFetch.mockResolvedValueOnce(jsonResponse({}, 200));

		await connector.uploadData(db as never);

		expect(mockFetch).toHaveBeenCalledWith(
			expect.stringMatching(/\/core\/users\/r3$/),
			expect.objectContaining({ method: 'DELETE', credentials: 'include' })
		);
	});

	it('calls transaction.complete() after all ops succeed', async () => {
		const ops = [
			makeCrudOp({ op: UpdateType.PUT }),
			makeCrudOp({ op: UpdateType.PATCH, id: 'r2' })
		];
		const { db, completeFn } = makeMockDatabase(ops);
		mockFetch.mockResolvedValue(jsonResponse({}, 200));

		await connector.uploadData(db as never);

		expect(completeFn).toHaveBeenCalledOnce();
	});

	it('does NOT call transaction.complete() when a fetch fails', async () => {
		const op = makeCrudOp({ op: UpdateType.PUT });
		const { db, completeFn } = makeMockDatabase([op]);
		mockFetch.mockResolvedValueOnce(textResponse('Server Error', 500));

		await expect(connector.uploadData(db as never)).rejects.toThrow(/500/);
		expect(completeFn).not.toHaveBeenCalled();
	});

	it('throws on unhandled CRUD operation type', async () => {
		const op = makeCrudOp({ op: 'UNKNOWN_OP' });
		const { db, completeFn } = makeMockDatabase([op]);

		await expect(connector.uploadData(db as never)).rejects.toThrow(
			/Unhandled CRUD operation type/
		);
		expect(completeFn).not.toHaveBeenCalled();
	});

	it('uses /core/ URL prefix (not /api/)', async () => {
		const op = makeCrudOp({ op: UpdateType.PUT, table: 'households' });
		const { db } = makeMockDatabase([op]);
		mockFetch.mockResolvedValueOnce(jsonResponse({}, 200));

		await connector.uploadData(db as never);

		const calledUrl = mockFetch.mock.calls[0][0] as string;
		expect(calledUrl).toContain('/core/');
		expect(calledUrl).not.toContain('/api/');
	});
});

// ===========================================================================
// Singleton lifecycle: createPowerSyncClient, initPowerSync, getPowerSyncDb,
//                      closePowerSync
// ===========================================================================
describe('createPowerSyncClient', () => {
	it('constructs a new PowerSyncDatabase with AppSchema and altair.db filename', () => {
		createPowerSyncClient();

		expect(PowerSyncDatabaseSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				database: { dbFilename: 'altair.db' }
			})
		);
	});
});

describe('singleton lifecycle', () => {
	it('getPowerSyncDb returns null before initialization', () => {
		expect(getPowerSyncDb()).toBeNull();
	});

	it('initPowerSync calls init() then connect() and returns the db', async () => {
		const db = await initPowerSync();

		expect(db).toBeDefined();
		expect(mockInit).toHaveBeenCalledOnce();
		expect(mockConnect).toHaveBeenCalledOnce();
	});

	it('initPowerSync returns the same instance on a second call', async () => {
		const first = await initPowerSync();
		const second = await initPowerSync();

		expect(first).toBe(second);
		// init and connect should only have been called once total
		expect(mockInit).toHaveBeenCalledOnce();
	});

	it('getPowerSyncDb returns the instance after successful init', async () => {
		await initPowerSync();

		expect(getPowerSyncDb()).not.toBeNull();
	});

	it('closePowerSync makes getPowerSyncDb return null', async () => {
		await initPowerSync();
		await closePowerSync();

		expect(getPowerSyncDb()).toBeNull();
	});

	it('closePowerSync calls close() on the database', async () => {
		await initPowerSync();
		await closePowerSync();

		expect(mockClose).toHaveBeenCalledOnce();
	});

	it('closePowerSync is a no-op when no db is initialized', async () => {
		// Should not throw even if called without init
		await expect(closePowerSync()).resolves.toBeUndefined();
		expect(mockClose).not.toHaveBeenCalled();
	});

	it('initPowerSync does not leave a partially initialized db when init() fails', async () => {
		mockInit.mockRejectedValueOnce(new Error('init failed'));

		await expect(initPowerSync()).rejects.toThrow('init failed');
		expect(getPowerSyncDb()).toBeNull();

		// After failure, a subsequent call should attempt init again (not return cached)
		mockInit.mockResolvedValueOnce(undefined);
		const db = await initPowerSync();
		expect(db).toBeDefined();
		expect(mockInit).toHaveBeenCalledTimes(2);
	});

	it('initPowerSync calls close() on the client when init() fails', async () => {
		mockInit.mockRejectedValueOnce(new Error('init boom'));

		await expect(initPowerSync()).rejects.toThrow('init boom');
		// The implementation tries to close the partially created client
		expect(mockClose).toHaveBeenCalledOnce();
	});

	it('initPowerSync does not leave a partially initialized db when connect() fails', async () => {
		mockConnect.mockRejectedValueOnce(new Error('connect failed'));

		await expect(initPowerSync()).rejects.toThrow('connect failed');
		expect(getPowerSyncDb()).toBeNull();
	});
});
