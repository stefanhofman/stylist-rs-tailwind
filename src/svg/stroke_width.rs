use stylist::{css, Style};

/// Tailwind `stroke-0`; css `stroke-width: 0;`
pub fn stroke_0() -> Style {
    Style::new(css!("stroke-width: 0;")).unwrap()
}

/// Tailwind `stroke-1`; css `stroke-width: 1;`
pub fn stroke_1() -> Style {
    Style::new(css!("stroke-width: 1;")).unwrap()
}

/// Tailwind `stroke-2`; css `stroke-width: 2;`
pub fn stroke_2() -> Style {
    Style::new(css!("stroke-width: 2;")).unwrap()
}

