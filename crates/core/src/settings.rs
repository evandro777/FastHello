//! User settings in one TOML file under the XDG data dir (`~/.local/share/MyApp`).
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

/// Folder name under the XDG data dir; `rename.sh` rewrites it.
pub const APP_DIR: &str = "MyApp";
pub const MIN_SIZE: (u32, u32) = (400, 300);
/// Upper bound for a hand-edited size; a huge value would fail at window creation.
pub const MAX_SIZE: u32 = 8192;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)] // older files keep loading when a field is added
pub struct Settings {
    /// "" follows the system locale; otherwise a gettext code such as "pt_BR".
    pub language: String,
    pub width: u32,
    pub height: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: String::new(),
            width: 600,
            height: 450,
        }
    }
}

fn parse(text: &str) -> Result<Settings, toml::de::Error> {
    toml::from_str(text)
}

fn data_dir() -> PathBuf {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join(APP_DIR)
}

fn path() -> PathBuf {
    data_dir().join("settings.toml")
}

impl Settings {
    pub fn load() -> Self {
        let mut s = match fs::read_to_string(path()) {
            Ok(text) => parse(&text).unwrap_or_else(|e| {
                // The file is hand-edited; the next save() must not overwrite what the user wrote.
                let bad = path().with_extension("toml.bad");
                eprintln!(
                    "settings.toml is invalid ({e}); kept as {}, using defaults",
                    bad.display()
                );
                let _ = fs::rename(path(), bad);
                Settings::default()
            }),
            Err(_) => Settings::default(),
        };
        s.normalize();
        s
    }

    fn normalize(&mut self) {
        self.width = self.width.clamp(MIN_SIZE.0, MAX_SIZE);
        self.height = self.height.clamp(MIN_SIZE.1, MAX_SIZE);
    }

    /// Atomic write so a crash can never leave a half-written file. Failures are reported on stderr;
    /// the app keeps working with the in-memory settings.
    pub fn save(&self) {
        if let Err(e) = self.try_save() {
            eprintln!("cannot save settings to {}: {e}", path().display());
        }
    }

    fn try_save(&self) -> std::io::Result<()> {
        let dir = data_dir();
        fs::create_dir_all(&dir)?;
        let text = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
        let tmp = dir.join("settings.toml.tmp");
        fs::write(&tmp, text)?;
        fs::rename(&tmp, path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_files_still_load() {
        let parsed: Settings = toml::from_str("width = 700").unwrap();
        assert_eq!((parsed.width, parsed.language.as_str()), (700, ""));
    }

    #[test]
    fn invalid_file_is_an_error_not_defaults() {
        assert!(parse("width = \"wide\"").is_err());
    }

    #[test]
    fn window_size_is_clamped_both_ways() {
        let mut s: Settings = toml::from_str("width = 4000000000\nheight = 1").unwrap();
        s.normalize();
        assert_eq!((s.width, s.height), (MAX_SIZE, MIN_SIZE.1));
    }
}
