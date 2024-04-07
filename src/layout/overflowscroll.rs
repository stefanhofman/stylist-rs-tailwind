use stylist::{css, Style};

/// Tailwind `overscroll-auto`; css `overscroll-behavior: auto;`
pub fn overscroll_auto() -> Style {
    Style::new(css!("overscroll-behavior: auto;")).unwrap()
}

/// Tailwind `overscroll-contain`; css `overscroll-behavior: contain;`
pub fn overscroll_contain() -> Style {
    Style::new(css!("overscroll-behavior: contain;")).unwrap()
}

/// Tailwind `overscroll-none`; css `overscroll-behavior: none;`
pub fn overscroll_none() -> Style {
    Style::new(css!("overscroll-behavior: none;")).unwrap()
}

/// Tailwind `overscroll-y-auto`; css `overscroll-behavior-y: auto;`
pub fn overscroll_y_auto() -> Style {
    Style::new(css!("overscroll-behavior-y: auto;")).unwrap()
}

/// Tailwind `overscroll-y-contain`; css `overscroll-behavior-y: contain;`
pub fn overscroll_y_contain() -> Style {
    Style::new(css!("overscroll-behavior-y: contain;")).unwrap()
}

/// Tailwind `overscroll-y-none`; css `overscroll-behavior-y: none;`
pub fn overscroll_y_none() -> Style {
    Style::new(css!("overscroll-behavior-y: none;")).unwrap()
}

/// Tailwind `overscroll-x-auto`; css `overscroll-behavior-x: auto;`
pub fn overscroll_x_auto() -> Style {
    Style::new(css!("overscroll-behavior-x: auto;")).unwrap()
}

/// Tailwind `overscroll-x-contain`; css `overscroll-behavior-x: contain;`
pub fn overscroll_x_contain() -> Style {
    Style::new(css!("overscroll-behavior-x: contain;")).unwrap()
}

/// Tailwind `overscroll-x-none`; css `overscroll-behavior-x: none;`
pub fn overscroll_x_none() -> Style {
    Style::new(css!("overscroll-behavior-x: none;")).unwrap()
}

