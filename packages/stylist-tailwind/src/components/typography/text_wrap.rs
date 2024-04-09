use stylist::{css, Style};

/// Tailwind `text-wrap`; css `text-wrap: wrap;`
pub fn text_wrap() -> Style {
    Style::new(css!("text-wrap: wrap;")).unwrap()
}

/// Tailwind `text-nowrap`; css `text-wrap: nowrap;`
pub fn text_nowrap() -> Style {
    Style::new(css!("text-wrap: nowrap;")).unwrap()
}

/// Tailwind `text-balance`; css `text-wrap: balance;`
pub fn text_balance() -> Style {
    Style::new(css!("text-wrap: balance;")).unwrap()
}

/// Tailwind `text-pretty`; css `text-wrap: pretty;`
pub fn text_pretty() -> Style {
    Style::new(css!("text-wrap: pretty;")).unwrap()
}

