# Minio Embedding Strategy

**Status**: Phase 0 Research
**Spec**: CORE-011-STORAGE-SERVICE
**Updated**: 2025-12-12

## Overview

This document describes the strategy for embedding Minio binary into Altair applications to provide local S3-compatible object storage without requiring external infrastructure.

## Platform-Specific Binary Paths

### Binary Naming Convention

Minio binaries follow platform-specific naming:

| Platform | Architecture | Binary Name          | Size (approx) |
| -------- | ------------ | -------------------- | ------------- |
| macOS    | arm64        | `minio-darwin-arm64` | ~70MB         |
| macOS    | x86_64       | `minio-darwin-amd64` | ~75MB         |
| Windows  | x86_64       | `minio-windows.exe`  | ~75MB         |
| Linux    | x86_64       | `minio-linux-amd64`  | ~70MB         |
| Linux    | arm64        | `minio-linux-arm64`  | ~65MB         |

### Resource Directory Location

Binaries will be bundled in Tauri's resource directory:

```
apps/{app}/src-tauri/resources/
  └── bin/
      ├── minio-darwin-arm64
      ├── minio-darwin-amd64
      ├── minio-windows.exe
      ├── minio-linux-amd64
      └── minio-linux-arm64
```

**Access via Tauri API:**

```rust
use tauri::api::path::resource_dir;

let resource_dir = resource_dir(&app.package_info(), &app.env())
    .expect("Failed to get resource directory");

let binary_name = get_platform_binary_name(); // e.g., "minio-darwin-arm64"
let binary_path = resource_dir.join("bin").join(binary_name);
```

### Platform Detection

Use Rust's compile-time platform detection:

```rust
fn get_platform_binary_name() -> &'static str {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return "minio-darwin-arm64";

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return "minio-darwin-amd64";

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return "minio-windows.exe";

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return "minio-linux-amd64";

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return "minio-linux-arm64";

    #[cfg(not(any(
        all(target_os = "macos", any(target_arch = "aarch64", target_arch = "x86_64")),
        all(target_os = "windows", target_arch = "x86_64"),
        all(target_os = "linux", any(target_arch = "x86_64", target_arch = "aarch64"))
    )))]
    compile_error!("Unsupported platform for Minio embedding");
}
```

## Data Directory Strategy

### Platform-Specific App Data Locations

Use the `directories` crate for platform-appropriate data directories:

| Platform | Data Directory Pattern                                 |
| -------- | ------------------------------------------------------ |
| macOS    | `~/Library/Application Support/com.altair.{app}/minio` |
| Windows  | `%LOCALAPPDATA%\altair\{app}\minio`                    |
| Linux    | `~/.local/share/altair/{app}/minio`                    |

**Implementation:**

```rust
use directories::ProjectDirs;

fn get_minio_data_dir(app_name: &str) -> PathBuf {
    let project_dirs = ProjectDirs::from("com", "altair", app_name)
        .expect("Failed to determine project directories");

    project_dirs.data_dir().join("minio")
}
```

### Data Directory Structure

```
{data_dir}/
  └── minio/
      ├── .minio.sys/       # Minio system metadata
      ├── {bucket-name}/    # User buckets
      │   └── {objects}
      └── config/           # Minio config (optional)
```

### Persistence

- Data persists between app restarts
- Survives app updates (located outside app bundle)
- Backed up with user data by OS backup systems

### First-Run Initialization

1. Check if data directory exists
2. If not, create directory structure
3. Initialize Minio on first start (creates `.minio.sys/`)
4. Create default bucket (`altair-{app}`)

## Process Lifecycle Management

### Design Decision: tokio::process vs Tauri Process API

**Choice: tokio::process::Command**

Rationale:

- Already using Tokio for async runtime
- More control over process lifecycle
- Better integration with Rust async ecosystem
- Easier to implement graceful shutdown

### Startup Sequence

1. **Pre-flight checks:**

   - Verify binary exists and is executable
   - Check data directory permissions
   - Validate not already running (check port 9000)

2. **Spawn process:**

   ```rust
   use tokio::process::Command;

   let mut child = Command::new(&binary_path)
       .arg("server")
       .arg(&data_dir)
       .arg("--address")
       .arg("localhost:9000")
       .arg("--console-address")
       .arg("localhost:9001")
       .env("MINIO_ROOT_USER", "altair")
       .env("MINIO_ROOT_PASSWORD", &generate_secure_password())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped())
       .spawn()?;
   ```

3. **Health check loop:**

   ```rust
   async fn wait_for_minio_ready(timeout: Duration) -> Result<(), MinioError> {
       let start = Instant::now();
       let client = reqwest::Client::new();

       while start.elapsed() < timeout {
           match client.get("http://localhost:9000/minio/health/live").send().await {
               Ok(response) if response.status().is_success() => return Ok(()),
               _ => tokio::time::sleep(Duration::from_millis(100)).await,
           }
       }

       Err(MinioError::StartupTimeout)
   }
   ```

4. **Store process handle:**
   - Keep `Child` handle in `AppState`
   - Enables shutdown control

### Shutdown Sequence

**Normal Shutdown:**

1. Send SIGTERM to Minio process
2. Wait up to 5 seconds for clean exit
3. If still running, send SIGKILL
4. Close data files

```rust
impl Drop for MinioManager {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            // Try graceful shutdown
            let _ = child.kill(); // Sends SIGTERM on Unix

            // Wait for exit with timeout
            let timeout = Duration::from_secs(5);
            let _ = tokio::time::timeout(timeout, child.wait()).await;
        }
    }
}
```

**Crash Recovery:**

- Minio handles crash recovery internally
- On next startup, Minio performs integrity check
- No manual recovery needed

**User Force-Quit:**

- Minio data remains intact
- Next startup performs health check
- Temporary files cleaned up automatically

### Error Handling

| Scenario                 | Handling                                       |
| ------------------------ | ---------------------------------------------- |
| Binary not found         | Fall back to external endpoint (if configured) |
| Port 9000 already in use | Fail fast, log error, suggest external config  |
| Startup timeout          | Fail fast, check logs, suggest troubleshooting |
| Unexpected termination   | Restart once, then fail to external config     |

## Fallback Configuration

### External Minio Endpoint

When embedded binary fails or is not desired:

**Configuration via Environment Variables:**

```bash
STORAGE_ENDPOINT=http://localhost:9000
STORAGE_ACCESS_KEY=minioadmin
STORAGE_SECRET_KEY=minioadmin
STORAGE_BUCKET=altair-guidance
```

**OR via Config File (future):**

```toml
# ~/.config/altair/storage.toml
[storage]
endpoint = "http://localhost:9000"
access_key = "minioadmin"
secret_key = "minioadmin"
bucket = "altair-guidance"
```

### Decision Logic

```rust
async fn init_storage(config: &AppConfig) -> Result<StorageService, StorageError> {
    // Check for external endpoint configuration
    if let Ok(endpoint) = env::var("STORAGE_ENDPOINT") {
        tracing::info!("Using external Minio endpoint: {}", endpoint);
        return init_external_storage(endpoint, config);
    }

    // Attempt embedded Minio
    match MinioManager::start().await {
        Ok(manager) => {
            tracing::info!("Started embedded Minio on localhost:9000");
            Ok(StorageService::new_embedded(manager))
        }
        Err(e) => {
            tracing::error!("Failed to start embedded Minio: {}", e);
            Err(StorageError::MinioStartupFailed(
                "Set STORAGE_ENDPOINT environment variable to use external Minio".to_string()
            ))
        }
    }
}
```

### Development Setup

For development with external Minio:

```bash
# Start Minio via Docker
docker run -d \
  -p 9000:9000 \
  -p 9001:9001 \
  -v ~/minio-data:/data \
  --name altair-minio \
  minio/minio server /data --console-address ":9001"

# Set environment variables
export STORAGE_ENDPOINT=http://localhost:9000
export STORAGE_ACCESS_KEY=minioadmin
export STORAGE_SECRET_KEY=minioadmin

# Run Altair app
pnpm --filter guidance dev
```

## Security Considerations

### Credential Management

1. **Root credentials generation:**

   - Generate secure random password on first run
   - Store in OS keychain (via `keyring` crate)
   - Never store in plaintext

2. **Access from app:**
   - App code reads credentials from keychain
   - Credentials only in memory during runtime
   - No credentials in config files

### Network Binding

- **Bind to localhost only** (--address localhost:9000)
- Prevents external network access
- Desktop apps don't need remote access

### File Permissions

- Data directory: User read/write only (chmod 700)
- Binary: User execute (chmod 755)
- Configuration: User read/write (chmod 600)

## Implementation Checklist

Phase 0.2 Tasks:

- [x] Document platform-specific binary paths
- [x] Design process lifecycle management
- [x] Document data directory strategy
- [x] Define fallback behavior

## References

- [Minio Documentation](https://min.io/docs/minio/linux/operations/installation.html)
- [Tauri Resource Directory](https://tauri.app/v1/api/js/path/#resourcedir)
- [directories crate](https://docs.rs/directories/latest/directories/)
- [tokio::process](https://docs.rs/tokio/latest/tokio/process/index.html)
