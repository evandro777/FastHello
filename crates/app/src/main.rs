use myapp_core::settings::Settings;
use slint::{Model, ModelRc, StandardListViewItem, VecModel};
use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

#[cfg(debug_assertions)]
mod fixtures;

/// One row of the "Files" table on the Widgets page: sample data that shows sorting by a text and a numeric column.
#[derive(Clone)]
pub struct FileRow {
    pub name: String,
    /// The file extension, not a translated word.
    pub kind: &'static str,
    pub kb: u32,
}

fn sample_files() -> Vec<FileRow> {
    let row = |name: &str, kind, kb| FileRow {
        name: name.into(),
        kind,
        kb,
    };
    vec![
        row("notes.txt", "txt", 4),
        row("holiday.png", "png", 2480),
        row("report.pdf", "pdf", 310),
        row("backup.tar.gz", "gz", 98_304),
        row("song.ogg", "ogg", 5120),
        row("todo.md", "md", 2),
    ]
}

fn file_rows(files: &[FileRow]) -> Vec<ModelRc<StandardListViewItem>> {
    files
        .iter()
        .map(|f| {
            let cells: Vec<StandardListViewItem> =
                vec![f.name.as_str().into(), f.kind.into(), f.kb.to_string().as_str().into()];
            ModelRc::new(VecModel::from(cells))
        })
        .collect()
}

/// Sorts by the column index the table reports (0 name, 1 type, 2 size); the size sorts as a number.
fn sort_files(files: &mut [FileRow], column: i32, ascending: bool) {
    match column {
        0 => files.sort_by_key(|f| f.name.to_lowercase()),
        1 => files.sort_by_key(|f| (f.kind, f.name.to_lowercase())),
        _ => files.sort_by_key(|f| f.kb),
    }
    if !ascending {
        files.reverse();
    }
}

/// "Larger than 1 MB" filter, in the table's size unit (KB).
const BIG_KB: u32 = 1024;

/// The table's data and what the user asked of it. Search, filters and sort are all applied here, so changing one
/// keeps the others: sorting a filtered table, or filtering a sorted one, shows what you expect.
struct FileTable {
    files: Vec<FileRow>,
    /// Column and direction last reported by the table header; `None` keeps the original order.
    sort: Option<(i32, bool)>,
    query: String,
    /// `""` = every type.
    kind: String,
    big_only: bool,
}

impl FileTable {
    fn new(files: Vec<FileRow>) -> Self {
        Self {
            files,
            sort: None,
            query: String::new(),
            kind: String::new(),
            big_only: false,
        }
    }

    /// The rows that pass the search (any cell, ignoring case) and the filters, in the chosen order.
    fn visible(&self) -> Vec<FileRow> {
        let q = self.query.trim().to_lowercase();
        let mut rows: Vec<FileRow> = self
            .files
            .iter()
            .filter(|f| {
                (self.kind.is_empty() || f.kind == self.kind)
                    && (!self.big_only || f.kb > BIG_KB)
                    && (q.is_empty()
                        || f.name.to_lowercase().contains(&q)
                        || f.kind.contains(&q)
                        || f.kb.to_string().contains(&q))
            })
            .cloned()
            .collect();
        if let Some((column, ascending)) = self.sort {
            sort_files(&mut rows, column, ascending);
        }
        rows
    }
}

/// Hands the table's current rows to Slint. The rows are built under a short borrow and given over afterwards.
fn show(table: &RefCell<FileTable>, model: &VecModel<ModelRc<StandardListViewItem>>) {
    let rows = file_rows(&table.borrow().visible());
    model.set_vec(rows);
}

/// A key worth typing into the search box: one visible character. Not whitespace (Space must still press the focused
/// button or checkbox), not a control character (Esc, Backspace, Return) and not Slint's private-use codes for arrows,
/// function keys and the like (U+F700 and up).
fn is_printable(text: &str) -> bool {
    let mut chars = text.chars();
    matches!(
        (chars.next(), chars.next()),
        (Some(c), None) if !c.is_whitespace() && !c.is_control() && !('\u{f700}'..='\u{f8ff}').contains(&c)
    )
}

fn main() {
    if std::env::args().skip(1).any(|a| a == "-V" || a == "--version") {
        return println!("myapp {}", env!("CARGO_PKG_VERSION"));
    }
    // Forced so the 100 MB GL renderer is never pulled in by accident.
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()
        .expect("cannot start the Slint software renderer");

    let settings = Settings::load();
    let ui = Main::new().unwrap();
    // One model for the whole run: the add-item closure and the fixtures must push into the same one.
    let items = Rc::new(VecModel::<StandardListViewItem>::default());
    ui.set_items(items.clone().into());
    ui.on_add_item({
        let items = items.clone();
        move |text| {
            let text = text.trim();
            if !text.is_empty() {
                items.push(text.into());
            }
        }
    });
    ui.on_remove_item({
        let items = items.clone();
        move |i| {
            if let Ok(i) = usize::try_from(i)
                && i < items.row_count()
            {
                items.remove(i);
            }
        }
    });
    select_language(&language(&settings));
    ui.set_language_index(LANGUAGES.iter().position(|l| *l == settings.language).unwrap_or(0) as i32);
    #[allow(unused_mut)] // only the debug-build fixtures replace it
    let mut sample = sample_files();
    // With fixtures (scripts/ui-snapshot.sh) nothing is scanned, so the real system is never read or changed.
    // Debug builds only: a release build must not have a hidden fake-data switch.
    #[cfg(debug_assertions)]
    if let Some(mode) = fixtures::mode() {
        fixtures::apply(&ui, &items, &mut sample, mode);
    }
    let table = Rc::new(RefCell::new(FileTable::new(sample)));
    let files_model = Rc::new(VecModel::from(file_rows(&table.borrow().visible())));
    ui.set_files(files_model.clone().into());
    ui.set_files_total(table.borrow().files.len() as i32);
    ui.on_sort_files({
        let (table, model) = (table.clone(), files_model.clone());
        move |column, ascending| {
            table.borrow_mut().sort = Some((column, ascending));
            show(&table, &model);
        }
    });
    ui.on_filter_changed(move |query, kind, big_only| {
        {
            let mut t = table.borrow_mut();
            t.query = query.into();
            t.kind = kind.into();
            t.big_only = big_only;
        }
        show(&table, &files_model);
    });
    ui.on_is_printable(|text| is_printable(&text));
    ui.set_color_scheme_index(match settings.theme.as_str() {
        "light" => 1,
        "dark" => 2,
        _ => 0,
    });
    let settings = Rc::new(RefCell::new(settings));
    // Saved as soon as it changes: a killed session would otherwise lose the choice.
    ui.on_theme_changed({
        let settings = settings.clone();
        move |index| {
            let mut s = settings.borrow_mut();
            let theme = theme_name(index);
            // Also fires once at start-up; saving then would rewrite the file (and, with fixtures, the real one).
            if s.theme != theme {
                s.theme = theme.into();
                s.save();
            }
        }
    });
    ui.on_language_changed({
        let settings = settings.clone();
        move |index| {
            let code = usize::try_from(index)
                .ok()
                .and_then(|i| LANGUAGES.get(i))
                .copied()
                .unwrap_or("");
            let mut s = settings.borrow_mut();
            // Same guard as the theme: the start-up notification must not rewrite the file.
            if s.language != code {
                s.language = code.into();
                s.save();
                select_language(&language(&s));
            }
        }
    });
    ui.set_version(env!("CARGO_PKG_VERSION").into());
    ui.on_open_website(|| {
        // Waiting in a thread reaps the child; a dropped `Child` would stay a zombie until exit.
        std::thread::spawn(|| {
            if let Err(e) = std::process::Command::new("xdg-open")
                .arg(env!("CARGO_PKG_REPOSITORY"))
                .status()
            {
                eprintln!("cannot open the website: {e}");
            }
        });
    });
    // Opens a page (or the About overlay) directly, for the same script.
    #[cfg(debug_assertions)]
    match std::env::var("MYAPP_PAGE").as_deref() {
        Ok("settings") => ui.set_settings_open(true),
        Ok("about") => ui.set_about_open(true),
        // `<page>` or `<page>.<tab>`, e.g. `1.2` opens the third tab of the Widgets page.
        Ok(p) => {
            let (page, tab) = p.split_once('.').unwrap_or((p, "0"));
            let number = |s: &str| {
                s.parse()
                    .unwrap_or_else(|_| panic!("MYAPP_PAGE={p} is not <page>, <page>.<tab>, 'settings' or 'about'"))
            };
            ui.set_page(number(page));
            ui.set_widgets_tab(number(tab));
        }
        Err(_) => {}
    }
    {
        let s = settings.borrow();
        ui.window()
            .set_size(slint::LogicalSize::new(s.width as f32, s.height as f32));
    }
    ui.run().unwrap();
    // Normal exit only; the snapshot script kills the app, so fixtures never overwrite the real file.
    let size = ui.window().size().to_logical(ui.window().scale_factor());
    let mut settings = settings.borrow_mut();
    settings.width = size.width.round() as u32;
    settings.height = size.height.round() as u32;
    settings.theme = theme_name(ui.get_color_scheme_index()).into();
    settings.save();
}

/// `settings.language` for each entry of the Language selector: system, English, Português.
const LANGUAGES: [&str; 3] = ["", "en", "pt_BR"];

/// An unknown locale (`de_DE`) leaves the previous language selected, so fall back to the English source strings
/// explicitly: otherwise going from Português back to System on such a desktop would stay Portuguese.
fn select_language(code: &str) {
    if slint::select_bundled_translation(code).is_err() {
        let _ = slint::select_bundled_translation("");
    }
}

/// The `settings.theme` value for the Appearance index (0 system, 1 light, 2 dark).
fn theme_name(index: i32) -> &'static str {
    match index {
        1 => "light",
        2 => "dark",
        _ => "",
    }
}

/// `settings.language`, or the system locale (`LC_ALL`/`LC_MESSAGES`/`LANGUAGE`/`LANG`, minus `.UTF-8`).
fn language(settings: &Settings) -> String {
    if !settings.language.is_empty() {
        return settings.language.clone();
    }
    ["LC_ALL", "LC_MESSAGES", "LANGUAGE", "LANG"]
        .iter()
        .filter_map(|v| std::env::var(v).ok())
        .find(|v| !v.is_empty())
        .map(|v| locale_code(&v).to_string())
        .unwrap_or_default()
}

/// `pt_BR.UTF-8` / `pt_BR:en` / `pt_BR@euro` -> `pt_BR`.
fn locale_code(v: &str) -> &str {
    v.split(['.', ':', '@']).next().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(rows: &[FileRow]) -> Vec<&str> {
        rows.iter().map(|f| f.name.as_str()).collect()
    }

    #[test]
    fn search_matches_name_type_and_size_ignoring_case() {
        let mut t = FileTable::new(sample_files());
        t.query = "HOLIDAY".into();
        assert_eq!(names(&t.visible()), ["holiday.png"]);
        t.query = "pdf".into();
        assert_eq!(names(&t.visible()), ["report.pdf"]);
        t.query = "98304".into();
        assert_eq!(names(&t.visible()), ["backup.tar.gz"]);
        t.query = "  ".into();
        assert_eq!(t.visible().len(), 6, "a blank search shows everything");
        t.query = "zzz".into();
        assert!(t.visible().is_empty());
    }

    #[test]
    fn type_and_size_filters_combine_with_search() {
        let mut t = FileTable::new(sample_files());
        t.kind = "png".into();
        assert_eq!(names(&t.visible()), ["holiday.png"]);
        t.kind.clear();
        t.big_only = true;
        // Over 1 MB: holiday.png (2480), backup.tar.gz (98304), song.ogg (5120).
        assert_eq!(t.visible().len(), 3);
        t.query = "ogg".into();
        assert_eq!(names(&t.visible()), ["song.ogg"]);
    }

    #[test]
    fn sort_survives_a_filter_change() {
        let mut t = FileTable::new(sample_files());
        t.sort = Some((2, false));
        t.big_only = true;
        assert_eq!(names(&t.visible()), ["backup.tar.gz", "song.ogg", "holiday.png"]);
    }

    #[test]
    fn only_a_visible_character_is_typed_into_the_search() {
        for ok in ["a", "Ç", "7", "."] {
            assert!(is_printable(ok), "{ok:?}");
        }
        // Space, Tab, Esc, Backspace, Delete, Return, an arrow (U+F700), two characters, nothing.
        for no in [" ", "\t", "\u{1b}", "\u{8}", "\u{7f}", "\n", "\u{f700}", "ab", ""] {
            assert!(!is_printable(no), "{no:?}");
        }
    }

    #[test]
    fn locale_code_drops_encoding_fallbacks_and_modifier() {
        assert_eq!(locale_code("pt_BR:en"), "pt_BR");
        assert_eq!(locale_code("pt_BR.UTF-8"), "pt_BR");
        assert_eq!(locale_code("pt_BR@euro"), "pt_BR");
        assert_eq!(locale_code("C"), "C");
    }

    #[test]
    fn size_sorts_as_a_number_not_as_text() {
        let mut files = sample_files();
        sort_files(&mut files, 2, true);
        let sizes: Vec<u32> = files.iter().map(|f| f.kb).collect();
        // As text "310" would come before "4" and "98304" before "2480".
        assert_eq!(sizes, [2, 4, 310, 2480, 5120, 98_304]);
        sort_files(&mut files, 2, false);
        assert_eq!(files[0].kb, 98_304);
    }
}
