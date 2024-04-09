use stylist::{css, Style};

/// Tailwind `decoration-auto`; css `text-decoration-thickness: auto;`
pub fn decoration_auto() -> Style {
    Style::new(css!("text-decoration-thickness: auto;")).unwrap()
}

/// Tailwind `decoration-from-font`; css `text-decoration-thickness: from-font;`
pub fn decoration_from_font() -> Style {
    Style::new(css!("text-decoration-thickness: from-font;")).unwrap()
}

/// Tailwind `decoration-0`; css `text-decoration-thickness: 0px;`
pub fn decoration_0() -> Style {
    Style::new(css!("text-decoration-thickness: 0px;")).unwrap()
}

/// Tailwind `decoration-1`; css `text-decoration-thickness: 1px;`
pub fn decoration_1() -> Style {
    Style::new(css!("text-decoration-thickness: 1px;")).unwrap()
}

/// Tailwind `decoration-2`; css `text-decoration-thickness: 2px;`
pub fn decoration_2() -> Style {
    Style::new(css!("text-decoration-thickness: 2px;")).unwrap()
}

/// Tailwind `decoration-4`; css `text-decoration-thickness: 4px;`
pub fn decoration_4() -> Style {
    Style::new(css!("text-decoration-thickness: 4px;")).unwrap()
}

/// Tailwind `decoration-8`; css `text-decoration-thickness: 8px;`
pub fn decoration_8() -> Style {
    Style::new(css!("text-decoration-thickness: 8px;")).unwrap()
}

