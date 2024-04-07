use stylist::{css, Style};

/// Tailwind `ease-linear`; css `transition-timing-function: linear;`
pub fn ease_linear() -> Style {
    Style::new(css!("transition-timing-function: linear;")).unwrap()
}

/// Tailwind `ease-in`; css `transition-timing-function: cubic-bezier(0.4, 0, 1, 1);`
pub fn ease_in() -> Style {
    Style::new(css!("transition-timing-function: cubic-bezier(0.4, 0, 1, 1);")).unwrap()
}

/// Tailwind `ease-out`; css `transition-timing-function: cubic-bezier(0, 0, 0.2, 1);`
pub fn ease_out() -> Style {
    Style::new(css!("transition-timing-function: cubic-bezier(0, 0, 0.2, 1);")).unwrap()
}

/// Tailwind `ease-in-out`; css `transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);`
pub fn ease_in_out() -> Style {
    Style::new(css!("transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);")).unwrap()
}

