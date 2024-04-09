use stylist::{css, Style};

/// Tailwind `justify-self-auto`; css `justify-self: auto;`
pub fn justify_self_auto() -> Style {
    Style::new(css!("justify-self: auto;")).unwrap()
}

/// Tailwind `justify-self-start`; css `justify-self: start;`
pub fn justify_self_start() -> Style {
    Style::new(css!("justify-self: start;")).unwrap()
}

/// Tailwind `justify-self-end`; css `justify-self: end;`
pub fn justify_self_end() -> Style {
    Style::new(css!("justify-self: end;")).unwrap()
}

/// Tailwind `justify-self-center`; css `justify-self: center;`
pub fn justify_self_center() -> Style {
    Style::new(css!("justify-self: center;")).unwrap()
}

/// Tailwind `justify-self-stretch`; css `justify-self: stretch;`
pub fn justify_self_stretch() -> Style {
    Style::new(css!("justify-self: stretch;")).unwrap()
}

