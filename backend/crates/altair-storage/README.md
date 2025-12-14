# altair-storage

S3-compatible object storage integration for Altair applications.

## Features

- **Presigned URLs**: Time-limited upload/download URLs for direct S3 access
- **MIME Type Validation**: Only allowed file types can be uploaded
- **Quota Management**: Per-user storage limits with quota tracking
- **Thumbnail Generation**: Automatic thumbnail creation for images
- **Secure Credentials**: OS keychain integration for credential storage
- **Embedded Minio**: Optional local S3-compatible storage server

## Quick Start

### Using External Storage (Recommended for Development)

The simplest way to get started is using Docker to run Minio:

```bash
# Start Minio container
docker run -d \
  --name altair-minio \
  -p 9000:9000 \
  -p 9001:9001 \
  -v ~/minio-data:/data \
  -e MINIO_ROOT_USER=altair \
  -e MINIO_ROOT_PASSWORD=altairdev123 \
  minio/minio server /data --console-address ":9001"

# Create a bucket (using Minio console at http://localhost:9001)
# Or using mc CLI:
mc alias set altair http://localhost:9000 altair altairdev123
mc mb altair/altair-guidance
```

Then set environment variables:

```bash
export STORAGE_ENDPOINT=http://localhost:9000
export STORAGE_BUCKET=altair-guidance
export S3_ACCESS_KEY_ID=altair
export S3_SECRET_ACCESS_KEY=altairdev123
```

### Using Embedded Minio

For production desktop apps, Minio can be embedded and managed automatically:

```rust
use altair_storage::{MinioConfig, MinioManager, init_storage_with_minio};

// Start with default configuration
let config = MinioConfig::new("guidance");
let (storage_config, manager) = init_storage_with_minio(config, "altair-guidance", None).await?;

// Manager will stop Minio when dropped
```

### Using OS Keychain

For secure credential storage:

```rust
use altair_storage::StorageConfig;

// Store credentials (run once during setup)
StorageConfig::store_credentials("my-access-key", "my-secret-key")?;

// Load from keychain
let config = StorageConfig::from_keychain()?;
```

## Environment Variables

| Variable               | Description                     | Default                 |
| ---------------------- | ------------------------------- | ----------------------- |
| `STORAGE_ENDPOINT`     | S3 endpoint URL                 | `http://localhost:9000` |
| `STORAGE_REGION`       | S3 region                       | `us-east-1`             |
| `STORAGE_BUCKET`       | Bucket name                     | `altair`                |
| `S3_ACCESS_KEY_ID`     | Access key (overrides keychain) | -                       |
| `S3_SECRET_ACCESS_KEY` | Secret key (overrides keychain) | -                       |
| `MINIO_BINARY_PATH`    | Custom Minio binary path        | Auto-detected           |

## Fallback Logic

The storage system uses this priority:

1. **External endpoint**: If `STORAGE_ENDPOINT` is set, use external S3-compatible service
2. **Embedded Minio**: Start bundled Minio binary from Tauri resources
3. **Error with instructions**: If both fail, provide clear guidance

## Platform Support

### Embedded Minio Binaries

| Platform | Architecture | Binary Name          |
| -------- | ------------ | -------------------- |
| macOS    | arm64        | `minio-darwin-arm64` |
| macOS    | x86_64       | `minio-darwin-amd64` |
| Windows  | x86_64       | `minio-windows.exe`  |
| Linux    | x86_64       | `minio-linux-amd64`  |
| Linux    | arm64        | `minio-linux-arm64`  |

### Data Directory Locations

| Platform | Location                                               |
| -------- | ------------------------------------------------------ |
| macOS    | `~/Library/Application Support/com.altair.{app}/minio` |
| Windows  | `%LOCALAPPDATA%\altair\{app}\minio`                    |
| Linux    | `~/.local/share/altair/{app}/minio`                    |

### Keychain Support

| Platform | Backend                    |
| -------- | -------------------------- |
| macOS    | Keychain                   |
| Windows  | Credential Manager         |
| Linux    | Secret Service (libsecret) |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     StorageService                               │
│  (High-level API: request_upload, confirm_upload, get_url)      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   S3Client   │  │ PresignedUrl │  │   Quota      │          │
│  │  (aws-sdk)   │  │   Service    │  │  Manager     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  Thumbnail   │  │   Checksum   │  │    MIME      │          │
│  │  Generator   │  │  Calculator  │  │  Validator   │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                      MinioManager (Optional)                     │
│  (Process lifecycle: start, health check, shutdown)              │
└─────────────────────────────────────────────────────────────────┘
```

## Development

### Running Tests

```bash
# Unit tests (no external services required)
cargo test -p altair-storage

# Integration tests (requires Minio)
docker-compose -f docker-compose.test.yml up -d
cargo test -p altair-storage --features integration-tests
```

### Adding Minio Binaries to Tauri

1. Download binaries from [Minio releases](https://github.com/minio/minio/releases)
2. Place in `apps/{app}/src-tauri/resources/bin/`
3. Configure `tauri.conf.json`:

```json
{
  "bundle": {
    "resources": ["resources/bin/*"]
  }
}
```

## Troubleshooting

### "Port 9000 already in use"

Another process is using port 9000. Either:

- Stop the existing process: `docker stop altair-minio`
- Use external endpoint: `export STORAGE_ENDPOINT=http://localhost:9000`

### "Minio binary not found"

Embedded Minio binaries not bundled. For development:

- Use Docker: See "Using External Storage" above
- Set `MINIO_BINARY_PATH` to a local Minio binary

### "Credentials not found"

Either store credentials in keychain:

```rust
StorageConfig::store_credentials("key", "secret")?;
```

Or set environment variables:

```bash
export S3_ACCESS_KEY_ID=your-key
export S3_SECRET_ACCESS_KEY=your-secret
```

## License

Part of the Altair project. See repository root for license.
