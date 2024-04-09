use stylist::{css, Style};

/// Tailwind `shrink`; css `flex-shrink: 1;`
pub fn shrink() -> Style {
    Style::new(css!("flex-shrink: 1;")).unwrap()
}

/// Tailwind `shrink-0`; css `flex-shrink: 0;`
pub fn shrink_0() -> Style {
    Style::new(css!("flex-shrink: 0;")).unwrap()
}

