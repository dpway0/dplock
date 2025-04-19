#!/usr/bin/env bash

set -e

APP_NAME="dplock"
BUILD_DIR="target"
OUTPUT_DIR="dists"
VERSION=$(awk -F\" '/^version =/ {print $2}' Cargo.toml)

# Create output directory
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"


# Build static Linux x86_64
cargo build --release --target x86_64-unknown-linux-musl
echo "✅ Linux x86_64 build done."
cp "$BUILD_DIR/x86_64-unknown-linux-musl/release/$APP_NAME" "$OUTPUT_DIR/${APP_NAME}-${VERSION}-linux-x86_64"

# Build static Linux ARM64
#target="aarch64-unknown-linux-musl"
#cargo build --release --target "$target"
#echo "✅ Linux ARM64 build done."
#cp "$BUILD_DIR/$target/release/$APP_NAME" "$OUTPUT_DIR/${APP_NAME}-linux-arm64"

# Build macOS x86_64
target="x86_64-apple-darwin"
cargo build --release --target "$target"
echo "✅ macOS x86_64 build done."
cp "$BUILD_DIR/$target/release/$APP_NAME" "$OUTPUT_DIR/${APP_NAME}-${VERSION}-macos-x86_64"

# Build macOS ARM64
target="aarch64-apple-darwin"
cargo build --release --target "$target"
echo "✅ macOS ARM64 build done."
cp "$BUILD_DIR/$target/release/$APP_NAME" "$OUTPUT_DIR/${APP_NAME}-${VERSION}-macos-arm64"

# Build Windows x86_64
#target="x86_64-pc-windows-gnu"
#cargo build --release --target "$target"
#echo "✅ Windows x86_64 build done."
#cp "$BUILD_DIR/$target/release/${APP_NAME}.exe" "$OUTPUT_DIR/${APP_NAME}-windows-x86_64.exe"

# Build Windows ARM64
#target="aarch64-pc-windows-gnu"
#cargo build --release --target "$target"
#echo "✅ Windows ARM64 build done."
#cp "$BUILD_DIR/$target/release/${APP_NAME}.exe" "$OUTPUT_DIR/${APP_NAME}-windows-arm64.exe"

# Build .deb package for Linux x86_64
target="x86_64-unknown-linux-musl"
cargo deb --target "$target"
echo "✅ .deb package build done."
DEB_FILE="$BUILD_DIR/$target/debian/${APP_NAME}_${VERSION}-1_amd64.deb" || true


if [ -z "$DEB_FILE" ]; then
    echo "❌ ERROR: .deb file not found!"
    exit 1
fi

cp "$DEB_FILE" "$OUTPUT_DIR"

# Call tar.sh to package binaries
bash "$(dirname "$0")/tar.sh"

echo "🎉 Build & packaging completed!"