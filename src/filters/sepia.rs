use stylist::{css, Style};

/// Tailwind `sepia-0`; css `filter: sepia(0);`
pub fn sepia_0() -> Style {
    Style::new(css!("filter: sepia(0);")).unwrap()
}

/// Tailwind `sepia`; css `filter: sepia(100%);`
pub fn sepia() -> Style {
    Style::new(css!("filter: sepia(100%);")).unwrap()
}

