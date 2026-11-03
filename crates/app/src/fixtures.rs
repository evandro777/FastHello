//! Canned data for `scripts/ui-snapshot.sh` (`MYAPP_FIXTURES=empty|single|many`): every page can be rendered with zero,
//! one or many rows without scanning or touching the real system. Fill the models in `apply` as pages are added.
use crate::Main;
use slint::{StandardListViewItem, VecModel};

/// 0 empty, 1 single, 2 many; `None` when the variable is unset or unknown.
pub fn mode() -> Option<usize> {
    match std::env::var("MYAPP_FIXTURES").ok()?.as_str() {
        "empty" => Some(0),
        "single" => Some(1),
        "many" => Some(2),
        _ => None,
    }
}

/// Use `[0, 1, 12][mode]` as the row count of each list, so one fixture covers all three states.
pub fn apply(ui: &Main, items: &VecModel<StandardListViewItem>, mode: usize) {
    for i in 0..[0, 1, 12][mode] {
        // One long name to check truncation.
        let text = if i == 1 {
            "A very long item name that should be cut off instead of stretching the whole list sideways".to_string()
        } else {
            format!("Item {}", i + 1)
        };
        items.push(text.as_str().into());
    }
    // Empty exercises the "Hello, world" fallback; the long one has spaces and one unbreakable word, to check wrapping.
    let names = [
        "",
        "Ana",
        "Maria Fernanda de Albuquerque Cavalcanti Pneumonoultramicroscopicsilicovolcanoconiosis Souza",
    ];
    ui.set_name(names[mode].into());
}
