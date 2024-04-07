use stylist::{css, Style};

/// Tailwind `italic`; css `font-style: italic;`
pub fn italic() -> Style {
    Style::new(css!("font-style: italic;")).unwrap()
}

/// Tailwind `not-italic`; css `font-style: normal;`
pub fn not_italic() -> Style {
    Style::new(css!("font-style: normal;")).unwrap()
}

