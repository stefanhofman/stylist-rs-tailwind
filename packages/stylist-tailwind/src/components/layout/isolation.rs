use stylist::{css, Style};

/// Tailwind `isolate`; css `isolation: isolate;`
pub fn isolate() -> Style {
    Style::new(css!("isolation: isolate;")).unwrap()
}

/// Tailwind `isolation-auto`; css `isolation: auto;`
pub fn isolation_auto() -> Style {
    Style::new(css!("isolation: auto;")).unwrap()
}

