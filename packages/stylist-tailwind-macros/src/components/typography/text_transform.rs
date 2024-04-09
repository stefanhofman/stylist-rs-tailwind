use stylist::{css, Style};

/// Tailwind `uppercase`; css `text-transform: uppercase;`
pub fn uppercase() -> Style {
    Style::new(css!("text-transform: uppercase;")).unwrap()
}

/// Tailwind `lowercase`; css `text-transform: lowercase;`
pub fn lowercase() -> Style {
    Style::new(css!("text-transform: lowercase;")).unwrap()
}

/// Tailwind `capitalize`; css `text-transform: capitalize;`
pub fn capitalize() -> Style {
    Style::new(css!("text-transform: capitalize;")).unwrap()
}

/// Tailwind `normal-case`; css `text-transform: none;`
pub fn normal_case() -> Style {
    Style::new(css!("text-transform: none;")).unwrap()
}

