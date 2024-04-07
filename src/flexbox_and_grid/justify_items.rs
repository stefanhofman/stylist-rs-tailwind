use stylist::{css, Style};

/// Tailwind `justify-items-start`; css `justify-items: start;`
pub fn justify_items_start() -> Style {
    Style::new(css!("justify-items: start;")).unwrap()
}

/// Tailwind `justify-items-end`; css `justify-items: end;`
pub fn justify_items_end() -> Style {
    Style::new(css!("justify-items: end;")).unwrap()
}

/// Tailwind `justify-items-center`; css `justify-items: center;`
pub fn justify_items_center() -> Style {
    Style::new(css!("justify-items: center;")).unwrap()
}

/// Tailwind `justify-items-stretch`; css `justify-items: stretch;`
pub fn justify_items_stretch() -> Style {
    Style::new(css!("justify-items: stretch;")).unwrap()
}

