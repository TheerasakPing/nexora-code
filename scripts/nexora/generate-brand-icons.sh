#!/usr/bin/env bash
# Regenerate the original Nexora Code icon family from the editable SVG master.
# Requires rsvg-convert + ImageMagick + macOS iconutil; no network access.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SOURCE="$ROOT/brand/nexora-mark.svg"
DEST="$ROOT/src-tauri/icons"
command -v rsvg-convert >/dev/null
command -v magick >/dev/null
command -v iconutil >/dev/null
[[ -f "$SOURCE" ]] || { echo "Missing master SVG" >&2; exit 2; }
TMP="$(mktemp -d /tmp/nexora-brand-iconset.XXXXXX)"
trap 'rm -rf "$TMP"' EXIT
ICONSET="$TMP/NexoraCode.iconset"
mkdir -p "$ICONSET" "$DEST"
for size in 16 32 64 128 256 512 1024; do
  rsvg-convert --width "$size" --height "$size" --output "$TMP/$size.png" "$SOURCE"
done
cp "$TMP/32.png" "$DEST/32x32.png"
cp "$TMP/128.png" "$DEST/128x128.png"
cp "$TMP/256.png" "$DEST/128x128@2x.png"
cp "$TMP/16.png" "$ICONSET/icon_16x16.png"
cp "$TMP/32.png" "$ICONSET/icon_16x16@2x.png"
cp "$TMP/32.png" "$ICONSET/icon_32x32.png"
cp "$TMP/64.png" "$ICONSET/icon_32x32@2x.png"
cp "$TMP/128.png" "$ICONSET/icon_128x128.png"
cp "$TMP/256.png" "$ICONSET/icon_128x128@2x.png"
cp "$TMP/256.png" "$ICONSET/icon_256x256.png"
cp "$TMP/512.png" "$ICONSET/icon_256x256@2x.png"
cp "$TMP/512.png" "$ICONSET/icon_512x512.png"
cp "$TMP/1024.png" "$ICONSET/icon_512x512@2x.png"
iconutil -c icns -o "$DEST/icon.icns" "$ICONSET"
magick "$TMP/16.png" "$TMP/32.png" "$TMP/64.png" "$TMP/256.png" "$DEST/icon.ico"
cp "$SOURCE" "$ROOT/public/icon.svg"
cp "$SOURCE" "$ROOT/public/nexora-mark.svg"
echo 'Nexora Code original SVG/PNG/ICNS/ICO generated'
