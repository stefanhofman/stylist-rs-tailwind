use stylist::{css, Style};

/// Tailwind `backdrop-grayscale-0`; css `backdrop-filter: grayscale(0);`
pub fn backdrop_grayscale_0() -> Style {
    Style::new(css!("backdrop-filter: grayscale(0);")).unwrap()
}

/// Tailwind `backdrop-grayscale`; css `backdrop-filter: grayscale(100%);`
pub fn backdrop_grayscale() -> Style {
    Style::new(css!("backdrop-filter: grayscale(100%);")).unwrap()
}

