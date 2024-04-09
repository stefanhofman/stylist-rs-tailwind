use stylist::{css, Style};

/// Tailwind `tracking-tighter`; css `letter-spacing: -0.05em;`
pub fn tracking_tighter() -> Style {
    Style::new(css!("letter-spacing: -0.05em;")).unwrap()
}

/// Tailwind `tracking-tight`; css `letter-spacing: -0.025em;`
pub fn tracking_tight() -> Style {
    Style::new(css!("letter-spacing: -0.025em;")).unwrap()
}

/// Tailwind `tracking-normal`; css `letter-spacing: 0em;`
pub fn tracking_normal() -> Style {
    Style::new(css!("letter-spacing: 0em;")).unwrap()
}

/// Tailwind `tracking-wide`; css `letter-spacing: 0.025em;`
pub fn tracking_wide() -> Style {
    Style::new(css!("letter-spacing: 0.025em;")).unwrap()
}

/// Tailwind `tracking-wider`; css `letter-spacing: 0.05em;`
pub fn tracking_wider() -> Style {
    Style::new(css!("letter-spacing: 0.05em;")).unwrap()
}

/// Tailwind `tracking-widest`; css `letter-spacing: 0.1em;`
pub fn tracking_widest() -> Style {
    Style::new(css!("letter-spacing: 0.1em;")).unwrap()
}

