use stylist::{css, Style};

/// Tailwind `grow`; css `flex-grow: 1;`
pub fn grow() -> Style {
    Style::new(css!("flex-grow: 1;")).unwrap()
}

/// Tailwind `grow-0`; css `flex-grow: 0;`
pub fn grow_0() -> Style {
    Style::new(css!("flex-grow: 0;")).unwrap()
}

