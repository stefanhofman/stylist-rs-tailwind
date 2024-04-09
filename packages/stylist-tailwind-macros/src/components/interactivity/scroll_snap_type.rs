use stylist::{css, Style};

/// Tailwind `snap-none`; css `scroll-snap-type: none;`
pub fn snap_none() -> Style {
    Style::new(css!("scroll-snap-type: none;")).unwrap()
}

/// Tailwind `snap-x`; css `scroll-snap-type: x var(--tw-scroll-snap-strictness);`
pub fn snap_x() -> Style {
    Style::new(css!("scroll-snap-type: x var(--tw-scroll-snap-strictness);")).unwrap()
}

/// Tailwind `snap-y`; css `scroll-snap-type: y var(--tw-scroll-snap-strictness);`
pub fn snap_y() -> Style {
    Style::new(css!("scroll-snap-type: y var(--tw-scroll-snap-strictness);")).unwrap()
}

/// Tailwind `snap-both`; css `scroll-snap-type: both var(--tw-scroll-snap-strictness);`
pub fn snap_both() -> Style {
    Style::new(css!("scroll-snap-type: both var(--tw-scroll-snap-strictness);")).unwrap()
}

/// Tailwind `snap-mandatory`; css `--tw-scroll-snap-strictness: mandatory;`
pub fn snap_mandatory() -> Style {
    Style::new(css!("--tw-scroll-snap-strictness: mandatory;")).unwrap()
}

/// Tailwind `snap-proximity`; css `--tw-scroll-snap-strictness: proximity;`
pub fn snap_proximity() -> Style {
    Style::new(css!("--tw-scroll-snap-strictness: proximity;")).unwrap()
}

