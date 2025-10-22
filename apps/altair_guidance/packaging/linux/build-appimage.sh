#!/usr/bin/env bash

# Script to build AppImage for Altair Guidance
# This creates a portable Linux application that runs on all distributions

set -e

# Configuration
APP_NAME="Altair Guidance"
APP_ID="com.getaltair.altair_guidance"
VERSION="${VERSION:-0.1.0}"
ARCH="${ARCH:-x86_64}"

# Directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_DIR="$APP_DIR/build/linux/x64/profile"
APPDIR="$BUILD_DIR/AppDir"
PACKAGING_DIR="$SCRIPT_DIR"

echo "======================================"
echo "Building AppImage for $APP_NAME"
echo "Version: $VERSION"
echo "Architecture: $ARCH"
echo "======================================"

# Step 1: Build the Flutter app in profile mode
echo ""
echo "[1/6] Building Flutter app in profile mode..."
cd "$APP_DIR"
flutter build linux --profile

# Step 2: Create AppDir structure
echo ""
echo "[2/6] Creating AppDir structure..."
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/lib"
mkdir -p "$APPDIR/usr/share/applications"
mkdir -p "$APPDIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$APPDIR/usr/share/metainfo"

# Step 3: Copy application files
echo ""
echo "[3/6] Copying application files..."
cp -r "$BUILD_DIR/bundle"/* "$APPDIR/usr/bin/"

# Step 4: Copy desktop file and icon
echo ""
echo "[4/6] Setting up desktop integration..."
cp "$PACKAGING_DIR/altair-guidance.desktop" "$APPDIR/usr/share/applications/"
cp "$APPDIR/usr/share/applications/altair-guidance.desktop" "$APPDIR/"

# Create icon (using a placeholder - should be replaced with actual icon)
# For now, we'll create a simple PNG placeholder
if command -v convert &> /dev/null; then
    convert -size 512x512 xc:'#FFD93D' -fill black -gravity center \
        -pointsize 48 -annotate +0+0 "Altair\nGuidance" \
        "$APPDIR/usr/share/icons/hicolor/512x512/apps/altair_guidance.png"
    cp "$APPDIR/usr/share/icons/hicolor/512x512/apps/altair_guidance.png" "$APPDIR/"
else
    echo "Warning: ImageMagick not found. Skipping icon generation."
    echo "Please provide an icon at: $APPDIR/altair_guidance.png"
fi

# Step 5: Create AppRun script
echo ""
echo "[5/6] Creating AppRun script..."
cat > "$APPDIR/AppRun" << 'EOF'
#!/bin/bash
APPDIR="$(dirname "$(readlink -f "${0}")")"
export LD_LIBRARY_PATH="${APPDIR}/usr/lib:${LD_LIBRARY_PATH}"
export PATH="${APPDIR}/usr/bin:${PATH}"
cd "${APPDIR}/usr/bin"
exec "${APPDIR}/usr/bin/altair_guidance" "$@"
EOF
chmod +x "$APPDIR/AppRun"

# Step 6: Build AppImage
echo ""
echo "[6/6] Building AppImage..."

# Download appimagetool if not present
APPIMAGETOOL="$BUILD_DIR/appimagetool-$ARCH.AppImage"
if [ ! -f "$APPIMAGETOOL" ]; then
    echo "Downloading appimagetool..."
    wget -q "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-$ARCH.AppImage" \
        -O "$APPIMAGETOOL"
    chmod +x "$APPIMAGETOOL"
fi

# Build the AppImage
OUTPUT_APPIMAGE="$BUILD_DIR/AltairGuidance-$VERSION-$ARCH.AppImage"
ARCH=$ARCH "$APPIMAGETOOL" "$APPDIR" "$OUTPUT_APPIMAGE"

echo ""
echo "======================================"
echo "✅ AppImage built successfully!"
echo "======================================"
echo ""
echo "Output: $OUTPUT_APPIMAGE"
echo "Size: $(du -h "$OUTPUT_APPIMAGE" | cut -f1)"
echo ""
echo "To run the AppImage:"
echo "  chmod +x $OUTPUT_APPIMAGE"
echo "  ./$OUTPUT_APPIMAGE"
echo ""
