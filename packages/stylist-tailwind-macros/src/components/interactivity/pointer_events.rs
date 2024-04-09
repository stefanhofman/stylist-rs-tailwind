use stylist::{css, Style};

/// Tailwind `pointer-events-none`; css `pointer-events: none;`
pub fn pointer_events_none() -> Style {
    Style::new(css!("pointer-events: none;")).unwrap()
}

/// Tailwind `pointer-events-auto`; css `pointer-events: auto;`
pub fn pointer_events_auto() -> Style {
    Style::new(css!("pointer-events: auto;")).unwrap()
}

