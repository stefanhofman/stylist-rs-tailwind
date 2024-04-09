use stylist::{css, Style};

/// Tailwind `resize-none`; css `resize: none;`
pub fn resize_none() -> Style {
    Style::new(css!("resize: none;")).unwrap()
}

/// Tailwind `resize-y`; css `resize: vertical;`
pub fn resize_y() -> Style {
    Style::new(css!("resize: vertical;")).unwrap()
}

/// Tailwind `resize-x`; css `resize: horizontal;`
pub fn resize_x() -> Style {
    Style::new(css!("resize: horizontal;")).unwrap()
}

/// Tailwind `resize`; css `resize: both;`
pub fn resize() -> Style {
    Style::new(css!("resize: both;")).unwrap()
}

