/**
 * Default base URL used when no explicit value is provided.
 * Override by passing `baseUrl` in ApiClientOptions.
 */
const DEFAULT_BASE_URL = '/api';

/** HTTP methods supported by the client. */
export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';

/** Structured error thrown on network or parse failures. */
export class ApiError extends Error {
	readonly url: string;
	readonly method: HttpMethod;
	readonly status: number;
	readonly responseBody: unknown;

	constructor(
		message: string,
		init: { url: string; method: HttpMethod; status: number; responseBody?: unknown }
	) {
		super(message);
		this.name = 'ApiError';
		this.url = init.url;
		this.method = init.method;
		this.status = init.status;
		this.responseBody = init.responseBody ?? null;
	}
}

export type ApiSuccess<T> = { ok: true; data: T; status: number };
export type ApiFailure = { ok: false; error: unknown; status: number };
export type ApiResponse<T> = ApiSuccess<T> | ApiFailure;

/** Minimal configuration accepted by the client. */
export interface ApiClientOptions {
	baseUrl?: string;
	headers?: Record<string, string>;
}

/**
 * Typed HTTP client that centralises fetch calls so that base URL,
 * auth headers, and error handling are configured once.
 */
export class ApiClient {
	private readonly baseUrl: string;
	private readonly defaultHeaders: Record<string, string>;

	constructor(options: ApiClientOptions = {}) {
		this.baseUrl = options.baseUrl ?? DEFAULT_BASE_URL;
		this.defaultHeaders = {
			Accept: 'application/json',
			...options.headers
		};
	}

	async get<T>(path: string, headers?: Record<string, string>): Promise<ApiResponse<T>> {
		return this.request<T>('GET', path, undefined, headers);
	}

	async post<T>(
		path: string,
		body?: unknown,
		headers?: Record<string, string>
	): Promise<ApiResponse<T>> {
		return this.request<T>('POST', path, body, headers);
	}

	async put<T>(
		path: string,
		body?: unknown,
		headers?: Record<string, string>
	): Promise<ApiResponse<T>> {
		return this.request<T>('PUT', path, body, headers);
	}

	async patch<T>(
		path: string,
		body?: unknown,
		headers?: Record<string, string>
	): Promise<ApiResponse<T>> {
		return this.request<T>('PATCH', path, body, headers);
	}

	async delete<T>(path: string, headers?: Record<string, string>): Promise<ApiResponse<T>> {
		return this.request<T>('DELETE', path, undefined, headers);
	}

	// Internal

	private async request<T>(
		method: HttpMethod,
		path: string,
		body?: unknown,
		headers?: Record<string, string>
	): Promise<ApiResponse<T>> {
		const url = `${this.baseUrl}${path}`;

		const mergedHeaders: Record<string, string> = { ...this.defaultHeaders, ...headers };
		if (body !== undefined) {
			mergedHeaders['Content-Type'] = 'application/json';
		}

		let response: Response;
		try {
			response = await fetch(url, {
				method,
				headers: mergedHeaders,
				body: body !== undefined ? JSON.stringify(body) : undefined
			});
		} catch (err) {
			throw new ApiError(`Network error: ${err instanceof Error ? err.message : String(err)}`, {
				url,
				method,
				status: 0
			});
		}

		if (!response.ok) {
			let responseBody: unknown;
			try {
				responseBody = await response.json();
			} catch {
				responseBody = await response.text().catch(() => null);
			}
			return { ok: false, error: responseBody, status: response.status };
		}

		let data: T;
		try {
			data = response.headers.get('content-type')?.includes('application/json')
				? await response.json()
				: ((await response.text()) as unknown as T);
		} catch (err) {
			throw new ApiError(
				`Failed to parse response body: ${err instanceof Error ? err.message : String(err)}`,
				{
					url,
					method,
					status: response.status
				}
			);
		}

		return { ok: true, data, status: response.status };
	}
}
