#!/usr/bin/env bash
# One-off: prepare a new app from the template (rename, reset the version, drop the template notes, first build).
# Usage: ./prepare.sh <crate> <DisplayName> <github-owner>
# e.g. ./prepare.sh penguincleaner PenguinCleaner evandro777
set -euo pipefail
cd "$(dirname "$0")"
crate=$1 name=$2 owner=$3
[[ $crate =~ ^[a-z][a-z0-9_]*$ && $name =~ ^[A-Za-z][A-Za-z0-9]*$ && $owner =~ ^[A-Za-z0-9-]+$ ]] || { echo "bad name"; exit 1; }
# What this template is called: every one of these is replaced by the new app's name.
old_crate=fasthello old_name=FastHello old_owner=evandro777
old_id=io.github.$old_owner.$old_name new_id=io.github.$owner.$name
# Env var prefix of the fixtures (FASTHELLO_FIXTURES, FASTHELLO_PAGE). No `-` in the crate name: `fasthello_core` would become an invalid identifier.
env_prefix=${crate^^}
# The file list comes from git, so a fresh copy without a repository gets one. Binary files (the icon and
# screenshots) and local settings are left alone, and so are the files shared verbatim by every Fast app: they name
# FastHello as the master copy and the author, which stays true in the new app.
git rev-parse --git-dir &>/dev/null || { git init -q && git add -A; }
shared='^crates/app/src/instance\.rs$|^crates/core/src/files\.rs$|^crates/app/ui/(family|about|controls)\.slint$|^docs/(UI|ICON)-STYLE\.md$'
# GitHub Pages host names are lower case, so that rule runs before the generic owner one.
git ls-files -z --cached --others --exclude-standard | grep -zvE "^prepare\.sh\$|\.png\$|^\.claude/settings|$shared" | xargs -0 sed -i \
  -e "s/${old_crate^^}_/${env_prefix}_/g" -e "s/$old_id/$new_id/g" -e "s#$old_owner/$old_name#$owner/$name#g" \
  -e "s/$old_owner\.github\.io/${owner,,}.github.io/g" -e "s/$old_crate/$crate/g" -e "s/$old_name/$name/g" -e "s/$old_owner/$owner/g"
for f in packaging/$old_id.*; do mv "$f" "${f/$old_id/$new_id}"; done
mv "flatpak/$old_id.yml" "flatpak/$new_id.yml"
# The template's public signing key must not be reused: step 3 below generates a fresh one for the new app.
rm -f flatpak/signing-key.b64
for po in crates/app/translations/*/LC_MESSAGES/$old_crate.po; do mv "$po" "${po/$old_crate.po/$crate.po}"; done
# A new app starts at 0.1.0, not at the template's version.
sed -i '0,/^version = /s/^version = .*/version = "0.1.0"/' Cargo.toml
sed -i -E "s/<release version=\"[^\"]*\" date=\"[^\"]*\"/<release version=\"0.1.0\" date=\"$(date +%F)\"/" "packaging/$new_id.metainfo.xml"
sed -i '/^> Template note:/d' README.md AGENTS.md
# FastHello's own pitch must not ship under a new name: the "Start your own app" blocks go, and the store text (summary,
# description, screenshots, release notes) becomes an EDIT placeholder.
sed -i '/<!-- template:start -->/,/<!-- template:end -->/d' README.md docs/site/index.html
meta="packaging/$new_id.metainfo.xml"
sed -i -e '/^  <screenshots>$/,/^  <\/screenshots>$/d' \
  -e "s#<summary>.*</summary>#<summary>EDIT: one line about $name</summary>#" \
  -e "/^  <description>\$/,/^  <\/description>\$/c\\  <description>\\n    <p>EDIT: describe $name for the app stores.</p>\\n  </description>" \
  -e "/^      <description>\$/,/^      <\/description>\$/c\\      <description>\\n        <p>First release.</p>\\n      </description>" "$meta"
# ci.yml's last job tests this script, which is deleted below: left in, every new app's first CI run fails with
# "./prepare.sh: No such file or directory". It is the final job of the file, so cut from its header to the end.
sed -i -e '/^  prepare:$/,$d' .github/workflows/ci.yml
sed -i -e :a -e '/^\n*$/{$d;N;ba' -e '}' .github/workflows/ci.yml
# Gone before the build: the rename cannot run twice, so a failed build must not leave a script inviting a re-run.
rm -- "$0"
python3 dev/gen_po.py
cargo build
cat <<EOF
Done. The template is renamed, the version is 0.1.0 and the first build passed. Still yours to do:
  1. rewrite README.md, AGENTS.md ("What this is", "Open items"), docs/site/index.html and the metainfo description
     (search for "EDIT"), then run scripts/docs-screenshots.sh for the screenshots (list them in the metainfo too:
     Flathub needs one)
  1b. draw the app icon in packaging/ following docs/ICON-STYLE.md (new tile color and symbol, then regenerate the PNG)
  2. create the GitHub repository named exactly $name (the Pages URL and the AppImage update information assume it)
  3. flatpak/new-signing-key.sh, then do the one-time setup in RELEASING.md before the first tag
EOF
