# Packaging Configuration

This directory contains packaging configurations and build scripts for creating standalone installers for Altair Guidance across different platforms.

## Directory Structure

```
packaging/
├── linux/
│   ├── build-appimage.sh       # Script to build AppImage
│   ├── altair-guidance.desktop # Desktop entry file
│   └── snapcraft.yaml          # Snap package configuration
├── aur/
│   ├── PKGBUILD                # Arch Linux AUR package
│   ├── .gitignore              # AUR build artifacts
│   └── README.md               # AUR package documentation
├── macos/
│   └── build-dmg.sh            # Script to build DMG installer
├── windows/
│   └── build-installer.ps1     # PowerShell script to build Inno Setup installer
└── README.md                   # This file
```

## Building Installers

### Quick Build (Any Platform)

```bash
# From repository root
./scripts/build-installers.sh

# Or build for specific platform
./scripts/build-installers.sh linux
./scripts/build-installers.sh macos    # macOS only
./scripts/build-installers.sh windows  # Windows only
```

### Platform-Specific Builds

#### Linux (AppImage)

```bash
cd packaging/linux
VERSION=0.1.0 bash build-appimage.sh
```

**Requirements:**

- Flutter SDK
- GTK 3 development files
- appimagetool (auto-downloaded)
- ImageMagick (optional, for icon generation)

**Output:** `build/linux/x64/release/AltairGuidance-0.1.0-x86_64.AppImage`

#### Linux (Snap)

```bash
cd ../..  # Back to app root
snapcraft --use-lxd
```

**Requirements:**

- snapcraft
- LXD or Multipass

**Output:** `altair-guidance_0.1.0_amd64.snap`

#### Linux (AUR)

```bash
cd packaging/aur

# Build
makepkg -f

# Install locally
makepkg -fi
```

**Requirements:**

- Arch Linux or derivative
- base-devel package group
- Flutter SDK

**Output:** `altair-guidance-0.1.0-1-x86_64.pkg.tar.zst`

#### macOS (DMG)

```bash
cd packaging/macos
VERSION=0.1.0 bash build-dmg.sh
```

**Requirements:**

- Flutter SDK
- Xcode Command Line Tools
- create-dmg (via Homebrew: `brew install create-dmg`)

**Output:** `build/macos/Build/Products/Release/AltairGuidance-0.1.0.dmg`

#### Windows (Inno Setup)

```powershell
cd packaging\windows
powershell -ExecutionPolicy Bypass -File build-installer.ps1 -Version "0.1.0"
```

**Requirements:**

- Flutter SDK
- Visual Studio 2022 with C++ tools
- Inno Setup 6 (`choco install innosetup`)

**Output:** `build\windows\installer\AltairGuidance-0.1.0-Setup.exe`

## Configuration Files

### Linux Desktop Entry

`linux/altair-guidance.desktop` defines how the application appears in the desktop environment:

- Application name and description
- Icon and executable paths
- Categories and keywords for menu integration
- MIME type associations (if any)

### Snap Configuration

`linux/snapcraft.yaml` defines the Snap package:

- Base and confinement settings
- Required plugs (home, network, desktop, etc.)
- Build dependencies and stage packages
- Flutter plugin configuration

### Windows Inno Setup

Generated dynamically by `windows/build-installer.ps1`:

- Application metadata
- Installation directories
- Desktop shortcuts and Start Menu entries
- Uninstaller configuration

## Customization

### Changing App Icon

**Linux:**

```bash
# Replace icon before building
cp your-icon-512x512.png packaging/linux/altair_guidance.png
```

**macOS:**

```bash
# Replace all icon sizes in:
# macos/Runner/Assets.xcassets/AppIcon.appiconset/
```

**Windows:**

```bash
# Replace:
# windows/runner/resources/app_icon.ico
```

### Changing App Name

Update the following files:

- `pubspec.yaml` - Application name
- `linux/CMakeLists.txt` - BINARY_NAME and APPLICATION_ID
- `macos/Runner/Configs/AppInfo.xcconfig` - PRODUCT_NAME
- `windows/runner/CMakeLists.txt` - BINARY_NAME

### Version Management

All build scripts accept a `VERSION` environment variable:

```bash
VERSION=1.0.0 ./build-appimage.sh
VERSION=1.0.0 ./build-dmg.sh
VERSION=1.0.0 powershell ./build-installer.ps1
```

## CI/CD Integration

GitHub Actions workflow (`.github/workflows/release.yml`) automatically builds installers when you push a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This triggers:

1. Parallel builds on Linux, macOS, and Windows runners
2. Artifact uploads for each platform
3. Draft GitHub Release creation
4. Installer attachments to the release

Manual workflow dispatch is also available via GitHub Actions UI.

## Testing Installers

### Linux (AppImage)

```bash
# Make executable
chmod +x AltairGuidance-*.AppImage

# Run
./AltairGuidance-*.AppImage

# Test desktop integration
./AltairGuidance-*.AppImage --appimage-integrate
```

### Linux (Snap)

```bash
# Install locally
sudo snap install --dangerous altair-guidance_*.snap

# Run
snap run altair-guidance

# Remove
sudo snap remove altair-guidance
```

### macOS (DMG)

```bash
# Mount DMG
hdiutil attach AltairGuidance-*.dmg

# Copy to Applications
cp -R "/Volumes/Altair Guidance/Altair Guidance.app" /Applications/

# Eject
hdiutil detach "/Volumes/Altair Guidance"

# Run
open /Applications/Altair\ Guidance.app
```

### Windows (Installer)

```powershell
# Silent install for testing
.\AltairGuidance-Setup.exe /VERYSILENT /DIR="C:\TestInstall"

# Run
& "C:\TestInstall\altair_guidance.exe"

# Uninstall
& "C:\TestInstall\unins000.exe" /VERYSILENT
```

## Troubleshooting

See [INSTALLERS.md](../../../docs/INSTALLERS.md#troubleshooting) for comprehensive troubleshooting guide.

### Common Issues

**Build fails with "Flutter not found":**

```bash
# Add Flutter to PATH
export PATH="$PATH:$HOME/flutter/bin"  # Linux/macOS
```

**AppImage won't run (Linux):**

```bash
# Extract and run manually
./AltairGuidance-*.AppImage --appimage-extract
./squashfs-root/AppRun
```

**"App is damaged" on macOS:**

```bash
# Remove quarantine
xattr -cr /Applications/Altair\ Guidance.app
```

**Inno Setup not found (Windows):**

```powershell
# Install via Chocolatey
choco install innosetup -y
```

## Distribution

### Linux

- **Flathub**: Requires Flatpak manifest (future work)
- **Snap Store**: `snapcraft upload --release=stable altair-guidance_*.snap`
- **AppImage Hub**: Submit via <https://www.appimagehub.com/>

### macOS

- **Mac App Store**: Requires Apple Developer account and code signing
- **Homebrew Cask**: Create formula in homebrew-cask repository
- **Direct Download**: Host DMG on website/GitHub Releases

### Windows

- **Microsoft Store**: Requires developer account and MSIX packaging
- **Chocolatey**: Create package in chocolatey repository
- **Winget**: Create manifest in winget-pkgs repository
- **Direct Download**: Host installer on website/GitHub Releases

## Further Reading

- [Flutter Desktop Documentation](https://docs.flutter.dev/desktop)
- [AppImage Documentation](https://docs.appimage.org/)
- [Snapcraft Documentation](https://snapcraft.io/docs)
- [Inno Setup Documentation](https://jrsoftware.org/ishelp/)
- [create-dmg GitHub](https://github.com/create-dmg/create-dmg)

---

**Maintained by:** Altair Development Team
**Last updated:** October 21, 2025
