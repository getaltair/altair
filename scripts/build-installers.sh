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

# Build Linux packages
build_linux() {
    local format=$1

    cd "$GUIDANCE_DIR"

    case $format in
        appimage|"")
            print_message "$YELLOW" "\nBuilding AppImage for Linux..."
            bash packaging/linux/build-appimage.sh
            print_message "$GREEN" "\n✅ Linux AppImage built successfully!"
            ;;

        snap)
            print_message "$YELLOW" "\nBuilding Snap package for Linux..."

            if ! command -v snapcraft &> /dev/null; then
                print_message "$RED" "Error: snapcraft not found"
                print_message "$YELLOW" "Install snapcraft:"
                print_message "$CYAN" "  sudo snap install snapcraft --classic"
                exit 1
            fi

            snapcraft --use-lxd
            print_message "$GREEN" "\n✅ Linux Snap package built successfully!"
            ;;

        aur)
            print_message "$YELLOW" "\nBuilding AUR package for Arch Linux..."

            if ! command -v makepkg &> /dev/null; then
                print_message "$RED" "Error: makepkg not found (not on Arch Linux?)"
                print_message "$YELLOW" "AUR packages can only be built on Arch Linux or derivatives"
                exit 1
            fi

            cd packaging/aur
            makepkg -f
            print_message "$GREEN" "\n✅ AUR package built successfully!"
            ;;

        all)
            print_message "$YELLOW" "\nBuilding all Linux formats..."

            # Build AppImage
            build_linux appimage

            # Build Snap if available
            if command -v snapcraft &> /dev/null; then
                print_message "$YELLOW" "\nBuilding Snap package..."
                cd "$GUIDANCE_DIR"
                build_linux snap
            else
                print_message "$YELLOW" "\nSkipping Snap (snapcraft not installed)"
            fi

            # Build AUR if on Arch
            if command -v makepkg &> /dev/null; then
                print_message "$YELLOW" "\nBuilding AUR package..."
                cd "$GUIDANCE_DIR"
                build_linux aur
            else
                print_message "$YELLOW" "\nSkipping AUR (not on Arch Linux)"
            fi

            print_message "$GREEN" "\n✅ All available Linux formats built successfully!"
            ;;

        *)
            print_message "$RED" "Error: Unknown Linux format: $format"
            print_message "$YELLOW" "Valid formats: appimage, snap, aur, all"
            exit 1
            ;;
    esac
}

# Build for specific platform
build_platform() {
    local platform=$1
    local format=$2

    print_message "$CYAN" "======================================"
    print_message "$CYAN" "Building installer for $platform"
    print_message "$CYAN" "Version: $VERSION"
    print_message "$CYAN" "======================================"

    case $platform in
        linux)
            build_linux "$format"
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

    # Parse arguments
    local target_platform=$PLATFORM
    local linux_format=""

    if [ $# -gt 0 ]; then
        target_platform=$1
        shift

        # Parse Linux-specific options
        if [ "$target_platform" = "linux" ] && [ $# -gt 0 ]; then
            case $1 in
                --appimage)
                    linux_format="appimage"
                    ;;
                --snap)
                    linux_format="snap"
                    ;;
                --aur)
                    linux_format="aur"
                    ;;
                --all)
                    linux_format="all"
                    ;;
                *)
                    print_message "$RED" "Error: Unknown option: $1"
                    show_usage
                    ;;
            esac
        fi

        print_message "$YELLOW" "Building for requested platform: $target_platform"
        if [ -n "$linux_format" ]; then
            print_message "$YELLOW" "Linux format: $linux_format"
        fi
    fi

    # Check if Flutter is installed
    if ! command -v flutter &> /dev/null; then
        print_message "$RED" "Error: Flutter not found in PATH"
        print_message "$YELLOW" "Please install Flutter: https://flutter.dev/docs/get-started/install"
        exit 1
    fi

    # Build for the platform
    build_platform "$target_platform" "$linux_format"

    print_message "$GREEN" "\n======================================"
    print_message "$GREEN" "All builds completed successfully!"
    print_message "$GREEN" "======================================"
}

# Show usage
show_usage() {
    echo "Usage: $0 [platform] [options]"
    echo ""
    echo "Build standalone installers for Altair Guidance"
    echo ""
    echo "Platforms:"
    echo "  linux      Build Linux installers (default: AppImage)"
    echo "  macos      Build DMG installer"
    echo "  windows    Build Windows installer (Inno Setup)"
    echo ""
    echo "Linux-specific options:"
    echo "  --appimage Build AppImage (default)"
    echo "  --snap     Build Snap package"
    echo "  --aur      Build AUR package (Arch Linux)"
    echo "  --all      Build all Linux formats"
    echo ""
    echo "If no platform is specified, builds for the current platform"
    echo ""
    echo "Environment variables:"
    echo "  VERSION    Version number (default: 0.1.0)"
    echo ""
    echo "Examples:"
    echo "  $0                           # Build for current platform (AppImage on Linux)"
    echo "  $0 linux                     # Build AppImage for Linux"
    echo "  $0 linux --snap              # Build Snap package"
    echo "  $0 linux --aur               # Build AUR package"
    echo "  $0 linux --all               # Build all Linux formats"
    echo "  VERSION=1.0.0 $0 linux       # Build with custom version"
    exit 0
}

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_usage
fi

main "$@"
