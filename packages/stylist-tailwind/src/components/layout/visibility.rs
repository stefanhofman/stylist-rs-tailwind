use stylist::{css, Style};

/// Tailwind `visible`; css `visibility: visible;`
pub fn visible() -> Style {
    Style::new(css!("visibility: visible;")).unwrap()
}

/// Tailwind `invisible`; css `visibility: hidden;`
pub fn invisible() -> Style {
    Style::new(css!("visibility: hidden;")).unwrap()
}

/// Tailwind `collapse`; css `visibility: collapse;`
pub fn collapse() -> Style {
    Style::new(css!("visibility: collapse;")).unwrap()
}

