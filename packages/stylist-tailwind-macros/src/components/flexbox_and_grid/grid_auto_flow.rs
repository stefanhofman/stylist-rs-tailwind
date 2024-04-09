use stylist::{css, Style};

/// Tailwind `grid-flow-row`; css `grid-auto-flow: row;`
pub fn grid_flow_row() -> Style {
    Style::new(css!("grid-auto-flow: row;")).unwrap()
}

/// Tailwind `grid-flow-col`; css `grid-auto-flow: column;`
pub fn grid_flow_col() -> Style {
    Style::new(css!("grid-auto-flow: column;")).unwrap()
}

/// Tailwind `grid-flow-dense`; css `grid-auto-flow: dense;`
pub fn grid_flow_dense() -> Style {
    Style::new(css!("grid-auto-flow: dense;")).unwrap()
}

/// Tailwind `grid-flow-row-dense`; css `grid-auto-flow: row dense;`
pub fn grid_flow_row_dense() -> Style {
    Style::new(css!("grid-auto-flow: row dense;")).unwrap()
}

/// Tailwind `grid-flow-col-dense`; css `grid-auto-flow: column dense;`
pub fn grid_flow_col_dense() -> Style {
    Style::new(css!("grid-auto-flow: column dense;")).unwrap()
}

