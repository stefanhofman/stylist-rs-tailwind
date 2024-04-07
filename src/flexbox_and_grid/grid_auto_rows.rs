use stylist::{css, Style};

/// Tailwind `auto-rows-auto`; css `grid-auto-rows: auto;`
pub fn auto_rows_auto() -> Style {
    Style::new(css!("grid-auto-rows: auto;")).unwrap()
}

/// Tailwind `auto-rows-min`; css `grid-auto-rows: min-content;`
pub fn auto_rows_min() -> Style {
    Style::new(css!("grid-auto-rows: min-content;")).unwrap()
}

/// Tailwind `auto-rows-max`; css `grid-auto-rows: max-content;`
pub fn auto_rows_max() -> Style {
    Style::new(css!("grid-auto-rows: max-content;")).unwrap()
}

/// Tailwind `auto-rows-fr`; css `grid-auto-rows: minmax(0, 1fr);`
pub fn auto_rows_fr() -> Style {
    Style::new(css!("grid-auto-rows: minmax(0, 1fr);")).unwrap()
}

