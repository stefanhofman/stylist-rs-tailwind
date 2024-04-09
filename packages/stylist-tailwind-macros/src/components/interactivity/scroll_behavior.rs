use stylist::{css, Style};

/// Tailwind `scroll-auto`; css `scroll-behavior: auto;`
pub fn scroll_auto() -> Style {
    Style::new(css!("scroll-behavior: auto;")).unwrap()
}

/// Tailwind `scroll-smooth`; css `scroll-behavior: smooth;`
pub fn scroll_smooth() -> Style {
    Style::new(css!("scroll-behavior: smooth;")).unwrap()
}

