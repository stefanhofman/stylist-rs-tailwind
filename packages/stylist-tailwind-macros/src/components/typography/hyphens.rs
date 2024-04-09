use stylist::{css, Style};

/// Tailwind `hyphens-none`; css `hyphens: none;`
pub fn hyphens_none() -> Style {
    Style::new(css!("hyphens: none;")).unwrap()
}

/// Tailwind `hyphens-manual`; css `hyphens: manual;`
pub fn hyphens_manual() -> Style {
    Style::new(css!("hyphens: manual;")).unwrap()
}

/// Tailwind `hyphens-auto`; css `hyphens: auto;`
pub fn hyphens_auto() -> Style {
    Style::new(css!("hyphens: auto;")).unwrap()
}

