use stylist::{css, Style};

/// Tailwind `decoration-solid`; css `text-decoration-style: solid;`
pub fn decoration_solid() -> Style {
    Style::new(css!("text-decoration-style: solid;")).unwrap()
}

/// Tailwind `decoration-double`; css `text-decoration-style: double;`
pub fn decoration_double() -> Style {
    Style::new(css!("text-decoration-style: double;")).unwrap()
}

/// Tailwind `decoration-dotted`; css `text-decoration-style: dotted;`
pub fn decoration_dotted() -> Style {
    Style::new(css!("text-decoration-style: dotted;")).unwrap()
}

/// Tailwind `decoration-dashed`; css `text-decoration-style: dashed;`
pub fn decoration_dashed() -> Style {
    Style::new(css!("text-decoration-style: dashed;")).unwrap()
}

/// Tailwind `decoration-wavy`; css `text-decoration-style: wavy;`
pub fn decoration_wavy() -> Style {
    Style::new(css!("text-decoration-style: wavy;")).unwrap()
}

