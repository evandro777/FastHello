# MyApp

[![CI](https://github.com/OWNER/MyApp/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/ci.yml)
[![Flatpak](https://github.com/OWNER/MyApp/actions/workflows/flatpak.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/flatpak.yml)
[![AppImage](https://github.com/OWNER/MyApp/actions/workflows/appimage.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/appimage.yml)
[![VirusTotal](https://github.com/OWNER/MyApp/actions/workflows/virustotal.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/virustotal.yml)
[![made-for-linux](https://img.shields.io/badge/Linux-1f425f?logo=linux)](https://www.linux.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-orange)](https://opensource.org/licenses/GPL-3.0)

Light and fast Linux app built with Rust + [Slint](https://slint.dev) (software renderer, no GL).

> Template note: this repository started from a template. Run `./rename.sh <crate> <DisplayName> <github-owner>` once,
> then rewrite this README, `AGENTS.md` ("What this is", "Open items"), `docs/site/index.html` and the metainfo
> description for the real app, and delete this note.

Landing page: <https://OWNER.github.io/MyApp/> (published by the Flatpak workflow on the first tag).

## Features

- A working "Hello" example: type your name and the greeting updates live. The **About** button opens a dialog with
  the icon, version, description and a link to the website. Replace it with your app, keep the About dialog.
- (Edit: what the app does.)

## Status

(Edit: for example "Early pilot. Works on X11, tested on Cinnamon.")

## Install

### Flatpak: easy install with automatic updates

Every release is also published as a Flatpak remote, so `flatpak update` (or your software centre) keeps MyApp current.
A `.flatpak` file downloaded from Releases is a one-off install: to get updates, add the remote below.

```bash
flatpak remote-add --system --if-not-exists myapp https://OWNER.github.io/MyApp/myapp.flatpakrepo
flatpak install --system myapp io.github.OWNER.MyApp
```

**`--system` vs `--user` matters for updates.** Linux Mint's Update Manager (and probably other software centres) only looks
at *system-wide* Flatpaks; a `--user` install never appears there. With `--user`, update by hand:
`flatpak update --user` (a plain `flatpak update` only checks the system installation).

The remote is GPG-signed (system-wide installs refuse unsigned remotes), but the key lives in this same repository, so it is
only as trustworthy as the repository it is built from. It needs the Flathub remote for the runtime
(`flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo`, with `--system` or `--user`
to match).

### AppImage (with delta updates)

Download `MyApp-<version>-x86_64.AppImage` from [Releases](https://github.com/OWNER/MyApp/releases), then `chmod +x` it and
run it. It uses the fontconfig, xkbcommon and X11 libraries already on your system.

It has update information built in, so a tool such as [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate)
(`appimageupdatetool MyApp-*.AppImage`), Gear Lever or AppManager downloads only the changed parts of the next release.
The AppImage does not update itself.

## Build & Configurations

[Build and configurations information](BUILD.md)

## Contributing and releases

See [`CONTRIBUTING.md`](CONTRIBUTING.md) to build, test and send a change, and [`RELEASING.md`](RELEASING.md) for how a
release is cut. CI (format, clippy, tests, MSRV, packaging checks, and the AppImage and Flatpak builds) runs on every pull
request.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE).
