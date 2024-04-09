use stylist::{css, Style};

/// Tailwind `forced-color-adjust-auto`; css `forced-color-adjust: auto;`
pub fn forced_color_adjust_auto() -> Style {
    Style::new(css!("forced-color-adjust: auto;")).unwrap()
}

/// Tailwind `forced-color-adjust-none`; css `forced-color-adjust: none;`
pub fn forced_color_adjust_none() -> Style {
    Style::new(css!("forced-color-adjust: none;")).unwrap()
}

