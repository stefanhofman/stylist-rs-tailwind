use stylist::{css, Style};

/// Tailwind `flex-wrap`; css `flex-wrap: wrap;`
pub fn flex_wrap() -> Style {
    Style::new(css!("flex-wrap: wrap;")).unwrap()
}

/// Tailwind `flex-wrap-reverse`; css `flex-wrap: wrap-reverse;`
pub fn flex_wrap_reverse() -> Style {
    Style::new(css!("flex-wrap: wrap-reverse;")).unwrap()
}

/// Tailwind `flex-nowrap`; css `flex-wrap: nowrap;`
pub fn flex_nowrap() -> Style {
    Style::new(css!("flex-wrap: nowrap;")).unwrap()
}

