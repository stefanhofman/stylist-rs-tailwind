use stylist::{css, Style};

/// Tailwind `saturate-0`; css `filter: saturate(0);`
pub fn saturate_0() -> Style {
    Style::new(css!("filter: saturate(0);")).unwrap()
}

/// Tailwind `saturate-50`; css `filter: saturate(.5);`
pub fn saturate_50() -> Style {
    Style::new(css!("filter: saturate(.5);")).unwrap()
}

/// Tailwind `saturate-100`; css `filter: saturate(1);`
pub fn saturate_100() -> Style {
    Style::new(css!("filter: saturate(1);")).unwrap()
}

/// Tailwind `saturate-150`; css `filter: saturate(1.5);`
pub fn saturate_150() -> Style {
    Style::new(css!("filter: saturate(1.5);")).unwrap()
}

/// Tailwind `saturate-200`; css `filter: saturate(2);`
pub fn saturate_200() -> Style {
    Style::new(css!("filter: saturate(2);")).unwrap()
}

