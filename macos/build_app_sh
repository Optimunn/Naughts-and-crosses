#!/bin/bash

APP_NAME="Naughts and crosses"
BINARY_PATH="../target/release/naughts_and_crosses"
ICON_PATH="./icon/nac_icon.png"   
OUTPUT_DIR="./"

APP_DIR="$OUTPUT_DIR/$APP_NAME.app"

if [ "$1" = "--clear" ] || [ "$1" = "-c" ]; then
    if [ -d "$APP_DIR" ]; then
        echo "Deleting previous version..."
        rm -rf "$APP_DIR"
    fi
    exit 0
elif [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Options:"
    echo "  --clear, -c    Clear previous version"
    echo "  --help, -h     Print this help message"
    echo "  --release      Build release binary"
    echo "  --all          Build release application"
    echo "  (no options)   Build app and activate it"
    exit 0
elif [ "$1" = "--release" ]; then
    echo "Build release binary..."
    cargo build --release
    exit 0
elif [ "$1" = "--all" ]; then
    echo "Build release application..."
    cargo build --release
fi

check_file_exists() {
    if [ ! -f "$1" ]; then
        echo "Error: File not found: $1"
        exit 1
    fi
}

check_executable() {
    if [ ! -x "$1" ]; then
        echo "Warning: File is not executable. Adding execute permission to $1"
        chmod +x "$1"
    fi
}

check_file_exists "$BINARY_PATH"
check_file_exists "$ICON_PATH"
check_executable "$BINARY_PATH"

CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
ASSETS_DIR="$CONTENTS_DIR/Assets"

if [ -d "$APP_DIR" ]; then
    echo "Deleting previous version..."
    rm -rf "$APP_DIR"
fi

echo "Creating directory structure..."
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"
mkdir -p "$ASSETS_DIR"

FONT_PATH="../assets/Arial.ttf"
check_file_exists "$FONT_PATH"

echo "Copying files..."
cp "$BINARY_PATH" "$MACOS_DIR/$APP_NAME"
chmod +x "$MACOS_DIR/$APP_NAME"

cp "$ICON_PATH" "$RESOURCES_DIR/AppIcon.png"

cp "$FONT_PATH" "$ASSETS_DIR/Arial.ttf"

echo "Creating Info.plist..."
cat > "$CONTENTS_DIR/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.AlexIlush.$APP_NAME</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15.0</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2025 Alex</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <true/>
</dict>
</plist>
EOF

echo "APPL????" > "$CONTENTS_DIR/PkgInfo"

echo "Application successfully created: $APP_DIR"
echo "Size: $(du -sh "$APP_DIR" | cut -f1)"