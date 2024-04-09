use stylist::{css, Style};

/// Tailwind `invert-0`; css `filter: invert(0);`
pub fn invert_0() -> Style {
    Style::new(css!("filter: invert(0);")).unwrap()
}

/// Tailwind `invert`; css `filter: invert(100%);`
pub fn invert() -> Style {
    Style::new(css!("filter: invert(100%);")).unwrap()
}

