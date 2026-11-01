# Signing the Flatpak remote

How the signed Flatpak repository is built on GitHub, and what you do once to set it up.

## Why it is signed

`flatpak.yml` publishes an OSTree repository (the "remote") on GitHub Pages. Flatpak refuses to install from an unsigned
remote when the install is system-wide (`--system`), and Linux Mint's Update Manager only sees system-wide installs. So the
repository is signed with a GPG key.

A key is a **pair** made together:

| Half    | File                         | Where it lives                      | What it does                                   |
|---------|------------------------------|-------------------------------------|------------------------------------------------|
| Public  | `flatpak/signing-key.b64`    | Committed to the repository         | Lets Flatpak on the user's machine *check* signatures. Not secret. |
| Private | a file outside the repo      | **Never** committed; GitHub secret  | Creates signatures. Whoever has it can sign updates as you. |

On a version tag, CI imports the private key from the secret `FLATPAK_GPG_PRIVATE_KEY`, signs the repository, and puts the
public key into `myapp.flatpakrepo` so users' Flatpak knows which key to trust.

## One-time setup

### 1. Make the key pair

```bash
flatpak/new-signing-key.sh
```

It makes the pair in a temporary keyring (your own GPG keys are not touched), writes `flatpak/signing-key.b64`, writes the
private key to `~/.local/share/myapp-flatpak-signing.asc` (or the path you pass as an argument), checks that the two
match, deletes the temporary keyring and prints the next steps. It refuses to run if `signing-key.b64` already exists.
Commit `flatpak/signing-key.b64`. Never commit the private file.

### 2. Store the private key as a GitHub secret

A *secret* is a value GitHub keeps for the repository: workflows can read it, nobody can view it afterwards. The name must
be exactly `FLATPAK_GPG_PRIVATE_KEY`.

**With the website:** repository **Settings** → **Secrets and variables** → **Actions** → **New repository secret**. Name:
`FLATPAK_GPG_PRIVATE_KEY`. Secret: open the private key file and paste everything, from `-----BEGIN PGP PRIVATE KEY BLOCK-----`
to the `END` line.

**With `gh`** (GitHub's command-line tool; `sudo apt install gh`, then `gh auth login` once):

```bash
gh secret set FLATPAK_GPG_PRIVATE_KEY < ~/.local/share/myapp-flatpak-signing.asc
```

`gh secret set NAME` creates or replaces the secret; `<` feeds the file's contents in as the value.

### 3. Back it up, then delete the loose file

GitHub never shows the secret again. Keep a copy in a password manager, then `shred -u` the file.

### 4. The other one-time GitHub settings

See [RELEASING.md](../RELEASING.md): Pages source, and the `v*` tag rule on the `github-pages` environment.

## What the script does by hand

If you cannot run the script, this is what it does (use a throwaway `GNUPGHOME`):

```bash
export GNUPGHOME=$(mktemp -d)
gpg --batch --pinentry-mode loopback --passphrase '' --quick-generate-key "MyApp Flatpak <you@example.com>" ed25519 sign never
fpr=$(gpg --list-keys --with-colons | awk -F: '/^fpr/ {print $10; exit}')
gpg --export "$fpr" | base64 -w0 > flatpak/signing-key.b64              # public: commit
gpg --export-secret-keys --armor "$fpr" > ~/.local/share/myapp-flatpak-signing.asc   # private: secret
base64 -d flatpak/signing-key.b64 | gpg --show-keys --with-colons | awk -F: '/^fpr/ {print $10; exit}'   # must equal $fpr
```

No passphrase because CI cannot type one; `never` because an expired key would silently break updates for every user.

## What CI does with the key

In `.github/workflows/flatpak.yml`, on a `v*` tag only (secrets are not available to pull requests):

1. It stops with a clear message if `flatpak/signing-key.b64` or the secret is missing.
2. `gpg --batch --import` loads the private key; the fingerprint is read from the committed public key.
3. `flatpak/build.sh --bundle` passes it to `flatpak-builder --gpg-sign=<fingerprint>`, which signs the repository.
4. The `pages` job writes `GPGKey=<contents of signing-key.b64>` into `myapp.flatpakrepo`.

Pushes and pull requests build the bundle without a key (with a warning), so CI works before you have made one.

## Checking a build

```bash
flatpak remote-add --system --if-not-exists myapp https://OWNER.github.io/MyApp/myapp.flatpakrepo
flatpak install --system myapp io.github.OWNER.MyApp
```

A wrongly signed repository fails with a GPG error. `ostree summary --view --repo=flatpak/repo` shows the signature of a
repository built locally.

## Replacing the key

If the private key is lost or leaked, delete `flatpak/signing-key.b64`, run `flatpak/new-signing-key.sh` again, commit the
new public key and replace the secret. Users who already added the remote trust the *old* key, so their updates fail until
they re-add it (`flatpak remote-delete --system myapp`, then `remote-add` again). That is why the key never expires and the
private key needs a backup.
