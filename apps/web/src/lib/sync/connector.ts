import {
  AbstractPowerSyncDatabase,
  UpdateType,
  type PowerSyncBackendConnector,
  type PowerSyncCredentials,
} from '@powersync/web';

export class AltairConnector implements PowerSyncBackendConnector {
  async fetchCredentials(): Promise<PowerSyncCredentials | null> {
    const response = await fetch('/api/auth/powersync-token', {
      method: 'GET',
      credentials: 'include',
    });
    if (!response.ok) {
      if (response.status === 401) {
        return null;
      }
      throw new Error(`Failed to fetch PowerSync credentials: ${response.status}`);
    }
    const data = (await response.json()) as { endpoint: string; token: string; expiresAt?: string };
    return {
      endpoint: data.endpoint,
      token: data.token,
      expiresAt: data.expiresAt ? new Date(data.expiresAt) : undefined,
    };
  }

  async uploadData(database: AbstractPowerSyncDatabase): Promise<void> {
    const batch = await database.getCrudBatch(100);
    if (!batch) {
      return;
    }

    for (const entry of batch.crud) {
      const { table, op, id, opData } = entry;

      switch (op) {
        case UpdateType.PUT: {
          await fetch(`/api/${table}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({ id, ...opData }),
          }).then(throwIfNotOk);
          break;
        }
        case UpdateType.PATCH: {
          await fetch(`/api/${table}/${id}`, {
            method: 'PATCH',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify(opData),
          }).then(throwIfNotOk);
          break;
        }
        case UpdateType.DELETE: {
          await fetch(`/api/${table}/${id}`, {
            method: 'DELETE',
            credentials: 'include',
          }).then(throwIfNotOk);
          break;
        }
      }
    }

    await batch.complete();
  }
}

async function throwIfNotOk(response: Response): Promise<Response> {
  if (!response.ok) {
    throw new Error(`Upload failed: ${response.status} ${response.statusText}`);
  }
  return response;
}
