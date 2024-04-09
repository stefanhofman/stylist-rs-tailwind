use stylist::{css, Style};

/// Tailwind `appearance-none`; css `appearance: none;`
pub fn appearance_none() -> Style {
    Style::new(css!("appearance: none;")).unwrap()
}

/// Tailwind `appearance-auto`; css `appearance: auto;`
pub fn appearance_auto() -> Style {
    Style::new(css!("appearance: auto;")).unwrap()
}

