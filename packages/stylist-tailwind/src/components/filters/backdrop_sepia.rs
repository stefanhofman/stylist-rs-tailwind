use stylist::{css, Style};

/// Tailwind `backdrop-sepia-0`; css `backdrop-filter: sepia(0);`
pub fn backdrop_sepia_0() -> Style {
    Style::new(css!("backdrop-filter: sepia(0);")).unwrap()
}

/// Tailwind `backdrop-sepia`; css `backdrop-filter: sepia(100%);`
pub fn backdrop_sepia() -> Style {
    Style::new(css!("backdrop-filter: sepia(100%);")).unwrap()
}

