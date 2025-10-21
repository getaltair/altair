#!/usr/bin/env bash

# Script to build DMG installer for Altair Guidance on macOS
# Creates a drag-and-drop installer with background image

set -e

# Configuration
APP_NAME="Altair Guidance"
BUNDLE_NAME="altair_guidance.app"
VERSION="${VERSION:-0.1.0}"
DMG_NAME="AltairGuidance-${VERSION}.dmg"

# Directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_DIR="$APP_DIR/build/macos/Build/Products/Release"
PACKAGING_DIR="$SCRIPT_DIR"

echo "======================================"
echo "Building DMG for $APP_NAME"
echo "Version: $VERSION"
echo "======================================"

# Step 1: Build the Flutter app in release mode
echo ""
echo "[1/5] Building Flutter app in release mode..."
cd "$APP_DIR"
flutter build macos --release

# Verify the app bundle was created
if [ ! -d "$BUILD_DIR/$BUNDLE_NAME" ]; then
    echo "Error: App bundle not found at $BUILD_DIR/$BUNDLE_NAME"
    exit 1
fi

# Step 2: Create temporary DMG directory
echo ""
echo "[2/5] Creating DMG staging directory..."
DMG_DIR="$BUILD_DIR/dmg"
rm -rf "$DMG_DIR"
mkdir -p "$DMG_DIR"

# Step 3: Copy app bundle and create Applications symlink
echo ""
echo "[3/5] Copying app bundle..."
cp -R "$BUILD_DIR/$BUNDLE_NAME" "$DMG_DIR/"
ln -s /Applications "$DMG_DIR/Applications"

# Step 4: Create DS_Store for proper icon positioning (optional)
echo ""
echo "[4/5] Setting up DMG layout..."
# Note: This would require additional tooling on macOS
# For now, we'll create a basic DMG

# Step 5: Create the DMG
echo ""
echo "[5/5] Creating DMG image..."
OUTPUT_DMG="$BUILD_DIR/$DMG_NAME"
rm -f "$OUTPUT_DMG"

if command -v create-dmg &> /dev/null; then
    # Use create-dmg if available (better UX)
    create-dmg \
        --volname "$APP_NAME" \
        --volicon "$BUILD_DIR/$BUNDLE_NAME/Contents/Resources/AppIcon.icns" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --icon "$BUNDLE_NAME" 175 120 \
        --hide-extension "$BUNDLE_NAME" \
        --app-drop-link 425 120 \
        "$OUTPUT_DMG" \
        "$DMG_DIR"
else
    # Fallback to hdiutil (basic but works)
    echo "Note: create-dmg not found, using hdiutil (install create-dmg for better DMGs)"
    hdiutil create -volname "$APP_NAME" \
        -srcfolder "$DMG_DIR" \
        -ov -format UDZO \
        "$OUTPUT_DMG"
fi

# Cleanup
rm -rf "$DMG_DIR"

echo ""
echo "======================================"
echo "✅ DMG built successfully!"
echo "======================================"
echo ""
echo "Output: $OUTPUT_DMG"
echo "Size: $(du -h "$OUTPUT_DMG" | cut -f1)"
echo ""
echo "To install:"
echo "  1. Double-click the DMG"
echo "  2. Drag Altair Guidance to Applications"
echo "  3. Eject the DMG"
echo ""
