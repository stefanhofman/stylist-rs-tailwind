use stylist::{css, Style};

/// Tailwind `list-inside`; css `list-style-position: inside;`
pub fn list_inside() -> Style {
    Style::new(css!("list-style-position: inside;")).unwrap()
}

/// Tailwind `list-outside`; css `list-style-position: outside;`
pub fn list_outside() -> Style {
    Style::new(css!("list-style-position: outside;")).unwrap()
}

