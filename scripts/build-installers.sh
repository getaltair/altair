#!/usr/bin/env bash

# Cross-platform script to build installers for Altair Guidance
# Automatically detects platform and builds appropriate installer

set -e

# Configuration
VERSION="${VERSION:-0.1.0}"
APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GUIDANCE_DIR="$APP_DIR/apps/altair_guidance"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Detect platform
detect_platform() {
    case "$(uname -s)" in
        Linux*)     PLATFORM=linux;;
        Darwin*)    PLATFORM=macos;;
        MINGW*|MSYS*|CYGWIN*)    PLATFORM=windows;;
        *)          PLATFORM=unknown;;
    esac
}

# Print colored message
print_message() {
    local color=$1
    shift
    echo -e "${color}$@${NC}"
}

# Build for specific platform
build_platform() {
    local platform=$1

    print_message "$CYAN" "======================================"
    print_message "$CYAN" "Building installer for $platform"
    print_message "$CYAN" "Version: $VERSION"
    print_message "$CYAN" "======================================"

    cd "$GUIDANCE_DIR"

    case $platform in
        linux)
            print_message "$YELLOW" "\nBuilding AppImage for Linux..."
            bash packaging/linux/build-appimage.sh

            print_message "$GREEN" "\n✅ Linux AppImage built successfully!"
            print_message "$YELLOW" "\nOptional: Build Snap package"
            print_message "$YELLOW" "To build Snap, install snapcraft and run:"
            print_message "$CYAN" "  cd $GUIDANCE_DIR"
            print_message "$CYAN" "  snapcraft --use-lxd"
            ;;

        macos)
            if [ "$(uname -s)" != "Darwin" ]; then
                print_message "$RED" "Error: macOS builds must be run on macOS"
                exit 1
            fi

            print_message "$YELLOW" "\nBuilding DMG for macOS..."
            bash packaging/macos/build-dmg.sh

            print_message "$GREEN" "\n✅ macOS DMG built successfully!"
            ;;

        windows)
            if [ "$(uname -s)" != "MINGW"* ] && [ "$(uname -s)" != "MSYS"* ]; then
                print_message "$RED" "Error: Windows builds must be run on Windows"
                print_message "$YELLOW" "Run the following command in PowerShell:"
                print_message "$CYAN" "  powershell.exe packaging/windows/build-installer.ps1"
                exit 1
            fi

            print_message "$YELLOW" "\nBuilding installer for Windows..."
            powershell.exe -ExecutionPolicy Bypass -File packaging/windows/build-installer.ps1 -Version "$VERSION"

            print_message "$GREEN" "\n✅ Windows installer built successfully!"
            ;;

        *)
            print_message "$RED" "Error: Unknown platform: $platform"
            exit 1
            ;;
    esac
}

# Main script
main() {
    detect_platform

    print_message "$CYAN" "======================================"
    print_message "$CYAN" "Altair Guidance Installer Builder"
    print_message "$CYAN" "======================================"
    print_message "$YELLOW" "Detected platform: $PLATFORM"
    print_message "$YELLOW" "Version: $VERSION"
    echo ""

    # Check if specific platform requested
    if [ $# -gt 0 ]; then
        PLATFORM=$1
        print_message "$YELLOW" "Building for requested platform: $PLATFORM"
    fi

    # Check if Flutter is installed
    if ! command -v flutter &> /dev/null; then
        print_message "$RED" "Error: Flutter not found in PATH"
        print_message "$YELLOW" "Please install Flutter: https://flutter.dev/docs/get-started/install"
        exit 1
    fi

    # Build for the platform
    build_platform "$PLATFORM"

    print_message "$GREEN" "\n======================================"
    print_message "$GREEN" "All builds completed successfully!"
    print_message "$GREEN" "======================================"
}

# Show usage
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 [platform]"
    echo ""
    echo "Build standalone installers for Altair Guidance"
    echo ""
    echo "Platforms:"
    echo "  linux      Build AppImage (and optionally Snap)"
    echo "  macos      Build DMG installer"
    echo "  windows    Build Windows installer (Inno Setup)"
    echo ""
    echo "If no platform is specified, builds for the current platform"
    echo ""
    echo "Environment variables:"
    echo "  VERSION    Version number (default: 0.1.0)"
    echo ""
    echo "Examples:"
    echo "  $0                    # Build for current platform"
    echo "  $0 linux              # Build for Linux"
    echo "  VERSION=1.0.0 $0     # Build with custom version"
    exit 0
fi

main "$@"
