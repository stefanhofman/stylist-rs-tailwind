use stylist::{css, Style};

/// Tailwind `border-solid`; css `border-style: solid;`
pub fn border_solid() -> Style {
    Style::new(css!("border-style: solid;")).unwrap()
}

/// Tailwind `border-dashed`; css `border-style: dashed;`
pub fn border_dashed() -> Style {
    Style::new(css!("border-style: dashed;")).unwrap()
}

/// Tailwind `border-dotted`; css `border-style: dotted;`
pub fn border_dotted() -> Style {
    Style::new(css!("border-style: dotted;")).unwrap()
}

/// Tailwind `border-double`; css `border-style: double;`
pub fn border_double() -> Style {
    Style::new(css!("border-style: double;")).unwrap()
}

/// Tailwind `border-hidden`; css `border-style: hidden;`
pub fn border_hidden() -> Style {
    Style::new(css!("border-style: hidden;")).unwrap()
}

/// Tailwind `border-none`; css `border-style: none;`
pub fn border_none() -> Style {
    Style::new(css!("border-style: none;")).unwrap()
}

