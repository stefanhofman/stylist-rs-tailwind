use stylist::{css, Style};

/// Tailwind `auto-cols-auto`; css `grid-auto-columns: auto;`
pub fn auto_cols_auto() -> Style {
    Style::new(css!("grid-auto-columns: auto;")).unwrap()
}

/// Tailwind `auto-cols-min`; css `grid-auto-columns: min-content;`
pub fn auto_cols_min() -> Style {
    Style::new(css!("grid-auto-columns: min-content;")).unwrap()
}

/// Tailwind `auto-cols-max`; css `grid-auto-columns: max-content;`
pub fn auto_cols_max() -> Style {
    Style::new(css!("grid-auto-columns: max-content;")).unwrap()
}

/// Tailwind `auto-cols-fr`; css `grid-auto-columns: minmax(0, 1fr);`
pub fn auto_cols_fr() -> Style {
    Style::new(css!("grid-auto-columns: minmax(0, 1fr);")).unwrap()
}

