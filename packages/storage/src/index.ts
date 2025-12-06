/**
 * @altair/storage - S3-compatible object storage utilities
 *
 * Provides a unified interface for S3-compatible object storage (Minio local, Backblaze B2 cloud).
 * Handles file uploads, downloads, presigned URLs, and metadata management with content-addressable storage.
 */

// ============================================================================
// Configuration
// ============================================================================

/**
 * S3-compatible storage configuration
 */
export interface StorageConfig {
  /** S3 endpoint URL (e.g., http://localhost:9000 for Minio, https://s3.us-west-002.backblazeb2.com for B2) */
  endpoint: string;
  /** S3 bucket name */
  bucket: string;
  /** AWS access key ID (or equivalent for B2) */
  accessKeyId: string;
  /** AWS secret access key (or equivalent for B2) */
  secretAccessKey: string;
  /** AWS region (e.g., 'us-east-1', or B2 region code) */
  region: string;
  /** Force path-style URLs (required for Minio, optional for B2) */
  forcePathStyle?: boolean;
}

/**
 * Default storage configuration (loads from environment or uses local Minio defaults)
 */
export const defaultStorageConfig: Partial<StorageConfig> = {
  endpoint: 'http://localhost:9000',
  bucket: 'altair-local',
  region: 'us-east-1',
  forcePathStyle: true,
};

// ============================================================================
// Storage Operations
// ============================================================================

/**
 * File upload options
 */
export interface UploadOptions {
  /** Content type (MIME type) */
  contentType?: string;
  /** Custom metadata key-value pairs */
  metadata?: Record<string, string>;
  /** Content-addressable hash (for deduplication) */
  contentHash?: string;
  /** Progress callback for upload tracking */
  onProgress?: (uploaded: number, total: number) => void;
}

/**
 * Upload result
 */
export interface UploadResult {
  /** Object key in S3 */
  key: string;
  /** Full URL to the object */
  url: string;
  /** ETag (entity tag) from S3 */
  etag: string;
  /** Content hash (if provided in options) */
  contentHash?: string;
  /** Upload timestamp */
  uploadedAt: Date;
}

/**
 * Download options
 */
export interface DownloadOptions {
  /** Byte range for partial downloads (e.g., 'bytes=0-1023') */
  range?: string;
  /** Progress callback for download tracking */
  onProgress?: (downloaded: number, total: number) => void;
}

/**
 * Download result
 */
export interface DownloadResult {
  /** File data as ArrayBuffer */
  data: ArrayBuffer;
  /** Content type */
  contentType: string;
  /** Content length in bytes */
  contentLength: number;
  /** Custom metadata */
  metadata?: Record<string, string>;
  /** ETag */
  etag: string;
}

/**
 * Presigned URL options
 */
export interface PresignedUrlOptions {
  /** URL expiration in seconds (default: 3600 = 1 hour) */
  expiresIn?: number;
  /** Content type to enforce on upload (PUT URLs only) */
  contentType?: string;
  /** Content disposition (e.g., 'attachment; filename="example.pdf"') */
  contentDisposition?: string;
}

/**
 * Presigned URL result
 */
export interface PresignedUrlResult {
  /** Presigned URL */
  url: string;
  /** Expiration timestamp */
  expiresAt: Date;
  /** Object key */
  key: string;
}

/**
 * Object metadata
 */
export interface ObjectMetadata {
  /** Object key */
  key: string;
  /** Content length in bytes */
  size: number;
  /** Last modified timestamp */
  lastModified: Date;
  /** ETag */
  etag: string;
  /** Content type */
  contentType?: string;
  /** Custom metadata */
  metadata?: Record<string, string>;
}

/**
 * List objects options
 */
export interface ListObjectsOptions {
  /** Prefix filter (e.g., 'uploads/2024/') */
  prefix?: string;
  /** Maximum number of objects to return */
  maxKeys?: number;
  /** Continuation token for pagination */
  continuationToken?: string;
}

/**
 * List objects result
 */
export interface ListObjectsResult {
  /** List of objects */
  objects: ObjectMetadata[];
  /** Continuation token for next page (if more results exist) */
  nextContinuationToken?: string;
  /** Whether more results are available */
  isTruncated: boolean;
}

// ============================================================================
// Storage Client Interface
// ============================================================================

/**
 * S3-compatible storage client
 *
 * Provides a high-level interface for interacting with S3-compatible object storage.
 * Implementations should handle SDK-specific details (AWS SDK v3, MinIO client, etc.).
 */
export interface StorageClient {
  /**
   * Upload a file to object storage
   *
   * @param key - Object key (path in bucket, e.g., 'uploads/2024/file.pdf')
   * @param data - File data as ArrayBuffer or Blob
   * @param options - Upload options
   * @returns Upload result with URL and metadata
   */
  upload(key: string, data: ArrayBuffer | Blob, options?: UploadOptions): Promise<UploadResult>;

  /**
   * Download a file from object storage
   *
   * @param key - Object key
   * @param options - Download options
   * @returns Download result with data and metadata
   */
  download(key: string, options?: DownloadOptions): Promise<DownloadResult>;

  /**
   * Delete an object from storage
   *
   * @param key - Object key
   * @returns True if deleted, false if object didn't exist
   */
  delete(key: string): Promise<boolean>;

  /**
   * Generate a presigned URL for GET (download)
   *
   * @param key - Object key
   * @param options - URL options (expiration, etc.)
   * @returns Presigned URL result
   */
  getPresignedUrl(key: string, options?: PresignedUrlOptions): Promise<PresignedUrlResult>;

  /**
   * Generate a presigned URL for PUT (upload)
   *
   * Useful for client-side direct uploads to S3 without proxying through backend.
   *
   * @param key - Object key
   * @param options - URL options (expiration, content type, etc.)
   * @returns Presigned URL result
   */
  putPresignedUrl(key: string, options?: PresignedUrlOptions): Promise<PresignedUrlResult>;

  /**
   * Get object metadata without downloading the object
   *
   * @param key - Object key
   * @returns Object metadata or null if not found
   */
  getMetadata(key: string): Promise<ObjectMetadata | null>;

  /**
   * List objects in a bucket with optional prefix filter
   *
   * @param options - List options (prefix, pagination, etc.)
   * @returns List of objects with pagination support
   */
  listObjects(options?: ListObjectsOptions): Promise<ListObjectsResult>;

  /**
   * Check if an object exists
   *
   * @param key - Object key
   * @returns True if object exists, false otherwise
   */
  exists(key: string): Promise<boolean>;
}

// ============================================================================
// Utilities
// ============================================================================

/**
 * Generate a content-addressable key from a hash
 *
 * Useful for deduplication: multiple uploads of the same content share the same key.
 *
 * @param hash - Content hash (e.g., SHA-256 hex digest)
 * @param extension - File extension (e.g., 'pdf', 'jpg')
 * @returns Object key (e.g., 'content/ab/cd/abcd1234...56789.pdf')
 *
 * @example
 * ```ts
 * const hash = await sha256(fileData);
 * const key = contentAddressableKey(hash, 'pdf');
 * // key = 'content/ab/cd/abcd1234567890abcdef1234567890abcdef.pdf'
 * ```
 */
export function contentAddressableKey(hash: string, extension: string): string {
  // Split hash into directory segments for better S3 performance
  // S3 performs better with evenly distributed prefixes
  const prefix = hash.slice(0, 2);
  const subPrefix = hash.slice(2, 4);
  return `content/${prefix}/${subPrefix}/${hash}.${extension}`;
}

/**
 * Generate a user-scoped key for uploaded files
 *
 * @param userId - User ID (UUID)
 * @param filename - Original filename
 * @returns Object key (e.g., 'uploads/user-123e4567-e89b-12d3-a456-426614174000/2024/12/06/example.pdf')
 *
 * @example
 * ```ts
 * const key = userScopedKey('123e4567-e89b-12d3-a456-426614174000', 'example.pdf');
 * await storageClient.upload(key, fileData);
 * ```
 */
export function userScopedKey(userId: string, filename: string): string {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `uploads/user-${userId}/${year}/${month}/${day}/${filename}`;
}

/**
 * Parse a storage URL to extract the object key
 *
 * @param url - Full S3 URL (e.g., 'http://localhost:9000/altair-local/uploads/file.pdf')
 * @param bucket - Bucket name
 * @returns Object key (e.g., 'uploads/file.pdf')
 *
 * @example
 * ```ts
 * const key = parseStorageUrl('http://localhost:9000/altair-local/uploads/file.pdf', 'altair-local');
 * // key = 'uploads/file.pdf'
 * ```
 */
export function parseStorageUrl(url: string, bucket: string): string {
  const urlObj = new URL(url);
  const pathSegments = urlObj.pathname.split('/').filter(Boolean);

  // Path-style URL: /{bucket}/{key}
  if (pathSegments[0] === bucket) {
    return pathSegments.slice(1).join('/');
  }

  // Virtual-hosted-style URL: {bucket}.endpoint/{key}
  return pathSegments.join('/');
}

/**
 * Format file size for human-readable display
 *
 * @param bytes - File size in bytes
 * @returns Formatted string (e.g., '1.5 MB', '342 KB')
 *
 * @example
 * ```ts
 * formatFileSize(1536000); // '1.5 MB'
 * formatFileSize(342000);  // '342 KB'
 * ```
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';

  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = bytes / Math.pow(k, i);

  return `${value.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}
