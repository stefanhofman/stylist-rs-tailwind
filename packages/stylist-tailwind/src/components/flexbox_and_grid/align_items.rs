use stylist::{css, Style};

/// Tailwind `items-start`; css `align-items: flex-start;`
pub fn items_start() -> Style {
    Style::new(css!("align-items: flex-start;")).unwrap()
}

/// Tailwind `items-end`; css `align-items: flex-end;`
pub fn items_end() -> Style {
    Style::new(css!("align-items: flex-end;")).unwrap()
}

/// Tailwind `items-center`; css `align-items: center;`
pub fn items_center() -> Style {
    Style::new(css!("align-items: center;")).unwrap()
}

/// Tailwind `items-baseline`; css `align-items: baseline;`
pub fn items_baseline() -> Style {
    Style::new(css!("align-items: baseline;")).unwrap()
}

/// Tailwind `items-stretch`; css `align-items: stretch;`
pub fn items_stretch() -> Style {
    Style::new(css!("align-items: stretch;")).unwrap()
}

