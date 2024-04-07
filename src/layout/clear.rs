use stylist::{css, Style};

/// Tailwind `clear-start`; css `clear: inline-start;`
pub fn clear_start() -> Style {
    Style::new(css!("clear: inline-start;")).unwrap()
}

/// Tailwind `clear-end`; css `clear: inline-end;`
pub fn clear_end() -> Style {
    Style::new(css!("clear: inline-end;")).unwrap()
}

/// Tailwind `clear-left`; css `clear: left;`
pub fn clear_left() -> Style {
    Style::new(css!("clear: left;")).unwrap()
}

/// Tailwind `clear-right`; css `clear: right;`
pub fn clear_right() -> Style {
    Style::new(css!("clear: right;")).unwrap()
}

/// Tailwind `clear-both`; css `clear: both;`
pub fn clear_both() -> Style {
    Style::new(css!("clear: both;")).unwrap()
}

/// Tailwind `clear-none`; css `clear: none;`
pub fn clear_none() -> Style {
    Style::new(css!("clear: none;")).unwrap()
}

