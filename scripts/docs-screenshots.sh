#!/usr/bin/env bash
# Writes the screenshots the README, the metainfo and the landing page use: docs/screenshots/<page>-light|dark.png.
# Same private Xvfb and canned data as ui-snapshot.sh; a scratch settings file fixes theme and window size.
set -euo pipefail
cd "$(dirname "$0")/.."

W=860 H=600
PAGES=(main widgets widgets-table widgets-buttons settings)
for theme in light dark; do
  data=$(mktemp -d)
  mkdir -p "$data/FastHello"
  printf 'theme = "%s"\nwidth = %d\nheight = %d\n' "$theme" "$W" "$H" >"$data/FastHello/settings.toml"
  XDG_DATA_HOME=$data MODES=many LC_ALL=C LANGUAGE= scripts/ui-snapshot.sh "${PAGES[@]}" >/dev/null
  for p in "${PAGES[@]}"; do
    # The snapshot is the whole 1280x800 screen; the window sits at the top left.
    convert "target/ui-snapshots/$p-many.png" -crop "${W}x${H}+0+0" +repage "docs/screenshots/$p-$theme.png"
  done
  rm -rf "$data"
done
ls docs/screenshots
