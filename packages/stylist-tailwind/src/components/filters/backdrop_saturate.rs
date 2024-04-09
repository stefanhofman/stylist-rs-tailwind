use stylist::{css, Style};

/// Tailwind `backdrop-saturate-0`; css `backdrop-filter: saturate(0);`
pub fn backdrop_saturate_0() -> Style {
    Style::new(css!("backdrop-filter: saturate(0);")).unwrap()
}

/// Tailwind `backdrop-saturate-50`; css `backdrop-filter: saturate(.5);`
pub fn backdrop_saturate_50() -> Style {
    Style::new(css!("backdrop-filter: saturate(.5);")).unwrap()
}

/// Tailwind `backdrop-saturate-100`; css `backdrop-filter: saturate(1);`
pub fn backdrop_saturate_100() -> Style {
    Style::new(css!("backdrop-filter: saturate(1);")).unwrap()
}

/// Tailwind `backdrop-saturate-150`; css `backdrop-filter: saturate(1.5);`
pub fn backdrop_saturate_150() -> Style {
    Style::new(css!("backdrop-filter: saturate(1.5);")).unwrap()
}

/// Tailwind `backdrop-saturate-200`; css `backdrop-filter: saturate(2);`
pub fn backdrop_saturate_200() -> Style {
    Style::new(css!("backdrop-filter: saturate(2);")).unwrap()
}

