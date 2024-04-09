use stylist::{css, Style};

/// Tailwind `table-auto`; css `table-layout: auto;`
pub fn table_auto() -> Style {
    Style::new(css!("table-layout: auto;")).unwrap()
}

/// Tailwind `table-fixed`; css `table-layout: fixed;`
pub fn table_fixed() -> Style {
    Style::new(css!("table-layout: fixed;")).unwrap()
}

