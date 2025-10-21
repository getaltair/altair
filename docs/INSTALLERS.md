# Standalone Installers

> **TL;DR:** Altair Guidance has standalone installers for Linux (AppImage), macOS (DMG), and Windows (Inno Setup). Download and run—no Flutter SDK required.

## Quick Start

**What you need to know in 60 seconds:**

- **Pre-built installers** available for all major platforms
- **No dependencies** required—everything bundled
- **Portable** options available (AppImage, unpacked Windows)
- **Auto-updates** coming in future releases

**Navigation:**

- [Installation](#installation) - Download and install
- [Building from Source](#building-from-source) - Create installers yourself
- [CI/CD Integration](#cicd-integration) - Automated builds
- [Troubleshooting](#troubleshooting) - Common issues

---

## Installation

### Linux (Multiple Options)

#### Option 1: AUR (Arch Linux)

For Arch Linux and derivatives (Manjaro, EndeavourOS, etc.):

```bash
# Using yay
yay -S altair-guidance

# Using paru
paru -S altair-guidance

# Manual installation
git clone https://aur.archlinux.org/altair-guidance.git
cd altair-guidance
makepkg -si
```

#### Option 2: AppImage (All Distributions)

AppImage is a portable format that runs on all major Linux distributions.

**Download:**

```bash
# From GitHub Releases
wget https://github.com/getaltair/altair/releases/latest/download/AltairGuidance-0.1.0-x86_64.AppImage
```

**Install and Run:**

```bash
# Make executable
chmod +x AltairGuidance-*.AppImage

# Run directly
./AltairGuidance-*.AppImage

# Optional: Integrate with system (requires appimaged)
./AltairGuidance-*.AppImage --appimage-integrate
```

**System Integration (optional):**

```bash
# Install appimaged for automatic desktop integration
sudo apt install appimaged  # Debian/Ubuntu
# OR
yay -S appimaged           # Arch

# The AppImage will appear in your application menu
```

### macOS (DMG)

DMG provides a familiar drag-and-drop installation experience.

**Download:**

- Visit [Releases](https://github.com/getaltair/altair/releases)
- Download `AltairGuidance-0.1.0.dmg`

**Install:**

1. Open the downloaded DMG file
2. Drag "Altair Guidance" to the Applications folder
3. Eject the DMG
4. Launch from Applications or Spotlight

**First Launch (Gatekeeper):**

```bash
# If macOS blocks the app (unsigned)
xattr -cr /Applications/Altair\ Guidance.app
```

Then right-click the app and select "Open" to bypass Gatekeeper.

### Windows (Installer)

Inno Setup installer with automatic uninstaller registration.

**Download:**

- Visit [Releases](https://github.com/getaltair/altair/releases)
- Download `AltairGuidance-0.1.0-Setup.exe`

**Install:**

1. Run the installer
2. Follow the installation wizard
3. Choose installation location
4. Optionally create desktop shortcut
5. Click Install

**Portable Mode (Advanced):**

```powershell
# Extract without installing
.\AltairGuidance-Setup.exe /VERYSILENT /DIR="C:\PortableApps\AltairGuidance"
```

---

## Building from Source

### Prerequisites

All platforms require:

- **Flutter SDK** 3.0+
- **Git**

Platform-specific:

- **Linux**: `libgtk-3-dev`, `appimagetool`, ImageMagick (optional)
- **macOS**: Xcode Command Line Tools, `create-dmg` (via Homebrew)
- **Windows**: Visual Studio 2022, Inno Setup 6

### Quick Build

```bash
# Clone repository
git clone https://github.com/getaltair/altair.git
cd altair

# Build for your platform
./scripts/build-installers.sh

# Build for specific platform
./scripts/build-installers.sh linux
./scripts/build-installers.sh macos
./scripts/build-installers.sh windows  # PowerShell on Windows
```

### Platform-Specific Instructions

#### Linux: Build AUR Package

```bash
cd apps/altair_guidance/packaging/aur

# Install dependencies (Arch Linux)
sudo pacman -S base-devel flutter gtk3 libsecret

# Build package
makepkg -f

# Install locally
makepkg -fi

# Output: altair-guidance-0.1.0-1-x86_64.pkg.tar.zst
```

#### Linux: Build AppImage

```bash
cd apps/altair_guidance

# Install dependencies
sudo apt-get install -y \
    clang cmake ninja-build pkg-config \
    libgtk-3-dev desktop-file-utils \
    fakeroot patchelf wget imagemagick

# Build
VERSION=0.1.0 bash packaging/linux/build-appimage.sh

# Output: build/linux/x64/release/AltairGuidance-0.1.0-x86_64.AppImage
```

**Build Snap (optional):**

```bash
cd apps/altair_guidance

# Install snapcraft
sudo snap install snapcraft --classic

# Build
snapcraft --use-lxd

# Output: altair-guidance_0.1.0_amd64.snap
```

#### macOS: Build DMG

```bash
cd apps/altair_guidance

# Install create-dmg
brew install create-dmg

# Build
VERSION=0.1.0 bash packaging/macos/build-dmg.sh

# Output: build/macos/Build/Products/Release/AltairGuidance-0.1.0.dmg
```

**Code Signing (optional):**

```bash
# Sign the app bundle (requires Apple Developer certificate)
codesign --deep --force --verify --verbose \
    --sign "Developer ID Application: Your Name" \
    build/macos/Build/Products/Release/altair_guidance.app

# Then build DMG
VERSION=0.1.0 bash packaging/macos/build-dmg.sh
```

#### Windows: Build Installer

```powershell
cd apps\altair_guidance

# Install Inno Setup
choco install innosetup -y

# Build
powershell.exe -ExecutionPolicy Bypass `
    -File packaging\windows\build-installer.ps1 `
    -Version "0.1.0"

# Output: build\windows\installer\AltairGuidance-0.1.0-Setup.exe
```

---

## CI/CD Integration

### GitHub Actions

Automated builds on version tags:

```yaml
# Create a new release
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions will:
# 1. Build installers for all platforms
# 2. Create draft GitHub Release
# 3. Upload installers as release assets
```

**Manual Trigger:**

```bash
# Via GitHub UI
# Actions → Release Installers → Run workflow
# Enter version: 0.1.0
```

**Workflow Configuration:**

See [`.github/workflows/release.yml`](../.github/workflows/release.yml) for details.

### Local CI Testing

```bash
# Test Linux build (on Linux)
docker run --rm -v $(pwd):/workspace -w /workspace ubuntu:22.04 bash -c "
    apt-get update && apt-get install -y git curl && \
    git clone https://github.com/flutter/flutter.git -b stable && \
    export PATH=\$PATH:\$PWD/flutter/bin && \
    cd /workspace && ./scripts/build-installers.sh linux
"

# Test macOS build (on macOS)
./scripts/build-installers.sh macos

# Test Windows build (on Windows)
.\scripts\build-installers.sh windows
```

---

## Troubleshooting

### Linux

**AppImage won't run:**

```bash
# Check for FUSE
modprobe fuse

# If FUSE unavailable, extract and run
./AltairGuidance-*.AppImage --appimage-extract
./squashfs-root/AppRun
```

**Missing libraries:**

```bash
# Check dependencies
ldd ./AltairGuidance-*.AppImage

# Install missing GTK libraries
sudo apt install libgtk-3-0
```

### macOS

**"App is damaged" error:**

```bash
# Remove quarantine attribute
xattr -cr /Applications/Altair\ Guidance.app
```

**Code signing issues:**

```bash
# Check signature
codesign -dv /Applications/Altair\ Guidance.app

# Verify bundle
codesign --verify --verbose /Applications/Altair\ Guidance.app
```

### Windows

**Installer blocked by SmartScreen:**

- Right-click installer
- Select "Properties"
- Check "Unblock"
- Click "OK"
- Run installer

**Missing DLL errors:**

```powershell
# Reinstall Visual C++ Redistributable
# Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe
```

### Build Issues

**Flutter SDK not found:**

```bash
# Verify Flutter installation
flutter --version

# Add to PATH
export PATH="$PATH:$HOME/flutter/bin"  # Linux/macOS
# OR
# Add C:\flutter\bin to System PATH    # Windows
```

**Build tools missing:**

```bash
# Linux
sudo apt install build-essential

# macOS
xcode-select --install

# Windows
# Install Visual Studio 2022 with C++ tools
```

---

## Advanced Topics

### Custom Branding

**Replace App Icon:**

Linux (AppImage):

```bash
# Replace icon before building
cp your-icon.png apps/altair_guidance/packaging/linux/altair_guidance.png
```

macOS (DMG):

```bash
# Replace app icons (multiple sizes)
# See: apps/altair_guidance/macos/Runner/Assets.xcassets/AppIcon.appiconset/
```

Windows (Installer):

```bash
# Replace app icon
# See: apps/altair_guidance/windows/runner/resources/app_icon.ico
```

### Portable Installations

**Linux (Portable AppImage):**

```bash
# AppImages are already portable
# Just copy to USB drive and run
```

**macOS (Portable):**

```bash
# Extract app from DMG
# Copy altair_guidance.app anywhere
# Run from any location
```

**Windows (Portable ZIP):**

```powershell
# Build without installer
cd apps\altair_guidance
flutter build windows --release

# Create portable ZIP
Compress-Archive -Path build\windows\x64\runner\Release\* `
    -DestinationPath AltairGuidance-Portable.zip
```

### Distribution Channels

**Linux:**

- [AUR](https://aur.archlinux.org/) (Arch User Repository - use provided `PKGBUILD`)
- [Flathub](https://flathub.org/) (requires Flatpak manifest - future work)
- [Snap Store](https://snapcraft.io/) (use provided `snapcraft.yaml`)
- [AppImage Hub](https://www.appimagehub.com/)

**macOS:**

- Mac App Store (requires Apple Developer account)
- Homebrew Cask (community-maintained)
- Direct download from website

**Windows:**

- Microsoft Store (requires developer account)
- Chocolatey (community package)
- Winget (community package)
- Direct download from website

---

## Release Checklist

Before publishing a new release:

- [ ] Version bumped in `pubspec.yaml`
- [ ] CHANGELOG.md updated with changes
- [ ] All tests passing (CI green)
- [ ] Installers built for all platforms
- [ ] Installers tested on clean systems
- [ ] Code signed (macOS/Windows)
- [ ] Release notes written
- [ ] GitHub Release created
- [ ] Download links verified
- [ ] Documentation updated
- [ ] Announcement prepared

---

## Related Documentation

- [Development Roadmap](./DEVELOPMENT-ROADMAP.md) - Feature timeline
- [Deployment Guide](./DEPLOYMENT-GUIDE.md) - Server deployment
- [Architecture Overview](./ARCHITECTURE-OVERVIEW.md) - System design

---

**Last updated:** October 21, 2025
**Next review:** Monthly
**Maintained by:** Altair Development Team
