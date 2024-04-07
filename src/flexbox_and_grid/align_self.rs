use stylist::{css, Style};

/// Tailwind `self-auto`; css `align-self: auto;`
pub fn self_auto() -> Style {
    Style::new(css!("align-self: auto;")).unwrap()
}

/// Tailwind `self-start`; css `align-self: flex-start;`
pub fn self_start() -> Style {
    Style::new(css!("align-self: flex-start;")).unwrap()
}

/// Tailwind `self-end`; css `align-self: flex-end;`
pub fn self_end() -> Style {
    Style::new(css!("align-self: flex-end;")).unwrap()
}

/// Tailwind `self-center`; css `align-self: center;`
pub fn self_center() -> Style {
    Style::new(css!("align-self: center;")).unwrap()
}

/// Tailwind `self-stretch`; css `align-self: stretch;`
pub fn self_stretch() -> Style {
    Style::new(css!("align-self: stretch;")).unwrap()
}

/// Tailwind `self-baseline`; css `align-self: baseline;`
pub fn self_baseline() -> Style {
    Style::new(css!("align-self: baseline;")).unwrap()
}

