use stylist::{css, Style};

/// Tailwind `bg-auto`; css `background-size: auto;`
pub fn bg_auto() -> Style {
    Style::new(css!("background-size: auto;")).unwrap()
}

/// Tailwind `bg-cover`; css `background-size: cover;`
pub fn bg_cover() -> Style {
    Style::new(css!("background-size: cover;")).unwrap()
}

/// Tailwind `bg-contain`; css `background-size: contain;`
pub fn bg_contain() -> Style {
    Style::new(css!("background-size: contain;")).unwrap()
}

