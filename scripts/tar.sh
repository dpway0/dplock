#!/usr/bin/env bash

set -e

APP_NAME="dplock"
VERSION=$(awk -F\" '/^version =/ {print $2}' Cargo.toml)
OUTPUT_DIR="dists"

cd "$OUTPUT_DIR"

FILES=(${APP_NAME}-${VERSION}-*)
if [ ${#FILES[@]} -eq 0 ]; then
    echo "❌ No files found to package!"
    exit 1
fi

for file in "${FILES[@]}"; do
    if [[ -f "$file" && ! "$file" =~ \.deb$ ]]; then
        platform=$(echo "$file" | sed "s/${APP_NAME}-${VERSION}-//")

        # Rename the bin file to 'dplock'
        mv "$file" "${APP_NAME}"
        echo "🔄 Renamed: $file -> ${APP_NAME}"

        tarball="${APP_NAME}-${VERSION}-${platform}.tar.gz"

        # Create the tar.gz file
        tar -czf "$tarball" "${APP_NAME}"
        echo "🗜️ Packed: $tarball"

        rm "${APP_NAME}"
    fi
done

echo "🎉 Packaging complete! Final files:"
ls *.tar.gz
