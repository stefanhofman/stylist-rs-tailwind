use stylist::{css, Style};

/// Tailwind `flex-row`; css `flex-direction: row;`
pub fn flex_row() -> Style {
    Style::new(css!("flex-direction: row;")).unwrap()
}

/// Tailwind `flex-row-reverse`; css `flex-direction: row-reverse;`
pub fn flex_row_reverse() -> Style {
    Style::new(css!("flex-direction: row-reverse;")).unwrap()
}

/// Tailwind `flex-col`; css `flex-direction: column;`
pub fn flex_col() -> Style {
    Style::new(css!("flex-direction: column;")).unwrap()
}

/// Tailwind `flex-col-reverse`; css `flex-direction: column-reverse;`
pub fn flex_col_reverse() -> Style {
    Style::new(css!("flex-direction: column-reverse;")).unwrap()
}

