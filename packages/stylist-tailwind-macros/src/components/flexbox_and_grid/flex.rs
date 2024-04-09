use stylist::{css, Style};

/// Tailwind `flex-1`; css `flex: 1 1 0%;`
pub fn flex_1() -> Style {
    Style::new(css!("flex: 1 1 0%;")).unwrap()
}

/// Tailwind `flex-auto`; css `flex: 1 1 auto;`
pub fn flex_auto() -> Style {
    Style::new(css!("flex: 1 1 auto;")).unwrap()
}

/// Tailwind `flex-initial`; css `flex: 0 1 auto;`
pub fn flex_initial() -> Style {
    Style::new(css!("flex: 0 1 auto;")).unwrap()
}

/// Tailwind `flex-none`; css `flex: none;`
pub fn flex_none() -> Style {
    Style::new(css!("flex: none;")).unwrap()
}

