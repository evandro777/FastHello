#!/usr/bin/env bash
# One-off: turn the template into your app. Usage: ./rename.sh <crate> <DisplayName> <github-owner>
# e.g. ./rename.sh penguincleaner PenguinCleaner evandro777
set -euo pipefail
cd "$(dirname "$0")"
crate=$1 name=$2 owner=$3
[[ $crate =~ ^[a-z][a-z0-9_]*$ && $name =~ ^[A-Za-z][A-Za-z0-9]*$ && $owner =~ ^[A-Za-z0-9-]+$ ]] || { echo "bad name"; exit 1; }
old_id=io.github.OWNER.MyApp new_id=io.github.$owner.$name
# Env var prefix of the fixtures (MYAPP_FIXTURES, MYAPP_PAGE). No `-` in the crate name: `myapp_core` would become an invalid identifier.
env_prefix=${crate^^}
# Needs a git repository (`git init && git add -A` in a fresh copy): the file list comes from git. Binary files
# (the icon and screenshots) and local settings are left alone.
# GitHub Pages host names are lower case (OWNER.github.io), so that rule runs before the generic OWNER one.
git ls-files -z --cached --others --exclude-standard | grep -zvE '^rename\.sh$|\.png$|^\.claude/settings' | xargs -0 sed -i \
  -e "s/MYAPP_/${env_prefix}_/g" -e "s/$old_id/$new_id/g" -e "s#OWNER/MyApp#$owner/$name#g" -e "s/myapp/$crate/g" -e "s/MyApp/$name/g" \
  -e "s/io\.github\.owner\"/io.github.${owner,,}\"/g" -e "s/OWNER\.github\.io/${owner,,}.github.io/g" -e "s/OWNER/$owner/g"
for f in packaging/$old_id.*; do mv "$f" "${f/$old_id/$new_id}"; done
mv "flatpak/$old_id.yml" "flatpak/$new_id.yml"
# The template's public signing key must not be reused: step 4 below generates a fresh one for the new app.
rm -f flatpak/signing-key.b64
for po in crates/app/translations/*/LC_MESSAGES/myapp.po; do mv "$po" "${po/myapp.po/$crate.po}"; done
cat <<EOF
Done. Now:
  1. delete rename.sh, run python3 dev/gen_po.py, cargo build
  2. rewrite README.md, AGENTS.md ("What this is", "Open items"), docs/site/index.html and the metainfo description
     (search for "EDIT"), and replace docs/screenshots/main.png with a real screenshot
  3. create the GitHub repository named exactly $name (the Pages URL and the AppImage update information assume it)
  4. flatpak/new-signing-key.sh, then do the one-time setup in RELEASING.md before the first tag
EOF
