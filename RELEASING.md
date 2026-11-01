# Releasing

Releases are Git tags. CI builds the AppImage (+ `.zsync`) and the Flatpak, publishes the signed Flatpak remote and the
landing page to GitHub Pages, and attaches both to the GitHub release; VirusTotal then scans each and adds its report to
the notes.

## One-time setup

Do this once per project, **before the first tag** (the Flatpak workflow fails without the first three).

1. **Signing key.** `flatpak/new-signing-key.sh` makes the pair: the **public** half goes to `flatpak/signing-key.b64`
   (commit it), the **private** half to a file outside the repository. Details: [docs/FLATPAK-SIGNING.md](docs/FLATPAK-SIGNING.md).
2. **Secret `FLATPAK_GPG_PRIVATE_KEY`** (Settings → Secrets and variables → Actions → New repository secret): paste the
   whole private-key file, from `-----BEGIN PGP PRIVATE KEY BLOCK-----` to the `END` line; or
   `gh secret set FLATPAK_GPG_PRIVATE_KEY < <private-key file>`. Never commit that file. GitHub will not show the secret
   again, so keep a private backup (a password manager); if the key is lost, a new key breaks every user's existing remote.
3. **GitHub Pages.** Settings → Pages → Source: **GitHub Actions**. Then Settings → Environments → `github-pages` →
   deployment branches and tags → add a **tag** rule `v*`: Pages creates the environment with a default-branch-only rule, so
   the first tag deploy would be refused ("Tag v… is not allowed to deploy").
4. **VirusTotal** (optional): repository secret `VIRUSTOTAL_API_KEY` with your VirusTotal API key, or delete `virustotal.yml`.
5. The GitHub repository must be named exactly `MyApp`: the Pages URL and the AppImage update information assume it.

## Cutting a release

1. Pick the version (`0.2.0`). Update:
    - `Cargo.toml` → `[workspace.package] version`
    - `packaging/io.github.OWNER.MyApp.metainfo.xml` → add a `<release version="0.2.0" date="YYYY-MM-DD">` entry with notes
      (newest first). `flatpak.yml` fails the tag without it.
    - Run `cargo check`, so `Cargo.lock` follows the new version (CI uses `--locked` and fails on a stale lock). Commit both.
2. `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`
3. Merge to `master` through a pull request, wait for CI.
4. Tag and push (the tag must equal the version, or `appimage.yml` and `flatpak.yml` fail):

    ```bash
    git tag v0.2.0 -m "MyApp 0.2.0"
    git push origin v0.2.0
    ```

5. Smoke-test the downloaded AppImage on a clean user, and the Flatpak: `flatpak update` from the previous version should
   offer the new one.

## Notes

- Every Pages deploy replaces the whole site (the OSTree repo is part of it), so the landing page changes only with a tag.
  Never add a Pages deploy from `master`: it would wipe the Flatpak repo.
- The release is created by whichever of the AppImage and Flatpak workflows gets there first; the Flatpak one waits for Pages.
- The AppImage `.DirIcon` comes from `packaging/io.github.OWNER.MyApp.png`: regenerate it when the SVG changes (command in `AGENTS.md`).
- Flathub: [`docs/FLATHUB.md`](docs/FLATHUB.md).
