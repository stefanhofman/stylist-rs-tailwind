use stylist::{css, Style};

/// Tailwind `underline`; css `text-decoration-line: underline;`
pub fn underline() -> Style {
    Style::new(css!("text-decoration-line: underline;")).unwrap()
}

/// Tailwind `overline`; css `text-decoration-line: overline;`
pub fn overline() -> Style {
    Style::new(css!("text-decoration-line: overline;")).unwrap()
}

/// Tailwind `line-through`; css `text-decoration-line: line-through;`
pub fn line_through() -> Style {
    Style::new(css!("text-decoration-line: line-through;")).unwrap()
}

/// Tailwind `no-underline`; css `text-decoration-line: none;`
pub fn no_underline() -> Style {
    Style::new(css!("text-decoration-line: none;")).unwrap()
}

