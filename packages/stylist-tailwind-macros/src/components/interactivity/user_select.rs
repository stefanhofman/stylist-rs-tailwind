use stylist::{css, Style};

/// Tailwind `select-none`; css `user-select: none;`
pub fn select_none() -> Style {
    Style::new(css!("user-select: none;")).unwrap()
}

/// Tailwind `select-text`; css `user-select: text;`
pub fn select_text() -> Style {
    Style::new(css!("user-select: text;")).unwrap()
}

/// Tailwind `select-all`; css `user-select: all;`
pub fn select_all() -> Style {
    Style::new(css!("user-select: all;")).unwrap()
}

/// Tailwind `select-auto`; css `user-select: auto;`
pub fn select_auto() -> Style {
    Style::new(css!("user-select: auto;")).unwrap()
}

