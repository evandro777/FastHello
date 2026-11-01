use myapp_core::settings::Settings;

slint::include_modules!();

mod fixtures;

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
    // Only fails when no translation matches; the source strings are English anyway.
    let _ = slint::select_bundled_translation(&language(&settings));
    // With fixtures (scripts/ui-snapshot.sh) nothing is scanned, so the real system is never read or changed.
    if let Some(mode) = fixtures::mode() {
        fixtures::apply(&ui, mode);
    }
    ui.set_version(env!("CARGO_PKG_VERSION").into());
    ui.on_open_website(|| {
        if let Err(e) = std::process::Command::new("xdg-open")
            .arg(env!("CARGO_PKG_REPOSITORY"))
            .spawn()
        {
            eprintln!("cannot open the website: {e}");
        }
    });
    // Opens a page (or the About overlay) directly, for the same script.
    match std::env::var("MYAPP_PAGE").as_deref() {
        Ok("about") => ui.set_about_open(true),
        Ok(p) => ui.set_page(p.parse().unwrap_or(0)),
        Err(_) => {}
    }
    ui.window()
        .set_size(slint::LogicalSize::new(settings.width as f32, settings.height as f32));
    ui.run().unwrap();
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
        .map(|v| v.split(['.', ':']).next().unwrap_or("").to_string())
        .unwrap_or_default()
}
