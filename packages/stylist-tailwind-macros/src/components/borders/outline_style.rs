use stylist::{css, Style};

/// Tailwind `outline-none`; css `outline: 2px solid transparent; outline-offset: 2px;`
pub fn outline_none() -> Style {
    Style::new(css!("outline: 2px solid transparent; outline-offset: 2px;")).unwrap()
}

/// Tailwind `outline`; css `outline-style: solid;`
pub fn outline() -> Style {
    Style::new(css!("outline-style: solid;")).unwrap()
}

/// Tailwind `outline-dashed`; css `outline-style: dashed;`
pub fn outline_dashed() -> Style {
    Style::new(css!("outline-style: dashed;")).unwrap()
}

/// Tailwind `outline-dotted`; css `outline-style: dotted;`
pub fn outline_dotted() -> Style {
    Style::new(css!("outline-style: dotted;")).unwrap()
}

/// Tailwind `outline-double`; css `outline-style: double;`
pub fn outline_double() -> Style {
    Style::new(css!("outline-style: double;")).unwrap()
}

