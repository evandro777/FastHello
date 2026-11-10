#!/usr/bin/env bash
# One-off: prepare a new app from the template (rename, reset the version, drop the template notes, first build).
# Usage: ./prepare.sh <crate> <DisplayName> <github-owner>
# e.g. ./prepare.sh penguincleaner PenguinCleaner evandro777
set -euo pipefail
cd "$(dirname "$0")"
crate=$1 name=$2 owner=$3
[[ $crate =~ ^[a-z][a-z0-9_]*$ && $name =~ ^[A-Za-z][A-Za-z0-9]*$ && $owner =~ ^[A-Za-z0-9-]+$ ]] || { echo "bad name"; exit 1; }
old_id=io.github.OWNER.MyApp new_id=io.github.$owner.$name
# Env var prefix of the fixtures (MYAPP_FIXTURES, MYAPP_PAGE). No `-` in the crate name: `myapp_core` would become an invalid identifier.
env_prefix=${crate^^}
# The file list comes from git, so a fresh copy without a repository gets one. Binary files (the icon and
# screenshots) and local settings are left alone.
git rev-parse --git-dir &>/dev/null || { git init -q && git add -A; }
# GitHub Pages host names are lower case (OWNER.github.io), so that rule runs before the generic OWNER one.
git ls-files -z --cached --others --exclude-standard | grep -zvE '^prepare\.sh$|\.png$|^\.claude/settings' | xargs -0 sed -i \
  -e "s/MYAPP_/${env_prefix}_/g" -e "s/$old_id/$new_id/g" -e "s#OWNER/MyApp#$owner/$name#g" -e "s/myapp/$crate/g" -e "s/MyApp/$name/g" \
  -e "s/io\.github\.owner\"/io.github.${owner,,}\"/g" -e "s/OWNER\.github\.io/${owner,,}.github.io/g" -e "s/OWNER/$owner/g"
for f in packaging/$old_id.*; do mv "$f" "${f/$old_id/$new_id}"; done
mv "flatpak/$old_id.yml" "flatpak/$new_id.yml"
# The template's public signing key must not be reused: step 3 below generates a fresh one for the new app.
rm -f flatpak/signing-key.b64
for po in crates/app/translations/*/LC_MESSAGES/myapp.po; do mv "$po" "${po/myapp.po/$crate.po}"; done
# A new app starts at 0.1.0, not at the template's version.
sed -i '0,/^version = /s/^version = .*/version = "0.1.0"/' Cargo.toml
sed -i -E "s/<release version=\"[^\"]*\" date=\"[^\"]*\"/<release version=\"0.1.0\" date=\"$(date +%F)\"/" "packaging/$new_id.metainfo.xml"
sed -i '/^> Template note:/d' README.md AGENTS.md
# Gone before the build: the rename cannot run twice, so a failed build must not leave a script inviting a re-run.
rm -- "$0"
python3 dev/gen_po.py
cargo build
cat <<EOF
Done. The template is renamed, the version is 0.1.0 and the first build passed. Still yours to do:
  1. rewrite README.md, AGENTS.md ("What this is", "Open items"), docs/site/index.html and the metainfo description
     (search for "EDIT"), and replace docs/screenshots/main.png with a real screenshot
  1b. draw the app icon in packaging/ following docs/ICON-STYLE.md (new tile color and symbol, then regenerate the PNG)
  2. create the GitHub repository named exactly $name (the Pages URL and the AppImage update information assume it)
  3. flatpak/new-signing-key.sh, then do the one-time setup in RELEASING.md before the first tag
EOF
