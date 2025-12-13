# Tasks: CORE-011-STORAGE-SERVICE

**Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)
**Branch**: `spec/core-011-storage-service`
**Status**: Ready for implementation

---

## Phase 0: Research & Dependencies

### 0.1 aws-sdk-s3 Integration Research

- [x] **Validate presigned PUT URL generation**

  - Acceptance: Create test demonstrating PUT presigned URL with content-type and content-length restrictions working with Minio
  - Files: `backend/crates/altair-storage/tests/aws_sdk_research.rs`
  - Notes: Must support 15-minute expiration, custom headers

- [x] **Validate presigned GET URL generation**

  - Acceptance: Create test demonstrating GET presigned URL with configurable expiration working with Minio
  - Files: `backend/crates/altair-storage/tests/aws_sdk_research.rs`
  - Notes: Must support 1-hour expiration

- [x] **Validate HEAD object operation**

  - Acceptance: Demonstrate head_object() returns correct metadata (size, content-type) for existing objects
  - Files: `backend/crates/altair-storage/tests/aws_sdk_research.rs`

- [x] **Validate GET object with streaming**

  - Acceptance: Download 50MB file using streaming body, prove memory usage stays bounded
  - Files: `backend/crates/altair-storage/tests/aws_sdk_research.rs`
  - Notes: Required for checksum calculation on large files

- [x] **Validate custom endpoint configuration for Minio**
  - Acceptance: aws-sdk-s3 client connects to localhost:9000 and performs basic operations
  - Files: `backend/crates/altair-storage/tests/aws_sdk_research.rs`

### 0.2 Minio Embedding Strategy Research

- [x] **Determine platform-specific binary paths**

  - Acceptance: Document binary locations for macOS arm64/x64, Windows x64, Linux x64
  - Files: `docs/minio-embedding.md`
  - Notes: Consider using `platform-dirs` crate for app data directory

- [x] **Design process lifecycle management**

  - Acceptance: Document start/stop strategy using Tauri process API or tokio::process
  - Files: `docs/minio-embedding.md`
  - Notes: Must handle app crash, user force-quit, graceful shutdown

- [x] **Choose data directory location**

  - Acceptance: Document data directory strategy (platform-specific app data dir)
  - Files: `docs/minio-embedding.md`
  - Notes: Should persist between app restarts

- [x] **Define fallback behavior**
  - Acceptance: Document external Minio endpoint configuration for when embedded binary fails
  - Files: `docs/minio-embedding.md`

### 0.3 OS Keychain Integration Research

- [x] **Validate store/retrieve S3 credentials**

  - Acceptance: Test `keyring` crate storing and retrieving access_key and secret_key on current platform
  - Files: `backend/crates/altair-storage/tests/keychain_research.rs`

- [x] **Validate platform support**

  - Acceptance: Document tested platforms (macOS Keychain, Windows Credential Manager, Linux Secret Service)
  - Files: `docs/keychain-support.md`

- [x] **Design error handling for unavailable keychain**

  - Acceptance: Define fallback behavior (environment variables, config file warning)
  - Files: `docs/keychain-support.md`

- [x] **Design first-run credential setup flow**
  - Acceptance: Document UX for users setting up storage credentials for first time
  - Files: `docs/keychain-support.md`

### 0.4 Image Processing Research

- [x] **Validate supported formats**

  - Acceptance: Test `image` crate decoding JPEG, PNG, GIF, WebP
  - Files: `backend/crates/altair-storage/tests/image_research.rs`

- [x] **Validate resize to max dimension**

  - Acceptance: Resize 3000×2000 image to 256×171 (aspect ratio preserved)
  - Files: `backend/crates/altair-storage/tests/image_research.rs`

- [x] **Validate JPEG encoding quality**

  - Acceptance: Encode thumbnail at 80% quality, verify file size reasonable (~10-20KB for typical photo)
  - Files: `backend/crates/altair-storage/tests/image_research.rs`

- [x] **Validate memory efficiency**
  - Acceptance: Process 10MB image, prove memory usage stays under 100MB
  - Files: `backend/crates/altair-storage/tests/image_research.rs`

---

## Phase 1: Core Infrastructure

### 1.1 Storage Configuration

- [x] **Define StorageConfig struct**

  - Acceptance: Struct with endpoint, region, bucket, access_key_id, secret_access_key fields
  - Files: `backend/crates/altair-storage/src/config.rs`
  - Notes: Use `url::Url` for endpoint validation

- [x] **Implement StorageConfig::from_keychain()**

  - Acceptance: Load credentials from keyring crate, construct config
  - Files: `backend/crates/altair-storage/src/config.rs`
  - Notes: Handle missing credentials with StorageError::CredentialsNotFound

- [x] **Define StorageError enum**

  - Acceptance: Variants for CredentialsNotFound, S3Error, InvalidMimeType, QuotaExceeded, etc.
  - Files: `backend/crates/altair-storage/src/error.rs`
  - Notes: Implement Display, Error, From<aws_sdk_s3::Error>

- [x] **Add config validation**
  - Acceptance: Validate endpoint format (http/https), bucket name (DNS-compliant)
  - Files: `backend/crates/altair-storage/src/config.rs`
  - Notes: Use regex for bucket name validation

### 1.2 S3 Client Wrapper

- [x] **Initialize aws-sdk-s3 client from StorageConfig**

  - Acceptance: Create S3Client struct with aws_sdk_s3::Client field, initialize from config
  - Files: `backend/crates/altair-storage/src/client.rs`
  - Notes: Use aws_config::BehaviorVersion::latest()

- [x] **Implement head_object()**

  - Acceptance: Return Result<ObjectMetadata, StorageError> with size, content_type
  - Files: `backend/crates/altair-storage/src/client.rs`

- [x] **Implement get_object() with streaming body**

  - Acceptance: Return Result<ByteStream, StorageError> for object data
  - Files: `backend/crates/altair-storage/src/client.rs`
  - Notes: Use aws_sdk_s3::primitives::ByteStream

- [x] **Implement delete_object()**

  - Acceptance: Delete object from S3, return Result<(), StorageError>
  - Files: `backend/crates/altair-storage/src/client.rs`

- [x] **Add connection health check method**
  - Acceptance: list_buckets() or HEAD bucket to verify connectivity
  - Files: `backend/crates/altair-storage/src/client.rs`

### 1.3 Presigned URL Generation

- [x] **Implement generate_upload_url() with PUT presigning**

  - Acceptance: Return PresignedUpload with url, key, expiration (15 min)
  - Files: `backend/crates/altair-storage/src/presigned.rs`
  - Notes: Use aws_sdk_s3::presigning::PresigningConfig

- [x] **Implement generate_download_url() with GET presigning**

  - Acceptance: Return presigned GET URL with 1-hour expiration
  - Files: `backend/crates/altair-storage/src/presigned.rs`

- [x] **Add content-type and content-length restrictions to upload URLs**

  - Acceptance: Upload URL enforces provided content-type and max content-length
  - Files: `backend/crates/altair-storage/src/presigned.rs`
  - Notes: Use PutObjectRequest::content_type() and content_length()

- [x] **Configure expiration times**

  - Acceptance: Upload URLs expire in 15 minutes, download URLs in 1 hour
  - Files: `backend/crates/altair-storage/src/presigned.rs`
  - Notes: Use Duration::from_secs()

- [x] **Generate UUID-prefixed object keys**
  - Acceptance: Object keys formatted as `{user_id}/{uuid}-{filename}`
  - Files: `backend/crates/altair-storage/src/presigned.rs`
  - Notes: Use uuid::Uuid::new_v4()

---

## Phase 2: Upload Flow

### 2.1 MIME Type Validation

- [x] **Define allowed MIME types constant**

  - Acceptance: Static array of allowed MIME types per spec (images, documents, audio)
  - Files: `backend/crates/altair-storage/src/mime.rs`
  - Notes: Reference spec.md FR-002 for complete list

- [x] **Implement validate_mime_type()**

  - Acceptance: Return Result<(), StorageError::InvalidMimeType>
  - Files: `backend/crates/altair-storage/src/mime.rs`

- [x] **Implement classify_media_type()**

  - Acceptance: Return MediaType enum (Photo, Audio, Video, Document) from MIME type
  - Files: `backend/crates/altair-storage/src/mime.rs`

- [x] **Add file extension to MIME type mapping**
  - Acceptance: Validate .jpg → image/jpeg, .pdf → application/pdf, etc.
  - Files: `backend/crates/altair-storage/src/mime.rs`
  - Notes: Uses once_cell::sync::Lazy for mapping

### 2.2 Checksum Calculation

- [x] **Implement in-memory SHA-256 for files ≤10MB**

  - Acceptance: Calculate correct SHA-256 for byte buffer, return hex string
  - Files: `backend/crates/altair-storage/src/checksum.rs`
  - Notes: Uses sha2::Sha256 and hex crate

- [x] **Implement streaming SHA-256 for files >10MB**

  - Acceptance: Calculate SHA-256 while streaming from S3, no full file in memory
  - Files: `backend/crates/altair-storage/src/checksum.rs`
  - Notes: Updates hash incrementally using ByteStream chunks

- [x] **Add async streaming from S3 GET response**

  - Acceptance: Integrate with get_object() ByteStream
  - Files: `backend/crates/altair-storage/src/checksum.rs`

- [x] **Return hex-encoded checksum string**
  - Acceptance: Checksum format matches test vectors (SHA-256 of "hello" = 2cf24dba...)
  - Files: `backend/crates/altair-storage/src/checksum.rs`

### 2.3 Upload Confirmation

- [x] **Implement request_upload() → PresignedUpload**

  - Acceptance: Validate MIME type, check quota, generate presigned URL
  - Files: `backend/crates/altair-storage/src/service.rs`
  - Notes: StorageService::request_upload() orchestrates mime.rs, presigned.rs (quota deferred to Phase 4)

- [x] **Implement confirm_upload() → UploadConfirmation**

  - Acceptance: HEAD check object exists, calculate checksum, return confirmation data
  - Files: `backend/crates/altair-storage/src/service.rs`
  - Notes: Returns UploadConfirmation struct; attachment record creation deferred to command layer

- [x] **Add HEAD check for object existence**

  - Acceptance: Return StorageError::ObjectNotFound if user claims upload but object missing
  - Files: `backend/crates/altair-storage/src/service.rs`

- [x] **Calculate checksum and return attachment metadata**

  - Acceptance: UploadConfirmation has storage_key, checksum, size_bytes, mime_type, media_type
  - Files: `backend/crates/altair-storage/src/service.rs`

- [x] **Update user quota**
  - Acceptance: storage_quota.bytes_used increases by file size
  - Files: `backend/crates/altair-storage/src/quota.rs`
  - Notes: Implemented as increment_quota() in Phase 4; integration into service layer via command layer in Phase 5

---

## Phase 3: Thumbnail Generation

### 3.1 Image Processing

- [x] **Implement generate_thumbnail() for supported formats**

  - Acceptance: Process JPEG, PNG, GIF, WebP inputs
  - Files: `backend/crates/altair-storage/src/thumbnail.rs`
  - Notes: Use image::open() from bytes

- [x] **Resize to 256×256 max dimension preserving aspect ratio**

  - Acceptance: 3000×2000 image → 256×171, 1000×1000 → 256×256
  - Files: `backend/crates/altair-storage/src/thumbnail.rs`
  - Notes: Use image::imageops::resize with Lanczos3 filter

- [x] **Encode as JPEG at 80% quality**

  - Acceptance: Output always JPEG regardless of input format
  - Files: `backend/crates/altair-storage/src/thumbnail.rs`
  - Notes: Use image::codecs::jpeg::JpegEncoder with quality 80

- [x] **Upload thumbnail to S3 with `_thumb` suffix key**
  - Acceptance: Original key `user/uuid-photo.jpg` → thumbnail key `user/uuid-photo_thumb.jpg`
  - Files: `backend/crates/altair-storage/src/thumbnail.rs`

### 3.2 Background Processing

- [x] **Queue thumbnail generation after upload confirmation**

  - Acceptance: confirm_upload() spawns background task for thumbnail generation
  - Files: `backend/crates/altair-storage/src/background.rs`, `backend/crates/altair-storage/src/service.rs`
  - Notes: Only for MediaType::Photo, via confirm_upload_with_thumbnail() method

- [x] **Use tokio::spawn for background processing**

  - Acceptance: Background task doesn't block confirm_upload() return
  - Files: `backend/crates/altair-storage/src/background.rs`

- [x] **Update attachment record with thumbnail_key when complete**

  - Acceptance: attachment.thumbnail_key populated after thumbnail upload
  - Files: `backend/crates/altair-storage/src/background.rs`
  - Notes: Callback-based design allows command layer to update database record

- [x] **Handle failures gracefully**
  - Acceptance: Failed thumbnail generation logs error, attachment still usable
  - Files: `backend/crates/altair-storage/src/background.rs`
  - Notes: Uses tracing::error! for logging, callback receives Result for flexible handling

---

## Phase 4: Quota Management

### 4.1 Database Schema

- [x] **Define storage_quota table**

  - Acceptance: Table with id, owner (user ref), bytes_used, bytes_limit, created_at, updated_at
  - Files: `backend/migrations/006_storage_quota.surql`
  - Notes: Follow altair-db table patterns

- [x] **Add CHANGEFEED 7d**

  - Acceptance: CHANGEFEED 7d on storage_quota table
  - Files: `backend/migrations/006_storage_quota.surql`

- [x] **Add index on owner field**

  - Acceptance: INDEX idx_storage_quota_owner ON storage_quota COLUMNS owner UNIQUE
  - Files: `backend/migrations/006_storage_quota.surql`

- [x] **Create default quota on user creation**
  - Acceptance: First storage access creates quota record with 5GB limit
  - Files: `backend/crates/altair-storage/src/quota.rs`
  - Notes: Implemented via get_quota() which auto-creates on first access

### 4.2 Quota Tracking

- [x] **Implement get_quota() returning usage and limit**

  - Acceptance: Return QuotaInfo { bytes_used, bytes_limit, bytes_available }
  - Files: `backend/crates/altair-storage/src/quota.rs`

- [x] **Implement check_quota() for pre-upload validation**

  - Acceptance: Return StorageError::QuotaExceeded if bytes_used + file_size > bytes_limit
  - Files: `backend/crates/altair-storage/src/quota.rs`

- [x] **Implement update_quota() for post-upload/delete updates**

  - Acceptance: Increment bytes_used on upload, decrement on delete
  - Files: `backend/crates/altair-storage/src/quota.rs`
  - Notes: Implemented as increment_quota() and decrement_quota() with SurrealDB atomic updates

- [x] **Add reconciliation method to sync with actual S3 usage**
  - Acceptance: List all user's objects in S3, sum sizes, compare with bytes_used, update if drift >1%
  - Files: `backend/crates/altair-storage/src/quota.rs`
  - Notes: Uses aws_sdk_s3::Client::list_objects_v2() with pagination

---

## Phase 5: Tauri Commands

### 5.1 Command Implementation

- [x] **Implement storage_request_upload command**

  - Acceptance: Tauri command validating inputs, calling StorageService::request_upload()
  - Files: `backend/crates/altair-commands/src/storage.rs`
  - Notes: #[tauri::command] with State<'\_, AppState>

- [x] **Implement storage_confirm_upload command**

  - Acceptance: Tauri command with storage_key, calling StorageService::confirm_upload()
  - Files: `backend/crates/altair-commands/src/storage.rs`

- [x] **Implement storage_get_url command**

  - Acceptance: Tauri command returning presigned download URL for attachment
  - Files: `backend/crates/altair-commands/src/storage.rs`

- [x] **Implement storage_delete command**

  - Acceptance: Tauri command archiving attachment, deleting S3 object, updating quota
  - Files: `backend/crates/altair-commands/src/storage.rs`

- [x] **Implement storage_get_quota command**
  - Acceptance: Tauri command returning QuotaInfo for current user
  - Files: `backend/crates/altair-commands/src/storage.rs`

### 5.2 Integration with AppState

- [x] **Initialize StorageService in AppState**

  - Acceptance: AppState.storage field populated with StorageService instance
  - Files: `apps/guidance/src-tauri/src/state.rs`
  - Notes: Load config from env or keychain during app init; graceful degradation if not configured

- [ ] **Start Minio process on app init**

  - Acceptance: Embedded Minio starts before AppState creation (if embedded mode)
  - Files: `backend/src/main.rs`
  - Notes: Deferred to Phase 6 (Minio Process Management)

- [ ] **Stop Minio process on app exit**

  - Acceptance: Graceful Minio shutdown in Tauri app cleanup
  - Files: `backend/src/main.rs`
  - Notes: Deferred to Phase 6 (Minio Process Management)

- [x] **Register storage commands with Tauri**
  - Acceptance: All 6 storage commands registered (5 + storage_is_available)
  - Files: `apps/guidance/src-tauri/src/lib.rs`, `apps/guidance/src-tauri/src/commands/storage.rs`
  - Notes: Uses tauri-specta for type-safe TypeScript bindings

---

## Phase 6: Minio Process Management

### 6.1 Embedded Binary

- [ ] **Locate Minio binary in app resources**

  - Acceptance: MinioManager finds platform-specific binary (minio-darwin-arm64, minio-windows.exe, etc.)
  - Files: `backend/crates/altair-storage/src/minio.rs`
  - Notes: Use tauri::api::path::resource_dir()

- [ ] **Determine data directory**

  - Acceptance: Data directory at platform-specific app data dir (e.g., ~/.local/share/altair/minio)
  - Files: `backend/crates/altair-storage/src/minio.rs`
  - Notes: Use directories crate for platform paths

- [ ] **Start Minio process with correct arguments**

  - Acceptance: Spawn Minio server with --address localhost:9000, --console-address localhost:9001
  - Files: `backend/crates/altair-storage/src/minio.rs`
  - Notes: Use tokio::process::Command

- [ ] **Health check loop until ready**

  - Acceptance: Poll localhost:9000/minio/health/live until 200 OK
  - Files: `backend/crates/altair-storage/src/minio.rs`
  - Notes: Timeout after 10 seconds

- [ ] **Graceful shutdown on app exit**
  - Acceptance: Send SIGTERM to Minio process, wait for clean exit
  - Files: `backend/crates/altair-storage/src/minio.rs`
  - Notes: Implement Drop trait for MinioManager

### 6.2 Fallback Configuration

- [ ] **Check for external Minio endpoint in config**

  - Acceptance: If STORAGE_ENDPOINT env var set, use external endpoint instead of embedded
  - Files: `backend/crates/altair-storage/src/config.rs`, `backend/crates/altair-storage/src/minio.rs`

- [ ] **Skip embedded binary if external configured**

  - Acceptance: MinioManager::new() returns Ok(None) if external endpoint configured
  - Files: `backend/crates/altair-storage/src/minio.rs`

- [ ] **Provide clear error if embedded fails and no fallback**

  - Acceptance: StorageError::MinioStartupFailed with instructions to set STORAGE_ENDPOINT
  - Files: `backend/crates/altair-storage/src/minio.rs`

- [ ] **Document fallback setup for development**
  - Acceptance: README section on running external Minio via Docker
  - Files: `backend/crates/altair-storage/README.md`

---

## Phase 7: Testing

### 7.1 Unit Tests

- [ ] **Config validation tests**

  - Acceptance: Test StorageConfig::validate() rejects invalid endpoints/buckets
  - Files: `backend/crates/altair-storage/src/config.rs`

- [ ] **Keychain mocking tests**

  - Acceptance: Test StorageConfig::from_keychain() with mock keyring (testable without actual keychain)
  - Files: `backend/crates/altair-storage/src/config.rs`

- [ ] **MIME validation tests**

  - Acceptance: Test allowed/disallowed MIME types
  - Files: `backend/crates/altair-storage/src/mime.rs`

- [ ] **Media classification tests**

  - Acceptance: Test classify_media_type() for all MediaType variants
  - Files: `backend/crates/altair-storage/src/mime.rs`

- [ ] **Checksum correctness tests**

  - Acceptance: Test SHA-256 against known test vectors
  - Files: `backend/crates/altair-storage/src/checksum.rs`

- [ ] **Quota calculation logic tests**
  - Acceptance: Test quota overflow prevention, negative bytes_available handling
  - Files: `backend/crates/altair-storage/src/quota.rs`

### 7.2 Integration Tests

- [ ] **TS-001: Upload small image (<1MB)**

  - Acceptance: End-to-end upload flow creates attachment record with correct metadata
  - Files: `backend/tests/storage/upload_flow_test.rs`
  - Notes: Use testcontainers for Minio

- [ ] **TS-002: Upload large file (50MB)**

  - Acceptance: Streaming checksum works, memory usage bounded
  - Files: `backend/tests/storage/upload_flow_test.rs`

- [ ] **TS-003: Upload with invalid MIME type**

  - Acceptance: request_upload() returns StorageError::InvalidMimeType
  - Files: `backend/tests/storage/upload_flow_test.rs`

- [ ] **TS-004: Upload exceeding quota**

  - Acceptance: request_upload() returns StorageError::QuotaExceeded
  - Files: `backend/tests/storage/quota_test.rs`

- [ ] **TS-005: Confirm upload for non-existent object**

  - Acceptance: confirm_upload() returns StorageError::ObjectNotFound
  - Files: `backend/tests/storage/upload_flow_test.rs`

- [ ] **TS-006: Download with expired URL**

  - Acceptance: Presigned GET URL returns 403 after expiration
  - Files: `backend/tests/storage/upload_flow_test.rs`
  - Notes: Mock time or use short expiration for testing

- [ ] **TS-008: Delete attachment cleanup**

  - Acceptance: delete() archives attachment, deletes S3 object, decrements quota
  - Files: `backend/tests/storage/upload_flow_test.rs`

- [ ] **TS-009: Thumbnail generation for various formats**
  - Acceptance: Thumbnails created for JPEG, PNG, GIF, WebP inputs
  - Files: `backend/tests/storage/thumbnail_test.rs`

### 7.3 Performance Tests

- [ ] **Presigned URL generation latency**

  - Acceptance: Average of 100 iterations <50ms
  - Files: `backend/benches/storage_bench.rs`
  - Notes: Use criterion crate

- [ ] **Confirm upload latency (1MB file)**

  - Acceptance: Average <500ms including checksum calculation
  - Files: `backend/benches/storage_bench.rs`

- [ ] **Thumbnail generation latency (10MB image)**
  - Acceptance: Average <2s
  - Files: `backend/benches/storage_bench.rs`

---

## Ready to Start

✅ **Constitution check passed** (all 7 principles aligned)
✅ **Dependencies validated** (all internal dependencies complete)
✅ **Risks identified and mitigated**

**Next Steps**:

1. Begin Phase 0 research tasks to validate external dependencies
2. Proceed to Phase 1 infrastructure once research complete
3. Implement phases sequentially with testing at each stage
4. Mark tasks complete in this file as you progress

**Estimated Effort**: ~5-7 days for experienced Rust developer (varies by Minio embedding complexity)
