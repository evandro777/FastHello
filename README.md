# MyApp

[![CI](https://github.com/OWNER/MyApp/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/ci.yml)
[![Flatpak](https://github.com/OWNER/MyApp/actions/workflows/flatpak.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/flatpak.yml)
[![AppImage](https://github.com/OWNER/MyApp/actions/workflows/appimage.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/appimage.yml)
[![VirusTotal](https://github.com/OWNER/MyApp/actions/workflows/virustotal.yml/badge.svg)](https://github.com/OWNER/MyApp/actions/workflows/virustotal.yml)
[![made-for-linux](https://img.shields.io/badge/Linux-1f425f?logo=linux)](https://www.linux.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-orange)](https://opensource.org/licenses/GPL-3.0)

Light and fast Linux app built with Rust + [Slint](https://slint.dev) (software renderer, no GL).

> Template note: this repository started from a template. Run `./rename.sh <crate> <DisplayName> <github-owner>` once, then rewrite this README, `AGENTS.md` ("What this is", "Open items"), `docs/site/index.html` and the metainfo description for the real app, and delete this note.

Page: <https://OWNER.github.io/MyApp/>

## Features

- The "Hello" example app:
    - Type your name and the greeting updates as you type.
    - A sidebar switches between Hello and Widgets. Its burger button opens the settings dialog (Appearance and Language). Its About button opens a dialog with the icon, version, description and website link.
    - The Widgets page has four tabs:
        - Inputs: checkbox, slider, spin box, switch, radio buttons, password field and text area, some of them linked to each other.
        - List: an add/remove list.
        - Table: a sortable table with a search box and a Filters button (file type, larger than 1 MB). Typing anywhere on this tab goes into the search box, no need to click it first. Esc empties the search box.
        - Feedback: a progress bar and spinner for a running task.
    - The settings dialog has an Appearance choice (System, Light or Dark) and a Language choice (System, English or Português), both remembered.
    - You are meant to replace the examples with your own app and keep the settings and About dialogs.
- (Edit: what the app does.): a placeholder for you to describe the real app after you rename the template.

## Install

### Flatpak

#### Install from file (no updates)

Download `MyApp-<version>-x86_64.flatpak` from [Releases](https://github.com/OWNER/MyApp/releases), then execute:

```bash
flatpak install --user ~/Downloads/MyApp-<version>-x86_64.flatpak
```

Installing from file, will not auto update.

#### Install with automatic updates

Every release is also published as a Flatpak remote, so `flatpak update` (or your software centre) keeps MyApp current. A `.flatpak` file downloaded from Releases is a one-off install: to get updates, add the remote below.

```bash
flatpak remote-add --system --if-not-exists myapp https://OWNER.github.io/MyApp/myapp.flatpakrepo
flatpak install --system myapp io.github.OWNER.MyApp
```

**`--system` vs `--user` matters for updates.** Linux Mint's Update Manager (and probably other software centres) only looks at *system-wide* Flatpaks; a `--user` install never appears there. With `--user`, update by hand: `flatpak update --user` (a plain `flatpak update` only checks the system installation).

The remote is GPG-signed (system-wide installs refuse unsigned remotes), but the key lives in this same repository, so it is only as trustworthy as the repository it is built from. It needs the Flathub remote for the runtime (`flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo`, with `--system` or `--user` to match).

### AppImage (with delta updates)

Download `MyApp-<version>-x86_64.AppImage` from [Releases](https://github.com/OWNER/MyApp/releases), then execute:

```bash
sudo chmod +x ~/Downloads/MyApp-<version>-x86_64.AppImage
~/Downloads/MyApp-<version>-x86_64.AppImage
```

It uses the fontconfig, xkbcommon and X11 libraries already on your system.

The AppImage does not update itself, but it has update information built in, so a tool such as [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`appimageupdatetool FastPaste-*.AppImage`), [Gear Lever](https://github.com/mijorus/gearlever) or [AppManager](https://github.com/kem-a/AppManager) downloads only the changed parts of the next release.

## Build & Configurations

[Build and configurations information](BUILD.md)

## Contributing and releases

See [`CONTRIBUTING.md`](CONTRIBUTING.md) to build, test and send a change, and [`RELEASING.md`](RELEASING.md) for how a release is cut. CI (format, clippy, tests, MSRV, packaging checks, and the AppImage and Flatpak builds) runs on every pull request.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE).
