use stylist::{css, Style};

/// Tailwind `break-inside-auto`; css `break-inside: auto;`
pub fn break_inside_auto() -> Style {
    Style::new(css!("break-inside: auto;")).unwrap()
}

/// Tailwind `break-inside-avoid`; css `break-inside: avoid;`
pub fn break_inside_avoid() -> Style {
    Style::new(css!("break-inside: avoid;")).unwrap()
}

/// Tailwind `break-inside-avoid-page`; css `break-inside: avoid-page;`
pub fn break_inside_avoid_page() -> Style {
    Style::new(css!("break-inside: avoid-page;")).unwrap()
}

/// Tailwind `break-inside-avoid-column`; css `break-inside: avoid-column;`
pub fn break_inside_avoid_column() -> Style {
    Style::new(css!("break-inside: avoid-column;")).unwrap()
}

