use stylist::{css, Style};

/// Tailwind `grayscale-0`; css `filter: grayscale(0);`
pub fn grayscale_0() -> Style {
    Style::new(css!("filter: grayscale(0);")).unwrap()
}

/// Tailwind `grayscale`; css `filter: grayscale(100%);`
pub fn grayscale() -> Style {
    Style::new(css!("filter: grayscale(100%);")).unwrap()
}

