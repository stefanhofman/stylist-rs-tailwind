use stylist::{css, Style};

/// Tailwind `border-collapse`; css `border-collapse: collapse;`
pub fn border_collapse() -> Style {
    Style::new(css!("border-collapse: collapse;")).unwrap()
}

/// Tailwind `border-separate`; css `border-collapse: separate;`
pub fn border_separate() -> Style {
    Style::new(css!("border-collapse: separate;")).unwrap()
}

