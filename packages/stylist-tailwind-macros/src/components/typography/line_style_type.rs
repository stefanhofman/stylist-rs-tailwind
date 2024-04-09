use stylist::{css, Style};

/// Tailwind `list-none`; css `list-style-type: none;`
pub fn list_none() -> Style {
    Style::new(css!("list-style-type: none;")).unwrap()
}

/// Tailwind `list-disc`; css `list-style-type: disc;`
pub fn list_disc() -> Style {
    Style::new(css!("list-style-type: disc;")).unwrap()
}

/// Tailwind `list-decimal`; css `list-style-type: decimal;`
pub fn list_decimal() -> Style {
    Style::new(css!("list-style-type: decimal;")).unwrap()
}

