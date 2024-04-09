use stylist::{css, Style};

/// Tailwind `backdrop-invert-0`; css `backdrop-filter: invert(0);`
pub fn backdrop_invert_0() -> Style {
    Style::new(css!("backdrop-filter: invert(0);")).unwrap()
}

/// Tailwind `backdrop-invert`; css `backdrop-filter: invert(100%);`
pub fn backdrop_invert() -> Style {
    Style::new(css!("backdrop-filter: invert(100%);")).unwrap()
}

