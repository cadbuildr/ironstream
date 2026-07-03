#!/usr/bin/env bash
# Fetch the pinned OCCT source mirror (headers + GTests) used as ground truth
# for the faithful Rust port. Re-runnable; /tmp is ephemeral across sessions.
set -euo pipefail
COMMIT=4f95ecaa3b690e34988d42e2ca7fe882e7a8bc7d
DEST=${1:-/tmp/occt-mirror}
if [ -d "$DEST/src" ] && [ "$(find "$DEST/src" -name '*.hxx' | head -1)" ]; then
  echo "mirror present at $DEST ($(find "$DEST/src" -name '*.hxx' | wc -l) headers)"; exit 0
fi
mkdir -p "$DEST"
curl -sS --max-time 180 -L \
  "https://codeload.github.com/Open-Cascade-SAS/OCCT/tar.gz/$COMMIT" -o /tmp/occt.tar.gz
TOP=$(tar -tzf /tmp/occt.tar.gz | head -1 | cut -d/ -f1)
tar -xzf /tmp/occt.tar.gz -C "$DEST" "$TOP/src"
rm -rf "$DEST/src"; mv "$DEST/$TOP/src" "$DEST/src"; rmdir "$DEST/$TOP"
echo "mirror ready: $(find "$DEST/src" -name '*.hxx' | wc -l) headers, $(find "$DEST/src" -path '*/GTests/*.cxx' | wc -l) gtests"
