use stylist::{css, Style};

/// Tailwind `sr-only`; css `position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0;`
pub fn sr_only() -> Style {
    Style::new(css!("position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0;")).unwrap()
}

/// Tailwind `not-sr-only`; css `position: static; width: auto; height: auto; padding: 0; margin: 0; overflow: visible; clip: auto; white-space: normal;`
pub fn not_sr_only() -> Style {
    Style::new(css!("position: static; width: auto; height: auto; padding: 0; margin: 0; overflow: visible; clip: auto; white-space: normal;")).unwrap()
}

